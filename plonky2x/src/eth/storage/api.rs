use crate::builder::BuilderAPI;
use crate::vars::{Bytes32Variable, Variable, U256Variable, BytesVariable, ByteVariable};
use crate::eth::types::{AddressVariable};

use super::types::{AccountVariable, ProofVariable};
use super::generator::{GetStorageProofGenerator};

pub struct StorageProofAPI {
    pub api: BuilderAPI,
}

impl StorageProofAPI {
    pub fn new(api: BuilderAPI) -> Self {
        Self { api }
    }

    pub fn merkle_trie_constraint(
        &mut self,
        _root: Bytes32Variable,
        _key: Bytes32Variable,
        _proof: ProofVariable,
        _value: &[ByteVariable],
    ) {
        todo!()
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
        );
        self.api.api.add_simple_generator(generator);

        self.merkle_trie_constraint(_state_root, address_hash, account_proof, account.serialize());
        self.merkle_trie_constraint(account.storage_hash, _location, storage_proof, value);
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