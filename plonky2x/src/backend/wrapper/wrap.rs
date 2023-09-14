use core::iter::once;
use std::fs::{self, File};
use std::path::Path;

use anyhow::Result;
use log::{debug, info};
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_data::{
    CommonCircuitData, VerifierCircuitTarget, VerifierOnlyCircuitData,
};
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::{ProofWithPublicInputs, ProofWithPublicInputsTarget};
use serde::Serialize;

use crate::backend::circuit::{CircuitBuild, PlonkParameters};
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::hash::sha::sha256::sha256;
use crate::frontend::vars::{ByteVariable, Bytes32Variable, CircuitVariable, EvmVariable};

#[derive(Debug)]
pub struct WrappedCircuit<
    InnerParameters: PlonkParameters<D>,
    OuterParameters: PlonkParameters<D, Field = InnerParameters::Field>,
    const D: usize,
> where
    <InnerParameters::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<InnerParameters::Field>,
{
    circuit: CircuitBuild<InnerParameters, D>,
    hash_circuit: CircuitBuild<InnerParameters, D>,
    circuit_proof_target: ProofWithPublicInputsTarget<D>,
    circuit_verifier_target: VerifierCircuitTarget,
    recursive_circuit: CircuitBuild<InnerParameters, D>,
    hash_verifier_target: VerifierCircuitTarget,
    hash_proof_target: ProofWithPublicInputsTarget<D>,
    wrapper_circuit: CircuitBuild<OuterParameters, D>,
    proof_target: ProofWithPublicInputsTarget<D>,
    verifier_target: VerifierCircuitTarget,
}

impl<
        InnerParameters: PlonkParameters<D>,
        OuterParameters: PlonkParameters<D, Field = InnerParameters::Field>,
        const D: usize,
    > WrappedCircuit<InnerParameters, OuterParameters, D>
