use std::fs::File;
use std::io::Read;

use itertools::Itertools;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2x::builder::CircuitBuilder;
use plonky2x::mapreduce::serialize::CircuitDataSerializable;
use plonky2x::prover::remote::ContextData;
use plonky2x::vars::{CircuitVariable, Variable};

extern crate base64;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Proof {
    bytes: String,
}

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

fn main() {
    let args = std::env::args().collect_vec();

    if args.len() > 1 && &args[1] == "build" {
        let mut builder = CircuitBuilder::<F, D>::new();
        let input = builder.init::<Variable>();
        let inputs = vec![input; 4];
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
        println!("Successfully built circuit.");

        let mut pw = PartialWitness::new();
        pw.set_target(input.targets()[0], GoldilocksField::ONE);
        circuit.prove(pw).unwrap();
    } else {
        let mut file = File::open("context").unwrap();
        let mut context = String::new();
        file.read_to_string(&mut context).unwrap();

        let context: ContextData = serde_json::from_str(context.as_str()).unwrap();
        let circuit_path = format!("./build/{}.circuit", context.circuit_id);
        if context.tag == "map" {
            let (circuit, input_targets) =
                CircuitData::<F, C, D>::load_with_input_targets(circuit_path.to_string());
            let input_values = context
                .input
                .iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec();

            let mut pw = PartialWitness::new();
            for i in 0..input_targets.len() {
                pw.set_target(
                    input_targets[i],
                    GoldilocksField::from_canonical_u64(input_values[i]),
                );
            }
            let proof = circuit.prove(pw).unwrap();
            circuit.verify(proof.clone()).unwrap();

            let proof = Proof {
                bytes: hex::encode(proof.to_bytes()),
            };
            let file_path = "./proof.json";
            let json = serde_json::to_string_pretty(&proof).unwrap();
            std::fs::write(file_path, json).unwrap();

            println!("Successfully generated proof.");
        } else if context.tag == "reduce" {
            let (circuit, child_circuit, proof_targets) =
                CircuitData::<F, C, D>::load_with_proof_targets(circuit_path.to_string());
            let proofs = context
                .input
                .iter()
                .map(|s| {
                    ProofWithPublicInputs::<F, C, D>::from_bytes(
                        base64::decode(s).unwrap(),
                        &child_circuit.common,
                    )
                    .unwrap()
                })
                .collect_vec();

            let mut pw = PartialWitness::new();
            for i in 0..proofs.len() {
                pw.set_proof_with_pis_target(&proof_targets[i], &proofs[i]);
            }
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
