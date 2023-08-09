use core::str::Bytes;

use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};
use plonky2::field::types::Field;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};

use super::{Variable, BoolVariable};
use crate::builder::BuilderAPI;
use crate::utils::{le_bits_to_bytes, bytes_to_bits};

#[derive(Debug, Clone, Copy)]
pub struct BytesVariable<const N: usize>(pub [BoolVariable;N]); // TODO make this private


pub trait WitnessMethods<F: Field>: Witness<F> {
    fn get_bits_le<const N: usize>(&self, bytes: BytesVariable<N>) -> [bool; N];
    fn get_bits_be<const N: usize>(&self, bytes: BytesVariable<N>) -> [bool; N];
    fn get_bytes_le<const N: usize>(&self, bytes: BytesVariable<N>) -> [bool; N];
    fn get_bytes_be<const N: usize>(&self, bytes: BytesVariable<N>) -> [bool; N];
}

pub trait WitnessWriteMethods<F: Field>: WitnessWrite<F> {
    fn set_from_bits_le<const N: usize>(&mut self, bytes: BytesVariable<N>, values: [bool; N]);
    fn set_from_bits_be<const N: usize>(&mut self, bytes: BytesVariable<N>, values: [bool; N]);
    fn set_from_bytes_le<const N: usize>(&mut self, bytes: BytesVariable<N>, values: [bool; N]);
    fn set_from_bytes_be<const N: usize>(&mut self, bytes: BytesVariable<N>, values: [bool; N]);
}

impl<'a, F: Field> WitnessMethods<F> for PartitionWitness<'a, F> {
    fn get_bits_le<const N: usize>(&self, bytes: BytesVariable<N>) -> [bool; N] {
        bytes.0.iter()
        .map(|variable| self.get_target(variable.0.0) == F::ONE)
        .collect::<Vec<bool>>()
        .try_into()
        .unwrap()
    }

    fn get_bits_be<const N: usize>(&self, bytes: BytesVariable<N>) -> [bool; N] {
        todo!()
    }

    fn get_bytes_be<const N: usize>(&self, bytes: BytesVariable<N>) -> [bool; N] {
        todo!()
    }

    fn get_bytes_le<const N: usize>(&self, bytes: BytesVariable<N>) -> [bool; N] {
        todo!()
    }
}

impl<'a, F:Field> WitnessWriteMethods<F> for GeneratedValues<F> {
    fn set_from_bits_le<const N: usize>(&mut self, bytes: BytesVariable<N>, values: [bool; N]) {
        for i in 0..N {
            let a = if values[i] { F::ONE } else { F::ZERO };
            self.set_target(bytes.0[i].0.0, a);
        }
    }

    fn set_from_bits_be<const N: usize>(&mut self, bytes: BytesVariable<N>, values: [bool; N]) {
        todo!()
    }

    fn set_from_bytes_be<const N: usize>(&mut self, bytes: BytesVariable<N>, values: [bool; N]) {
        todo!()
    }

    fn set_from_bytes_le<const N: usize>(&mut self, bytes: BytesVariable<N>, values: [bool; N]) {
        todo!()
    }
}
