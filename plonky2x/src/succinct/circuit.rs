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
use plonky2::plonk::proof::ProofWithPublicInputs;

use crate::vars::CircuitVariable;
use crate::builder::CircuitBuilder;
use crate::vars::{ByteVariable, Bytes32Variable};
use crate::utils::serializer::{load_circuit, save_circuit};


type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

pub trait CircuitTrait {
    fn compile(builder : &mut CircuitBuilder<F, 2>) -> Self;
}

pub struct CircuitFunction {
}

impl CircuitFunction {
    pub fn generate_fixture(builder: CircuitBuilder<F, D>, input_bytes: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        // Run the circuit with witness generation only to generate fixture
        todo!()
    }

    pub fn build(builder: CircuitBuilder<F, D>, path: String) -> CircuitData<F, C, 2> {
        if !builder.can_build_evm() {
            panic!("Haven't adde appropriate constraints to builder for EVM usage");
        }
        let circuit_build = builder.build::<C>();
        save_circuit(&circuit_build, path);
        return circuit_build;
    }

    pub fn load_and_prove(path: &String, input_bytes: &[u8]) -> ProofWithPublicInputs<F, C, D> {
        let input_targets, loaded_circuit_build: CircuitData<F, C, D> = load_circuit(&path);
        let mut pw = PartialWitness::new();
        if input_targets.len() != input_bytes.len() {
            panic!("Input targets and input bytes must be the same length");
        }
        for (i, input_byte) in input_targets.iter().enumerate() {
            input_byte[i].set(&mut pw, *input_bytes[i]);
        }
        let proof = loaded_circuit_build.prove(pw).unwrap();
        return proof;
    }
}

pub mod test {
    use plonky2::hash::hash_types::RichField;
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;
    use crate::utils::bytes32;

    use super::*;

    #[test]
    pub fn test_circuit_function() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        pub fn my_circuit() -> CircuitBuilder<F, D> {
            // Define my circuit
            let mut builder = CircuitBuilder::<F, D>::new();
            let input_bytes32 = builder.read_bytes(32);
            // TODO get the hash of input_bytes32
            builder.write_bytes(&input_bytes32);
            builder.constraint_onchain();
            return builder;
        }

        let path = "test_circuit_function".to_string();
        let builder = my_circuit();
        let mut circuit_data = CircuitFunction::build(builder, path);

        let builder2 = my_circuit();
        let path2 = "test_circuit_function".to_string();
        let proof: ProofWithPublicInputs<GoldilocksField, PoseidonGoldilocksConfig, 2> = CircuitFunction::load_and_prove(builder2, &path2, bytes32!("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef").as_bytes());
        circuit_data.verify(proof).unwrap();

        // let circuit_build = builder.build::<C>();
        // save_circuit(&circuit_build, "test_circuit_function".to_string());

        // let loaded_circuit_build: CircuitData<F, C, D> = load_circuit(&"test_circuit_function".to_string());
        // let pw = circuit_function.set_witness(bytes32!("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef").as_bytes());
        // let proof = loaded_circuit_build.prove(pw).unwrap();
        // loaded_circuit_build.verify(proof).unwrap();
    }
}
