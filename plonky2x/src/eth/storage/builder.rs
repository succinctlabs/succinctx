use ethers::providers::{JsonRpcClient, Provider};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use super::generator::EthStorageProofGenerator;
use crate::builder::CircuitBuilder;
use crate::eth::vars::AddressVariable;
use crate::vars::Bytes32Variable;

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn get_eth_storage_slot_at<P: Clone + JsonRpcClient + 'static>(
        &mut self,
        provider: Provider<P>,
        address: AddressVariable,
        storage_key: Bytes32Variable,
        block_number: u64,
    ) -> Bytes32Variable {
        let generator =
            EthStorageProofGenerator::new(self, provider, address, storage_key, block_number);
        self.api.add_simple_generator(generator.clone());
        generator.value
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use ethers::providers::Http;
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;

    use super::*;
    use crate::utils::{address, bytes32};
    use crate::vars::CircuitVariable;

    #[test]
    fn test_get_storage_at_location() {
        dotenv::dotenv().ok();

        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let rpc_url = env::var("EXECUTION_RPC_URL").unwrap();
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();

        let mut builder = CircuitBuilder::<F, D>::new();
        let state_root = builder.init::<Bytes32Variable>();
        let address = builder.init::<AddressVariable>();
        let location = builder.init::<Bytes32Variable>();
        let claimed_value = builder.init::<Bytes32Variable>();
        let block_number = 17880427u64;

        let value = builder.get_eth_storage_slot_at(provider, address, location, block_number);

        let mut pw = PartialWitness::new();
        state_root.set(
            &mut pw,
            bytes32!("0xff90251f501c864f21d696c811af4c3aa987006916bd0e31a6c06cc612e7632e"),
        );
        address.set(
            &mut pw,
            address!("0x55032650b14df07b85bF18A3a3eC8E0Af2e028d5"),
        );
        location.set(
            &mut pw,
            bytes32!("0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5"),
        );
        claimed_value.set(
            &mut pw,
            bytes32!("0x0000000000000000000000dd4bc51496dc93a0c47008e820e0d80745476f2201"),
        );

        println!("Building circuit");
        let circuit = builder.build::<C>();
        println!("Proving circuit");
        println!("Address {:?}", address);
        println!("Address in witness {:?}", address.get(&pw));
        println!("{:?}", value);
        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }
}
