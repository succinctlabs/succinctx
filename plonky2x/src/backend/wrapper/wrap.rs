use core::marker::PhantomData;
use std::fs::File;
use std::path::Path;

use anyhow::Result;
use log::{debug, info};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_data::{
    CircuitData, CommonCircuitData, VerifierCircuitTarget, VerifierOnlyCircuitData,
};
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::{ProofWithPublicInputs, ProofWithPublicInputsTarget};
use serde::Serialize;

use super::plonky2_config::PoseidonBN128GoldilocksConfig;
use crate::backend::circuit::io::{CircuitInput, CircuitOutput};
use crate::backend::circuit::Circuit;
use crate::frontend::builder::CircuitBuilder;

#[derive(Debug)]
pub struct WrappedCircuit<
    F: RichField + Extendable<D>,
    InnerConfig: GenericConfig<D, F = F> + 'static,
    OuterConfig: GenericConfig<D, F = F>,
    const D: usize,
> where
    InnerConfig::Hasher: AlgebraicHasher<F>,
{
    circuit: Circuit<F, InnerConfig, D>,
    recursive_data: CircuitData<F, InnerConfig, D>,
    circuit_proof_target: ProofWithPublicInputsTarget<D>,
    circuit_verifier_target: VerifierCircuitTarget,
    wrapper_data: CircuitData<F, OuterConfig, D>,
    proof_target: ProofWithPublicInputsTarget<D>,
    verifier_target: VerifierCircuitTarget,
}

impl<
        F: RichField + Extendable<D>,
        InnerConfig: GenericConfig<D, F = F>,
        OuterConfig: GenericConfig<D, F = F>,
        const D: usize,
    > WrappedCircuit<F, InnerConfig, OuterConfig, D>
