use std::fs::File;
use std::io::Read;

use itertools::Itertools;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use plonky2::fri::proof;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2x::builder::CircuitBuilder;
use plonky2x::mapreduce::serialize::CircuitDataSerializable;
use plonky2x::vars::{CircuitVariable, Variable};

extern crate base64;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Proof {
    bytes: String,
}

fn parse_u64s(input: &str) -> Result<Vec<u64>, std::num::ParseIntError> {
    input.split_whitespace().map(|s| s.parse::<u64>()).collect()
}

fn main() {
    type F = GoldilocksField;
    type C = PoseidonGoldilocksConfig;
    const D: usize = 2;

    let args = std::env::args().collect_vec();

    if args.len() > 1 && &args[1] == "build" {
        let mut builder = CircuitBuilder::<F, D>::new();
        let input = builder.init::<Variable>();
        let inputs = vec![input; 128];
        let output = builder.mapreduce::<Variable, Variable, C, _, _>(
            inputs,
            |input, builder| {
                let constant = builder.constant::<Variable>(1);
                let sum = builder.add(input, constant);
                sum
            },
            |left, right, builder| {
                let sum = builder.add(left, right);
                sum
            },
        );
        builder.register_public_inputs(output.targets().as_slice());
        let circuit = builder.build::<C>();
        circuit.save(input, format!("./build/{}.circuit", circuit.id()));
    } else if args.len() > 1 && &args[1] == "test" {
        let mut builder = CircuitBuilder::<F, D>::new();
        let input = builder.init::<Variable>();
        let inputs = vec![input, input, input, input];
        let output = builder.mapreduce::<Variable, Variable, C, _, _>(
            inputs,
            |input, builder| {
                let constant = builder.constant::<Variable>(1);
                let sum = builder.add(input, constant);
                sum
            },
            |left, right, builder| {
                let sum = builder.add(left, right);
                sum
            },
        );
        builder.register_public_inputs(output.targets().as_slice());
        let circuit = builder.build::<C>();
        circuit.save(input, format!("./build/{}.circuit", circuit.id()));

        let mut pw = PartialWitness::new();
        pw.set_target(input.0, GoldilocksField::from_canonical_u64(1));
        let proof = circuit.prove(pw).unwrap();
        circuit.verify(proof).unwrap();
        println!("SUCCESS.");
    } else {
        let mut file = File::open("context").unwrap();
        let mut context = String::new();
        file.read_to_string(&mut context).unwrap();

        let args: Vec<String> = context.split_whitespace().map(|s| s.to_string()).collect();
        println!("{:?}", args);
        let cmd = &args[0];

        if cmd == "map" {
            // Read arguments from command line.
            let circuit_path = &args[1];
            let input_values = parse_u64s(&args[2]).unwrap();

            // Load the circuit.
            let (circuit, input_targets) =
                CircuitData::<F, C, D>::load_with_input_targets(circuit_path.to_string());

            // Set input targets.
            let mut pw = PartialWitness::new();
            for i in 0..input_targets.len() {
                pw.set_target(
                    input_targets[i],
                    GoldilocksField::from_canonical_u64(input_values[i]),
                );
            }

            // Generate proof.
            let proof = circuit.prove(pw).unwrap();
            circuit.verify(proof.clone()).unwrap();

            // Save proof.
            let proofA = Proof {
                bytes: hex::encode(proof.to_bytes()),
            };

            // Deserialization assertion...
            println!("{}", proof.to_bytes().len());
            let proof = ProofWithPublicInputs::<F, C, D>::from_bytes(
                hex::decode(hex::encode(proof.to_bytes())).unwrap(),
                &circuit.common,
            )
            .unwrap();
            println!("hmm {}", hex::encode(proof.to_bytes()));

            let file_path = "./proof.json";
            let json = serde_json::to_string_pretty(&proofA).unwrap();
            std::fs::write(file_path, json).unwrap();
            println!("Successfully generated proof.");
        } else if cmd == "reduce" {
            // Read arguments from command line.
            let circuit_path = &args[1];
            let proof_bytes_list = &args[2]
                .split(",")
                .map(|s| hex::decode(s).unwrap())
                .collect_vec();
            println!("{:?}", proof_bytes_list.len());

            // Load the circuit.
            let (circuit, child_circuit, proof_targets) =
                CircuitData::<F, C, D>::load_with_proof_targets(circuit_path.to_string());

            // Set inputs.
            let mut proofs = Vec::new();
            println!("{}", hex::encode(proof_bytes_list[0].clone()));
            for i in 0..proof_bytes_list.len() {
                // println!("{}", i);
                // println!("{:#?}", child_circuit.common);
                let proof = ProofWithPublicInputs::<F, C, D>::from_bytes(
                    proof_bytes_list[i].clone(),
                    &child_circuit.common,
                )
                .unwrap();
                proofs.push(proof);
            }
            let mut pw = PartialWitness::new();
            for i in 0..proof_bytes_list.len() {
                pw.set_proof_with_pis_target(&proof_targets[i], &proofs[i]);
            }

            // Generate proof.
            let proof = circuit.prove(pw).unwrap();
            circuit.verify(proof.clone()).unwrap();
            let proof = Proof {
                bytes: hex::encode(proof.to_bytes()),
            };
            let file_path = "./proof.json";
            let json = serde_json::to_string_pretty(&proof).unwrap();
            std::fs::write(file_path, json).unwrap();
            println!("Successfully generated proof.");
        } else {
            println!("Unsupported.")
        }
    }
}

// if proofs exists {
//     load_with_proofs(format!("./build/{}.circuit", circuit));
// }

// beacon-validator-statistics build
// beacon-validator-statistics prove ./build/0x1fad70fc4cc951fb2cd4.circuit --input $INPUT
// beacon-validator-statistics prove ./build/0x1fad70fc4cc951fb2cd4.circuit --proofs $PROOFS

// Option 2
// If we implement ProofWithPublicInputsVariable, then we can do:
// - save() we serialize the
// - load() returns CircuitData, Vec<Targets> where the second argument is respectively the input targets.
// - the $INPUT parameter is automatically set to the Vec<Targets>

// Need to implement ProofWithPublicInputsVariable.
// Setting of inputs happens via setting the serialized version of the proof.

// {
//   "proof": "0x1fad70fc4cc951fb2cd4",
//   "inputs": [],
//   "outputs": [],
// }
