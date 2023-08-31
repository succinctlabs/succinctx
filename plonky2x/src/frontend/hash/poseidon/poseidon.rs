
use plonky2::{hash::{hash_types::RichField}, field::extension::Extendable, iop::target::BoolTarget};

use crate::{frontend::{builder::CircuitBuilder as Plonky2xCircuitBuilder, vars::Bytes32Variable}, prelude::{ByteVariable, BytesVariable, CircuitVariable, BoolVariable}};

/// Implements SHA256 implementation for CircuitBuilder
impl<F: RichField + Extendable<D>, const D: usize> Plonky2xCircuitBuilder<F, D> {
    pub fn poseidon(&mut self, input: &[ByteVariable]) -> Bytes32Variable {
        
        let input_bool: Vec<BoolTarget> = input
            .iter()
            .flat_map(|byte| byte.as_bool_targets().to_vec())
            .collect();


        if self.input_buffer.is_empty() {
            return;
        }

        for input_chunk in self.input_buffer.chunks(H::AlgebraicPermutation::RATE) {
            // Overwrite the first r elements with the inputs. This differs from a standard sponge,
            // where we would xor or add in the inputs. This is a well-known variant, though,
            // sometimes called "overwrite mode".
            self.sponge_state.set_from_slice(input_chunk, 0);
            self.sponge_state = builder.permute::<H>(self.sponge_state);
        }

        self.output_buffer = self.sponge_state.squeeze().to_vec();

        self.input_buffer.clear();

        let mut hash_bytes_array = [ByteVariable::init(self); 32];
        hash_bytes_array.copy_from_slice(&hash_bytes_vec);
        Bytes32Variable(BytesVariable(hash_bytes_array))
    }
}