use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(about = "Compile a circuit.")]
pub struct BuildArgs {
    #[arg(long, default_value = "./build")]
    pub build_dir: String,
}

#[derive(Parser, Debug, Clone)]
#[command(about = "Generate a proof for a circuit.")]
pub struct ProveArgs {
    #[arg(long, default_value = "./build")]
    pub build_dir: String,

    #[clap(long)]
    pub input_json: String,

    #[arg(long, default_value = "")]
    pub wrapper_path: String,
}

#[derive(Parser, Debug, Clone)]
#[command(
    about = "Generate a proof for a circuit and wrap it into a groth16 proof using the gnark verifier."
)]
pub struct ProveWrappedArgs {
    #[arg(long, default_value = "./build")]
    pub build_dir: String,

    #[clap(long)]
    pub input_json: String,

    #[arg(long)]
    pub wrapper_path: String,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Build(BuildArgs),
    Prove(ProveArgs),
    ProveWrapped(ProveWrappedArgs),
}

#[derive(Parser, Debug, Clone)]
#[command(about = "A tool for building and proving circuits.")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}
