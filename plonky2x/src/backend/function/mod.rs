mod cli;
mod request;
mod result;

use std::fs::File;
use std::io::Write;

use clap::Parser;
use log::info;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
pub use request::{
    BytesRequestData, ElementsRequestData, FunctionRequest, FunctionRequestBase,
    RecursiveProofsRequestData,
};

use self::cli::{BuildArgs, ProveArgs};
use super::circuit::{GateRegistry, PlonkParameters, WitnessGeneratorRegistry};
use crate::backend::circuit::{Circuit, DefaultParameters};
use crate::backend::function::cli::{Args, Commands};
use crate::backend::function::result::FunctionResult;

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

        let result = FunctionResult::new(proof, output);
        let json = serde_json::to_string_pretty(&result).unwrap();
        let mut file = File::create("output.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
        info!("Successfully saved proof to disk at output.json.");
    }

    /// The entry point for the function when using CLI-based tools.
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
        }
    }
}
