use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use super::generators::block::EthBlockGenerator;
use super::generators::storage::{
    EthAccountProofGenerator, EthLogGenerator, EthStorageKeyGenerator, EthStorageProofGenerator,
};
use super::vars::{EthAccountVariable, EthHeaderVariable, EthLogVariable};
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::vars::AddressVariable;
use crate::frontend::uint::uint256::U256Variable;
use crate::frontend::vars::Bytes32Variable;

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn get_storage_key_at(
        &mut self,
        mapping_location: U256Variable,
        map_key: Bytes32Variable,
    ) -> Bytes32Variable {
        let generator = EthStorageKeyGenerator::new(self, mapping_location, map_key);
        self.add_simple_generator(&generator);
        generator.value
    }

    #[allow(non_snake_case)]
    pub fn eth_get_storage_at(
        &mut self,
        block_hash: Bytes32Variable,
        address: AddressVariable,
        storage_key: Bytes32Variable,
    ) -> Bytes32Variable {
        let generator = EthStorageProofGenerator::new(self, block_hash, address, storage_key);
        self.add_simple_generator(&generator);
        generator.value
    }

    #[allow(non_snake_case)]
    pub fn eth_get_block_by_hash(&mut self, block_hash: Bytes32Variable) -> EthHeaderVariable {
        let generator = EthBlockGenerator::new(self, block_hash);
        self.add_simple_generator(&generator);
        generator.value
    }

    #[allow(non_snake_case)]
    pub fn eth_get_account(
        &mut self,
        address: AddressVariable,
        block_hash: Bytes32Variable,
    ) -> EthAccountVariable {
        let generator = EthAccountProofGenerator::new(self, address, block_hash);
        self.add_simple_generator(&generator);
        generator.value
    }

    #[allow(non_snake_case)]
    pub fn eth_get_transaction_log(
        &mut self,
        transaction_hash: Bytes32Variable,
        block_hash: Bytes32Variable,
        log_index: u64,
    ) -> EthLogVariable {
        let generator = EthLogGenerator::new(self, transaction_hash, block_hash, log_index);
        self.add_simple_generator(&generator);
        generator.value
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use ethers::providers::{Http, Provider};
    use ethers::types::{U256, U64};
    use plonky2::plonk::config::PoseidonGoldilocksConfig;

    use super::*;
    use crate::backend::circuit::serialization::{GateRegistry, WitnessGeneratorRegistry};
    use crate::frontend::eth::storage::utils::get_map_storage_location;
    use crate::frontend::eth::storage::vars::{EthAccount, EthHeader, EthLog};
    use crate::prelude::CircuitBuilderX;
    use crate::utils::{address, bytes32};

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    #[allow(non_snake_case)]
    fn test_eth_get_storage_at() {
        dotenv::dotenv().ok();
        let rpc_url = env::var("RPC_1").unwrap();
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();

        // This is the circuit definition
        let mut builder = CircuitBuilderX::new();
        builder.set_execution_client(provider);
        let block_hash = builder.evm_read::<Bytes32Variable>();
        let address = builder.evm_read::<AddressVariable>();
        let location = builder.evm_read::<Bytes32Variable>();
        let value = builder.eth_get_storage_at(block_hash, address, location);
        builder.evm_write(value);

        // Build your circuit.
        let circuit = builder.build::<PoseidonGoldilocksConfig>();

        // Write to the circuit input.
        // These values are taken from Ethereum block https://etherscan.io/block/17880427
        let mut input = circuit.input();
        // block hash
        input.evm_write::<Bytes32Variable>(bytes32!(
            "0x281dc31bb78779a1ede7bf0f4d2bc5f07ddebc9f9d1155e413d8804384604bbe"
        ));
        // address
        input.evm_write::<AddressVariable>(address!("0x55032650b14df07b85bF18A3a3eC8E0Af2e028d5"));
        // location
        input.evm_write::<Bytes32Variable>(bytes32!(
            "0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5"
        ));

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output.
        let circuit_value = output.evm_read::<Bytes32Variable>();
        println!("{:?}", circuit_value);
        assert_eq!(
            circuit_value,
            bytes32!("0x0000000000000000000000dd4bc51496dc93a0c47008e820e0d80745476f2201"),
        );

        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    #[allow(non_snake_case)]
    fn test_get_storage_key_at() {
        dotenv::dotenv().ok();
        // This is the circuit definition
        let mut builder = CircuitBuilderX::new();
        let mapping_location = builder.read::<U256Variable>();
        let map_key = builder.read::<Bytes32Variable>();

        let value = builder.get_storage_key_at(mapping_location, map_key);
        builder.write(value);

        // Build your circuit.
        let circuit = builder.build::<PoseidonGoldilocksConfig>();

        // Write to the circuit input.
        let mut input = circuit.input();
        let mapping_location = U256::from("0x0");
        // mapping_location
        input.write::<U256Variable>(mapping_location);

        let map_key =
            bytes32!("0x281dc31bb78779a1ede7bf0f4d2bc5f07ddebc9f9d1155e413d8804384604bbe");
        // map_key
        input.write::<Bytes32Variable>(map_key);

        println!(
            "storage key: {:?}",
            get_map_storage_location(mapping_location.as_u128(), map_key)
        );

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output.
        let circuit_value = output.read::<Bytes32Variable>();
        println!("{:?}", circuit_value);
        assert_eq!(
            circuit_value,
            bytes32!("0xca77d4e79102603cb6842afffd8846a3123877159ed214aeadfc4333d595fd50"),
        );

        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    #[allow(non_snake_case)]
    fn test_eth_get_block_by_hash() {
        dotenv::dotenv().ok();
        let rpc_url = env::var("RPC_1").unwrap();
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();

        // This is the circuit definition
        let mut builder = CircuitBuilderX::new();
        builder.set_execution_client(provider);
        let block_hash = builder.read::<Bytes32Variable>();

        let value = builder.eth_get_block_by_hash(block_hash);
        builder.write(value);

        // Build your circuit.
        let circuit = builder.build::<PoseidonGoldilocksConfig>();

        // Write to the circuit input.
        // These values are taken from Ethereum block https://etherscan.io/block/17880427
        let mut input = circuit.input();
        // block hash
        input.write::<Bytes32Variable>(bytes32!(
            "0x281dc31bb78779a1ede7bf0f4d2bc5f07ddebc9f9d1155e413d8804384604bbe"
        ));

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output
        let circuit_value = output.read::<EthHeaderVariable>();
        println!("{:?}", circuit_value);
        assert_eq!(
            circuit_value,
            EthHeader {
                parent_hash: bytes32!(
                    "0x7b012bf12a831368d7278edad91eb968df7912902aeb45bce0948f1ec8b411df"
                ),
                uncle_hash: bytes32!(
                    "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"
                ),
                coinbase: address!("0xa8c62111e4652b07110a0fc81816303c42632f64"),
                root: bytes32!(
                    "0xff90251f501c864f21d696c811af4c3aa987006916bd0e31a6c06cc612e7632e"
                ),
                tx_hash: bytes32!(
                    "0x8d0a3c10b76930ebda83551649856882b51455de61689184c9db535ef5c29e93"
                ),
                receipt_hash: bytes32!(
                    "0x8fa46ad6b448faefbfc010736a3d39595ca68eb8bdd4e6b4ab30513bab688068"
                ),
                difficulty: U256::from("0x0"),
                number: U64::from("0x110d56b"),
                gas_limit: U256::from("0x1c9c380"),
                gas_used: U256::from("0x16041f6"),
                time: U256::from("0x64d41817"),
            }
        );

        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    #[allow(non_snake_case)]
    fn test_eth_get_transaction_log() {
        dotenv::dotenv().ok();
        let rpc_url = env::var("RPC_1").unwrap();
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();

        // This is the circuit definition
        let mut builder = CircuitBuilderX::new();
        builder.set_execution_client(provider);
        let transaction_hash = builder.read::<Bytes32Variable>();
        let block_hash = builder.read::<Bytes32Variable>();
        let log_index = 0u64;

        let value = builder.eth_get_transaction_log(transaction_hash, block_hash, log_index);
        builder.write(value);

        // Build your circuit.
        let circuit = builder.build::<PoseidonGoldilocksConfig>();

        // Write to the circuit input.
        // These values are taken from Ethereum block https://etherscan.io/block/17880427
        let mut input = circuit.input();
        // transaction hash
        input.write::<Bytes32Variable>(bytes32!(
            "0xead2251970404128e6f9bdff0133badb7338c5fa7ea4eec24e88af85a6d03cf2"
        ));
        // block hash
        input.write::<Bytes32Variable>(bytes32!(
            "0x281dc31bb78779a1ede7bf0f4d2bc5f07ddebc9f9d1155e413d8804384604bbe"
        ));

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output.
        let circuit_value = output.read::<EthLogVariable>();
        println!("{:?}", circuit_value);
        assert_eq!(
            circuit_value,
            EthLog {
                address: address!("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"),
                topics: [
                    bytes32!("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"),
                    bytes32!("0x00000000000000000000000059b4bb1f5d943cf71a10df63f6b743ee4a4489ee"),
                    bytes32!("0x000000000000000000000000def1c0ded9bec7f1a1670819833240f027b25eff")
                ],
                data_hash: bytes32!(
                    "0x5cdda96947975d4afbc971c9aa8bb2cc684e158d10a0d878b3a5b8b0f895262c"
                )
            }
        );

        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    #[allow(non_snake_case)]
    fn test_eth_get_account() {
        dotenv::dotenv().ok();
        let rpc_url = env::var("RPC_1").unwrap();
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();

        // This is the circuit definition
        let mut builder = CircuitBuilderX::new();
        builder.set_execution_client(provider);
        let block_hash = builder.read::<Bytes32Variable>();
        let address = builder.read::<AddressVariable>();

        let value = builder.eth_get_account(address, block_hash);
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

        // Generate a proof.
        let (proof, output) = circuit.prove(&input);

        // Verify proof.
        circuit.verify(&proof, &input, &output);

        // Read output.
        let circuit_value = output.read::<EthAccountVariable>();
        println!("{:?}", circuit_value);
        assert_eq!(
            circuit_value,
            EthAccount {
                nonce: U64::from(1),
                balance: U256::from(0),
                code_hash: bytes32!(
                    "0xb9c1c929064cd21734c102a698e68bf617feefcfa5a9f62407c45401546736bf"
                ),
                storage_hash: bytes32!(
                    "0x073d71569b4b986bc20b6921dbbc1b74145588f765627dd5e566d65a6b7b33cc"
                ),
            },
        );

        circuit.test_default_serializers();
    }
}
