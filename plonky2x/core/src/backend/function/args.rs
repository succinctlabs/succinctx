use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(about = "Compile a circuit.")]
pub struct BuildArgs {
    #[arg(long, default_value = "./build")]
    pub build_dir: String,

    // This argument is needed to generate the FunctionVerifier.sol contract that contains
    // the verification smart contract for the gnark-plonky-2 verifier.
    // This needs to match the argument in `ProveArgs`
    #[arg(long, default_value = "/verifier-build")]
    pub wrapper_path: String,
}

#[derive(Parser, Debug, Clone)]
#[command(
    about = "Generate a proof for a circuit and wrap it into a groth16 proof using the gnark verifier."
)]
pub struct ProveArgs {
    pub input_json: String,

    #[arg(long, default_value = "./build")]
    pub build_dir: String,

    #[arg(long, default_value = "/verifier-build")]
    pub wrapper_path: String,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Build(BuildArgs),
    Prove(ProveArgs),
}

#[derive(Parser, Debug, Clone)]
#[command(about = "A tool for building and proving circuits.")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}
