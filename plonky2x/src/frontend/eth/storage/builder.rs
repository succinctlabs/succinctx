use ethers::providers::{JsonRpcClient, Provider};
use ethers::types::{Address, U256};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use super::generators::storage::EthStorageProofGenerator;
use super::vars::{EthAccountVariable, EthHeaderVariable, EthLogVariable};
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::vars::AddressVariable;
use crate::frontend::vars::Bytes32Variable;

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn get_storage_key_at(
        &mut self,
        mapping_location: U256,
        map_key: Bytes32Variable,
    ) -> Bytes32Variable {
        todo!();
    }

    pub fn eth_getStorageAt(
        &mut self,
        address: AddressVariable,
        storage_key: Bytes32Variable,
        block_hash: Bytes32Variable,
    ) -> Bytes32Variable {
        let generator = EthStorageProofGenerator::new(self, address, storage_key, block_hash);
        generator.value
    }

    pub fn eth_getBlockByHash(&mut self, block_hash: Bytes32Variable) -> EthHeaderVariable {
        todo!()
    }

    pub fn eth_getAccount(
        &mut self,
        address: Address,
        block_hash: Bytes32Variable,
    ) -> EthAccountVariable {
        todo!()
    }

    pub fn eth_getTransactionReceipt(
        &mut self,
        transaction_hash: Bytes32Variable,
        block_hash: Bytes32Variable,
        log_index: usize,
    ) -> EthLogVariable {
        todo!()
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
    use crate::prelude::{CircuitBuilderX, CircuitVariable};
    use crate::utils::{address, bytes32};

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_eth_getStorageAt() {
        dotenv::dotenv().ok();
        let rpc_url = env::var("RPC_1").unwrap();
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();

        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        // This is the circuit definition
        let mut builder = CircuitBuilderX::new();
        builder.set_execution_client(provider);
        let block_hash = builder.read::<Bytes32Variable>();
        let address = builder.read::<AddressVariable>();
        let location = builder.read::<Bytes32Variable>();
        let value = builder.eth_getStorageAt(address, location, block_hash);
        builder.write(value);

        // Build your circuit.
        let circuit = builder.build::<PoseidonGoldilocksConfig>();

        // Write to the circuit input.
        // These values are taken from Ethereum block https://etherscan.io/block/17880427
        let mut input = circuit.input();
        input.write::<Bytes32Variable>(bytes32!(
            "0x281dc31bb78779a1ede7bf0f4d2bc5f07ddebc9f9d1155e413d8804384604bbe"
        ));
        input.write::<AddressVariable>(address!("0x55032650b14df07b85bF18A3a3eC8E0Af2e028d5"));
        input.write::<Bytes32Variable>(bytes32!(
            "0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5"
        ));

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output.
        let circuit_value = output.read::<Bytes32Variable>();
        println!("{}", circuit_value);
        assert_eq!(
            circuit_value,
            bytes32!("0x0000000000000000000000dd4bc51496dc93a0c47008e820e0d80745476f2201"),
        );
    }
}
