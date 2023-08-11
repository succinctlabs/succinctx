use plonky2::field::types::Field;
use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};

use super::BytesVariable;
use crate::utils::le_bits_to_bytes;

// pub trait Experiment<F: Field>: Witness<F> {
//     fn get_bits_le<T: Variable>(&self, variable: T) -> Vec<bool>;
//     fn get_bits_be<T: Variable>(&self, variable: T) -> Vec<bool>;
//     fn get_bytes_le<T: Variable>(&self, variable: T) -> Vec<u8>;
//     fn get_bytes_be<T: Variable>(&self, variable: T) -> Vec<u8>;

//     fn set_bits_le<T: Variable>(&mut self, variable: T, values: &[bool]);
//     fn set_bits_be<T: Variable>(&mut self, variable: T, values: &[bool]);
//     fn set_bytes_le<T: Variable>(&mut self, variable: T, values: &[u8]);
//     fn set_bytes_be<T: Variable>(&mut self, variable: T, values: &[u8]);
// }

pub trait ReadableWitness<F: Field>: Witness<F> {
    fn get_bits_le<const N: usize>(&self, bytes: BytesVariable<N>) -> Vec<bool>;
    fn get_bits_be<const N: usize>(&self, bytes: BytesVariable<N>) -> Vec<bool>;
    fn get_bytes_le<const N: usize>(&self, bytes: BytesVariable<N>) -> [u8; N];
    fn get_bytes_be<const N: usize>(&self, bytes: BytesVariable<N>) -> [u8; N];
    fn get_hex_string<const N: usize>(&self, bytes: BytesVariable<N>) -> String;
}

pub trait WriteableWitness<F: Field>: WitnessWrite<F> {
    fn set_from_bits_le<const N: usize>(&mut self, bytes: BytesVariable<N>, values: &[bool]);
    fn set_from_bits_be<const N: usize>(&mut self, bytes: BytesVariable<N>, values: &[bool]);
    fn set_from_bytes_le<const N: usize>(&mut self, bytes: BytesVariable<N>, values: [u8; N]);
    fn set_from_bytes_be<const N: usize>(&mut self, bytes: BytesVariable<N>, values: [u8; N]);
}

impl<'a, F: Field> ReadableWitness<F> for PartitionWitness<'a, F> {
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

    fn get_hex_string<const N: usize>(&self, bytes: BytesVariable<N>) -> String {
        let bytes = self.get_bytes_be(bytes);
        hex::encode(bytes)
    }
}

impl<'a, F: Field> WriteableWitness<F> for GeneratedValues<F> {
    fn set_from_bits_le<const N: usize>(&mut self, bytes: BytesVariable<N>, values: &[bool]) {
        for i in 0..N / 8 {
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
            bits.extend_from_slice(&{
                let input = values[i];
                let mut bits = [false; 8];
                for i in 0..8 {
                    bits[i] = (input & (1 << i)) != 0;
                }
                bits
            })
        }
        self.set_from_bits_le(bytes, &bits)
    }

    fn set_from_bytes_be<const N: usize>(&mut self, bytes: BytesVariable<N>, values: [u8; N]) {
        self.set_from_bytes_le(bytes, values)
    }
}
