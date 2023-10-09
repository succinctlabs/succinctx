pub mod args;
pub mod request;
pub mod result;

use std::fs::File;
use std::io::{BufReader, Write};
use std::{fs, path};

use clap::Parser;
use log::info;
use plonky2::field::types::PrimeField64;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig, GenericHashOut};
pub use request::*;
pub use result::*;
use serde::Serialize;
use sha2::Digest;

use self::args::{CompileArgs, ProveArgs};
use crate::backend::circuit::*;
use crate::backend::function::args::{Args, Commands};
use crate::backend::wrapper::wrap::WrappedCircuit;
use crate::frontend::builder::CircuitIO;
use crate::prelude::{CircuitBuilder, GateRegistry, HintRegistry};

const VERIFIER_CONTRACT: &str = include_str!("../../resources/Verifier.sol");

/// `Plonky2xFunction`s have all necessary code for a circuit to be deployed end-to-end.
pub trait Plonky2xFunction {
    /// Builds the circuit and saves it to disk.
    fn compile<L: PlonkParameters<D>, const D: usize>(args: CompileArgs)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>;

    /// Generates a proof for the circuit and saves it to disk.
    fn prove<
        InnerParameters: PlonkParameters<D>,
        OuterParameters: PlonkParameters<D, Field = InnerParameters::Field>,
        const D: usize,
    >(
        args: ProveArgs,
        request: ProofRequest<InnerParameters, D>,
    ) where
        <InnerParameters::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<InnerParameters::Field>,
        OuterParameters::Config: Serialize;

    /// The entry point for the function when using the CLI.
    fn entrypoint();

    /// Returns the verifier contract for the circuit.
    fn verifier(circuit_digest: &str) -> String;
}

impl<C: Circuit> Plonky2xFunction for C {
    fn compile<L: PlonkParameters<D>, const D: usize>(args: CompileArgs)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        // Build the circuit.
        info!("Building circuit...");
        let mut builder = CircuitBuilder::<L, D>::new();
        C::define::<L, D>(&mut builder);
        let circuit = builder.build();
        info!("Successfully built circuit.");
        info!("> Circuit: {}", circuit.id());
        info!("> Degree: {}", circuit.data.common.degree());
        info!("> Number of Gates: {}", circuit.data.common.gates.len());

        // Serialize the circuit to disk.
        let path = format!("{}/main.circuit", args.build_dir);
        let mut generator_registry = HintRegistry::new();
        let mut gate_registry = GateRegistry::new();
        C::register_generators::<L, D>(&mut generator_registry);
        C::register_gates::<L, D>(&mut gate_registry);
        circuit.save(&path, &gate_registry, &generator_registry);
        info!("Successfully saved circuit to disk at {}.", path);

