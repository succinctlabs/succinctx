// Given a plonky2 proof generates a wrapped version of it

use std::fs;

use plonky2::field::extension::Extendable;
use plonky2::field::types::{Field, PrimeField64};
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::{CircuitConfig, CircuitData};
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig, PoseidonGoldilocksConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::plonk::prover::prove;
use plonky2::util::timing::TimingTree;

use crate::backend::wrapper::plonky2_config::PoseidonBN128GoldilocksConfig;

const D: usize = 2;
type C = PoseidonGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;

fn get_test_proof() -> ProofWithPublicInputs<F, C, D> {
    let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_ecc_config());
    let mut pw: PartialWitness<F> = PartialWitness::new();

    let inner_data = builder.build();
    let inner_proof = inner_data.prove(pw);
    // inner_data.verify(inner_proof.unwrap()).unwrap();
    return inner_proof.unwrap();
}

fn wrap_proof(inner_data: CircuitData<F, C, D>, inner_proof: ProofWithPublicInputs<F, C, D>) {
    let mut outer_builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_config());
    let outer_proof_target = outer_builder.add_virtual_proof_with_pis(&inner_data.common);
    let outer_verifier_data =
        outer_builder.add_virtual_verifier_data(inner_data.common.config.fri_config.cap_height);
    outer_builder.verify_proof::<C>(
        &outer_proof_target,
        &outer_verifier_data,
        &inner_data.common,
    );
    outer_builder.register_public_inputs(&outer_proof_target.public_inputs);
    outer_builder.register_public_inputs(&outer_verifier_data.circuit_digest.elements);

    let outer_data = outer_builder.build::<PoseidonBN128GoldilocksConfig>();

    let mut outer_pw = PartialWitness::new();
    outer_pw.set_proof_with_pis_target(&outer_proof_target, &inner_proof);
    outer_pw.set_verifier_data_target(&outer_verifier_data, &inner_data.verifier_only);

    let outer_proof = outer_data.prove(outer_pw).unwrap();
    // // let mut timing = TimingTree::new("step proof gen", Level::Info);
    // let outer_proof = prove::<F, PoseidonBN128GoldilocksConfig, D>(
    //     &outer_data.prover_only,
    //     &outer_data.common,
    //     outer_pw.clone()
    // )
    // .unwrap();

    let ret = outer_data.verify(outer_proof.clone());

    // Verify the public inputs:

    // assert_eq!(outer_proof.public_inputs.len(), 36);

    // // Blake2b hash of the public inputs
    // assert_eq!(
    //     outer_proof.public_inputs[0..32]
    //         .iter()
    //         .map(|element| u8::try_from(element.to_canonical_u64()).unwrap())
    //         .collect::<Vec<_>>(),
    //     hex::decode(BLOCK_530527_PUBLIC_INPUTS_HASH).unwrap(),
    // );

    /*  TODO:  It appears that the circuit digest changes after every different run, even if none of the code changes.  Need to find out why.
    // Step circuit's digest
    assert_eq!(
        outer_proof.public_inputs[32..36].iter()
        .map(|element| element.to_canonical_u64()).collect::<Vec<_>>(),
        [17122441374070351185, 18368451173317844989, 5752543660850962321, 1428786498560175815],
    );
    */

    for gate in outer_data.common.gates.iter() {
        println!("outer circuit: gate is {:?}", gate);
    }

    println!(
        "Recursive circuit digest is {:?}",
        outer_data.verifier_only.circuit_digest
    );

    let outer_common_circuit_data_serialized = serde_json::to_string(&outer_data.common).unwrap();
    fs::write(
        "step_recursive.common_circuit_data.json",
        outer_common_circuit_data_serialized,
    )
    .expect("Unable to write file");

    let outer_verifier_only_circuit_data_serialized =
        serde_json::to_string(&outer_data.verifier_only).unwrap();
    fs::write(
        "step_recursive.verifier_only_circuit_data.json",
        outer_verifier_only_circuit_data_serialized,
    )
    .expect("Unable to write file");

    let outer_proof_serialized = serde_json::to_string(&outer_proof).unwrap();
    fs::write(
        "step_recursive.proof_with_public_inputs.json",
        outer_proof_serialized,
    )
    .expect("Unable to write file");
}
