use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use serde::{Deserialize, Serialize};

use super::CircuitBuilder;
use crate::frontend::hint::simple::hint::Hint;
use crate::frontend::uint::uint32::U32Variable;
use crate::frontend::vars::{EvmVariable, ValueStream, VariableStream};
use crate::prelude::{ArrayVariable, PlonkParameters, Variable};
use crate::utils::hash::sha256;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RandomPermutationHint<const B: usize>;

impl<L: PlonkParameters<D>, const D: usize, const B: usize> Hint<L, D>
    for RandomPermutationHint<B>
{
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let inputs = input_stream.read_value::<ArrayVariable<U32Variable, B>>();
        let dummy = input_stream.read_value::<U32Variable>();
        let nonce = input_stream.read_value::<U32Variable>();

        let mut filtered_inputs = Vec::new();
        for i in 0..inputs.len() {
            if inputs[i] != dummy {
                filtered_inputs.push(inputs[i]);
            }
        }

        filtered_inputs.sort_by_key(|x| {
            let mut bytes = Vec::new();
            bytes.extend(x.to_be_bytes());
            bytes.extend(nonce.to_be_bytes());
            u32::from_be_bytes(sha256(&bytes)[0..4].try_into().unwrap())
        });
        filtered_inputs.resize(B, dummy);

        output_stream.write_value::<ArrayVariable<U32Variable, B>>(filtered_inputs);
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    // @no-audit-okay
    pub fn permute_with_dummy<const B: usize>(
        &mut self,
        inputs: ArrayVariable<U32Variable, B>,
        dummy: U32Variable,
        gamma: Variable,
        nonce: U32Variable,
    ) -> ArrayVariable<U32Variable, B>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        // Compute the filtered accumulator.
        let mut filtered_acc = self.one::<Variable>();
        for i in 0..inputs.len() {
            let is_dummy = self.is_equal(inputs[i], dummy);
            let term = self.sub(gamma, inputs[i].0);
            let acc = self.mul(filtered_acc, term);
            filtered_acc = self.select(is_dummy, filtered_acc, acc);
        }

        // Get the permuted inputs.
        let mut input_stream = VariableStream::new();
        input_stream.write(&inputs);
        input_stream.write(&dummy);
        input_stream.write(&nonce);
        let output_stream = self.hint(input_stream, RandomPermutationHint::<B> {});
        let permuted_inputs = output_stream.read::<ArrayVariable<U32Variable, B>>(self);

        // Compute the permtued filtered accumulator.
        let mut permuted_filtered_acc = self.one::<Variable>();
        for i in 0..inputs.len() {
            let is_dummy = self.is_equal(permuted_inputs[i], dummy);
            let term = self.sub(gamma, permuted_inputs[i].0);
            let acc = self.mul(permuted_filtered_acc, term);
            permuted_filtered_acc = self.select(is_dummy, permuted_filtered_acc, acc);
        }

        // Assert that the permuted filtered accumulator is the same as the filtered accumulator.
        self.assert_is_equal(permuted_filtered_acc, filtered_acc);

        // Check the metric ordering.
        let mut metrics = Vec::new();
        for i in 0..permuted_inputs.len() {
            let mut bytes = Vec::new();
            bytes.extend(permuted_inputs[i].encode(self));
            bytes.extend(nonce.encode(self));
            let h = self.curta_sha256(&bytes);
            let metric = U32Variable::decode(self, &h.0[0..4]);
            metrics.push(metric);
        }

        let t = self._true();
        let f = self._false();
        let mut seen_dummy = self._false();
        for i in 0..metrics.len() - 1 {
            // If the next is dummy and we've seen one, panic.
            let next_is_dummy = self.is_equal(permuted_inputs[i + 1], dummy);
            let not_next_is_dummy = self.not(next_is_dummy);
            let seen_dummy_and_not_next_is_dummy = self.and(seen_dummy, not_next_is_dummy);
            self.assert_is_equal(seen_dummy_and_not_next_is_dummy, f);

            // The next metric should be less than or equal to or the next is valid.
            let lte = self.lte(metrics[i], metrics[i + 1]);
            let valid = self.or(lte, next_is_dummy);
            self.assert_is_equal(valid, t);

            // If the next thing is a dummy, we've seen a dummy.
            seen_dummy = self.select(next_is_dummy, t, seen_dummy);
        }

        permuted_inputs
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use plonky2::field::types::Field;

    use crate::frontend::uint::uint32::U32Variable;
    use crate::prelude::*;
    use crate::utils;

    #[test]
    fn test_simple_circuit_with_field_io() {
        utils::setup_logger();
        let mut builder = DefaultBuilder::new();

        let inputs = builder.constant::<ArrayVariable<U32Variable, 5>>(vec![0, 1, 2, 3, 4]);
        let dummy = builder.constant::<U32Variable>(0);
        let gamma = builder.constant::<Variable>(GoldilocksField::from_canonical_u64(3));
        let nonce = builder.constant::<U32Variable>(1);

        let permuted_inputs = builder.permute_with_dummy(inputs, dummy, gamma, nonce);
        for i in 0..permuted_inputs.len() {
            builder.watch(
                &permuted_inputs[i],
                format!("permuted_inputs[{}]", i).as_str(),
            );
        }

        let circuit = builder.build();

        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }
}