        // Serialize the verifier contract to disk.
        if let CircuitIO::Bytes(_) = circuit.io {
            info!("Building verifier contract...");
            let contract_path = format!("{}/FunctionVerifier.sol", args.build_dir);
            let mut contract_file = File::create(&contract_path).unwrap();

            let circuit_digest_bytes = circuit
                .data
                .verifier_only
                .circuit_digest
                .to_vec()
                .iter()
                .flat_map(|e| e.to_canonical_u64().to_be_bytes())
                .collect::<Vec<u8>>();
            let full_circuit_digest_bytes = circuit
                .data
                .verifier_only
                .constants_sigmas_cap
                .0
                .iter()
                .flat_map(|x| {
                    x.elements
                        .iter()
                        .flat_map(|e| e.to_canonical_u64().to_be_bytes())
                })
                .chain(circuit_digest_bytes.iter().copied())
                .collect::<Vec<u8>>();
            let circuit_digest_hash = sha2::Sha256::digest(full_circuit_digest_bytes);
            assert!(
                circuit_digest_hash.len() <= 32,
                "circuit digest must be <= 32 bytes"
            );

            let mut padded = vec![0u8; 32];
            let digest_len = circuit_digest_hash.len();
            padded[(32 - digest_len)..].copy_from_slice(&circuit_digest_hash);
            let circuit_digest = format!("0x{}", hex::encode(padded));

            let verifier_contract = Self::verifier(&circuit_digest);
            contract_file
                .write_all(verifier_contract.as_bytes())
                .unwrap();
            info!(
                "Successfully saved verifier contract to disk at {}.",
                contract_path
            );
        }
    }

    fn prove<
        InnerParameters: PlonkParameters<D>,
        OuterParameters: PlonkParameters<D, Field = InnerParameters::Field>,
        const D: usize,
    >(
        args: ProveArgs,
        request: ProofRequest<InnerParameters, D>,
    ) where
        <InnerParameters::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<InnerParameters::Field>,
        OuterParameters::Config: Serialize,
    {
        let mut generator_registry = HintRegistry::new();
        let mut gate_registry = GateRegistry::new();
        C::register_generators::<InnerParameters, D>(&mut generator_registry);
        C::register_gates::<InnerParameters, D>(&mut gate_registry);

        let mut path = match request {
            ProofRequest::Bytes(_) => {
                format!("{}/main.circuit", args.build_dir)
            }
            ProofRequest::Elements(ref request) => {
                format!("{}/{}.circuit", args.build_dir, request.data.circuit_id)
            }
            ProofRequest::RecursiveProofs(ref request) => {
                format!("{}/{}.circuit", args.build_dir, request.data.circuit_id)
            }
            _ => todo!(),
        };
        if fs::metadata(&path).is_err() {
            path = format!("{}/main.circuit", args.build_dir);
        }

        info!("Loading circuit from {}...", path);
        let circuit =
            CircuitBuild::<InnerParameters, D>::load(&path, &gate_registry, &generator_registry)
                .unwrap();
        info!("Successfully loaded circuit.");

        let input = request.input();
        let (proof, output) = circuit.prove(&input);
        info!(
            "Successfully generated proof, wrapping proof with {}",
            args.wrapper_path
        );

        if let PublicOutput::Bytes(output_bytes) = output {
            let wrapped_circuit =
                WrappedCircuit::<InnerParameters, OuterParameters, D>::build(circuit);
            let wrapped_proof = wrapped_circuit.prove(&proof).expect("failed to wrap proof");
            wrapped_proof
                .save("wrapped")
                .expect("failed to save wrapped proof");

            // Call go wrapper
            let verifier_output =
                std::process::Command::new(path::Path::new(&args.wrapper_path).join("verifier"))
                    .arg("-prove")
                    .arg("-circuit")
                    .arg("wrapped")
                    .arg("-data")
                    .arg(path::Path::new(&args.wrapper_path))
                    .stdout(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit())
                    .output()
                    .expect("failed to execute process");

            if !verifier_output.status.success() {
                panic!("verifier failed");
            }

            // Read result from gnark verifier
            let file = std::fs::File::open("proof.json").unwrap();
            let rdr = std::io::BufReader::new(file);
            let result_data =
                serde_json::from_reader::<BufReader<File>, BytesResultData>(rdr).unwrap();

            // Write full result with output bytes to output.json
            let result: ProofResult<OuterParameters, D> =
                ProofResult::from_bytes(result_data.proof, output_bytes);
            let json = serde_json::to_string_pretty(&result).unwrap();
            let mut file = File::create("output.json").unwrap();
            file.write_all(json.as_bytes()).unwrap();
            info!("Successfully saved full result to disk at output.json.");
        } else {
            let result = ProofResult::from_proof_output(proof, output);
            let json = serde_json::to_string_pretty(&result).unwrap();
            let mut file = File::create("output.json").unwrap();
            file.write_all(json.as_bytes()).unwrap();
            info!("Successfully saved proof to disk at output.json.");
        }
    }

    /// The entry point for the function when using the CLI.
    fn entrypoint() {
        type L = DefaultParameters;
        const D: usize = 2;

        dotenv::dotenv().ok();
        env_logger::try_init().unwrap_or_default();

        let args = Args::parse();
        match args.command {
            Commands::Compile(args) => {
                Self::compile::<L, D>(args);
            }
            Commands::Prove(args) => {
                let request = ProofRequest::<L, D>::load(&args.input_json);
                Self::prove::<L, Groth16WrapperParameters, D>(args, request);
            }
        }
    }

    fn verifier(circuit_digest: &str) -> String {
        let generated_contract = VERIFIER_CONTRACT
            .replace("pragma solidity ^0.8.0;", "pragma solidity ^0.8.16;")
            .replace("uint256[3] calldata input", "uint256[3] memory input");

        let verifier_contract = "

interface IFunctionVerifier {
    function verify(bytes32 _inputHash, bytes32 _outputHash, bytes memory _proof) external view returns (bool);

    function verificationKeyHash() external pure returns (bytes32);
}

contract FunctionVerifier is IFunctionVerifier, Verifier {

    bytes32 public constant CIRCUIT_DIGEST = {CIRCUIT_DIGEST};

    function verify(bytes32 _inputHash, bytes32 _outputHash, bytes memory _proof) external view returns (bool) {
        (uint256[2] memory a, uint256[2][2] memory b, uint256[2] memory c) =
            abi.decode(_proof, (uint256[2], uint256[2][2], uint256[2]));

        uint256[3] memory input = [uint256(CIRCUIT_DIGEST), uint256(_inputHash), uint256(_outputHash)];
        input[0] = input[0] & ((1 << 253) - 1);
        input[1] = input[1] & ((1 << 253) - 1);
        input[2] = input[2] & ((1 << 253) - 1); 

        return verifyProof(a, b, c, input);
    }

    function verificationKeyHash() external pure returns (bytes32) {
        return keccak256(abi.encode(verifyingKey()));
    }
}
".replace("{CIRCUIT_DIGEST}", circuit_digest);
        generated_contract + &verifier_contract
    }
}
