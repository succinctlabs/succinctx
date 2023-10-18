pub mod args;
pub mod request;
pub mod result;

use std::fs::File;
use std::io::{BufReader, Write};
use std::{fs, path};

use clap::Parser;
use log::info;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig, GenericHashOut};
pub use request::*;
pub use result::*;
use serde::Serialize;

use self::args::{BuildArgs, ProveArgs};
use crate::backend::circuit::*;
use crate::backend::function::args::{Args, Commands};
use crate::backend::wrapper::wrap::WrappedCircuit;
use crate::frontend::builder::CircuitIO;
use crate::prelude::{CircuitBuilder, GateRegistry, HintRegistry};

/// `Plonky2xFunction`s have all necessary code for a circuit to be deployed end-to-end.
pub trait Plonky2xFunction {
    /// Builds the circuit and saves it to disk.
    fn build<
        L: PlonkParameters<D>,
        WrapperParameters: PlonkParameters<D, Field = L::Field>,
        const D: usize,
    >(
        args: BuildArgs,
    ) where
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
    fn verifier(circuit_digest: &str, wrapper_path: &str) -> String;
}

impl<C: Circuit> Plonky2xFunction for C {
    fn build<
        L: PlonkParameters<D>,
        WrapperParameters: PlonkParameters<D, Field = L::Field>,
        const D: usize,
    >(
        args: BuildArgs,
    ) where
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

            // The wrapper circuit digest will get saved in the Solidity smart contract, which will
            // use this value as a public input `VerifierDigest` in the gnark plonky2 verifier.
            info!("First building wrapper circuit to get the wrapper circuit digest...");
            let wrapped_circuit = WrappedCircuit::<L, WrapperParameters, D>::build(circuit);

            // to_bytes() returns the representation as LE, but we want to save it on-chain as BE
            // because that is the format of the public input to the gnark plonky2 verifier.
            let mut circuit_digest_bytes = wrapped_circuit
                .wrapper_circuit
                .data
                .verifier_only
                .circuit_digest
                .to_bytes();
            circuit_digest_bytes.reverse();

            // The VerifierDigest is stored onchain as a bytes32, so we need to pad it with 0s
            // to store it in the solidity smart contract.
            //
            // Note that we don't need to do any sort of truncation of the most significant bits
            // because the circuit digest already lives in the bn254 field because the prover config
            // uses the Poseidon bn254 hasher.
            //
            // In the solidity smart contract we should not truncate the 3 most significant bits
            // like we do with input_hash and output_hash as the circuit digest has a small
            // probability of being greater than 2^253 given that the field modulus is 254 bits.
            let mut padded = vec![0u8; 32];
            let digest_len = circuit_digest_bytes.len();
            padded[(32 - digest_len)..].copy_from_slice(&circuit_digest_bytes);
            let circuit_digest = format!("0x{}", hex::encode(padded));

            let verifier_contract = Self::verifier(&circuit_digest, &args.wrapper_path);
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
        let gnark_wrapper_process = if !args.wrapper_path.is_empty() {
            // If the wrapper path is provided, then we know we will be wrapping the proof
            let child_process =
                std::process::Command::new(path::Path::new(&args.wrapper_path).join("verifier"))
                    .arg("-prove")
                    .arg("-data")
                    .arg(path::Path::new(&args.wrapper_path))
                    .stdout(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit())
                    .stdin(std::process::Stdio::piped())
                    .spawn()
                    .expect("Failed to start gnark wrapper process");
            Some(child_process)
        } else {
            None
        };

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

        if let PublicInput::Bytes(input_bytes) = input {
            info!("Input Bytes: 0x{}", hex::encode(input_bytes));
        }

        if let PublicOutput::Bytes(output_bytes) = output {
            // It's quite fast (~5-10 seconds) to rebuild the wrapped circuit. Because of this we
            // choose to rebuild here instead of loading from disk.
            info!("Output Bytes: 0x{}", hex::encode(output_bytes.clone()));
            let wrapped_circuit =
                WrappedCircuit::<InnerParameters, OuterParameters, D>::build(circuit);
            let wrapped_proof = wrapped_circuit.prove(&proof).expect("failed to wrap proof");
            wrapped_proof
                .save("wrapped")
                .expect("failed to save wrapped proof");

            // The gnark_wrapper_process should have been started.
            let mut gnark_wrapper_process = gnark_wrapper_process.unwrap();
            let mut stdin_opt = None;
            while stdin_opt.is_none() {
                stdin_opt = match gnark_wrapper_process.stdin.as_mut() {
                    Some(stdin) => {
                        info!("Got stdin of child process");
                        Some(stdin)
                    }
                    None => {
                        info!("Failed to open stdin of gnark wrapper. Retrying...");
                        std::thread::sleep(std::time::Duration::from_secs(1));
                        None
                    }
                };
            }

            let stdin = stdin_opt.unwrap();
            stdin
                .write_all(b"wrapped\n")
                .expect("Failed to write to stdin");
            let verifier_output = gnark_wrapper_process
                .wait_with_output()
                .expect("failed to execute process");

            if !verifier_output.status.success() {
                panic!("verifier failed");
            }

            // Read result from gnark verifier.
            let file = std::fs::File::open("proof.json").unwrap();
            let rdr = std::io::BufReader::new(file);
            let result_data =
                serde_json::from_reader::<BufReader<File>, BytesResultData>(rdr).unwrap();

            // Write full result with output bytes to output.json.
            let result: ProofResult<OuterParameters, D> =
                ProofResult::from_bytes(result_data.proof, output_bytes);
            let json = serde_json::to_string_pretty(&result).unwrap();
            info!("output.json:\n{}", json);
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
            Commands::Build(args) => {
                Self::build::<L, Groth16WrapperParameters, D>(args);
            }
            Commands::Prove(args) => {
                let request = ProofRequest::<L, D>::load(&args.input_json);
                Self::prove::<L, Groth16WrapperParameters, D>(args, request);
            }
        }
    }

    fn verifier(circuit_digest: &str, wrapper_path: &str) -> String {
        let wrapper_verifier_path = format!("{}/Verifier.sol", wrapper_path);
        let wrapper_verifier_contract = fs::read_to_string(wrapper_verifier_path)
            .expect("Failed to read wrapper_verifier_path");
        let generated_contract = wrapper_verifier_contract
            .replace("pragma solidity ^0.8.19;", "pragma solidity ^0.8.16;")
            .replace("function Verify", "function verifyProof");

        let verifier_contract = "

interface IFunctionVerifier {
    function verify(bytes32 _inputHash, bytes32 _outputHash, bytes memory _proof) external view returns (bool);

    function verificationKeyHash() external pure returns (bytes32);
}

contract FunctionVerifier is IFunctionVerifier, PlonkVerifier {

    bytes32 public constant CIRCUIT_DIGEST = {CIRCUIT_DIGEST};

    function verify(bytes32 _inputHash, bytes32 _outputHash, bytes memory _proof) external view returns (bool) {
        uint256[] memory input = new uint256[](3);
        input[0] = uint256(CIRCUIT_DIGEST);
        input[1] = uint256(_inputHash) & ((1 << 253) - 1);
        input[2] = uint256(_outputHash) & ((1 << 253) - 1); 

        return this.verifyProof(_proof, input);
    }

    function verificationKeyHash() external pure returns (bytes32) {
        return CIRCUIT_DIGEST;
    }
}
".replace("{CIRCUIT_DIGEST}", circuit_digest);
        generated_contract + &verifier_contract
    }
}
