
use crate::builder::BuilderAPI;
use crate::vars::{Bytes32Variable, U256Variable, BoolVariable, WitnessWriteMethods, WitnessMethods};
use crate::eth::types::{AddressVariable};
use ethers::providers::{Http, Middleware, Provider};
use plonky2::iop::witness::{PartialWitness, WitnessWrite};

use super::types::{AccountVariable, ProofVariable};
use super::generator::{GetStorageProofGenerator};

pub struct StorageProofAPI<'a> {
    pub api: &'a mut BuilderAPI,
    pub provider: Provider<Http>,
}

impl<'a> StorageProofAPI<'a> {
    pub fn new(api: &'a mut BuilderAPI, provider: Provider<Http>) -> Self {
        Self { api, provider }
    }

    // Constraint that a merkle trie with root _root has _value at _key, with _proof as "evidence"
    pub fn merkle_trie_constraint(
        &mut self,
        _root: Bytes32Variable,
        _key: Bytes32Variable,
        _proof: ProofVariable,
        _value: Vec<BoolVariable>,
    ) {
        return
    }

    pub fn get_storage_at_position(
        &mut self, 
        _state_root: Bytes32Variable,
        _address: AddressVariable,
        _position: U256Variable,
    ) -> Bytes32Variable {
        todo!()
    }

    // Implementation taken from Solidity code at:
    // https://github.com/succinctlabs/telepathy-contracts/blob/main/src/libraries/StateProofHelper.sol#L22
    // getStorageRoot(bytes[] memory proof, address contractAddress, bytes32 stateRoot)
        // bytes32 addressHash = keccak256(abi.encodePacked(contractAddress));
        // bytes memory acctRlpBytes = MerkleTrie.get(abi.encodePacked(addressHash), proof, stateRoot);
        // require(acctRlpBytes.length > 0, "Account does not exist");
        // RLPReader.RLPItem[] memory acctFields = acctRlpBytes.toRLPItem().readList();
        // require(acctFields.length == 4);
        // return bytes32(acctFields[2].readUint256());
    pub fn get_storage_at_location(
        &mut self,
        _state_root: Bytes32Variable,
        _address: AddressVariable,
        _location: Bytes32Variable,
        _block_number: u64,
    ) -> Bytes32Variable {
        let account = AccountVariable{
            balance: self.api.init_u256(),
            code_hash: self.api.init_bytes32(),
            nonce: self.api.init_u256(),
            storage_hash: self.api.init_bytes32()
        };
        let account_proof = ProofVariable{proof: self.api.init_bytes32()};
        let storage_proof = ProofVariable{proof: self.api.init_bytes32()};

        // TODO bytes32 addressHash = keccak256(abi.encodePacked(contractAddress));
        let address_hash = self.api.init_bytes32();
        let value = self.api.init_bytes32();

        let generator: GetStorageProofGenerator<plonky2::field::goldilocks_field::GoldilocksField, 2> = GetStorageProofGenerator::new(
            _address,
            _location,
            account,
            account_proof,
            storage_proof,
            value, 
            _block_number,
            self.provider.clone()
        );
        self.api.api.add_simple_generator(generator);

        self.merkle_trie_constraint(_state_root, address_hash, account_proof, account.serialize());
        self.merkle_trie_constraint(account.storage_hash, _location, storage_proof, value.0.into());
        value
    }

    pub fn get_storage_at_locations<const NUM: usize>(
        &mut self,
        _address: AddressVariable,
        _locations: [Bytes32Variable; NUM]
    ) -> [Bytes32Variable; NUM] {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Add;

    use super::*;

    use anyhow::Result;
    use ethers::types::{H256, Address};
    use plonky2::iop::witness::{PartialWitness, WitnessWrite};
    use plonky2::plonk::circuit_builder::CircuitBuilder;
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
    use subtle_encoding::hex::decode;

    #[test]
    fn test_get_storage_at_location() {
        // TODO: read this RPC url from an .env
        let rpc_url = "https://eth-mainnet.g.alchemy.com/v2/hIxcf_hqT9It2hS8iCFeHKklL8tNyXNF";
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();
        let mut api = BuilderAPI::new();
        // Instantiate the variables going into the circuit
        // TODO: technically we should also register all these inputs as public
        // Public inputs are the initial value (provided below) and the result (which is generated).
        // builder.register_public_input(initial);
        // builder.register_public_input(cur_target);
        let state_root = api.init_bytes32();
        let address = api.init_address();
        let location = api.init_bytes32();
        let claimed_value = api.init_bytes32();
        let block_number = 17880427u64;

        let mut storage_api = StorageProofAPI::new(&mut api, provider);
        let value = storage_api.get_storage_at_location(state_root, address, location, block_number);
        // api.assert_is_equal_bytes32(claimed_value, value);

        let mut pw = PartialWitness::new();
        
        let state_root_input_raw = "0xff90251f501c864f21d696c811af4c3aa987006916bd0e31a6c06cc612e7632e".parse::<H256>().unwrap();
        let state_root_input = state_root_input_raw.as_fixed_bytes();
        pw.set_from_bytes_be(state_root.into(), *state_root_input);

        let address_input_raw = "0x55032650b14df07b85bF18A3a3eC8E0Af2e028d5".parse::<Address>().unwrap();
        let address_input = address_input_raw.as_fixed_bytes();
        pw.set_from_bytes_be(address.into(), *address_input);   

        let location_input_raw = "0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5".parse::<H256>().unwrap();
        let location_input = location_input_raw.as_fixed_bytes();
        pw.set_from_bytes_be(location.into(), *location_input);

        let claimed_value_raw = "0x0000000000000000000000dd4bc51496dc93a0c47008e820e0d80745476f2201".parse::<H256>().unwrap();
        let claimed_value_input = claimed_value_raw.as_fixed_bytes();
        pw.set_from_bytes_be(claimed_value.into(), *claimed_value_input);

        println!("Building circuit");
        let data = api.build();
        println!("Proving circuit");
        println!("Address {:?}", address);
        println!("Address in witness {:?}", pw.get_bytes_be(address.into()));
        let proof = data.prove(pw).unwrap();
        data.verify(proof).unwrap();

    }


}

