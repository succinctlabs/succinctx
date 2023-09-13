mod cli;
mod request;
mod result;

use std::fs::File;
use std::io::{BufReader, Write};
use std::path;

use clap::Parser;
use log::info;
use plonky2::field::types::PrimeField64;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig, GenericHashOut};
pub use request::{
    BytesRequestData, ElementsRequestData, FunctionRequest, FunctionRequestBase,
    RecursiveProofsRequestData,
};
use serde::Serialize;

use self::cli::{BuildArgs, ProveArgs, ProveWrappedArgs};
use super::circuit::{GateRegistry, PlonkParameters, WitnessGeneratorRegistry};
use crate::backend::circuit::{
    Circuit, DefaultParameters, Groth16VerifierParameters, PublicOutput,
};
use crate::backend::function::cli::{Args, Commands};
use crate::backend::function::result::{BytesResultData, FunctionResult};
use crate::backend::wrapper::wrap::WrappedCircuit;

const VERIFIER_CONTRACT: &str = include_str!("../../resources/Verifier.sol");

/// Circuits that implement `CircuitFunction` have all necessary code for end-to-end deployment.
///
/// Conforming to this trait enables remote machines can generate proofs for you. In particular,
/// this trait ensures that the circuit can be built, serialized, and deserialized.
///
/// You may need to override the default implementation for `generators` and `gates` if you are
/// using custom gates or custom witness generators.
///
/// Look at the `plonky2x/examples` for examples of how to use this trait.
pub trait CircuitFunction {
    /// Builds the circuit.
    fn build<L: PlonkParameters<D>, const D: usize>() -> Circuit<L, D>;

    /// Generates the witness registry.
    fn generators<L: PlonkParameters<D>, const D: usize>() -> WitnessGeneratorRegistry<L, D>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        WitnessGeneratorRegistry::<L, D>::new()
    }

    /// Geneates the gate registry.
    fn gates<L: PlonkParameters<D>, const D: usize>() -> GateRegistry<L, D>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        GateRegistry::<L, D>::new()
    }

    /// Builds the circuit and saves it to disk.
    fn compile<L: PlonkParameters<D>, const D: usize>(args: BuildArgs)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        info!("Building circuit...");
        let circuit = Self::build::<L, D>();
        info!("Successfully built circuit.");
        info!("> Circuit: {}", circuit.id());
        info!("> Degree: {}", circuit.data.common.degree());
        info!("> Number of Gates: {}", circuit.data.common.gates.len());
        let path = format!("{}/main.circuit", args.build_dir);
        circuit.save(&path, &Self::gates::<L, D>(), &Self::generators::<L, D>());
        info!("Successfully saved circuit to disk at {}.", path);

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

        assert!(
            circuit_digest_bytes.len() <= 32,
            "circuit digest must be <= 32 bytes"
        );

        let mut padded = vec![0u8; 32];
        let digest_len = circuit_digest_bytes.len();
        padded[(32 - digest_len)..].copy_from_slice(&circuit_digest_bytes);
        let circuit_digest = format!("0x{}", hex::encode(padded));

        let generated_contract = VERIFIER_CONTRACT
            .replace("pragma solidity ^0.8.0;", "pragma solidity ^0.8.16;")
            .replace("uint256[3] calldata input", "uint256[3] memory input");
        contract_file
            .write_all(generated_contract.as_bytes())
            .unwrap();

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
".replace("{CIRCUIT_DIGEST}", &circuit_digest);
        contract_file
            .write_all(verifier_contract.as_bytes())
            .unwrap();
        info!(
            "Successfully saved verifier contract to disk at {}.",
            contract_path
        );
    }

    fn prove<L: PlonkParameters<D>, const D: usize>(args: ProveArgs, request: FunctionRequest<L, D>)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        let path = format!("{}/main.circuit", args.build_dir);
        info!("Loading circuit from {}...", path);
        let gates = Self::gates::<L, D>();
        let generators = Self::generators::<L, D>();
        let circuit = Circuit::<L, D>::load(&path, &gates, &generators).unwrap();
        info!("Successfully loaded circuit.");

        let input = request.input();
        let (proof, output) = circuit.prove(&input);
        info!("Successfully generated proof.");

        let result = FunctionResult::from_proof_output(proof, output);
        let json = serde_json::to_string_pretty(&result).unwrap();
        let mut file = File::create("plonky2x_output.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
        info!("Successfully saved proof to disk at plonky2x_output.json.");
    }

    fn prove_wrapped<
        InnerParameters: PlonkParameters<D>,
        OuterParameters: PlonkParameters<D, Field = InnerParameters::Field>,
        const D: usize,
    >(
        args: ProveWrappedArgs,
        request: FunctionRequest<InnerParameters, D>,
    ) where
        <<InnerParameters as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<InnerParameters::Field>,
        OuterParameters::Config: Serialize,
    {
        let path = format!("{}/main.circuit", args.build_dir);
        info!("Loading circuit from {}...", path);
        let gates = Self::gates::<InnerParameters, D>();
        let generators = Self::generators::<InnerParameters, D>();
        let circuit = Circuit::<InnerParameters, D>::load(&path, &gates, &generators).unwrap();
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
            let result: FunctionResult<OuterParameters, D> =
                FunctionResult::from_bytes(result_data.proof, output_bytes);
            let json = serde_json::to_string_pretty(&result).unwrap();
            let mut file = File::create("output.json").unwrap();
            file.write_all(json.as_bytes()).unwrap();
            info!("Successfully saved full result to disk at output.json.");
        } else {
            panic!("output is not bytes")
        }
    }

    /// The entry point for the function when using the CLI.
    fn cli() {
        type L = DefaultParameters;
        const D: usize = 2;

        let args = Args::parse();
        match args.command {
            Commands::Build(args) => {
                Self::compile::<L, D>(args);
            }
            Commands::Prove(args) => {
                let request = FunctionRequest::<L, D>::load(&args.input_json);
                Self::prove(args, request);
            }
            Commands::ProveWrapped(args) => {
                let request = FunctionRequest::<L, D>::load(&args.input_json);
                Self::prove_wrapped::<L, Groth16VerifierParameters, D>(args, request);
            }
        }
    }
}
