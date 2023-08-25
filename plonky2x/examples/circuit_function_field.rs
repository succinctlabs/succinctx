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

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::AlgebraicHasher;
use plonky2x::circuit::Circuit;
use plonky2x::function::CircuitFunction;
use plonky2x::prelude::CircuitBuilder;
use plonky2x::vars::Variable;

struct Function {}

impl CircuitFunction for Function {
    fn build<F, C, const D: usize>() -> Circuit<F, C, D>
    where
        F: RichField + Extendable<D>,
        C: plonky2::plonk::config::GenericConfig<D, F = F> + 'static,
        <C as plonky2::plonk::config::GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
    {
        let mut builder = CircuitBuilder::<F, D>::new();
        let a = builder.read::<Variable>();
        let b = builder.read::<Variable>();
        let c = builder.add(a, b);
        builder.write(c);
        builder.build::<C>()
    }
}

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    Function::cli();
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use plonky2::field::types::Field;
    use plonky2x::prelude::{GoldilocksField, PoseidonGoldilocksConfig};

    use super::*;

    type F = GoldilocksField;
    type C = PoseidonGoldilocksConfig;
    const D: usize = 2;

    #[test]
    fn test_circuit_function_field() {
        let circuit = Function::build::<F, C, D>();
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
        Function::test::<F, C, D>(path);
    }
}
