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
//! To prove the circuit using evm io:
//!
//!    `./target/release/circuit_function_evm prove --input-json src/bin/circuit_function_evm_input.json`
//!
//! Note that this circuit will not work with field-based io.

use std::env;

use plonky2x::backend::circuit::Circuit;
use plonky2x::backend::config::PlonkParameters;
use plonky2x::backend::function::CircuitFunction;
use plonky2x::frontend::vars::ByteVariable;
use plonky2x::prelude::CircuitBuilder;

struct Function {}

impl CircuitFunction for Function {
    fn build<L: PlonkParameters<D>, const D: usize>() -> Circuit<L, D> {
        let mut builder = CircuitBuilder::<L, D>::new();
        let a = builder.evm_read::<ByteVariable>();
        let b = builder.evm_read::<ByteVariable>();
        let c = builder.xor(a, b);
        builder.evm_write(c);
        builder.build()
    }
}

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::try_init().unwrap_or_default();
    Function::cli();
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use plonky2x::backend::config::DefaultParameters;
    use plonky2x::prelude::{GoldilocksField, PoseidonGoldilocksConfig};

    use super::*;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_circuit_function_evm() {
        let circuit = Function::build::<L, D>();
        let mut input = circuit.input();
        input.evm_write::<ByteVariable>(0u8);
        input.evm_write::<ByteVariable>(1u8);
        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        let xor = output.evm_read::<ByteVariable>();
        assert_eq!(xor, 1u8);
    }

    #[test]
    fn test_circuit_function_evm_input_json() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let path = format!(
            "{}/examples/circuit_function_evm_input.json",
            root.display()
        );
        Function::test::<F, C, D>(path);
    }
}
