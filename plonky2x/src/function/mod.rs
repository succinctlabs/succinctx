mod cli;
mod io;

use std::fs::File;
use std::io::{Read, Write};

use clap::Parser;
use curta::math::prelude::PrimeField64;
use log::info;
use plonky2::field::extension::Extendable;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig, PoseidonGoldilocksConfig};

use self::cli::{BuildArgs, ProveArgs};
use crate::circuit::Circuit;
use crate::function::cli::{Args, Commands, IO};
use crate::function::io::{FunctionInput, FunctionOutput};

pub trait CircuitFunction {
    /// Builds the circuit.
    fn build<F, C, const D: usize>() -> Circuit<F, C, D>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>;

    /// Builds the circuit and saves it to disk.
    fn compile<F, C, const D: usize>(args: BuildArgs)
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
    {
        info!("Building circuit...");
        let circuit = Self::build::<F, C, D>();
        info!("Successfully built circuit.");
        info!("> Circuit: {}", circuit.id());
        info!("> Degree: {}", circuit.data.common.degree());
        info!("> Number of Gates: {}", circuit.data.common.gates.len());
        let path = format!("{}/main.circuit", args.build_dir);
        circuit.save(&path);
        info!("Successfully saved circuit to disk at {}.", path);
    }

    /// Generates a proof with evm-based inputs and outputs.
    fn prove_with_evm_io<F, C, const D: usize>(args: ProveArgs, bytes: Vec<u8>)
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
    {
        let path = format!("{}/main.circuit", args.build_dir);
        info!("Loading circuit from {}...", path);
        let circuit = Circuit::<F, C, D>::load(&path).unwrap();
        info!("Successfully loaded circuit.");

        let mut input = circuit.input();
        input.evm_write_all(&bytes);

        info!("Generating proof...");
        let (proof, output) = circuit.prove(&input);
        info!("Proof generated.");
        circuit.verify(&proof, &input, &output);
        info!("Proof verified.");
        let output_bytes = output.evm_read_all();

        let function_output = FunctionOutput {
            io: "evm".to_string(),
            bytes: Some(hex::encode(output_bytes.clone())),
            elements: None,
            proof: proof.to_bytes(),
        };
        let json = serde_json::to_string_pretty(&function_output).unwrap();
        let mut file = File::create("output.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
        info!(
            "Succesfully wrote output of {} bytes and proof to output.json.",
            output_bytes.len()
        );
    }

    /// Generates a proof with field-based inputs and outputs.
    fn prove_with_field_io<F, C, const D: usize>(args: ProveArgs, elements: Vec<F>)
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
    {
        let path = format!("{}/main.circuit", args.build_dir);
        info!("Loading circuit from {}...", path);
        let circuit = Circuit::<F, C, D>::load(&path).unwrap();
        info!("Successfully loaded circuit.");

        let mut input = circuit.input();
        input.write_all(&elements);

        info!("Generating proof...");
        let (proof, output) = circuit.prove(&input);
        info!("Proof generated.");
        circuit.verify(&proof, &input, &output);
        info!("Proof verified.");
        let output_elements = output.read_all();

        let function_output = FunctionOutput {
            io: "field".to_string(),
            bytes: None,
            elements: Some(
                output_elements
                    .iter()
                    .map(|e| e.as_canonical_u64())
                    .collect(),
            ),
            proof: proof.to_bytes(),
        };
        let json = serde_json::to_string_pretty(&function_output).unwrap();
        let mut file = File::create("output.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
        info!(
            "Succesfully wrote output of {} elements and proof to output.json.",
            output_elements.len()
        );
    }

    /// The entry point for the function when using CLI-based tools.
    fn run() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let args = Args::parse();
        match args.command {
            Commands::Build(args) => {
                Self::compile::<F, C, D>(args);
            }
            Commands::Prove(args) => {
                let mut file = File::open(args.clone().input_json).unwrap();
                let mut data = String::new();
                file.read_to_string(&mut data).unwrap();
                let input: FunctionInput = serde_json::from_str(&data).unwrap();
                match args.io {
                    IO::Evm => {
                        Self::prove_with_evm_io::<F, C, D>(args, input.bytes());
                    }
                    IO::Field => {
                        Self::prove_with_field_io::<F, C, D>(args, input.elements());
                    }
                }
            }
        }
    }
}
