mod cli;
mod request;
mod result;

use std::fs::File;
use std::io::Write;

use clap::Parser;
use log::info;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
pub use request::{
    BytesRequestData, ElementsRequestData, ProofRequest, ProofRequestBase,
    RecursiveProofsRequestData,
};
pub use result::{
    BytesResultData, ElementsResultData, ProofResult, ProofResultBase, RecursiveProofsResultData,
};

use self::cli::{BuildArgs, ProveArgs};
use crate::backend::circuit::Circuit;
use crate::backend::function::cli::{Args, Commands};

struct VerifiableFunction<C: Circuit> {}
/// Circuits that implement `CircuitFunction` have all necessary code for end-to-end deployment.
///
/// Conforming to this trait enables remote machines can generate proofs for you. In particular,
/// this trait ensures that the circuit can be built, serialized, and deserialized.
///
/// You may need to override the default implementation for `generators` and `gates` if you are
/// using custom gates or custom witness generators.
///
/// Look at the `plonky2x/examples` for examples of how to use this trait.
impl<C: Circuit> VerifiableFunction<C> {
    /// Builds the circuit and saves it to disk.
    fn compile<L: PlonkParameters<D>, const D: usize>(args: BuildArgs)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        info!("Building circuit...");
        let circuit = C::define::<L, D>();
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

    fn prove<L: PlonkParameters<D>, const D: usize>(args: ProveArgs, request: ProofRequest<L, D>)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        let path = format!("{}/main.circuit", args.build_dir);
        info!("Loading circuit from {}...", path);
        let gates = Self::gates::<L, D>();
        let generators = Self::generators::<L, D>();
        let circuit = CircuitBuild::<L, D>::load(&path, &gates, &generators).unwrap();
        info!("Successfully loaded circuit.");

        let input = request.input();
        let (proof, output) = circuit.prove(&input);
        info!("Successfully generated proof.");

        let result = ProofResult::new(proof, output);
        let json = serde_json::to_string_pretty(&result).unwrap();
        let mut file = File::create("output.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
        info!("Successfully saved proof to disk at output.json.");
    }

    /// The entry point for the function when using the CLI.
    fn entrypoint() {
        type L = DefaultParameters;
        const D: usize = 2;

        let args = Args::parse();
        match args.command {
            Commands::Build(args) => {
                Self::compile::<L, D>(args);
            }
            Commands::Prove(args) => {
                let request = ProofRequest::<L, D>::load(&args.input_json);
                Self::prove(args, request);
            }
        }
    }
}
