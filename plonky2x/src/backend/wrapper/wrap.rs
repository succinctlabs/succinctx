use core::str::Bytes;
use std::fs::File;
use std::path::Path;

use anyhow::Result;
use log::{debug, info};
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::{BoolTarget, Target};
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
use crate::frontend::hash::sha::sha256::sha256;
use crate::frontend::vars::{Bytes32Variable, EvmVariable};
use crate::prelude::{BoolVariable, ByteVariable, CircuitVariable, Variable};

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
    recursive_circuit: Circuit<F, InnerConfig, D>,
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
        let Some(evm_io) = circuit.io.evm.as_ref() else {
            panic!("CircuitIO must be EVM")
        };

        // Standartize the public inputs/outputs to their hash and verify the circuit recursively
        let mut hash_builder = CircuitBuilder::<F, D>::new();
        let circuit_proof_target = hash_builder.add_virtual_proof_with_pis(&circuit.data.common);
        let circuit_verifier_target = hash_builder
            .api
            .add_virtual_verifier_data(circuit.data.common.config.fri_config.cap_height);
        hash_builder.verify_proof::<InnerConfig>(
            &circuit_proof_target,
            &circuit_verifier_target,
            &circuit.data.common,
        );

        let num_input_targets = evm_io
            .input_bytes
            .iter()
            .map(|x| x.targets().len())
            .sum::<usize>();
        let (input_targets, output_targets) = circuit_proof_target
            .public_inputs
            .split_at(num_input_targets);

        let input_bytes = input_targets
            .chunks_exact(ByteVariable::nb_elements::<F, D>())
            .map(ByteVariable::from_targets)
            .collect::<Vec<_>>();
        let output_bytes = output_targets
            .chunks_exact(ByteVariable::nb_elements::<F, D>())
            .map(ByteVariable::from_targets)
            .collect::<Vec<_>>();

        let input_bits = input_bytes
            .iter()
            .flat_map(|b| b.to_le_bits())
            .map(|b| BoolTarget::new_unsafe(b.targets()[0]))
            .collect::<Vec<_>>();

        let output_bits = output_bytes
            .iter()
            .flat_map(|b| b.to_le_bits())
            .map(|b| BoolTarget::new_unsafe(b.targets()[0]))
            .collect::<Vec<_>>();

        let input_hash: [Target; 256] = sha256(&mut hash_builder.api, &input_bits)
            .into_iter()
            .map(|x| x.target)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let output_hash: [Target; 256] = sha256(&mut hash_builder.api, &output_bits)
            .into_iter()
            .map(|x| x.target)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let input_hash_bytes = Bytes32Variable::from_targets(&input_hash);
        let output_hash_bytes = Bytes32Variable::from_targets(&output_hash);

        hash_builder.write(input_hash_bytes);
        hash_builder.write(output_hash_bytes);

        let hash_circuit = hash_builder.build::<InnerConfig>();

        // An inner recursion to standartize the degree
        let mut recursive_builder = CircuitBuilder::<F, D>::new();
        let circuit_proof_target =
            recursive_builder.add_virtual_proof_with_pis(&hash_circuit.data.common);
        let circuit_verifier_target = recursive_builder
            .api
            .add_virtual_verifier_data(hash_circuit.data.common.config.fri_config.cap_height);
        recursive_builder.verify_proof::<InnerConfig>(
            &circuit_proof_target,
            &circuit_verifier_target,
            &hash_circuit.data.common,
        );

        let recursive_circuit = recursive_builder.build::<InnerConfig>();
        debug!(
            "Recursive circuit degree: {}",
            recursive_circuit.data.common.degree()
        );

        // Finally, wrap this in the outer circuit
        let mut wrapper_builder = CircuitBuilder::<F, D>::new();
        let proof_target =
            wrapper_builder.add_virtual_proof_with_pis(&recursive_circuit.data.common);
        let verifier_target = wrapper_builder
            .api
            .add_virtual_verifier_data(recursive_circuit.data.common.config.fri_config.cap_height);
        wrapper_builder.verify_proof::<InnerConfig>(
            &proof_target,
            &verifier_target,
            &recursive_circuit.data.common,
        );

        let wrapper_data = wrapper_builder.build::<OuterConfig>().data;
        debug!("Wrapped circuit degree: {}", wrapper_data.common.degree());

        Self {
            circuit,
            recursive_circuit,
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
    ) -> WrappedOutput<F, OuterConfig, D> {
        let mut pw = PartialWitness::new();
        pw.set_verifier_data_target(
            &self.circuit_verifier_target,
            &self.circuit.data.verifier_only,
        );
        pw.set_proof_with_pis_target(&self.circuit_proof_target, &inner_proof);

        let recursive_proof = self.recursive_circuit.data.prove(pw).unwrap();
        self.recursive_circuit
            .data
            .verify(recursive_proof.clone())
            .unwrap();
        debug!("Successfully verified recursive proof");

        let mut pw = PartialWitness::new();
        pw.set_verifier_data_target(
            &self.verifier_target,
            &self.recursive_circuit.data.verifier_only,
        );
        pw.set_proof_with_pis_target(&self.proof_target, &recursive_proof);

        let proof = self.wrapper_data.prove(pw).unwrap();
        self.wrapper_data.verify(proof.clone()).unwrap();

        WrappedOutput {
            proof,
            common_data: self.wrapper_data.common.clone(),
            verifier_data: self.wrapper_data.verifier_only.clone(),
        }
    }
}

#[derive(Debug)]
pub struct WrappedOutput<F: RichField + Extendable<D>, C: GenericConfig<D, F = F>, const D: usize> {
    pub proof: ProofWithPublicInputs<F, C, D>,
    pub common_data: CommonCircuitData<F, D>,
    pub verifier_data: VerifierOnlyCircuitData<C, D>,
}

impl<F: RichField + Extendable<D>, C: GenericConfig<D, F = F>, const D: usize>
    WrappedOutput<F, C, D>
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

#[cfg(test)]
mod tests {
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
        builder.init_evm_io();
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
        // builder.init_evm_io();
        let targets = msg_bits
            .iter()
            .map(|b| builder.api.constant_bool(*b))
            .collect::<Vec<_>>();
        let msg_hash = sha256(&mut builder.api, &targets);
        for _ in 0..5 {
            let _msg_hash = sha256(&mut builder.api, &targets);
        }

        let a = builder.evm_read::<ByteVariable>();
        builder.evm_write(a);

        for i in 0..digest_bits.len() {
            if digest_bits[i] {
                builder.api.assert_one(msg_hash[i].target);
            } else {
                builder.api.assert_zero(msg_hash[i].target);
            }
        }

        let circuit = builder.build::<InnerConfig>();
        let mut input = circuit.input();
        input.evm_write::<ByteVariable>(0u8);
        let (proof, output) = circuit.prove(&input);

        let wrapped_circuit = WrappedCircuit::<F, InnerConfig, OuterConfig, D>::build(circuit);

        assert_eq!(
            wrapped_circuit.wrapper_data.common,
            dummy_wrapper.wrapper_data.common,
        );

        let wrapped_proof = wrapped_circuit.prove(&proof, &input, &output);
        wrapped_proof.save(path).unwrap();
    }
}
