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
//!     cargo run --example evm prove ./examples/evm.json
//!
//! Note that this circuit will not work with field-based io.

use plonky2x::backend::circuit::{Circuit, PlonkParameters};
use plonky2x::backend::function::Plonky2xFunction;
use plonky2x::frontend::vars::ByteVariable;
use plonky2x::prelude::CircuitBuilder;

#[derive(Debug, Clone)]
struct SimpleAdditionCircuit;

impl Circuit for SimpleAdditionCircuit {
    fn define<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>) {
        let a = builder.evm_read::<ByteVariable>();
        let b = builder.evm_read::<ByteVariable>();
        let c = builder.xor(a, b);
        builder.evm_write(c);
    }
}

fn main() {
    SimpleAdditionCircuit::entrypoint();
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use plonky2x::backend::circuit::config::{DefaultParameters, Groth16WrapperParameters};
    use plonky2x::backend::wrapper::wrap::WrappedCircuit;
    use plonky2x::prelude::PoseidonGoldilocksConfig;

    use super::*;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_circuit_function_evm() {
        let mut builder = CircuitBuilder::<L, D>::new();
        SimpleAdditionCircuit::define(&mut builder);
        let circuit = builder.build();
        let mut input = circuit.input();
        input.evm_write::<ByteVariable>(0u8);
        input.evm_write::<ByteVariable>(1u8);
        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        let xor = output.evm_read::<ByteVariable>();
        assert_eq!(xor, 1u8);
    }

    #[test]
    fn test_evm_wrap() {
        utils::setup_logger();
        let mut builder = CircuitBuilder::<L, D>::new();
        SimpleAdditionCircuit::define(&mut builder);
        let circuit = builder.build();
        let mut input = circuit.input();
        input.evm_write::<ByteVariable>(2u8);
        input.evm_write::<ByteVariable>(9u8);
        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        let xor = output.evm_read::<ByteVariable>();
        assert_eq!(xor, 11u8);
        let wrapper: WrappedCircuit<_, _, 2> =
            WrappedCircuit::<L, Groth16WrapperParameters, D>::build(circuit);
        let wrapped_proof = wrapper.prove(&proof).unwrap();
    }
}
