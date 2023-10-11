use std::fs::File;
use std::io::Write;

use clap::Parser;
use log::info;
use plonky2x::backend::function::{ProofRequest, ProofResult};
use plonky2x::prelude::DefaultParameters;

use crate::args::{Args, BuildArgs, Commands};
use crate::program::Program;

pub trait RustFunction {
    fn build(args: BuildArgs);

    fn prove(input_json: String);

    fn entrypoint();

    fn verifier(tx_origin: &str) -> String;
}

impl<P: Program> RustFunction for P {
    fn build(args: BuildArgs) {
        info!("Building verifier contract...");
        let contract_path = format!("{}/FunctionVerifier.sol", args.build_dir);
        let mut contract_file = File::create(&contract_path).unwrap();
        let tx_origin = P::tx_origin();
        let verifier_contract = Self::verifier(&tx_origin);
        contract_file
            .write_all(verifier_contract.as_bytes())
            .unwrap();
        info!(
            "Successfully saved verifier contract to disk at {}.",
            contract_path
        );
    }

    fn prove(input_json: String) {
        info!("Loading input...");
        let proof_request = ProofRequest::<DefaultParameters, 2>::load(&input_json);
        if let ProofRequest::Bytes(request) = proof_request {
            let input_bytes = request.data.input;
            info!("Running function...");
            let result_bytes = P::run(input_bytes);
            info!("Got result bytes.");
            let proof_result =
                ProofResult::<DefaultParameters, 2>::from_bytes(vec![], result_bytes);
            let json = serde_json::to_string_pretty(&proof_result).unwrap();
            let mut file = File::create("output.json").unwrap();
            file.write_all(json.as_bytes()).unwrap();
            info!("Successfully saved proof to disk at output.json.");
        } else {
            panic!("Invalid proof request type.");
        }
    }

    fn entrypoint() {
        dotenv::dotenv().ok();
        env_logger::try_init().unwrap_or_default();

        let args = Args::parse();
        match args.command {
            Commands::Build(args) => {
                Self::build(args);
            }
            Commands::Prove(args) => {
                Self::prove(args.input_json);
            }
        }
    }

    fn verifier(tx_origin: &str) -> String {
        "// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

interface IFunctionVerifier {
    function verify(bytes32 _inputHash, bytes32 _outputHash, bytes memory _proof) external view returns (bool);

    function verificationKeyHash() external pure returns (bytes32);
}

contract FunctionVerifier is IFunctionVerifier {

    address public constant TX_ORIGIN = {TX_ORIGIN};

    function verify(bytes32, bytes32, bytes memory) external view returns (bool) {
        return tx.origin == TX_ORIGIN;
    }

    function verificationKeyHash() external pure returns (bytes32) {
        return bytes32(0);
    }
}
".replace("{TX_ORIGIN}", tx_origin)
    }
}
