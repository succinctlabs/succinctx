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

use plonky2x::backend::circuit::{Circuit, PlonkParameters};
use plonky2x::backend::function::{RustFunction, VerifiableRustFunction};
use plonky2x::prelude::{CircuitBuilder, Variable};

struct EthCall {}

impl RustFunction for EthCall {
    fn run(input_bytes: Vec<u8>) -> Vec<u8> {
        input_bytes[0..32].to_vec()
    }
}

fn main() {
    VerifiableRustFunction::<EthCall>::entrypoint();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eth_call() {
        let output = EthCall::run(vec![1u8; 64]);
        assert_eq!(output, vec![1u8; 32]);
    }
}
