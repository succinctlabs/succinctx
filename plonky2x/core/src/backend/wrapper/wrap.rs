use std::fs::{self, File};
use std::path::Path;

use anyhow::Result;
use log::{debug, info};
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_data::{
    CommonCircuitData, VerifierCircuitTarget, VerifierOnlyCircuitData,
};
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::{ProofWithPublicInputs, ProofWithPublicInputsTarget};
use serde::Serialize;

use crate::backend::circuit::{CircuitBuild, PlonkParameters};
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::{ByteVariable, CircuitVariable, Variable};
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
    pub wrapper_circuit: CircuitBuild<OuterParameters, D>,
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
        // Standartize the public inputs/outputs to their hash and verify the circuit recursively.
        let mut hash_builder = CircuitBuilder::<InnerParameters, D>::new();
        let circuit_proof_target = hash_builder.add_virtual_proof_with_pis(&circuit.data.common);
        let circuit_verifier_target =
            hash_builder.constant_verifier_data::<InnerParameters>(&circuit.data);
        hash_builder.verify_proof::<InnerParameters>(
            &circuit_proof_target,
            &circuit_verifier_target,
            &circuit.data.common,
        );

        let num_input_targets = circuit.io.input().len();
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

        hash_builder.watch_slice(&input_bytes, "input_bytes");
        hash_builder.watch_slice(&output_bytes, "output_bytes");

        let input_hash = hash_builder.curta_sha256(&input_bytes);
        let output_hash = hash_builder.curta_sha256(&output_bytes);

        hash_builder.watch(&input_hash, "input_hash");
        hash_builder.watch(&output_hash, "output_hash");

        // We must truncate the top 3 bits because in the gnark-plonky2-verifier, the input_hash
        // and output_hash are both represented as 1 field element in the BN254 field to reduce
        // onchain verification costs.
        let input_hash_zeroed = hash_builder.mask_be_bits(input_hash, 3);
        let output_hash_zeroed = hash_builder.mask_be_bits(output_hash, 3);

        hash_builder.watch(&input_hash_zeroed, "input_hash_truncated");
        hash_builder.watch(&output_hash_zeroed, "output_hash_truncated");

        let input_vars = input_hash_zeroed
            .as_bytes()
            .iter()
            .map(|b| b.to_variable(&mut hash_builder))
            .collect::<Vec<Variable>>();

        let output_vars = output_hash_zeroed
            .as_bytes()
            .iter()
            .map(|b| b.to_variable(&mut hash_builder))
            .collect::<Vec<Variable>>();

        hash_builder.watch_slice(&input_vars, "input_hash_truncated as vars");
        hash_builder.watch_slice(&output_vars, "output_hash_truncated as vars");

        // Write input_hash, output_hash to public_inputs. In the gnark-plonky2-verifier, these
        // 64 bytes get summed to 2 field elements that correspond to the input_hash and output_hash
        // respectively as public inputs.
        input_vars
            .clone()
            .into_iter()
            .chain(output_vars)
            .for_each(|v| {
                hash_builder.write(v);
            });
        let hash_circuit = hash_builder.build();

        // An inner recursion to standardize the degree.
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
        assert_eq!(hash_proof_target.public_inputs.len(), 32usize * 2);

        recursive_builder
            .api
            .register_public_inputs(&hash_proof_target.public_inputs);

        let recursive_circuit = recursive_builder.build();
        debug!(
            "Recursive circuit degree: {}",
            recursive_circuit.data.common.degree()
        );

        // Finally, wrap this in the outer circuit.
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

        wrapper_builder
            .api
            .register_public_inputs(&proof_target.public_inputs);

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

        let (hash_proof, _) = self.hash_circuit.prove_with_partial_witness(pw);
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
    use super::*;
    use crate::backend::circuit::{DefaultParameters, Groth16WrapperParameters};
    use crate::utils;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_wrapper() {
        const D: usize = 2;
        type InnerParameters = DefaultParameters;
        type OuterParameters = Groth16WrapperParameters;

        utils::setup_logger();

        let build_path = "../verifier/data".to_string();
        let path = format!("{}/test_circuit/", build_path);
        let dummy_path = format!("{}/dummy/", build_path);

        // Create an inner circuit for verification.
        let mut builder = CircuitBuilder::<DefaultParameters, 2>::new();
        let a = builder.evm_read::<ByteVariable>();
        let b = builder.evm_read::<ByteVariable>();
        let c = builder.xor(a, b);
        builder.evm_write(c);

        // Set up the dummy circuit and wrapper.
        let dummy_circuit = builder.build();
        let mut dummy_input = dummy_circuit.input();
        dummy_input.evm_write::<ByteVariable>(0u8);
        dummy_input.evm_write::<ByteVariable>(1u8);
        let (dummy_inner_proof, dummy_output) = dummy_circuit.prove(&dummy_input);
        dummy_circuit.verify(&dummy_inner_proof, &dummy_input, &dummy_output);
        println!("Verified dummy_circuit");

        let dummy_wrapper =
            WrappedCircuit::<InnerParameters, OuterParameters, D>::build(dummy_circuit);
        let dummy_wrapped_proof = dummy_wrapper.prove(&dummy_inner_proof).unwrap();
        dummy_wrapped_proof.save(dummy_path).unwrap();
        println!("Saved dummy_circuit");

        // Set up a inner circuit and wrapper.
        let mut builder = CircuitBuilder::<DefaultParameters, 2>::new();
        let a = builder.evm_read::<ByteVariable>();
        let _ = builder.evm_read::<ByteVariable>();
        builder.evm_write(a);

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
