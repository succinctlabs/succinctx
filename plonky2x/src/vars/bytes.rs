use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};
use plonky2::field::types::Field;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};

use super::{Variable, BoolVariable};
use crate::utils::{le_bits_to_bytes, bytes_to_bits};

#[derive(Debug, Clone, Copy)]
pub struct BytesVariable<const N: usize>(pub [BoolVariable;N]);

pub trait VariableMethods<const N: usize> {
    fn get_bits<F: Field, W: Witness<F>>(&self, witness: W) -> [bool; N];
    fn get_bytes_le<F: Field, W: Witness<F>>(&self, witness: W) -> [u8; N / 8];
    fn set_from_bits<F: Field, G: WitnessWrite<F>>(&self, values: [bool; N], out_buffer: &mut G);
    fn set_from_bytes<F: Field, G: WitnessWrite<F>>(&self, values: [u8; N / 8], out_buffer: &mut G);
}

impl<const N: usize> VariableMethods<N> for BytesVariable<N> {
    fn get_bits<F: Field, W: Witness<F>>(&self, witness: W) -> [bool; N] {
        self.0.iter()
            .map(|variable| witness.get_target(variable.0.0) == F::ONE)
            .collect::<Vec<bool>>()
            .try_into()
            .unwrap()
    }

    fn get_bytes_le<F: Field, W: Witness<F>>(&self, witness: W) -> [u8; N / 8] {
        le_bits_to_bytes::<N>(self.get_bits(witness))
    }

    fn set_from_bits<F: Field, G: WitnessWrite<F>>(&self, values: [bool; N], out_buffer: &mut G) {
        for i in 0..N {
            let a = if values[i] { F::ONE } else { F::ZERO };
            out_buffer.set_target(self.0[i].0.0, a);
        }
    }

    fn set_from_bytes<F: Field, G: WitnessWrite<F>>(&self, values: [u8; N / 8], out_buffer: &mut G) {
        self.set_from_bits(bytes_to_bits::<N>(values), out_buffer)
    }
}

// pub mod macros {
//     use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};
//     use plonky2::field::types::Field;
//     use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};

//     #[macro_export]
//     macro_rules! impl_variable_methods {
//         ($struct_name:ident, $size:expr) => {
//             impl $struct_name {
//                 pub fn get_bits<F: Field, W: Witness<F>>(&self, witness: W) -> [bool; $size] {
//                     self.0.iter()
//                         .map(|variable| witness.get_target(variable.0.0) == F::ONE)
//                         .collect::<Vec<bool>>()
//                         .try_into()
//                         .unwrap()
//                 }

//                 pub fn get_bytes_le<F: Field, W: Witness<F>>(&self, witness: W) -> [u8; $size / 8] {
//                     le_bits_to_bytes::<$size>(self.get_bits(witness))
//                 }

//                 pub fn set_from_bits<F: Field, G: WitnessWrite<F>>(&self, values: [bool; $size], out_buffer: &mut G) {
//                     for i in 0..$size {
//                         let a = if values[i] { F::ONE } else { F::ZERO };
//                         out_buffer.set_target(self.0[i].0.0, a);
//                     }
//                 }

//                 pub fn set_from_bytes<F: Field, G: WitnessWrite<F>>(&self, values: [u8; $size / 8], out_buffer: &mut G) {
//                     self.set_from_bits(bytes_to_bits::<$size>(values), out_buffer)
//                 }
//             }
//         };
//     }
// }