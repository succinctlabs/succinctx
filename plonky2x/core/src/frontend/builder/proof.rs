use plonky2::plonk::circuit_data::{CircuitData, CommonCircuitData, VerifierCircuitTarget};
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig, GenericHashOut};
use plonky2::plonk::proof::ProofWithPublicInputsTarget;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn add_virtual_proof_with_pis(
        &mut self,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> ProofWithPublicInputsTarget<D> {
        self.api.add_virtual_proof_with_pis(common_data)
    }

    pub fn verify_proof<P: PlonkParameters<D, Field = L::Field>>(
        &mut self,
        proof_with_pis: &ProofWithPublicInputsTarget<D>,
        inner_verifier_data: &VerifierCircuitTarget,
        inner_common_data: &CommonCircuitData<L::Field, D>,
    ) where
        <<P as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        self.api
            .verify_proof::<P::Config>(proof_with_pis, inner_verifier_data, inner_common_data);
    }

    pub fn constant_verifier_data<P: PlonkParameters<D, Field = L::Field>>(
        &mut self,
        data: &CircuitData<P::Field, P::Config, D>,
    ) -> VerifierCircuitTarget {
        // Set the verifier data target to be the verifier data, which is a constant.
        let vd = self
            .api
            .add_virtual_verifier_data(data.common.config.fri_config.cap_height);

        // Set the circuit digest.
        for i in 0..vd.circuit_digest.elements.len() {
            let constant = self
                .api
                .constant(data.verifier_only.circuit_digest.to_vec()[i]);
            self.api.connect(vd.circuit_digest.elements[i], constant);
        }

        // Set the constant sigmas cap.
        for i in 0..vd.constants_sigmas_cap.0.len() {
            let cap = vd.constants_sigmas_cap.0[i].elements;
            for j in 0..cap.len() {
                let constant = self
                    .api
                    .constant(data.verifier_only.constants_sigmas_cap.0[i].to_vec()[j]);
                self.api.connect(cap[j], constant);
            }
        }

        vd
    }
}
