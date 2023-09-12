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

use plonky2x::backend::circuit::{CircuitBuild, PlonkParameters};
use plonky2x::backend::function::CircuitFunction;
use plonky2x::prelude::{CircuitBuilder, Variable};

struct Function {}

impl CircuitFunction for Function {
    fn build<L: PlonkParameters<D>, const D: usize>() -> CircuitBuild<L, D> {
        let mut builder = CircuitBuilder::<L, D>::new();
        let a = builder.read::<Variable>();
        let b = builder.read::<Variable>();
        let c = builder.add(a, b);
        builder.write(c);
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

    use plonky2::field::types::Field;
    use plonky2x::prelude::{GoldilocksField, PoseidonGoldilocksConfig};

    use super::*;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_circuit_function_field() {
        let circuit = Function::build::<L, D>();
        let mut input = circuit.input();
        input.write::<Variable>(F::from_canonical_u64(1));
        input.write::<Variable>(F::from_canonical_u64(2));
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        let sum = output.read::<Variable>();
        assert_eq!(sum, F::from_canonical_u64(3));
    }

    #[test]
    fn test_circuit_function_field_input_json() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let path = format!(
            "{}/examples/circuit_function_field_input.json",
            root.display()
        );
        Function::test::<L, D>(path);
    }
}
