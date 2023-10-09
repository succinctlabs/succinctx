use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(about = "Compile a program.")]
pub struct CompileArgs {
    #[arg(long, default_value = "./build")]
    pub build_dir: String,
}

#[derive(Parser, Debug, Clone)]
#[command(about = "Run a program.")]
pub struct ProveArgs {
    #[arg(long, default_value = "./build")]
    pub build_dir: String,

    #[clap(long)]
    pub input_json: String,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Compile(CompileArgs),
    Prove(ProveArgs),
}

#[derive(Parser, Debug, Clone)]
#[command(about = "A tool for building and proving programs.")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}
