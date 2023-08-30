mod cli;
mod io;

use std::fs::File;
use std::io::{Read, Write};

use clap::Parser;
use curta::math::prelude::PrimeField64;
pub use io::{FunctionInput, FunctionOutput};
use itertools::Itertools;
use log::info;
use plonky2::field::extension::Extendable;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig, PoseidonGoldilocksConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;

use self::cli::{BuildArgs, ProveArgs};
use crate::backend::circuit::Circuit;
use crate::backend::function::cli::{Args, Commands};
use crate::backend::prover::remote::ContextData;

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
        let contract = "pragma solidity ^0.8.16;

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
";
        contract_file.write_all(contract.as_bytes()).unwrap();
        info!(
            "Successfully saved verifier contract to disk at {}.",
            contract_path
        );
    }

    /// Generates a proof with evm-based inputs and outputs.
    fn prove_with_evm_io<F, C, const D: usize>(args: ProveArgs, bytes: Vec<u8>)
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
    {
        let path = format!("{}/main.circuit", args.build_dir);
        info!("Loading circuit from {}...", path);
        let circuit = Circuit::<F, C, D>::load(&path).unwrap();
        info!("Successfully loaded circuit.");

        let mut input = circuit.input();
        input.evm_write_all(&bytes);

        info!("Generating proof...");
        let (proof, output) = circuit.prove(&input);
        info!("Proof generated.");
        circuit.verify(&proof, &input, &output);
        info!("Proof verified.");
        let output_bytes = output.evm_read_all();

        let function_output = FunctionOutput {
            bytes: Some(hex::encode(output_bytes.clone())),
            elements: None,
            proof: hex::encode(proof.to_bytes()),
        };
        let json = serde_json::to_string_pretty(&function_output).unwrap();
        let mut file = File::create("output.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
        info!(
            "Succesfully wrote output of {} bytes and proof to output.json.",
            output_bytes.len()
        );
    }

    /// Generates a proof with field-based inputs and outputs.
    fn prove_with_field_io<F, C, const D: usize>(args: ProveArgs, elements: Vec<F>)
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
    {
        let path = format!("{}/main.circuit", args.build_dir);
        info!("Loading circuit from {}...", path);
        let circuit = Circuit::<F, C, D>::load(&path).unwrap();
        info!("Successfully loaded circuit.");

        let mut input = circuit.input();
        input.write_all(&elements);

        info!("Generating proof...");
        let (proof, output) = circuit.prove(&input);
        info!("Proof generated.");
        circuit.verify(&proof, &input, &output);
        info!("Proof verified.");
        let output_elements = output.read_all();

        let function_output = FunctionOutput {
            bytes: None,
            elements: Some(
                output_elements
                    .iter()
                    .map(|e| e.as_canonical_u64())
                    .collect(),
            ),
            proof: hex::encode(proof.to_bytes()),
        };

        ProofWithPublicInputs::<F, C, D>::from_bytes(
            hex::decode(function_output.clone().proof).unwrap(),
            &circuit.data.common,
        )
        .unwrap();
        let json = serde_json::to_string_pretty(&function_output).unwrap();
        let mut file = File::create("output.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
        info!(
            "Succesfully wrote output of {} elements and proof to output.json.",
            output_elements.len()
        );
    }

    /// Reads the function input from a JSON file path.
    fn read_function_input(input_json: String) -> FunctionInput {
        let mut file = File::open(input_json).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        serde_json::from_str(&data).unwrap()
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
                let input = Self::read_function_input(args.clone().input_json);
                if input.bytes.is_some() {
                    Self::prove_with_evm_io::<F, C, D>(args, input.bytes());
                } else if input.elements.is_some() {
                    Self::prove_with_field_io::<F, C, D>(args, input.elements());
                } else {
                    panic!("No input bytes or elements found in input.json.");
                }
            }
            Commands::ProveChild(_) => {
                let mut file = File::open("context").unwrap();
                let mut context = String::new();
                file.read_to_string(&mut context).unwrap();
                let context: ContextData = serde_json::from_str(context.as_str()).unwrap();
                let circuit_path = format!("./build/{}.circuit", context.circuit_id);

                if context.tag == "map" {
                    let circuit = Circuit::<F, C, D>::load(circuit_path.as_str()).unwrap();
                    let input_values = context
                        .input
                        .iter()
                        .map(|s| F::from_canonical_u64(s.parse::<u64>().unwrap()))
                        .collect_vec();
                    let mut input = circuit.input();
                    input.write_all(&input_values);
                    let (proof, output) = circuit.prove(&input);
                    circuit.verify(&proof, &input, &output);
                    let file_path = "./proof.json";
                    let json = serde_json::to_string_pretty(&proof).unwrap();
                    std::fs::write(file_path, json).unwrap();
                    println!("Successfully generated proof.");
                } else if context.tag == "reduce" {
                    let circuit = Circuit::<F, C, D>::load(circuit_path.as_str()).unwrap();
                    let io = circuit.io.recursive_proof.as_ref().unwrap();
                    let mut input = circuit.input();
                    for i in 0..io.child_circuit_ids.len() {
                        let path = format!("./build/{}.circuit", io.child_circuit_ids[i]);
                        let child_circuit = Circuit::<F, C, D>::load(&path).unwrap();
                        let proof = ProofWithPublicInputs::<F, C, D>::from_bytes(
                            hex::decode(context.input[i].as_str()).unwrap(),
                            &child_circuit.data.common,
                        )
                        .unwrap();
                        input.proof_write(proof);
                    }

                    let input_values = context
                        .input
                        .iter()
                        .map(|s| F::from_canonical_u64(s.parse::<u64>().unwrap()))
                        .collect_vec();
                    input.write_all(&input_values);

                    let (proof, output) = circuit.prove(&input);
                    circuit.verify(&proof, &input, &output);
                    let file_path = "./proof.json";
                    let json = serde_json::to_string_pretty(&proof).unwrap();
                    std::fs::write(file_path, json).unwrap();
                    println!("Successfully generated proof.");
                }
            }
        }
    }

    fn test<F, C, const D: usize>(input_json: String)
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
            input_json: input_json.clone(),
        };
        let input = Self::read_function_input(input_json);
        if input.bytes.is_some() {
            Self::prove_with_evm_io::<F, C, D>(prove_args, input.bytes());
        } else if input.elements.is_some() {
            Self::prove_with_field_io::<F, C, D>(prove_args, input.elements());
        } else {
            panic!("No input bytes or field elements found in input.json.")
        }
    }
}
