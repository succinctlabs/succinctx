use clap::{Parser, Subcommand, ValueEnum};

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
