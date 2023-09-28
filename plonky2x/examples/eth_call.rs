//! An example of a basic circuit function which wraps an existing circuit and makes it compatible
//! with a standard for serializing and deserializing inputs and outputs.
//!
//! To build the binary:
//!
//!     `cargo build --release --bin circuit_function_field`
//!
//! To build the circuit:
//!
//!     `./target/release/circuit_function_field build`
//!
//! To prove the circuit using field-based io:
//!
//!    `./target/release/circuit_function_field prove --input-json input.json`
//!
//! Note that this circuit will not work with evm-based io.

use std::env;

use alloy_primitives::{Address, U160};
use alloy_sol_types::{sol, SolType};
use ethers::middleware::Middleware;
use ethers::providers::{Http, Provider};
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::{Eip1559TransactionRequest, H160};
use plonky2x::backend::circuit::{Circuit, PlonkParameters};
use plonky2x::backend::function::{RustFunction, VerifiableRustFunction};
use plonky2x::prelude::{CircuitBuilder, Variable};
use tokio;
type SolTuple = sol! { tuple(uint32, uint64, address, address, bytes) };

struct EthCall {}

impl RustFunction for EthCall {
    fn run(input_bytes: Vec<u8>) -> Vec<u8> {
        let result = SolTuple::decode_single(&input_bytes, true).unwrap();
        let (chain_id, block_number, from_address, to_address, calldata_) = result;
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            dotenv::dotenv().ok();
            let rpc_str = format!("RPC_{}", chain_id);
            let rpc_url = env::var(rpc_str)
                .unwrap_or_else(|_| panic!("RPC_{} environment variable was not found", chain_id));
            let provider: Provider<Http> = Provider::<Http>::try_from(rpc_url).unwrap();
            let mut tx = Eip1559TransactionRequest::new();
            tx = tx.from::<H160>(from_address.0 .0.into());
            tx = tx.to::<H160>(to_address.0 .0.into());
            tx = tx.data(calldata_);

            let result = provider
                .call(&TypedTransaction::Eip1559(tx), Some(block_number.into()))
                .await;
            let bytes = result.unwrap();
            bytes.0.into()
        })
    }
}

fn main() {
    VerifiableRustFunction::<EthCall>::entrypoint();
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{address, bytes};

    use super::*;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_eth_call() {
        let encoded_data = SolTuple::encode_single(&(
            5u32,
            9591261u64,
            address!("355348b048b5b491793110bb76d7a2723262d175"),
            address!("d555cd8277b0e16860f0ae44fcbc2ed94dfce9da"),
            bytes!("050DDCCE00000000000000000000000003ACD25623E5999FDDC27BE7FDB904358700F3AE000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000064EB9D300000000000000000000000000000000000000000000000000000000000000000").to_vec()
        ));

        let input_hex = hex::encode(encoded_data.clone());
        assert_eq!(
            input_hex,
            "0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000500000000000000000000000000000000000000000000000000000000009259dd000000000000000000000000355348b048b5b491793110bb76d7a2723262d175000000000000000000000000d555cd8277b0e16860f0ae44fcbc2ed94dfce9da00000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000a4050ddcce00000000000000000000000003acd25623e5999fddc27be7fdb904358700f3ae000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000064eb9d30000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        );

        let output = EthCall::run(encoded_data);
        let result = hex::encode(output);
        assert_eq!(
            result,
            "000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000064eb9d90000000000000000000000000000000000000000000000000000048198737bd73000000000000000000000000000000000000000000000000000000006c6f600000000000000000000000000000000000000000000000021dcea0a4ea31b41370"
        );
    }
}
