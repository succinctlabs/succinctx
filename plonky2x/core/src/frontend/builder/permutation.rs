use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use serde::{Deserialize, Serialize};

use super::CircuitBuilder;
use crate::frontend::extension::CubicExtensionVariable;
use crate::frontend::hint::simple::hint::Hint;
use crate::frontend::uint::uint32::U32Variable;
use crate::frontend::vars::{EvmVariable, ValueStream, VariableStream};
use crate::prelude::{ArrayVariable, PlonkParameters};
use crate::utils::hash::sha256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomPermutationHint<const B: usize>;

impl<L: PlonkParameters<D>, const D: usize, const B: usize> Hint<L, D>
    for RandomPermutationHint<B>
{
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let inputs = input_stream.read_value::<ArrayVariable<U32Variable, B>>();
        let dummy = input_stream.read_value::<U32Variable>();
        let seed = input_stream.read_value::<U32Variable>();

        let mut filtered_inputs = Vec::new();
        for i in 0..inputs.len() {
            if inputs[i] != dummy {
                filtered_inputs.push(inputs[i]);
            }
        }

        filtered_inputs.sort_by_key(|x| {
            let mut bytes = Vec::new();
            bytes.extend(x.to_be_bytes());
            bytes.extend(seed.to_be_bytes());
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
        gamma: CubicExtensionVariable,
        seed: U32Variable,
    ) -> ArrayVariable<U32Variable, B>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        // Compute the filtered accumulator.
        let mut filtered_acc = self.one::<CubicExtensionVariable>();
        for i in 0..inputs.len() {
            let is_dummy = self.is_equal(inputs[i], dummy);
            let input_extension = inputs[i].variable.as_cubic_extension(self);
            let term = self.sub(gamma, input_extension);
            let acc = self.mul(filtered_acc, term);
            filtered_acc = self.select(is_dummy, filtered_acc, acc);
        }

        // Get the permuted inputs.
        let mut input_stream = VariableStream::new();
        input_stream.write(&inputs);
        input_stream.write(&dummy);
        input_stream.write(&seed);
        let output_stream = self.hint(input_stream, RandomPermutationHint::<B> {});
        let permuted_inputs = output_stream.read::<ArrayVariable<U32Variable, B>>(self);

        // Compute the permuted filtered accumulator.
        let mut permuted_filtered_acc = self.one::<CubicExtensionVariable>();
        for i in 0..inputs.len() {
            let is_dummy = self.is_equal(permuted_inputs[i], dummy);
            let permuted_input_extension = permuted_inputs[i].variable.as_cubic_extension(self);
            let term = self.sub(gamma, permuted_input_extension);
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
            bytes.extend(seed.encode(self));
            let h = self.curta_sha256(&bytes);
            let metric = U32Variable::decode(self, &h.0[0..4]);
            metrics.push(metric);
        }

        let t = self._true();
        let f = self._false();
        let mut seen_dummy = self._false();
        for i in 0..metrics.len() {
            // If we have seen a dummy but the current one is not, panic.
            let current_is_dummy = self.is_equal(permuted_inputs[i], dummy);
            let not_current_is_dummy = self.not(current_is_dummy);
            let seen_dummy_and_not_current_is_dummy = self.and(seen_dummy, not_current_is_dummy);
            self.assert_is_equal(seen_dummy_and_not_current_is_dummy, f);

            // If the current value is not a dummy, then the current metric should be greater than
            // or equal to the previous metric.
            if i > 0 {
                let gte = self.gte(metrics[i], metrics[i - 1]);
                let not_gte = self.not(gte);
                let not_gte_and_not_current_is_dummy = self.and(not_gte, not_current_is_dummy);
                self.assert_is_equal(not_gte_and_not_current_is_dummy, f);
            }

            // If the next thing is a dummy, we've seen a dummy.
            seen_dummy = self.select(current_is_dummy, t, seen_dummy);
        }

        permuted_inputs
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use crate::frontend::extension::CubicExtensionVariable;
    use crate::prelude::*;
    use crate::utils;

    #[test]
    fn test_simple_circuit_with_field_io() {
        utils::setup_logger();
        let mut builder = DefaultBuilder::new();

        let inputs = builder.constant::<ArrayVariable<U32Variable, 5>>(vec![0, 1, 2, 3, 4]);
        let dummy = builder.constant::<U32Variable>(0);
        let gamma = builder.constant::<CubicExtensionVariable>(CubicElement([
            GoldilocksField::ONE,
            GoldilocksField::ZERO,
            GoldilocksField::ZERO,
        ]));
        let seed = builder.constant::<U32Variable>(1);

        let permuted_inputs = builder.permute_with_dummy(inputs, dummy, gamma, seed);
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
