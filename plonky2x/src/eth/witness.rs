use ethers::types::Address;
use plonky2::field::types::Field;
use plonky2::iop::generator::GeneratedValues;

use super::vars::{AddressVariable, BLSPubkeyVariable};
use crate::ethutils::BLSPubkey;
use crate::vars::WriteableWitness;

pub trait EthWriteableWitness<F: Field>: WriteableWitness<F> {
    fn set_bls_pubkey(&mut self, variable: BLSPubkeyVariable, value: BLSPubkey) {}
    fn set_address(&mut self, variable: AddressVariable, address: Address) {}
}

impl<F: Field> EthWriteableWitness<F> for GeneratedValues<F> {
    fn set_bls_pubkey(&mut self, variable: BLSPubkeyVariable, value: BLSPubkey) {
        self.set_from_bytes_be(variable.0, value.0);
    }

    fn set_address(&mut self, variable: AddressVariable, address: Address) {
        self.set_from_bytes_be(variable.0, address.0);
    }
}
