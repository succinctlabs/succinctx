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
use plonky2x::backend::circuit::{Circuit, PlonkParameters};
use plonky2x::backend::function::VerifiableFunction;
use plonky2x::frontend::hint::simple::hint::Hint;
use plonky2x::prelude::{
    ArrayVariable, BoolVariable, CircuitBuilder, U64Variable, ValueStream, Variable, VariableStream,
};
use serde::{Deserialize, Serialize};

const INPUT_SIZE: usize = 3;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MaskHint;

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for MaskHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let input = input_stream.read_value::<ArrayVariable<U64Variable, INPUT_SIZE>>();
        let mask = input_stream.read_value::<ArrayVariable<BoolVariable, INPUT_SIZE>>();

        let mut masked_input = vec![0; 3];
        let mut j = 0;
        for i in 0..input.len() {
            if mask[i] {
                masked_input[j] = input[i];
                j += 1;
            }
        }

        output_stream.write_value::<ArrayVariable<U64Variable, INPUT_SIZE>>(masked_input);
    }
}

// 1. Generic instead of U64Variable
// 2. Security -- use two Variables
// 3. Make into a easy to use function

#[derive(Debug, Clone)]
struct CheckArray;

impl Circuit for CheckArray {
    fn define<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>) {
        let input = builder.read::<ArrayVariable<U64Variable, INPUT_SIZE>>();
        let mask = builder.read::<ArrayVariable<BoolVariable, INPUT_SIZE>>();

        let zero = builder.zero();
        let one = builder.one();

        // Offload output computation to the prover
        let hint = MaskHint;
        let mut input_stream = VariableStream::new();
        input_stream.write(&input);
        input_stream.write(&mask);

        let output_stream = builder.hint(input_stream, hint);
        let output = output_stream.read::<ArrayVariable<U64Variable, INPUT_SIZE>>(builder);

        // random number
        let random = builder.constant::<U64Variable>(7);
        let mut r = builder.constant::<U64Variable>(1);

        // Compute polynomial commitment w.r.t masked_input (output)
        let mut commitment_1: U64Variable = zero;
        for i in 0..INPUT_SIZE {
            let summand = builder.mul(output[i], r);
            commitment_1 = builder.add(commitment_1, summand);

            r = builder.mul(r, random);
        }

        // Compute polynomial commitment w.r.t input, mask
        let mut r = builder.constant::<U64Variable>(1);
        let mut commitment_2: U64Variable = zero;
        for i in 0..INPUT_SIZE {
            let s: U64Variable = builder.select(mask[i], one, zero);
            let r_val = builder.mul(s, r);
            let summand = builder.mul(input[i], r_val);
            commitment_2 = builder.add(commitment_2, summand);

            let s: U64Variable = builder.select(mask[i], random, one);
            r = builder.mul(r, s);
        }

        let result = builder.is_equal(commitment_1, commitment_2);
        builder.write(result);
    }
}

fn main() {
    VerifiableFunction::<CheckArray>::entrypoint();
}

#[cfg(test)]
mod tests {
    use plonky2x::prelude::DefaultParameters;

    use super::*;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_storage() {
        let mut builder = CircuitBuilder::<L, D>::new();
        CheckArray::define(&mut builder);
        let circuit = builder.build();
        let mut input = circuit.input();
        input.write::<ArrayVariable<U64Variable, INPUT_SIZE>>(vec![4, 6, 33]);
        input.write::<ArrayVariable<BoolVariable, INPUT_SIZE>>(vec![false, true, true]);
        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        let result = output.read::<BoolVariable>();
        assert!(result);
    }
}
