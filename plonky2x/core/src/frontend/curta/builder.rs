use curta::chip::{AirParameters, Chip};
use curta::machine::bytes::proof::ByteStarkProofTarget;
use curta::machine::bytes::stark::ByteStark;
use curta::plonky2::stark::config::StarkyConfig;
use curta::plonky2::stark::gadget::StarkGadget;
use curta::plonky2::stark::proof::StarkProofTarget;
use curta::plonky2::stark::Starky;
use curta::plonky2::Plonky2Air;

use super::proof::{ByteStarkProofVariable, StarkProofVariable};
use crate::prelude::*;

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn verify_stark_proof<A: Plonky2Air<L::Field, D>>(
        &mut self,
        config: &StarkyConfig<L::CurtaConfig, D>,
        stark: &Starky<A>,
        proof: StarkProofVariable<D>,
        public_inputs: &[Variable],
    ) {
        let proof_target = StarkProofTarget::from(proof);
        let public_inputs_target = public_inputs.iter().map(|v| v.0).collect::<Vec<_>>();
        self.api
            .verify_stark_proof(config, stark, &proof_target, &public_inputs_target);
    }

    pub fn verify_byte_stark_proof<P>(
        &mut self,
        byte_stark: &ByteStark<P, L::CurtaConfig, D>,
        proof: ByteStarkProofVariable<D>,
        public_inputs: &[Variable],
    ) where
        P: AirParameters<Field = L::Field, CubicParams = L::CubicParams>,
        Chip<P>: Plonky2Air<L::Field, D>,
    {
        let proof_target = ByteStarkProofTarget::from(proof);
        let public_inputs_target = public_inputs.iter().map(|v| v.0).collect::<Vec<_>>();
        byte_stark.verify_circuit(&mut self.api, &proof_target, &public_inputs_target);
    }
}