where
    <InnerParameters::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<InnerParameters::Field>,
{
    pub fn build(circuit: CircuitBuild<InnerParameters, D>) -> Self {
        // Standartize the public inputs/outputs to their hash and verify the circuit recursively
        let mut hash_builder = CircuitBuilder::<InnerParameters, D>::new();
        let circuit_proof_target = hash_builder.add_virtual_proof_with_pis(&circuit.data.common);
        let circuit_verifier_target =
            hash_builder.constant_verifier_data::<InnerParameters>(&circuit.data);
        hash_builder.verify_proof::<InnerParameters>(
            &circuit_proof_target,
            &circuit_verifier_target,
            &circuit.data.common,
        );

        let circuit_digest_bits = circuit_verifier_target
            .constants_sigmas_cap
            .0
            .iter()
            .chain(once(&circuit_verifier_target.circuit_digest))
            .flat_map(|x| x.elements)
            .flat_map(|x| {
                let mut bits = hash_builder.api.split_le(x, 64);
                bits.reverse();
                bits
            })
            .collect::<Vec<_>>();

        let circuit_digest_hash: [Target; 256] =
            sha256(&mut hash_builder.api, &circuit_digest_bits)
                .into_iter()
                .map(|x| x.target)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

        let circuit_digest_bytes = Bytes32Variable::from_targets(&circuit_digest_hash);
        hash_builder.write(circuit_digest_bytes);

        let num_input_targets = circuit
            .io
            .input()
            .iter()
            .map(|x| x.targets().len())
            .sum::<usize>();
        let (input_targets, output_targets) = circuit_proof_target
            .public_inputs
            .split_at(num_input_targets);

        let input_bytes = input_targets
            .chunks_exact(ByteVariable::nb_elements())
            .map(ByteVariable::from_targets)
            .collect::<Vec<_>>();
        let output_bytes = output_targets
            .chunks_exact(ByteVariable::nb_elements())
            .map(ByteVariable::from_targets)
            .collect::<Vec<_>>();

        let input_bits = input_bytes
            .iter()
            .flat_map(|b| b.to_le_bits::<InnerParameters, D>(&mut hash_builder))
            .map(|b| BoolTarget::new_unsafe(b.targets()[0]))
            .collect::<Vec<_>>();

        let output_bits = output_bytes
            .iter()
            .flat_map(|b| b.to_le_bits(&mut hash_builder))
            .map(|b| BoolTarget::new_unsafe(b.targets()[0]))
            .collect::<Vec<_>>();

        let mut input_hash = sha256(&mut hash_builder.api, &input_bits)
            .into_iter()
            .map(|x| x.target)
            .collect::<Vec<_>>();
        // Remove the last bit to make the hash 255 bits and replace with zero
        input_hash.pop();
        input_hash.push(hash_builder.api.constant_bool(false).target);

        let mut output_hash = sha256(&mut hash_builder.api, &output_bits)
            .into_iter()
            .map(|x| x.target)
            .collect::<Vec<_>>();
        // Remove the last bit to make the hash 255 bits and replace with zero
        output_hash.pop();
        output_hash.push(hash_builder.api.constant_bool(false).target);

        let input_hash_truncated: [Target; 256] = input_hash.try_into().unwrap();
        let output_hash_truncated: [Target; 256] = output_hash.try_into().unwrap();

        let input_hash_bytes = Bytes32Variable::from_targets(&input_hash_truncated);
        let output_hash_bytes = Bytes32Variable::from_targets(&output_hash_truncated);

        hash_builder.write(input_hash_bytes);
        hash_builder.write(output_hash_bytes);

        let hash_circuit = hash_builder.build();

        // An inner recursion to standartize the degree
        let mut recursive_builder = CircuitBuilder::<InnerParameters, D>::new();
        let hash_proof_target =
            recursive_builder.add_virtual_proof_with_pis(&hash_circuit.data.common);
        let hash_verifier_target =
            recursive_builder.constant_verifier_data::<InnerParameters>(&hash_circuit.data);
        recursive_builder.verify_proof::<InnerParameters>(
            &hash_proof_target,
            &hash_verifier_target,
            &hash_circuit.data.common,
        );

        recursive_builder.register_public_inputs(&hash_proof_target.public_inputs);

        let recursive_circuit = recursive_builder.build();
        debug!(
            "Recursive circuit degree: {}",
            recursive_circuit.data.common.degree()
        );

        // Finally, wrap this in the outer circuit
        let mut wrapper_builder = CircuitBuilder::<OuterParameters, D>::new();
        let proof_target =
            wrapper_builder.add_virtual_proof_with_pis(&recursive_circuit.data.common);
        let verifier_target =
            wrapper_builder.constant_verifier_data::<InnerParameters>(&recursive_circuit.data);
        wrapper_builder.verify_proof::<InnerParameters>(
            &proof_target,
            &verifier_target,
            &recursive_circuit.data.common,
        );

        wrapper_builder.register_public_inputs(&proof_target.public_inputs);

        let wrapper_circuit = wrapper_builder.build();
        debug!(
            "Wrapped circuit degree: {}",
            wrapper_circuit.data.common.degree()
        );

        Self {
            circuit,
            hash_circuit,
            recursive_circuit,
            circuit_proof_target,
            circuit_verifier_target,
            hash_proof_target,
            hash_verifier_target,
            wrapper_circuit,
            proof_target,
            verifier_target,
        }
    }

    pub fn prove(
        &self,
        inner_proof: &ProofWithPublicInputs<InnerParameters::Field, InnerParameters::Config, D>,
    ) -> Result<WrappedOutput<OuterParameters, D>> {
        let mut pw = PartialWitness::new();
        pw.set_verifier_data_target(
            &self.circuit_verifier_target,
            &self.circuit.data.verifier_only,
        );
        pw.set_proof_with_pis_target(&self.circuit_proof_target, inner_proof);

        let hash_proof = self.hash_circuit.data.prove(pw)?;
        self.hash_circuit.data.verify(hash_proof.clone())?;
        debug!("Successfully verified hash proof");

        let mut pw = PartialWitness::new();
        pw.set_verifier_data_target(
            &self.hash_verifier_target,
            &self.hash_circuit.data.verifier_only,
        );
        pw.set_proof_with_pis_target(&self.hash_proof_target, &hash_proof);

        let recursive_proof = self.recursive_circuit.data.prove(pw)?;
        self.recursive_circuit
            .data
            .verify(recursive_proof.clone())?;
        debug!("Successfully verified recursive proof");

        let mut pw = PartialWitness::new();
        pw.set_verifier_data_target(
            &self.verifier_target,
            &self.recursive_circuit.data.verifier_only,
        );
        pw.set_proof_with_pis_target(&self.proof_target, &recursive_proof);

        let proof = self.wrapper_circuit.data.prove(pw)?;
        self.wrapper_circuit.data.verify(proof.clone())?;
        debug!("Successfully verified wrapper proof");

        Ok(WrappedOutput {
            proof,
            common_data: self.wrapper_circuit.data.common.clone(),
            verifier_data: self.wrapper_circuit.data.verifier_only.clone(),
        })
    }
}

