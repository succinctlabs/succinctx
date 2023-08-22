use plonky2::fri::proof::{
    FriInitialTreeProofTarget, FriProofTarget, FriQueryRoundTarget, FriQueryStepTarget,
};
use plonky2::hash::hash_types::MerkleCapTarget;
use plonky2::hash::merkle_proofs::MerkleProofTarget;
use plonky2::iop::target::Target;
use plonky2::plonk::proof::{OpeningSetTarget, ProofTarget, ProofWithPublicInputsTarget};

fn merkle_cap_to_targets(merkle_cap: MerkleCapTarget) -> Vec<Target> {
    let mut targets = Vec::new();
    let hash_out_targets = merkle_cap.0;
    for i in 0..hash_out_targets.len() {
        targets.extend(hash_out_targets[i].elements);
    }
    targets
}

fn opening_set_to_targets<const D: usize>(opening_set: OpeningSetTarget<D>) -> Vec<Target> {
    let mut targets = Vec::new();
    for i in 0..opening_set.constants.len() {
        targets.extend(opening_set.constants[i].0);
    }
    for i in 0..opening_set.plonk_sigmas.len() {
        targets.extend(opening_set.plonk_sigmas[i].0);
    }
    for i in 0..opening_set.wires.len() {
        targets.extend(opening_set.wires[i].0);
    }
    for i in 0..opening_set.plonk_zs.len() {
        targets.extend(opening_set.plonk_zs[i].0);
    }
    for i in 0..opening_set.plonk_zs_next.len() {
        targets.extend(opening_set.plonk_zs_next[i].0);
    }
    for i in 0..opening_set.partial_products.len() {
        targets.extend(opening_set.partial_products[i].0);
    }
    for i in 0..opening_set.quotient_polys.len() {
        targets.extend(opening_set.quotient_polys[i].0);
    }
    for i in 0..opening_set.lookup_zs.len() {
        targets.extend(opening_set.lookup_zs[i].0);
    }
    for i in 0..opening_set.next_lookup_zs.len() {
        targets.extend(opening_set.next_lookup_zs[i].0);
    }
    targets
}

fn merkle_proof_to_targets(m: MerkleProofTarget) -> Vec<Target> {
    let mut targets = Vec::new();
    for i in 0..m.siblings.len() {
        targets.extend(m.siblings[i].elements);
    }
    targets
}

fn fri_initial_tree_proof_to_targets(v: FriInitialTreeProofTarget) -> Vec<Target> {
    let mut targets = Vec::new();
    for i in 0..v.evals_proofs.len() {
        let eval_proof = v.evals_proofs[i].clone();
        targets.extend(eval_proof.0);
        targets.extend(merkle_proof_to_targets(eval_proof.1))
    }
    targets
}

fn fri_query_step_to_targets<const D: usize>(f: FriQueryStepTarget<D>) -> Vec<Target> {
    let mut targets = Vec::new();
    for i in 0..f.evals.len() {
        targets.extend(f.evals[i].0.clone());
    }
    targets.extend(merkle_proof_to_targets(f.merkle_proof));
    targets
}

fn fri_query_round_to_targets<const D: usize>(f: FriQueryRoundTarget<D>) -> Vec<Target> {
    let mut targets = Vec::new();
    targets.extend(fri_initial_tree_proof_to_targets(f.initial_trees_proof));
    for i in 0..f.steps.len() {
        targets.extend(fri_query_step_to_targets(f.steps[i].clone()));
    }
    targets
}

fn fri_proof_to_targets<const D: usize>(fri_proof: FriProofTarget<D>) -> Vec<Target> {
    let mut targets = Vec::new();
    for i in 0..fri_proof.commit_phase_merkle_caps.len() {
        targets.extend(merkle_cap_to_targets(
            fri_proof.commit_phase_merkle_caps[i].clone(),
        ));
    }
    for i in 0..fri_proof.query_round_proofs.len() {
        targets.extend(fri_query_round_to_targets(
            fri_proof.query_round_proofs[i].clone(),
        ));
    }
    for i in 0..fri_proof.final_poly.0.len() {
        targets.extend(fri_proof.final_poly.0[i].0.clone());
    }
    targets.extend(vec![fri_proof.pow_witness]);
    targets
}

fn proof_to_targets<const D: usize>(proof: ProofTarget<D>) -> Vec<Target> {
    let mut targets = Vec::new();
    targets.extend(merkle_cap_to_targets(proof.wires_cap));
    targets.extend(merkle_cap_to_targets(proof.plonk_zs_partial_products_cap));
    targets.extend(merkle_cap_to_targets(proof.quotient_polys_cap));
    targets.extend(opening_set_to_targets(proof.openings));
    targets.extend(fri_proof_to_targets(proof.opening_proof));
    targets
}

pub fn proof_with_pis_to_targets<const D: usize>(
    proof_with_pis: ProofWithPublicInputsTarget<D>,
) -> Vec<Target> {
    let mut targets = Vec::new();
    targets.extend(proof_to_targets(proof_with_pis.proof));
    targets.extend(proof_with_pis.public_inputs);
    targets
}
