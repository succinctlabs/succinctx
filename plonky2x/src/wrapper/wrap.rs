use anyhow::Result;
use plonky2::field::extension::Extendable;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_data::{CircuitData, CommonCircuitData, VerifierOnlyCircuitData};
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::{ProofWithPublicInputs, ProofWithPublicInputsTarget};

use super::plonky2_config::PoseidonBN128GoldilocksConfig;
use crate::builder::CircuitBuilder;

type GF = GoldilocksField;
const GD: usize = 2;

#[derive(Debug)]
pub struct WrapperCircuitData<
    F: RichField + Extendable<D>,
    InnerConfig: GenericConfig<D, F = F>,
    OuterConfig: GenericConfig<D, F = F>,
    const D: usize,
> where
    InnerConfig::Hasher: AlgebraicHasher<F>,
{
    inner_data: CircuitData<F, InnerConfig, D>,
    pub wrapper_data: CircuitData<F, OuterConfig, D>,
    proof_with_pis: ProofWithPublicInputsTarget<D>,
}

#[derive(Debug)]
pub struct WrappedProof<F: RichField + Extendable<D>, C: GenericConfig<D, F = F>, const D: usize> {
    pub proof: ProofWithPublicInputs<F, C, D>,
    pub common_data: CommonCircuitData<F, D>,
    pub verifier_data: VerifierOnlyCircuitData<C, D>,
}

impl CircuitBuilder<GF, GD> {
    pub fn wrap_build<C: GenericConfig<GD, F = GF> + 'static>(
        self,
    ) -> WrapperCircuitData<GF, C, PoseidonBN128GoldilocksConfig, GD>
    where
        C::Hasher: AlgebraicHasher<GF>,
    {
        let inner_data = self.build::<C>();

        let mut builder = CircuitBuilder::<GF, GD>::new();

        let verifier_data = builder.constant_verifier_data(&inner_data);
        let proof_with_pis = builder.add_virtual_proof_with_pis(&inner_data.common);

        builder.verify_proof::<C>(&proof_with_pis, &verifier_data, &inner_data.common);

        let wrapper_data = builder.build::<PoseidonBN128GoldilocksConfig>();

        WrapperCircuitData {
            inner_data,
            wrapper_data,
            proof_with_pis,
        }
    }
}

impl<
        F: RichField + Extendable<D>,
        InnerConfig: GenericConfig<D, F = F>,
        OuterConfig: GenericConfig<D, F = F>,
        const D: usize,
    > WrapperCircuitData<F, InnerConfig, OuterConfig, D>
where
    InnerConfig::Hasher: AlgebraicHasher<F>,
{
    pub fn prove(&self, inputs: PartialWitness<F>) -> Result<WrappedProof<F, OuterConfig, D>> {
        let inner_proof = self.inner_data.prove(inputs)?;
        self.inner_data.verify(inner_proof.clone())?; // Verify the inner proof

        let mut pw = PartialWitness::new();
        pw.set_proof_with_pis_target(&self.proof_with_pis, &inner_proof);
        let proof = self.wrapper_data.prove(pw)?;
        self.wrapper_data.verify(proof.clone())?; // Verify the outer proof

        let common_data = self.wrapper_data.common.clone();
        let verifier_data = self.wrapper_data.verifier_only.clone();

        Ok(WrappedProof {
            proof,
            common_data,
            verifier_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use plonky2::field::types::Field;

    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_wrapper() {
        type F = GoldilocksField;
        const D: usize = 2;
        type InnerConfig = PoseidonGoldilocksConfig;

        let mut builder = CircuitBuilder::<F, D>::new();

        let a = builder.init::<Variable>();
        let b = builder.init::<Variable>();
        let _ = builder.add(a, b);

        let data = builder.wrap_build::<InnerConfig>();

        let mut pw = PartialWitness::new();
        a.set(&mut pw, F::ONE);
        b.set(&mut pw, F::ZERO);

        let wrapped_proof = data.prove(pw).unwrap();

        let serialized_proof = serde_json::to_string(&wrapped_proof.proof).unwrap();
        let serialized_common_data = serde_json::to_string(&wrapped_proof.common_data).unwrap();
        let serialized_verifier_data = serde_json::to_string(&wrapped_proof.verifier_data).unwrap();

        let path = format!("build/wrapper_test/");

        fs::write(
            format!("{}proof_with_public_inputs.json", path),
            serialized_proof,
        )
        .unwrap();
        fs::write(
            format!("{}common_circuit_data.json", path),
            serialized_common_data,
        )
        .unwrap();
        fs::write(
            format!("{}verifier_only_circuit_data.json", path),
            serialized_verifier_data,
        )
        .unwrap();
    }
}