#[derive(Debug)]
pub struct WrappedOutput<L: PlonkParameters<D>, const D: usize> {
    pub proof: ProofWithPublicInputs<L::Field, L::Config, D>,
    pub common_data: CommonCircuitData<L::Field, D>,
    pub verifier_data: VerifierOnlyCircuitData<L::Config, D>,
}

impl<L: PlonkParameters<D>, const D: usize> WrappedOutput<L, D> {
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()>
    where
        L::Config: Serialize,
    {
        if !path.as_ref().exists() {
            fs::create_dir_all(&path)?;
        }
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

#[cfg(test)]
mod tests {
    use hex::decode;
    use plonky2::field::types::Field;

    use super::*;
    use crate::backend::circuit::{DefaultParameters, Groth16VerifierParameters};
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
        type InnerParameters = DefaultParameters;
        type OuterParameters = Groth16VerifierParameters;

        setup_logger();

        let build_path = "../plonky2x-verifier/data".to_string();
        let path = format!("{}/test_circuit/", build_path);
        let dummy_path = format!("{}/dummy/", build_path);

        let mut builder = CircuitBuilder::<DefaultParameters, 2>::new();
        let _ = builder.constant::<Variable>(F::ONE);

        // Set up the dummy circuit and wrapper
        let dummy_circuit = builder.build();
        let dummy_input = dummy_circuit.input();
        let (dummy_inner_proof, dummy_output) = dummy_circuit.prove(&dummy_input);
        dummy_circuit.verify(&dummy_inner_proof, &dummy_input, &dummy_output);

        let dummy_wrapper =
            WrappedCircuit::<InnerParameters, OuterParameters, D>::build(dummy_circuit);
        let dummy_wrapped_proof = dummy_wrapper.prove(&dummy_inner_proof).unwrap();
        dummy_wrapped_proof.save(dummy_path).unwrap();

        // Set up the circuit and wrapper
        let msg = b"plonky2";
        let msg_bits = to_bits(msg.to_vec());
        let expected_digest = "8943a85083f16e93dc92d6af455841daacdae5081aa3125b614a626df15461eb";
        let digest_bits = to_bits(decode(expected_digest).unwrap());

        let mut builder = CircuitBuilder::<DefaultParameters, 2>::new();
        let targets = msg_bits
            .iter()
            .map(|b| builder.api.constant_bool(*b))
            .collect::<Vec<_>>();
        let msg_hash = sha256(&mut builder.api, &targets);
        for _ in 0..5 {
            let _msg_hash = sha256(&mut builder.api, &targets);
        }

        let a = builder.evm_read::<ByteVariable>();
        let _ = builder.evm_read::<ByteVariable>();
        builder.evm_write(a);

        for i in 0..digest_bits.len() {
            if digest_bits[i] {
                builder.api.assert_one(msg_hash[i].target);
            } else {
                builder.api.assert_zero(msg_hash[i].target);
            }
        }

        let circuit = builder.build();
        let mut input = circuit.input();
        input.evm_write::<ByteVariable>(0u8);
        input.evm_write::<ByteVariable>(0u8);
        let (proof, _output) = circuit.prove(&input);

        let wrapped_circuit = WrappedCircuit::<InnerParameters, OuterParameters, D>::build(circuit);

        assert_eq!(
            wrapped_circuit.wrapper_circuit.data.common,
            dummy_wrapper.wrapper_circuit.data.common,
        );

        let wrapped_proof = wrapped_circuit.prove(&proof).unwrap();
        wrapped_proof.save(path).unwrap();
    }
}
