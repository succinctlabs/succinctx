//! An example of a basic EVM function which takes in on-chain input bytes and returns output bytes
//! that correspond to the result of an `eth_call`.
//!
//! To build the binary:
//!
//!     `cargo build --example eth_call --release`
//!
//! To build the function, which saves the verifier contract:
//!
//!     `./target/release/example/eth_call build`
//!
//! To generate the output and proof:
//!
//!    `./target/release/example/eth_call prove --input-json input.json`
//!

use std::env;

use alloy_sol_types::{sol, SolType};
use ethers::middleware::Middleware;
use ethers::providers::{Http, Provider};
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::{Eip1559TransactionRequest, H160};
use rustx::function::RustFunction;
use rustx::program::Program;

/// The tuple which encodes an eth_call request from the EVM.
type EthCallRequestTuple = sol! { tuple(uint32, uint64, address, address, bytes) };

#[derive(Debug, Clone)]
struct EthCall;

impl Program for EthCall {
    fn run(input_bytes: Vec<u8>) -> Vec<u8> {
        // Decode the input bytes into the request tuple.
        let (chain_id, block_number, from_address, to_address, calldata) =
            EthCallRequestTuple::abi_decode_sequence(&input_bytes, true).unwrap();

        // Get relevant environment variables and initialize the HTTP provider.
        dotenv::dotenv().ok();
        let rpc_url = env::var(format!("RPC_{}", chain_id))
            .unwrap_or_else(|_| panic!("RPC_{} environment variable was not found", chain_id));
        let provider: Provider<Http> = Provider::<Http>::try_from(rpc_url).unwrap();

        // Construct the transaction with the decoded values.
        let mut tx = Eip1559TransactionRequest::new();
        tx = tx.from::<H160>(from_address.0 .0.into());
        tx = tx.to::<H160>(to_address.0 .0.into());
        tx = tx.data(calldata);
        let tx = TypedTransaction::Eip1559(tx);

        // Execute the eth_call via rpc.
        let rt = tokio::runtime::Runtime::new().unwrap();
        let bytes = rt
            .block_on(async { provider.call(&tx, Some(block_number.into())).await })
            .unwrap()
            .0;
        bytes.into()
    }
}

fn main() {
    EthCall::entrypoint();
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{address, bytes};
    use hex;

    use super::*;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_eth_call() {
        // Construct the request.
        let request = EthCallRequestTuple::abi_encode_sequence(&(
            5u32,
            9591261u64,
            address!("355348b048b5b491793110bb76d7a2723262d175"),
            address!("d555cd8277b0e16860f0ae44fcbc2ed94dfce9da"),
            bytes!("050DDCCE00000000000000000000000003ACD25623E5999FDDC27BE7FDB904358700F3AE000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000064EB9D300000000000000000000000000000000000000000000000000000000000000000").to_vec()
        ));

        // Assert that the encoding matches.
        let input_hex = hex::encode(request.clone());
        assert_eq!(
            input_hex,
            "000000000000000000000000000000000000000000000000000000000000000500000000000000000000000000000000000000000000000000000000009259dd000000000000000000000000355348b048b5b491793110bb76d7a2723262d175000000000000000000000000d555cd8277b0e16860f0ae44fcbc2ed94dfce9da00000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000a4050ddcce00000000000000000000000003acd25623e5999fddc27be7fdb904358700f3ae000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000064eb9d30000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        );

        // Compute the output.
        let output = EthCall::run(request);

        // Assert that the output matches.
        let result = hex::encode(output);
        assert_eq!(
            result,
            "000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000064eb9d90000000000000000000000000000000000000000000000000000048198737bd73000000000000000000000000000000000000000000000000000000006c6f600000000000000000000000000000000000000000000000021dcea0a4ea31b41370"
        );
    }
}
