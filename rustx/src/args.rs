use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(about = "Compile a program.")]
pub struct BuildArgs {
    #[arg(long, default_value = "./build")]
    pub build_dir: String,
}

#[derive(Parser, Debug, Clone)]
#[command(about = "Run a program.")]
pub struct ProveArgs {
    pub input_json: String,

    #[arg(long, default_value = "./build")]
    pub build_dir: String,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Build(BuildArgs),
    Prove(ProveArgs),
}

#[derive(Parser, Debug, Clone)]
#[command(about = "A tool for building and proving programs.")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}
