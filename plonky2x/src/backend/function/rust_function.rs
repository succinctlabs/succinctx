use std::fs::File;
use std::io::{BufReader, Write};
use std::path;

use clap::Parser;
use log::info;
use plonky2::plonk::proof::Proof;
use serde::Serialize;
use sha2::Digest;

use super::cli::{Args, BuildArgs, Commands};
pub use super::request::{
    BytesRequestData, ElementsRequestData, ProofRequest, ProofRequestBase,
    RecursiveProofsRequestData,
};
pub use super::result::{
    BytesResultData, ElementsResultData, ProofResult, ProofResultBase, RecursiveProofsResultData,
};
use crate::backend::circuit::DefaultParameters;

const VERIFIER_CONTRACT: &str = include_str!("../../resources/Verifier.sol");

pub trait RustFunction {
    fn run(input_bytes: Vec<u8>) -> Vec<u8>;
    fn tx_origin() -> String {
        return "0xDEd0000E32f8F40414d3ab3a830f735a3553E18e".to_string();
    }
}

pub struct VerifiableRustFunction<F: RustFunction> {
    _phantom: std::marker::PhantomData<F>,
}

/// Functions that implement `Function` have all necessary code for end-to-end deployment.
///
///
/// Look at the `plonky2x/examples` for examples of how to use this trait.
impl<F: RustFunction> VerifiableRustFunction<F> {
    /// Saves the verifier contract to disk.
    pub fn compile(args: BuildArgs) {
        info!("Building verifier contract...");
        let contract_path = format!("{}/FunctionVerifier.sol", args.build_dir);
        let mut contract_file = File::create(&contract_path).unwrap();

        let tx_origin = F::tx_origin();
        let verifier_contract = Self::get_verifier_contract(&tx_origin);

        contract_file
            .write_all(verifier_contract.as_bytes())
            .unwrap();
        info!(
            "Successfully saved verifier contract to disk at {}.",
            contract_path
        );
    }

    pub fn prove(input_json: String) {
        info!("Loading input.");
        let proof_request = ProofRequest::<DefaultParameters, 2>::load(&input_json);
        if let ProofRequest::Bytes(request) = proof_request {
            let input_bytes = request.data.input;
            info!("Running function.");
            let result_bytes = F::run(input_bytes);
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

    /// The entry point for the function when using the CLI.
    pub fn entrypoint() {
        dotenv::dotenv().ok();
        env_logger::try_init().unwrap_or_default();

        let args = Args::parse();
        match args.command {
            Commands::Build(args) => {
                Self::compile(args);
            }
            Commands::Prove(args) => {
                Self::prove(args.input_json);
            }
            Commands::ProveWrapped(args) => {
                Self::prove(args.input_json);
            }
        }
    }

    fn get_verifier_contract(tx_origin: &str) -> String {
        let generated_contract = VERIFIER_CONTRACT
            .replace("pragma solidity ^0.8.0;", "pragma solidity ^0.8.16;")
            .replace("uint256[3] calldata input", "uint256[3] memory input");

        let verifier_contract = "

interface IFunctionVerifier {
    function verify(bytes32 _inputHash, bytes32 _outputHash, bytes memory _proof) external view returns (bool);

    function verificationKeyHash() external pure returns (bytes32);
}

contract FunctionVerifier is IFunctionVerifier, Verifier {

    address public constant TX_ORIGIN = {TX_ORIGIN};

    function verify(bytes32 _inputHash, bytes32 _outputHash, bytes memory _proof) external view returns (bool) {
        return tx.origin == TX_ORIGIN;
    }

    function verificationKeyHash() external pure returns (bytes32) {
        return bytes32(0);
    }
}
".replace("{TX_ORIGIN}", tx_origin);
        generated_contract + &verifier_contract
    }
}
