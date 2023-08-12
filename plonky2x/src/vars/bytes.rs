use plonky2::field::types::Field;
use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartialWitness, PartitionWitness, Witness, WitnessWrite};

use super::BoolVariable;
use crate::utils::{byte_to_bits_le, le_bits_to_bytes};

// BytesVariable stores the underlying BoolVariable slice as "LE"
#[derive(Debug, Clone, Copy)]
pub struct BytesVariable<const N: usize>(pub [[BoolVariable; 8]; N]);

impl<const N: usize> BytesVariable<N> {
    pub fn to_targets(&self) -> Vec<Target> {
        self.0
            .iter()
            .flat_map(|byte| byte.iter().map(|bit| bit.0 .0))
            .collect::<Vec<Target>>()
    }
}

pub trait WitnessMethods<F: Field>: Witness<F> {
    fn get_hex_string<const N: usize>(&self, bytes: BytesVariable<N>) -> String;
    fn get_bits_le<const N: usize>(&self, bytes: BytesVariable<N>) -> Vec<bool>;
    fn get_bits_be<const N: usize>(&self, bytes: BytesVariable<N>) -> Vec<bool>;
    fn get_bytes_le<const N: usize>(&self, bytes: BytesVariable<N>) -> [u8; N];
    fn get_bytes_be<const N: usize>(&self, bytes: BytesVariable<N>) -> [u8; N];
    // fn get_bits_le_as_fixed<const N: usize, const M: usize>(&self, bytes: BytesVariable<N>) -> [bool; M];
}

pub trait WitnessWriteMethods<F: Field>: WitnessWrite<F> {
    fn set_from_bits_le<const N: usize>(&mut self, bytes: BytesVariable<N>, values: &[bool]);
    fn set_from_bits_be<const N: usize>(&mut self, bytes: BytesVariable<N>, values: &[bool]);
    fn set_from_bytes_le<const N: usize>(&mut self, bytes: BytesVariable<N>, values: [u8; N]);
    fn set_from_bytes_be<const N: usize>(&mut self, bytes: BytesVariable<N>, values: [u8; N]);
}

impl<'a, F: Field> WitnessMethods<F> for PartitionWitness<'a, F> {
    fn get_hex_string<const N: usize>(&self, bytes: BytesVariable<N>) -> String {
        let bytes = self.get_bytes_be(bytes);
        format!("0x{}", hex::encode(bytes))
    }

    fn get_bits_le<const N: usize>(&self, bytes: BytesVariable<N>) -> Vec<bool> {
        bytes
            .0
            .iter()
            .flat_map(|byte| byte.iter().map(|bit| self.get_target(bit.0 .0) == F::ONE))
            .collect::<Vec<bool>>()
    }

    fn get_bits_be<const N: usize>(&self, bytes: BytesVariable<N>) -> Vec<bool> {
        let mut bits = self.get_bits_le(bytes);
        bits.reverse();
        bits
    }

    fn get_bytes_le<const N: usize>(&self, bytes: BytesVariable<N>) -> [u8; N] {
        let bits = self.get_bits_le(bytes);
        le_bits_to_bytes::<N>(bits.as_slice())
    }

    fn get_bytes_be<const N: usize>(&self, bytes: BytesVariable<N>) -> [u8; N] {
        let mut bytes = self.get_bytes_le(bytes);
        bytes.reverse();
        bytes
    }
}

impl<F: Field> WitnessMethods<F> for PartialWitness<F> {
    fn get_hex_string<const N: usize>(&self, bytes: BytesVariable<N>) -> String {
        let bytes = self.get_bytes_be(bytes);
        format!("0x{}", hex::encode(bytes))
    }

    fn get_bits_le<const N: usize>(&self, bytes: BytesVariable<N>) -> Vec<bool> {
        bytes
            .0
            .iter()
            .flat_map(|byte| {
                byte.iter()
                    .map(|bit| self.try_get_target(bit.0 .0).unwrap() == F::ONE)
            })
            .collect::<Vec<bool>>()
    }

    fn get_bits_be<const N: usize>(&self, bytes: BytesVariable<N>) -> Vec<bool> {
        let mut bits = self.get_bits_le(bytes);
        bits.reverse();
        bits
    }

