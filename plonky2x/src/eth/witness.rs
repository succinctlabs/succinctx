use plonky2::field::types::Field;
use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::witness::WitnessWrite;

use super::types::{AddressVariable, BLSPubkeyVariable};
use crate::ethutils::{Address, BLSPubkey};
use crate::vars::WitnessWriteMethods;

pub trait EthWriteableWitness<F: Field>: WitnessWriteMethods<F> {
    fn set_bls_pubkey(&mut self, variable: BLSPubkeyVariable, value: BLSPubkey) {}
    fn set_address(&mut self, variable: AddressVariable, address: Address) {}
}

impl<F: Field> EthWriteableWitness<F> for GeneratedValues<F> {
    fn set_bls_pubkey(&mut self, variable: BLSPubkeyVariable, value: BLSPubkey) {
        for i in 0..384 {
            let v = if value.0[i] { F::ONE } else { F::ZERO };
            self.set_target(variable.0[i].0 .0, v);
        }
    }

    fn set_address(&mut self, variable: AddressVariable, value: Address) {
        for i in 0..160 {
            let v = if value.0[i] { F::ONE } else { F::ZERO };
            self.set_target(variable.0[i].0 .0, v);
        }
    }
}
