// use std::env;
// use std::fs::{File, create_dir_all};
// use std::io::Write;
// use std::process;
// use std::path::Path;
// use std::time::Instant;

// use plonky2::hash::hash_types::RichField;
// use plonky2::field::extension::Extendable;
// use plonky2::plonk::config::PoseidonGoldilocksConfig;
// use plonky2::plonk::circuit_data::CircuitData;
// use plonky2::field::goldilocks_field::GoldilocksField;

// use crate::builder::CircuitBuilder;
// use crate::succinct::circuit::{Circuit, CircuitFunction};
// use crate::utils::serializer::{load_circuit, save_circuit};
// use crate::utils::bytes;

// use crate::wrapper::wrap::WrapperCircuit;
// use crate::wrapper::plonky2_config::PoseidonBN128GoldilocksConfig;

// type F = GoldilocksField;
// type C = PoseidonGoldilocksConfig;
// const D: usize = 2;

// // Normally run with:     
// // let args: Vec<String> = env::args().collect();
// // run::<CircuitType>(args);
// fn run<CircuitType: Circuit<F, D>>(args: Vec<String>) {
//     let prove_flag = args.contains(&"--prove".to_string());
//     let fixture_flag = args.contains(&"--fixture".to_string());
//     let input_arg = args.iter().find(|&arg| arg.starts_with("--input="));

//     let mut builder = CircuitBuilder::<F, D>::new();
//     let mut circuit_function: CircuitFunction<F, D, CircuitType> = CircuitFunction::define(
//         &mut builder
//     );
//     let path = "build/circuit_function.bin";
//     let wrapper_path = "build/wrapper.bin";

//     if let Some(input) = input_arg {
//         println!("input: {}", input);
//         let input_bytes: Vec<u8> = bytes!(&input["--input=".len()..]);
        
//         if prove_flag {
//             let circuit_function_build: CircuitData<F, C, D> = load_circuit(&path.to_string());
//             println!("proving circuit for input: {:?}", hex::encode(&input_bytes));
//             let pw = circuit_function.set_witness(&input_bytes);
//             let proof = circuit_function_build.prove(pw);
//             match proof {
//                 Ok(proof) => {
//                     // Verify the proof
//                     let proof_clone = proof.clone();
//                     circuit_function_build.verify(proof).unwrap();
//                     println!("Circuit Function proof verified");

//                     let mut wrapper_builder = CircuitBuilder::<F, D>::new();
//                     let wrapper: WrapperCircuit = WrapperCircuit::define(&mut wrapper_builder.api, &circuit_function_build);
//                     // TODO: in the future load the wrapper_build from a saved file 
//                     let mut start = Instant::now();
//                     let wrapper_build = WrapperCircuit::build(wrapper_builder.api);
//                     let duration = start.elapsed();
//                     start = Instant::now();
//                     println!("Building wrapper circuit took: {:?}", duration);
//                     let wrapper_witness = wrapper.set_witness(&circuit_function_build, &proof_clone);
//                     let duration = start.elapsed();
//                     start = Instant::now();
//                     println!("Setting wrapper witness took: {:?}", duration);
//                     let wrapper_proof = wrapper_build.prove(wrapper_witness);
//                     let duration = start.elapsed();
//                     println!("Proving wrapper circuit took: {:?}", duration);
//                     match wrapper_proof {
//                         Ok(wrapper_proof) => {
//                             // Verify the wrapper proof
//                             wrapper_build.verify(wrapper_proof.clone()).unwrap();
//                             println!("wrapper proof verified");

//                             WrapperCircuit::save_proof(&wrapper_build, &wrapper_proof, "build/wrapper".to_string());
//                         }
//                         Err(e) => println!("Failed to prove wrapper circuit: {}", e),
//                     }
//                 }
//                 Err(e) => println!("Failed to prove circuit: {}", e),
//             }
//             return;
//         }
        
//         // TODO add an option to generate a fixture with the outside wrapper as well for double-checking testing
//         if fixture_flag {
//             println!("generating fixture for input: {:?}", hex::encode(&input_bytes));
//             match circuit_function.generate_fixture(&input_bytes) {
//                 Ok(fixture) => {
//                     todo!("export fixture")
//                 }
//                 Err(e) => println!("Failed to generate fixture: {}", e),
//             }
//             return;
//         }
//     }

//     println!("compiling and building circuit artifacts");
//     let circuit_build = builder.build::<C>();
//     let parent_dir = Path::new(path).parent().unwrap();
//     // Ensure the directory exists, create it if it doesn't
//     if !parent_dir.exists() {
//         create_dir_all(&parent_dir).unwrap();
//     }
//     save_circuit(&circuit_build, path.to_string());

//     // TODO in the future when we are able to save the "wrapper circuit",
//     // then save it as well
//     // // Now we have to save the outer "wrapper" circuit
//     // let wrapper_circuit = WrapperCircuit::build(circuit_build);
//     // let wrapper_path = "build/wrapper_circuit.bin";
//     // save_circuit(&wrapper_circuit, wrapper_path.to_string());
// }

// pub mod test {
//     use super::*;
//     use crate::succinct::circuit::test::TestCircuit;

//     #[test]
//     pub fn test_run_build() {
//         let build_args = vec![];
//         run::<TestCircuit>(build_args);

//         let prove_args = vec![
//             "--prove".to_string(),
//             "--input=0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
//         ];
//         run::<TestCircuit>(prove_args);
//     }


//     #[test]
//     pub fn test_run_fixture() {
//         let fixture_args = vec![
//             "--fixture".to_string(),
//         ];
//         run::<TestCircuit>(fixture_args);
//     }
// }