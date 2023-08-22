// Given a plonky2 proof generates a wrapped version of it

use std::fs;

use plonky2::field::extension::Extendable;
use plonky2::field::types::{Field, PrimeField64};
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::{CircuitConfig, CircuitData};
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig, PoseidonGoldilocksConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::plonk::prover::prove;
use plonky2::util::timing::TimingTree;
use plonky2::plonk::circuit_data::VerifierCircuitTarget;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::plonk::proof::ProofWithPublicInputsTarget;
use crate::wrapper::plonky2_config::PoseidonBN128GoldilocksConfig;

const D: usize = 2;
type C = PoseidonGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;


pub struct WrapperCircuit {
    outer_proof_target: ProofWithPublicInputsTarget<D>,
    outer_verifier_data: VerifierCircuitTarget,
}

impl WrapperCircuit {
    pub fn define(builder: &mut CircuitBuilder<F, D>, inner_data: &CircuitData<F, C, D>) -> Self {
        let outer_proof_target = builder.add_virtual_proof_with_pis(&inner_data.common);
        let outer_verifier_data =
        builder.add_virtual_verifier_data(inner_data.common.config.fri_config.cap_height);
        builder.verify_proof::<C>(
            &outer_proof_target,
            &outer_verifier_data,
            &inner_data.common,
        );
        builder.register_public_inputs(&outer_proof_target.public_inputs);
        builder.register_public_inputs(&outer_verifier_data.circuit_digest.elements);
        return WrapperCircuit {
            outer_proof_target,
            outer_verifier_data,
        };
    }

    pub fn build(builder: CircuitBuilder<F, D>) -> CircuitData<GoldilocksField, PoseidonBN128GoldilocksConfig, 2> {
        let outer_data = builder.build::<PoseidonBN128GoldilocksConfig>();
        return outer_data;

    }

    pub fn set_witness(&self, inner_data: &CircuitData<F, C, D>, inner_proof: &ProofWithPublicInputs<F, C, D>) -> PartialWitness<GoldilocksField>{
        let mut outer_pw = PartialWitness::new();
        outer_pw.set_proof_with_pis_target(&self.outer_proof_target, &inner_proof);
        outer_pw.set_verifier_data_target(&self.outer_verifier_data, &inner_data.verifier_only);
        return outer_pw
    }

    pub fn save_proof(outer_data: &CircuitData<F, PoseidonBN128GoldilocksConfig, D>, proof: &ProofWithPublicInputs<F, PoseidonBN128GoldilocksConfig, D>, path: String) {
            let outer_common_circuit_data_serialized =
                serde_json::to_string(&outer_data.common).unwrap();
            fs::write(
                format!("{}.wrapper_common_circuit_data.json", path),
                outer_common_circuit_data_serialized,
            )
            .expect("Unable to write file");

            let outer_verifier_only_circuit_data_serialized =
                serde_json::to_string(&outer_data.verifier_only).unwrap();
            fs::write(
                format!("{}.wrapper_verifier_only_data.json", path),
                outer_verifier_only_circuit_data_serialized,
            )
            .expect("Unable to write file");

            let outer_proof_serialized = serde_json::to_string(&proof).unwrap();
            fs::write(
                format!("{}.wrapper_proof.json", path),
                outer_proof_serialized,
            )
            .expect("Unable to write file");
    }
}

pub mod test {
    use plonky2::gates::public_input;

    use super::*;
    use crate::vars::Bytes32Variable;
    use crate::utils::{address, bytes32};
    use crate::vars::CircuitVariable;
    use crate::builder::CircuitBuilder;

    #[test]
    fn test_wrap_proof() {
        let mut builder = CircuitBuilder::new();
        let input_hash = builder.init::<Bytes32Variable>();
        let output_hash = builder.init::<Bytes32Variable>();

        let mut pw: PartialWitness<F> = PartialWitness::new();
        input_hash.set(&mut pw, bytes32!("0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5"));
        output_hash.set(&mut pw, bytes32!("0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5"));

        let mut public_inputs = input_hash.targets();
        public_inputs.append(&mut output_hash.targets());
        builder.register_public_inputs(&public_inputs);

        let inner_data = builder.build();
        let inner_proof = inner_data.prove(pw).unwrap();

        println!("inner proof is {:?}", inner_proof.public_inputs);
        
        let mut builderx = CircuitBuilder::new();
        let mut outer_builder = builderx.api;
        let wrapper = WrapperCircuit::define(&mut outer_builder, &inner_data);
        let outer_data = WrapperCircuit::build(outer_builder);

        let outer_pw = wrapper.set_witness(&inner_data, &inner_proof);
        let outer_proof = outer_data.prove(outer_pw).unwrap();
        let ret = outer_data.verify(outer_proof.clone()).unwrap();
    }
}