    fn get_bytes_le<const N: usize>(&self, bytes: BytesVariable<N>) -> [u8; N] {
        let bits = self.get_bits_le(bytes);
        le_bits_to_bytes::<N>(bits.as_slice())
    }

    fn get_bytes_be<const N: usize>(&self, bytes: BytesVariable<N>) -> [u8; N] {
        let mut bytes = self.get_bytes_le(bytes);
        bytes.reverse();
        bytes
    }
}

impl<'a, F: Field> WitnessWriteMethods<F> for GeneratedValues<F> {
    fn set_from_bits_le<const N: usize>(&mut self, bytes: BytesVariable<N>, values: &[bool]) {
        for i in 0..N {
            for j in 0..8 {
                let a = if values[i * 8 + j] { F::ONE } else { F::ZERO };
                self.set_target(bytes.0[i][j].0 .0, a);
            }
        }
    }

    fn set_from_bits_be<const N: usize>(&mut self, bytes: BytesVariable<N>, values: &[bool]) {
        let mut reversed: Vec<bool> = vec![];
        for i in values.len()..0 {
            reversed.push(values[i])
        }
        self.set_from_bits_le(bytes, &reversed)
    }

    fn set_from_bytes_le<const N: usize>(&mut self, bytes: BytesVariable<N>, values: [u8; N]) {
        let mut bits: Vec<bool> = vec![];
        for i in 0..N {
            bits.extend_from_slice(&byte_to_bits_le(values[i]))
        }
        self.set_from_bits_le(bytes, &bits)
    }

    fn set_from_bytes_be<const N: usize>(&mut self, bytes: BytesVariable<N>, values: [u8; N]) {
        let reversed: [u8; N] = values
            .iter()
            .cloned()
            .rev()
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        self.set_from_bytes_le(bytes, reversed)
    }
}

// TODO make this a macro with the above
impl<'a, F: Field> WitnessWriteMethods<F> for PartialWitness<F> {
    fn set_from_bits_le<const N: usize>(&mut self, bytes: BytesVariable<N>, values: &[bool]) {
        for i in 0..N {
            for j in 0..8 {
                let a = if values[i * 8 + j] { F::ONE } else { F::ZERO };
                self.set_target(bytes.0[i][j].0 .0, a);
            }
        }
    }

    fn set_from_bits_be<const N: usize>(&mut self, bytes: BytesVariable<N>, values: &[bool]) {
        let mut reversed: Vec<bool> = vec![];
        for i in values.len()..0 {
            reversed.push(values[i])
        }
        self.set_from_bits_le(bytes, &reversed)
    }

    fn set_from_bytes_le<const N: usize>(&mut self, bytes: BytesVariable<N>, values: [u8; N]) {
        let mut bits: Vec<bool> = vec![];
        for i in 0..N {
            bits.extend_from_slice(&byte_to_bits_le(values[i]))
        }
        self.set_from_bits_le(bytes, &bits)
    }

    fn set_from_bytes_be<const N: usize>(&mut self, bytes: BytesVariable<N>, values: [u8; N]) {
        let reversed: [u8; N] = values
            .iter()
            .cloned()
            .rev()
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        self.set_from_bytes_le(bytes, reversed)
    }
}

#[cfg(test)]
mod tests {

    use ethers::types::{Address, H256};
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::field::types::Field;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

    use super::*;
    use crate::builder::BuilderAPI;

    #[test]
    fn test_set_from_bytes_be() {
        let mut api = BuilderAPI::new();
        let bytes_var = api.init_bytes32();

        let sample = "0xff90251f501c864f21d696c811af4c3aa987006916bd0e31a6c06cc612e7632e"
            .parse::<H256>()
            .unwrap();
        let sample_bytes32 = sample.as_fixed_bytes();

        let mut pw = PartialWitness::<GoldilocksField>::new();
        pw.set_from_bytes_be(bytes_var.into(), *sample_bytes32);

        let retrieved = pw.get_bytes_be(bytes_var.into());
        assert_eq!(*sample_bytes32, retrieved);
    }
}
