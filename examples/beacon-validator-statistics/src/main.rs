use std::env;

use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::iop::witness::PartialWitness;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2x::builder::CircuitBuilder;
use plonky2x::mapreduce::utils::{load_circuit, load_circuit_variable, save_circuit};
use plonky2x::vars::{CircuitVariable, Variable};

extern crate base64;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Proof {
    #[serde(with = "base64_format")]
    bytes: Vec<u8>,
}

mod base64_format {
    use base64::{decode, encode};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded = encode(bytes);
        serializer.serialize_str(&encoded)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let encoded = String::deserialize(deserializer)?;
        decode(&encoded).map_err(serde::de::Error::custom)
    }
}

fn main() {
    type F = GoldilocksField;
    type C = PoseidonGoldilocksConfig;
    const D: usize = 2;

    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    if input == "build" {
        let mut builder = CircuitBuilder::<F, D>::new();
        let a = builder.constant::<Variable>(0);
        let b = builder.constant::<Variable>(1);
        let c = builder.constant::<Variable>(2);
        let d = builder.constant::<Variable>(3);
        let inputs = vec![a, b, c, d];
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
        let data = builder.build::<C>();
        save_circuit(&data, "./build/entry.circuit".to_string());
        println!("Successfully built and saved circuit.");
    } else if input == "prove" {
        let target = &args[2];
        let circuit_path = format!("./build/{}", target);
        let circuit = load_circuit::<F, C, D>(&circuit_path);
        let another = &args[3];
        let input_path = format!("./build/{}", another);
        let input = load_circuit_variable::<Variable>(&input_path);
        let mut pw = PartialWitness::new();
        input.set(&mut pw, 5);
        let proof = circuit.prove(pw).unwrap();
        circuit.verify(proof.clone()).unwrap();
        let proof = Proof {
            bytes: proof.to_bytes(),
        };
        let file_path = "./proof.json";
        let json = serde_json::to_string_pretty(&proof).unwrap();
        std::fs::write(file_path, json).unwrap();
        println!("Successfully generated proof.");
    } else {
        println!("Unsupported.")
    }
}
