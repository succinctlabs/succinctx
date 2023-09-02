mod cli;
mod io;
pub mod request;
pub mod result;

use std::fs::File;
use std::io::{BufReader, Write};

use clap::Parser;
use itertools::Itertools;
use log::info;
use plonky2::field::extension::Extendable;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig, PoseidonGoldilocksConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;

use self::cli::{BuildArgs, ProveArgs};
use self::io::FunctionRequest;
use crate::backend::circuit::Circuit;
use crate::backend::function::cli::{Args, Commands};
use crate::backend::function::result::{
    BytesResult, ElementsResult, FunctionResult, RecursiveProofsResult,
};

pub trait CircuitFunction {
    /// Builds the circuit.
    fn build<F, C, const D: usize>() -> Circuit<F, C, D>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>;

    /// Builds the circuit and saves it to disk.
    fn compile<F, C, const D: usize>(args: BuildArgs)
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
    {
        info!("Building circuit...");
        let circuit = Self::build::<F, C, D>();
        info!("Successfully built circuit.");
        info!("> Circuit: {}", circuit.id());
        info!("> Degree: {}", circuit.data.common.degree());
        info!("> Number of Gates: {}", circuit.data.common.gates.len());
        let path = format!("{}/main.circuit", args.build_dir);
        circuit.save(&path);
        info!("Successfully saved circuit to disk at {}.", path);

        info!("Building verifier contract...");
        let contract_path = format!("{}/FunctionVerifier.sol", args.build_dir);
        let mut contract_file = File::create(&contract_path).unwrap();
        let contract = r#"
        pragma solidity ^0.8.16;

        interface IFunctionVerifier {
            function verify(bytes32 _inputHash, bytes32 _outputHash, bytes memory _proof) external view returns (bool);

            function verificationKeyHash() external pure returns (bytes32);
        }

        contract FunctionVerifier is IFunctionVerifier {
            function verify(bytes32, bytes32, bytes memory) external pure returns (bool) {
                return true;
            }

            function verificationKeyHash() external pure returns (bytes32) {
                return keccak256(\"\");
            }
        }
        "#;
        contract_file.write_all(contract.as_bytes()).unwrap();
        info!(
            "Successfully saved verifier contract to disk at {}.",
            contract_path
        );
    }

    fn prove<F, C, const D: usize>(args: ProveArgs) -> FunctionResult
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
    {
        let file = File::open(args.request_json_path).unwrap();
        let reader = BufReader::new(file);
        let request: FunctionRequest = serde_json::from_reader(reader).unwrap();

        let (circuit, input) = match request {
            FunctionRequest::Bytes(ref request) => {
                let path = format!("{}/main.circuit", args.build_dir);
                info!("Loading circuit from {}...", path);
                let circuit = Circuit::<F, C, D>::load(&path).unwrap();
                info!("Successfully loaded circuit.");
                let bytes = hex::decode(request.data.input.clone()).unwrap();
                let mut input = circuit.input();
                input.evm_write_all(&bytes);
                (circuit, input)
            }
            FunctionRequest::Elements(ref request) => {
                let path = format!("{}/main.circuit", args.build_dir);
                info!("Loading circuit from {}...", path);
                let circuit = Circuit::<F, C, D>::load(&path).unwrap();
                info!("Successfully loaded circuit.");
                let elements = request
                    .data
                    .input
                    .iter()
                    .map(|s| F::from_canonical_u64(s.parse::<u64>().unwrap()))
                    .collect_vec();
                let mut input = circuit.input();
                input.write_all(&elements);
                (circuit, input)
            }
            FunctionRequest::RecursiveProofs(ref request) => {
                let path = if request.data.subfunction.is_some() {
                    format!(
                        "{}/{}.circuit",
                        args.build_dir,
                        request.data.subfunction.clone().unwrap()
                    )
                } else {
                    format!("{}/main.circuit", args.build_dir)
                };
                info!("Loading circuit from {}...", path);
                let circuit = Circuit::<F, C, D>::load(&path).unwrap();
                info!("Successfully loaded circuit.");

                let mut input = circuit.input();
                let io = circuit.io.recursive_proof.as_ref().unwrap();
                for i in 0..io.child_circuit_ids.len() {
                    let path = format!("./build/{}.circuit", io.child_circuit_ids[i]);
                    let child_circuit = Circuit::<F, C, D>::load(&path).unwrap();
                    let child_proof = ProofWithPublicInputs::<F, C, D>::from_bytes(
                        hex::decode(request.data.proofs[i].as_str()).unwrap(),
                        &child_circuit.data.common,
                    )
                    .unwrap();
                    input.proof_write(child_proof);
                }
                let input_elements = request
                    .data
                    .input
                    .iter()
                    .map(|s| F::from_canonical_u64(s.parse::<u64>().unwrap()))
                    .collect_vec();
                input.write_all(&input_elements);
                (circuit, input)
            }
        };

        info!("Generating proof...");
        let (proof, output) = circuit.prove(&input);
        info!("Proof generated.");
        circuit.verify(&proof, &input, &output);

        let proof = hex::encode(proof.to_bytes());
        let result: FunctionResult = match request {
            FunctionRequest::Bytes(_) => BytesResult {
                proof,
                output: hex::encode(output.evm_read_all()),
            }
            .into(),
            FunctionRequest::Elements(_) => ElementsResult {
                proof,
                output: output.read_all().iter().map(|e| e.to_string()).collect(),
            }
            .into(),
            FunctionRequest::RecursiveProofs(_) => RecursiveProofsResult {
                proof,
                output: output.read_all().iter().map(|e| e.to_string()).collect(),
            }
            .into(),
        };
        let json = serde_json::to_string_pretty(&result).unwrap();
        let mut file = File::create("result.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
        info!("Successfully saved result to disk at result.json.");

        result
    }

    /// The entry point for the function when using CLI-based tools.
    fn cli() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let args = Args::parse();
        match args.command {
            Commands::Build(args) => {
                Self::compile::<F, C, D>(args);
            }
            Commands::Prove(args) => {
                Self::prove::<F, C, D>(args);
            }
        }
    }

    /// Compiles the circuit and generates a proof for the given request.
    fn test_request_fixture<F, C, const D: usize>(request_json_path: String) -> FunctionResult
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
    {
        let build_args = BuildArgs {
            build_dir: "./build".to_string(),
        };
        Self::compile::<F, C, D>(build_args);
        let prove_args = ProveArgs {
            build_dir: "./build".to_string(),
            request_json_path,
        };
        Self::prove::<F, C, D>(prove_args)
    }
}
