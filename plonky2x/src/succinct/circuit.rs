use std::fmt;
use std::error::Error;
use std::collections::HashMap;
use std::marker::PhantomData;

use plonky2::iop::witness::PartialWitness;
use plonky2::hash::hash_types::RichField;
use plonky2::field::extension::Extendable;
use ethers::types::H256;
use plonky2::plonk::config::GenericConfig;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::field::goldilocks_field::GoldilocksField;

use crate::vars::CircuitVariable;
use crate::builder::CircuitBuilder;
use crate::vars::{ByteVariable, Bytes32Variable};
use crate::utils::serializer::{load_circuit, save_circuit};

pub trait Circuit<F: RichField + Extendable<D>, const D: usize> {
    fn get_input_bytes(&self) -> Vec<ByteVariable>;
    fn get_output_bytes(&self) -> Vec<ByteVariable>;
    fn set_witness(&self, pw: &mut PartialWitness<F>, input_bytes: &[u8]);
    fn define(builder: &mut CircuitBuilder<F, D>) -> Self;
}

pub struct CircuitFunction<F: RichField + Extendable<D>, const D: usize, C: Circuit<F, D>> {
    input_hash: Bytes32Variable,
    output_hash: Bytes32Variable,
    circuit: C,
    _marker: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize, C: Circuit<F, D>> CircuitFunction<F, D, C> {
    pub fn set_witness(&mut self, input_bytes: &[u8]) -> PartialWitness<F> {
        let mut pw = PartialWitness::new();
        // TODO actually hash input_bytes to get `input_bytes_hash` below
        let input_bytes_hash = H256::from_slice(&input_bytes[..]);
        self.input_hash.set(&mut pw, input_bytes_hash);

        // Set the witness of the subcircuit
        self.circuit.set_witness(&mut pw, input_bytes);

        let mut output_bytes_value = Vec::new();
        for output_byte in self.circuit.get_output_bytes() {
            output_bytes_value.push(output_byte.value(&pw));
        }
        // TODO actually hash output_bytes_values to get `output_bytes_hash` below
        let output_bytes_hash = H256::from_slice(&output_bytes_value[..]);
        self.output_hash.set(&mut pw, output_bytes_hash);
        return pw;
    }

    pub fn define(builder: &mut CircuitBuilder<F, D>) -> Self {
        // TODO: we can maybe save `builder` within the CircuitFunction struct and have a function `save_build` that serializes the build and saves it.
        // This would be nice as it would prevent others from adding to the builder afterwards, which should not happen.
        let input_hash = builder.init::<Bytes32Variable>();
        let output_hash = builder.init::<Bytes32Variable>();
        let inner_circuit = C::define(builder);
        // TODO: constraint input_hash to the result of inner_circuit.get_input_bytes()
        // TODO: constraint outer_hash to the result of inner_circuit.get_output_bytes()
        CircuitFunction {
            input_hash,
            output_hash,
            circuit: inner_circuit,
            _marker: PhantomData,
        }
    }

    pub fn generate_fixture(&self, input_bytes: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        // Run the circuit with witness generation only to generate fixture
        todo!()
    }
}

pub mod test {
    use plonky2::hash::hash_types::RichField;
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;
    use crate::utils::bytes32;

    use super::*;

    pub struct TestCircuit {
        input_bytes: Vec<ByteVariable>,
        output_bytes: Vec<ByteVariable>,
    }

    impl<F: RichField + Extendable<D>, const D: usize> Circuit<F, D> for TestCircuit {
        fn get_input_bytes(&self) -> Vec<ByteVariable> {
            self.input_bytes.clone() // Clone to avoid moving.
        }

        fn get_output_bytes(&self) -> Vec<ByteVariable> {
            self.output_bytes.clone() // Clone to avoid moving.
        }

        fn set_witness(&self, pw: &mut PartialWitness<F>, input_bytes: &[u8]) {
            for i in 0..input_bytes.len() {
                self.input_bytes[i].set(pw, input_bytes[i]);
                self.output_bytes[i].set(pw, input_bytes[i]);
            }
        }

        fn define(builder: &mut CircuitBuilder<F, D>) -> Self {
            let mut input_bytes = Vec::new();
            let mut output_bytes = Vec::new();
            for _ in 0..32 {
                input_bytes.push(builder.init::<ByteVariable>());
                output_bytes.push(builder.init::<ByteVariable>());
            }
            TestCircuit {
                input_bytes,
                output_bytes,
            }
        }
    }

    #[test]
    pub fn test_circuit_function() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();
        let mut circuit_function: CircuitFunction<F, D, TestCircuit> = CircuitFunction::define(
            &mut builder
        );
        let circuit_build = builder.build::<C>();
        save_circuit(&circuit_build, "test_circuit_function".to_string());

        let loaded_circuit_build: CircuitData<F, C, D> = load_circuit(&"test_circuit_function".to_string());
        let pw = circuit_function.set_witness(bytes32!("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef").as_bytes());
        let proof = loaded_circuit_build.prove(pw).unwrap();
        loaded_circuit_build.verify(proof).unwrap();
    }
}
