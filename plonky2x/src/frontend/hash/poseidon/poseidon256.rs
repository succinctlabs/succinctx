
use array_macro::array;
use plonky2::{hash::{hash_types::RichField, poseidon::{SPONGE_RATE}}, field::extension::Extendable, iop::target::{Target}, plonk::config::{AlgebraicHasher}};
use plonky2::hash::hashing::PlonkyPermutation;
use crate::{frontend::{builder::CircuitBuilder as Plonky2xCircuitBuilder}, prelude::{ByteVariable, BytesVariable, CircuitVariable, BoolVariable, Variable}};

/// Implements SHA256 implementation for CircuitBuilder
impl<F: RichField + Extendable<D>, const D: usize> Plonky2xCircuitBuilder<F, D> 
{
    pub fn poseidon<H: AlgebraicHasher<F>>(&mut self, input: &[ByteVariable]) -> BytesVariable<64> {

        let zero = self.api.zero();
        let mut sponge_state = H::AlgebraicPermutation::new(std::iter::repeat(zero));
        
        let input_targets: Vec<Target> = input
            .iter()
            .flat_map(|byte| byte.targets().to_vec())
            .collect();


        for input_chunk in input_targets.chunks(SPONGE_RATE) {
            // Overwrite the first r elements with the inputs. This differs from a standard sponge,
            // where we would xor or add in the inputs. This is a well-known variant, though,
            // sometimes called "overwrite mode".
            sponge_state.set_from_slice(input_chunk, 0);
            sponge_state = self.api.permute::<H>(sponge_state);
        }

        // Each target is the size of a field element

        // We need to decompose each field element into 8 bytes

        // Each byte decomposes into 8 bits (BoolVariables)

        let output_buffer = sponge_state.squeeze().to_vec();


        println!("Output buffer: {:?}", output_buffer[0]);

        let hash_bytes_vec = output_buffer
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
        
        // hash_bytes_vec.iter().for_each(|variable| {
        //     self.watch(&variable, "c");

        // });

        // // Convert targets into ByteVariable vec
        // let output_buffer = BytesVariable::from_targets(&output_buffer);

        let mut hash_bytes_array = [ByteVariable::init(self); 64];
        hash_bytes_array.copy_from_slice(&hash_bytes_vec);

        BytesVariable(hash_bytes_array)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
    use crate::frontend::vars::Bytes32Variable;
    use crate::utils::{bytes32, setup_logger};
    use crate::{frontend::{builder::CircuitBuilder as Plonky2xCircuitBuilder}};


    use super::*;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_poseidon() -> Result<()> {
        setup_logger();

        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut builder = Plonky2xCircuitBuilder::<F, D>::new();

        let leaf = builder.constant::<Bytes32Variable>(bytes32!(
            "d68d62c262c2ec08961c1104188cde86f51695878759666ad61490c8ec66745c"
        ));

        // Convert Bytes32Variable to array of ByteVariable
        let leaf_bytes = leaf.as_bytes();

        let hash = builder.poseidon::<<plonky2::plonk::config::PoseidonGoldilocksConfig as plonky2::plonk::config::GenericConfig<D>>::InnerHasher>(&leaf_bytes);

        builder.watch(&hash, "hash");

        builder.write(hash);

        // Build your circuit.
        let circuit = builder.build::<PoseidonGoldilocksConfig>();

        // Write to the circuit input.
        let input = circuit.input();

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output.
        let sum = output.read::<Variable>();
        println!("{}", sum.0);

        // println!("Hash: {:?}", hash);

        Ok(())
    }
}