where
    InnerConfig::Hasher: AlgebraicHasher<F>,
{
    pub fn build(circuit: Circuit<F, InnerConfig, D>) -> Self {
        // An inner recursion to standartize the degree
        let mut recursive_builder = CircuitBuilder::<F, D>::new();
        let circuit_proof_target =
            recursive_builder.add_virtual_proof_with_pis(&circuit.data.common);
        let circuit_verifier_target = recursive_builder
            .api
            .add_virtual_verifier_data(circuit.data.common.config.fri_config.cap_height);
        recursive_builder.verify_proof::<InnerConfig>(
            &circuit_proof_target,
            &circuit_verifier_target,
            &circuit.data.common,
        );

        let recursive_data = recursive_builder.build::<InnerConfig>().data;
        debug!(
            "Recursive circuit degree: {}",
            recursive_data.common.degree()
        );

        // Finally, wrap this in the outer circuit
        let mut wrapper_builder = CircuitBuilder::<F, D>::new();
        let proof_target = wrapper_builder.add_virtual_proof_with_pis(&recursive_data.common);
        let verifier_target = wrapper_builder
            .api
            .add_virtual_verifier_data(recursive_data.common.config.fri_config.cap_height);
        wrapper_builder.verify_proof::<InnerConfig>(
            &proof_target,
            &verifier_target,
            &recursive_data.common,
        );

        let wrapper_data = wrapper_builder.build::<OuterConfig>().data;
        debug!("Wrapped circuit degree: {}", wrapper_data.common.degree());

        Self {
            circuit,
            recursive_data,
            circuit_proof_target,
            circuit_verifier_target,
            wrapper_data,
            proof_target,
            verifier_target,
        }
    }

    pub fn prove(
        &self,
        inner_proof: &ProofWithPublicInputs<F, InnerConfig, D>,
        input: &CircuitInput<F, D>,
        output: &CircuitOutput<F, D>,
    ) -> WrappedProof<F, OuterConfig, D> {
        let mut pw = PartialWitness::new();
        pw.set_verifier_data_target(
            &self.circuit_verifier_target,
            &self.circuit.data.verifier_only,
        );
        pw.set_proof_with_pis_target(&self.circuit_proof_target, &inner_proof);

        let recursive_proof = self.recursive_data.prove(pw).unwrap();
        self.recursive_data.verify(recursive_proof.clone()).unwrap();
        debug!("Successfully verified recursive proof");

        let mut pw = PartialWitness::new();
        pw.set_verifier_data_target(&self.verifier_target, &self.recursive_data.verifier_only);
        pw.set_proof_with_pis_target(&self.proof_target, &recursive_proof);

        let proof = self.wrapper_data.prove(pw).unwrap();
        self.wrapper_data.verify(proof.clone()).unwrap();

        WrappedProof {
            proof,
            common_data: self.wrapper_data.common.clone(),
            verifier_data: self.wrapper_data.verifier_only.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Wrapper<
    F: RichField + Extendable<D>,
    InnerConfig: GenericConfig<D, F = F>,
    OuterConfig: GenericConfig<D, F = F>,
    const D: usize,
> where
    InnerConfig::Hasher: AlgebraicHasher<F>,
{
    _marker: PhantomData<(F, InnerConfig, OuterConfig)>,
}

#[derive(Debug)]
pub struct WrappedProof<F: RichField + Extendable<D>, C: GenericConfig<D, F = F>, const D: usize> {
    pub proof: ProofWithPublicInputs<F, C, D>,
    pub common_data: CommonCircuitData<F, D>,
    pub verifier_data: VerifierOnlyCircuitData<C, D>,
}

impl<F: RichField + Extendable<D>, C: GenericConfig<D, F = F>, const D: usize>
    WrappedProof<F, C, D>
{
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()>
    where
        C: Serialize,
    {
        let common_data_file = File::create(path.as_ref().join("common_circuit_data.json"))?;
        serde_json::to_writer(&common_data_file, &self.common_data)?;
        info!("Succesfully wrote common circuit data to common_circuit_data.json");

        let verifier_data_file =
            File::create(path.as_ref().join("verifier_only_circuit_data.json"))?;
        serde_json::to_writer(&verifier_data_file, &self.verifier_data)?;
        info!("Succesfully wrote verifier data to verifier_only_circuit_data.json");

        let proof_file = File::create(path.as_ref().join("proof_with_public_inputs.json"))?;
        serde_json::to_writer(&proof_file, &self.proof)?;
        info!("Succesfully wrote proof to proof_with_public_inputs.json");

        Ok(())
    }
}

impl<
        F: RichField + Extendable<D>,
        InnerConfig: GenericConfig<D, F = F>,
        OuterConfig: GenericConfig<D, F = F>,
        const D: usize,
    > Wrapper<F, InnerConfig, OuterConfig, D>
where
    InnerConfig::Hasher: AlgebraicHasher<F>,
{
    pub fn generate_standard_proof(
        circuit: &CircuitData<F, InnerConfig, D>,
        pw: PartialWitness<F>,
        path: &str,
    ) -> WrappedProof<F, OuterConfig, D> {
        let inner_proof = circuit.prove(pw).unwrap();
        circuit.verify(inner_proof.clone()).unwrap();
        let data = circuit;

        // An inner recursion to standartize the degree
        let mut recursive_builder = CircuitBuilder::<F, D>::new();
        let rec_proof_target = recursive_builder.add_virtual_proof_with_pis(&data.common);
        let rec_verifier_data = recursive_builder
            .api
            .add_virtual_verifier_data(data.common.config.fri_config.cap_height);
        recursive_builder.verify_proof::<InnerConfig>(
            &rec_proof_target,
            &rec_verifier_data,
            &data.common,
        );

        let recursive_data = recursive_builder.build::<InnerConfig>().data;
        let mut pw = PartialWitness::new();
        pw.set_verifier_data_target(&rec_verifier_data, &data.verifier_only);
        pw.set_proof_with_pis_target(&rec_proof_target, &inner_proof);

        let recursive_proof = recursive_data.prove(pw).unwrap();
        recursive_data.verify(recursive_proof.clone()).unwrap();

        // Finally, wrap this in the outer circuit
        let mut wrapper_builder = CircuitBuilder::<F, D>::new();
        let proof_target = wrapper_builder.add_virtual_proof_with_pis(&recursive_data.common);
        let verifier_data = wrapper_builder
            .api
            .add_virtual_verifier_data(recursive_data.common.config.fri_config.cap_height);
        wrapper_builder.verify_proof::<InnerConfig>(
            &proof_target,
            &verifier_data,
            &recursive_data.common,
        );

        let wrapper_data = wrapper_builder.build::<OuterConfig>();

        let mut pw = PartialWitness::new();
        pw.set_verifier_data_target(&verifier_data, &recursive_data.verifier_only);
        pw.set_proof_with_pis_target(&proof_target, &recursive_proof);
        let wrapper_proof = wrapper_data.data.prove(pw).unwrap();
        wrapper_data.data.verify(wrapper_proof.clone()).unwrap();

        // for gate in wrapper_data.data.common.gates.iter() {
        //     println!("{:?}", gate);
        // }

        WrappedProof {
            proof: wrapper_proof,
            common_data: wrapper_data.data.common,
            verifier_data: wrapper_data.data.verifier_only,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use hex::decode;
    use plonky2::field::types::Field;

    use super::*;
    use crate::backend::wrapper::plonky2_config::PoseidonBN128GoldilocksConfig;
    use crate::frontend::builder::CircuitBuilder;
    use crate::frontend::hash::sha::sha256::sha256;
    use crate::prelude::*;
    use crate::utils::setup_logger;

    fn to_bits(msg: Vec<u8>) -> Vec<bool> {
        let mut res = Vec::new();
        for bit in msg {
            let char = bit;
            for j in 0..8 {
                if (char & (1 << (7 - j))) != 0 {
                    res.push(true);
                } else {
                    res.push(false);
                }
            }
        }
        res
    }

    #[test]
    fn test_wrapper() {
        type F = GoldilocksField;
        const D: usize = 2;
        type InnerConfig = PoseidonGoldilocksConfig;
        type OuterConfig = PoseidonBN128GoldilocksConfig;

        setup_logger();

        let build_path = format!("../../zk/gnark-plonky2-verifier/verifier/data");
        let path = format!("{}/wrapper/", build_path);
        let dummy_path = format!("{}/dummy/", build_path);

        let mut builder = CircuitBuilder::<F, D>::new();
        builder.init_field_io();
        let _ = builder.constant::<Variable>(F::ONE);

        // Set up the dummy circuit and wrapper
        let dummy_circuit = builder.build::<InnerConfig>();
        let dummy_input = dummy_circuit.input();
        let (dummy_inner_proof, dummy_output) = dummy_circuit.prove(&dummy_input);
        dummy_circuit.verify(&dummy_inner_proof, &dummy_input, &dummy_output);

        let dummy_wrapper = WrappedCircuit::<F, InnerConfig, OuterConfig, D>::build(dummy_circuit);
        let dummy_wrapped_proof =
            dummy_wrapper.prove(&dummy_inner_proof, &dummy_input, &dummy_output);
        dummy_wrapped_proof.save(dummy_path).unwrap();

        // Set up the circuit and wrapper
        let msg = b"plonky2";
        let msg_bits = to_bits(msg.to_vec());
        let expected_digest = "8943a85083f16e93dc92d6af455841daacdae5081aa3125b614a626df15461eb";
        let digest_bits = to_bits(decode(expected_digest).unwrap());

        let mut builder = CircuitBuilder::<F, D>::new();
        builder.init_field_io();
        let targets = msg_bits
            .iter()
            .map(|b| builder.api.constant_bool(*b))
            .collect::<Vec<_>>();
        let msg_hash = sha256(&mut builder.api, &targets);
        for _ in 0..5 {
            let _msg_hash = sha256(&mut builder.api, &targets);
        }

        for i in 0..digest_bits.len() {
            if digest_bits[i] {
                builder.api.assert_one(msg_hash[i].target);
            } else {
                builder.api.assert_zero(msg_hash[i].target);
            }
        }

        let circuit = builder.build::<InnerConfig>();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);

        let wrapped_circuit = WrappedCircuit::<F, InnerConfig, OuterConfig, D>::build(circuit);

        assert_eq!(
            wrapped_circuit.wrapper_data.common,
            dummy_wrapper.wrapper_data.common
        );

        let wrapped_proof = wrapped_circuit.prove(&proof, &input, &output);
        wrapped_proof.save(path).unwrap();
    }
}
