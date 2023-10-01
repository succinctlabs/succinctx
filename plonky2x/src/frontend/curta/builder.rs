use curta::plonky2::stark::config::StarkyConfig;
use curta::plonky2::stark::gadget::StarkGadget;
use curta::plonky2::stark::proof::StarkProofTarget;
use curta::plonky2::stark::Starky;
use curta::plonky2::Plonky2Air;

use super::proof::StarkProofVariable;
use crate::prelude::*;

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn verify_stark_proof<A: Plonky2Air<L::Field, D>>(
        &mut self,
        config: &StarkyConfig<L::CurtaConfig, D>,
        stark: &Starky<A>,
        proof: &StarkProofVariable<D>,
        public_inputs: &[Target],
    ) {
        let proof_target = StarkProofTarget::from(proof.clone());
        self.api
            .verify_stark_proof(config, stark, &proof_target, public_inputs);
    }
}
