use std::fs::File;
use std::io::Read;

use ethers::providers::{Http, Middleware, Provider};
pub(crate) use ethers::types::EIP1186ProofResponse;
use tokio::runtime::Runtime;

use crate::utils::{address, bytes32};

fn generate_fixtures() {
    // TODO: don't have mainnet RPC url here, read from a .env
    let rpc_url = "https://eth-mainnet.g.alchemy.com/v2/hIxcf_hqT9It2hS8iCFeHKklL8tNyXNF";
    let provider = Provider::<Http>::try_from(rpc_url).unwrap();

    let block_number = 17880427u64;
    let _state_root =
        bytes32!("0xff90251f501c864f21d696c811af4c3aa987006916bd0e31a6c06cc612e7632e");
    let address = address!("0x55032650b14df07b85bF18A3a3eC8E0Af2e028d5");
    let location = bytes32!("0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5");

    // Nouns contract
    // let address = address!("0x9c8ff314c9bc7f6e59a9d9225fb22946427edc03");
    // let location = bytes32!("0x0000000000000000000000000000000000000000000000000000000000000003");

    let get_proof_closure = || -> EIP1186ProofResponse {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            provider
                .get_proof(address, vec![location], Some(block_number.into()))
                .await
                .unwrap()
        })
    };
    let storage_result: EIP1186ProofResponse = get_proof_closure();
    let serialized = serde_json::to_string(&storage_result).unwrap();
    println!("{}", serialized);
    // TODO: save this to fixtures/example.json programatically instead of copy-paste
}

pub(crate) fn read_fixture(filename: &str) -> EIP1186ProofResponse {
    let mut file = File::open(filename).unwrap();
    let mut context = String::new();
    file.read_to_string(&mut context).unwrap();

    let context: EIP1186ProofResponse = serde_json::from_str(context.as_str()).unwrap();
    context
}
