use plonky2::field::types::Field;
use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::witness::WitnessWrite;

use crate::ethutils::{Address, BLSPubkey};
use crate::vars::{BoolVariable, BytesVariable, WitnessWriteMethods};

#[derive(Debug, Clone, Copy)]
pub struct BLSPubkeyVariable(pub [BoolVariable; 384]);

#[derive(Debug, Clone, Copy)]
pub struct AddressVariable(pub [BoolVariable; 160]);

impl From<AddressVariable> for BytesVariable<20> {
    fn from(value: AddressVariable) -> Self {
        let mut result: [[BoolVariable; 8]; 20] = Default::default();

        for (i, item) in value.0.iter().enumerate() {
            result[i / 8][i % 8] = *item;
        }

        BytesVariable::<20>(result)
    }
}

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
