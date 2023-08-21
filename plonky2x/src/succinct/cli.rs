use std::env;
use std::fs::File;
use std::io::Write;
use std::process;

use plonky2::hash::hash_types::RichField;
use plonky2::field::extension::Extendable;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::field::goldilocks_field::GoldilocksField;

use crate::builder::CircuitBuilder;
use crate::succinct::circuit::{Circuit, CircuitFunction};


fn run<F: RichField + Extendable<D>, const D: usize, C: Circuit<F, D>>(circuit: C) {
    let args: Vec<String> = env::args().collect();

    let prove_flag = args.contains(&"--prove".to_string());
    let fixture_flag = args.contains(&"--fixture".to_string());
    let input_arg = args.iter().find(|&arg| arg.starts_with("--input="));

    // type F = GoldilocksField;
    type Config = PoseidonGoldilocksConfig;
    // const D: usize = 2;

    let mut builder = CircuitBuilder::<F, D>::new();
    let mut circuit_function: CircuitFunction<F, D, C> = CircuitFunction::define(
        &mut builder
    );

    if let Some(input) = input_arg {
        let input_bytes = hex::decode(&input["--input=".len()..]).expect("Invalid hex input");
        
        if prove_flag {
            println!("proving circuit for input: {:?}", hex::encode(&input_bytes));
            let pw = circuit_function.set_witness(input_bytes.clone());
            // TODO: import the circuit build from the build artifacts
            // Right now we just rebuild it here
            let circuit_build = builder.build::<Config>();
            let proof = circuit_build.prove(pw);
            match proof {
                Ok(proof) => {
                    todo!("export proof");
                    // if let Err(e) = proof.export("proof.json") {
                    //     println!("Failed to export proof: {}", e);
                    // }
                }
                Err(e) => println!("Failed to prove circuit: {}", e),
            }
            return;
        }
        
        if fixture_flag {
            println!("generating fixture for input: {:?}", hex::encode(&input_bytes));
            match circuit_function.generate_fixture(&input_bytes) {
                Ok(fixture) => {
                    todo!("export fixture")
                    // if let Err(e) = fixture.export("fixture.json") {
                    //     println!("Failed to export fixture: {}", e);
                    // }
                }
                Err(e) => println!("Failed to generate fixture: {}", e),
            }
            return;
        }
    }

    println!("compiling and building circuit artifacts");
    let circuilt_build = builder.build();
    // TODO: save circuit_build to a file
    // match circuit_function.build() {
    //     Ok(build) => build.export(),
    //     Err(e) => println!("Failed to build circuit: {}", e),
    // }
}