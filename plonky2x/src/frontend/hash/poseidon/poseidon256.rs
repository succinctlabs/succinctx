use array_macro::array;
use plonky2::iop::target::BoolTarget;
use plonky2::plonk::config::AlgebraicHasher;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::Bytes32Variable;
use crate::prelude::{BoolVariable, ByteVariable, BytesVariable, CircuitVariable};

/// Implements the Poseidon hash for CircuitBuilder.
impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Note: This Poseidon implementation operates on bytes, not field elements. The input bytes to
    /// the Poseidon hash are converted into field elements internally. Specifically, we convert the
    /// [ByteVariable; N] into a [u32; N/4] and then represent the u32 as a [F; N/4]. We use u32's
    /// instead of u64's to represent the bytes because of the Goldilocks field size.
    pub fn poseidon<H: AlgebraicHasher<L::Field>>(
        &mut self,
        input: &[ByteVariable],
    ) -> Bytes32Variable {
        let input_targets: Vec<BoolTarget> = input
            .iter()
            .flat_map(|byte| byte.as_bool_targets().to_vec())
            .collect();

        // Call le_sum on chunks of 32 bits (4 byte targets) from input_targets.
        let inputs = input_targets
            .chunks(32)
            .map(|chunk| self.api.le_sum(chunk.iter()))
            .collect::<Vec<_>>();

        let hash = self.api.hash_n_to_hash_no_pad::<H>(inputs);

        // Convert each field element (~64 bits) into 8 bytes.
        let hash_bytes_vec = hash
            .elements
            .iter()
            .flat_map(|chunk| {
                let bit_list = self.api.split_le(*chunk, 64);

                let hash_byte_vec = bit_list
                    .chunks(8)
                    .map(|chunk| ByteVariable(array![i => BoolVariable::from(chunk[i].target); 8]))
                    .collect::<Vec<_>>();

                hash_byte_vec
            })
            .collect::<Vec<_>>();

        let mut hash_bytes_array = [ByteVariable::init_unsafe(self); 32];
        hash_bytes_array.copy_from_slice(&hash_bytes_vec);

        Bytes32Variable(BytesVariable(hash_bytes_array))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use plonky2::plonk::config::GenericConfig;

    use crate::backend::circuit::{DefaultParameters, PlonkParameters};
    use crate::frontend::vars::Bytes32Variable;
    use crate::prelude::CircuitBuilder;
    use crate::utils::{self, bytes32};

    #[test]
    fn test_poseidon() -> Result<()> {
        utils::setup_logger();

        type L = DefaultParameters;
        const D: usize = 2;
        type H = <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::InnerHasher;
        let mut builder = CircuitBuilder::<L, D>::new();

        let leaf = builder.constant::<Bytes32Variable>(bytes32!(
            "d68d62c262c2ec08961c1104188cde86f51695878759666ad61490c8ec66745c"
        ));

        let expected_hash = builder.constant::<Bytes32Variable>(bytes32!(
            "faa1095f1959da5713d6ad8b21b54936f167dc8e3f205b129b8eb8740aa10c0b"
        ));

        // Convert Bytes32Variable to array of ByteVariable
        let leaf_bytes = leaf.as_bytes();

        let computed_hash = builder.poseidon::<H>(&leaf_bytes);

        builder.assert_is_equal(computed_hash, expected_hash);

        builder.watch(&computed_hash, "computed_hash");

        // Build your circuit.
        let circuit = builder.build();

        // Write to the circuit input.
        let input = circuit.input();

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        Ok(())
    }
}
