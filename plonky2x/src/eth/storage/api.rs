use ethers::providers::{Http, Provider};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use super::generator::GetStorageProofGenerator;
use super::vars::{EthAccountVariable, EthProofVariable};
use crate::builder::CircuitBuilder;
use crate::eth::vars::AddressVariable;
use crate::vars::{BoolVariable, Bytes32Variable, U256Variable};

pub struct StorageProofAPI<'a, F: RichField + Extendable<D>, const D: usize> {
    pub api: &'a mut CircuitBuilder<F, D>,
    pub provider: Provider<Http>,
}

impl<'a, F: RichField + Extendable<D>, const D: usize> StorageProofAPI<'a, F, D> {
    pub fn new(api: &'a mut CircuitBuilder<F, D>, provider: Provider<Http>) -> Self {
        Self { api, provider }
    }

    pub fn merkle_trie_constraint(
        &mut self,
        _root: Bytes32Variable,
        _key: Bytes32Variable,
        _proof: EthProofVariable,
        _value: &Vec<BoolVariable>,
    ) {
        return;
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
        let account = self.api.init::<EthAccountVariable>();
        let account_proof = self.api.init::<EthProofVariable>();
        let storage_proof = self.api.init::<EthProofVariable>();
        let address_hash = self.api.init::<Bytes32Variable>();
        let value = self.api.init::<Bytes32Variable>();

        let generator: GetStorageProofGenerator<F, D> = GetStorageProofGenerator::new(
            _address,
            _location,
            account,
            account_proof,
            storage_proof,
            value,
            _block_number,
            self.provider.clone(),
        );
        self.api.add_simple_generator(generator);

        self.merkle_trie_constraint(
            _state_root,
            address_hash,
            account_proof,
            &account.serialize(),
        );
        self.merkle_trie_constraint(account.storage_hash, _location, storage_proof, &vec![]);
        value
    }

    pub fn get_storage_at_locations<const NUM: usize>(
        &mut self,
        _address: AddressVariable,
        _locations: [Bytes32Variable; NUM],
    ) -> [Bytes32Variable; NUM] {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use ethers::types::{Address, H256};
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
        let mut storage = StorageProofAPI::new(&mut builder, provider);
        let value = storage.get_storage_at_location(state_root, address, location, block_number);

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
        let data = builder.build::<C>();
        println!("Proving circuit");
        println!("Address {:?}", address);
        println!("Address in witness {:?}", address.value(&pw));
        println!("{:?}", value);
        let proof = data.prove(pw).unwrap();
        data.verify(proof).unwrap();
    }
}
