use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;

use super::{Prover, ProverInputTargets, ProverInputValues};
use crate::mapreduce::serialize::CircuitDataSerializable;

/// A prover that generates proofs locally.
pub struct LocalProver {}

impl Prover for LocalProver {
    fn new() -> Self {
        Self {}
    }

    async fn prove<F, C, const D: usize>(
        &self,
        circuit: &CircuitData<F, C, D>,
        _: ProverInputTargets<D>,
        values: ProverInputValues<F, C, D>,
    ) -> ProofWithPublicInputs<F, C, D>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        C::Hasher: AlgebraicHasher<F>,
    {
        match values {
            ProverInputValues::Bytes(_) => {
                todo!()
            }
            ProverInputValues::FieldElements(elements) => {
                let circuit_id = circuit.id();
                let circuit_path = format!("./build/{}.circuit", circuit_id);
                let (circuit, input_targets) =
                    CircuitData::<F, C, D>::load_with_input_targets(circuit_path);
                let mut pw = PartialWitness::<F>::new();
                assert_eq!(elements.len(), input_targets.len());
                for i in 0..input_targets.len() {
                    pw.set_target(input_targets[i], elements[i]);
                }
                let proof = circuit.prove(pw).unwrap();
                circuit.verify(proof.clone()).unwrap();
                proof
            }
            ProverInputValues::Proofs(proofs) => {
                let circuit_id = circuit.id();
                let circuit_path = format!("./build/{}.circuit", circuit_id);
                let (circuit, _, input_proof_targets) =
                    CircuitData::<F, C, D>::load_with_proof_targets(circuit_path);
                let mut pw = PartialWitness::<F>::new();
                assert_eq!(proofs.len(), input_proof_targets.len());
                for i in 0..input_proof_targets.len() {
                    pw.set_proof_with_pis_target(&input_proof_targets[i], &proofs[i]);
                }
                let proof = circuit.prove(pw).unwrap();
                circuit.verify(proof.clone()).unwrap();
                proof
            }
        }
    }

    async fn prove_batch<F, C, const D: usize>(
        &self,
        circuit: &CircuitData<F, C, D>,
        targets: ProverInputTargets<D>,
        values: Vec<ProverInputValues<F, C, D>>,
    ) -> Vec<ProofWithPublicInputs<F, C, D>>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F> + 'static,
        C::Hasher: AlgebraicHasher<F>,
    {
        let mut proofs = Vec::new();
        for i in 0..values.len() {
            let proof = self
                .prove(circuit, targets.clone(), values[i].clone())
                .await;
            proofs.push(proof);
        }
        proofs
    }
}
