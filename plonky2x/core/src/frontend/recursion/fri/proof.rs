use plonky2::field::extension::Extendable;
use plonky2::field::polynomial::PolynomialCoeffs;
use plonky2::fri::proof::{
    FriInitialTreeProof, FriInitialTreeProofTarget, FriProof, FriProofTarget, FriQueryRound,
    FriQueryRoundTarget, FriQueryStep, FriQueryStepTarget,
};
use plonky2::fri::FriParams;
use plonky2::gadgets::polynomial::PolynomialCoeffsExtTarget;
use plonky2::hash::hash_types::MerkleCapTarget;
use plonky2::iop::ext_target::ExtensionTarget;
use plonky2::plonk::config::AlgebraicHasher;

use crate::frontend::recursion::extension::ExtensionVariable;
use crate::frontend::recursion::hash::{MerkleCapVariable, MerkleProofVariable};
use crate::frontend::recursion::polynomial::PolynomialCoeffsExtVariable;
use crate::frontend::vars::{ValueStream, VariableStream};
use crate::prelude::{CircuitBuilder, OutputVariableStream, PlonkParameters, Variable};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FriProofVariable<const D: usize> {
    pub commit_phase_merkle_caps: Vec<MerkleCapVariable>,
    pub query_round_proofs: Vec<FriQueryRoundVariable<D>>,
    pub final_poly: PolynomialCoeffsExtVariable<D>,
    pub pow_witness: Variable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FriQueryRoundVariable<const D: usize> {
    pub initial_trees_proof: FriInitialTreeProofVariable,
    pub steps: Vec<FriQueryStepVariable<D>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FriInitialTreeProofVariable {
    pub evals_proofs: Vec<(Vec<Variable>, MerkleProofVariable)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FriQueryStepVariable<const D: usize> {
    pub evals: Vec<ExtensionVariable<D>>,
    pub merkle_proof: MerkleProofVariable,
}

impl VariableStream {
    pub fn read_fri_proof<const D: usize>(
        &mut self,
        num_leaves_per_oracle: &[usize],
        params: &FriParams,
    ) -> FriProofVariable<D> {
        let cap_height = params.config.cap_height;
        let num_queries = params.config.num_query_rounds;

        let commit_phase_merkle_caps = (0..params.reduction_arity_bits.len())
            .map(|_| self.read_merkle_cap(cap_height))
            .collect::<Vec<_>>();

        let query_round_proofs = (0..num_queries)
            .map(|_| self.read_fri_query_round(num_leaves_per_oracle, params))
            .collect::<Vec<_>>();

        let final_poly = self.read_poly_coeff_ext(params.final_poly_len());
        let pow_witness = self.read::<Variable>();
        FriProofVariable {
            commit_phase_merkle_caps,
            query_round_proofs,
            final_poly,
            pow_witness,
        }
    }

    pub fn read_poly_coeff_ext<const D: usize>(
        &mut self,
        len: usize,
    ) -> PolynomialCoeffsExtVariable<D> {
        PolynomialCoeffsExtVariable(
            (0..len)
                .map(|_| self.read::<ExtensionVariable<D>>())
                .collect(),
        )
    }

    pub fn read_fri_query_round<const D: usize>(
        &mut self,
        num_leaves_per_oracle: &[usize],
        params: &FriParams,
    ) -> FriQueryRoundVariable<D> {
        let cap_height = params.config.cap_height;
        assert!(params.lde_bits() >= cap_height);
        let mut merkle_proof_len = params.lde_bits() - cap_height;

        let initial_trees_proof =
            self.read_fri_initial_trees_proof(num_leaves_per_oracle, merkle_proof_len);

        let mut steps = Vec::with_capacity(params.reduction_arity_bits.len());
        for &arity_bits in &params.reduction_arity_bits {
            assert!(merkle_proof_len >= arity_bits);
            merkle_proof_len -= arity_bits;
            steps.push(self.read_fri_query_step(arity_bits, merkle_proof_len));
        }

        FriQueryRoundVariable {
            initial_trees_proof,
            steps,
        }
    }

    fn read_fri_initial_trees_proof(
        &mut self,
        num_leaves_per_oracle: &[usize],
        initial_merkle_proof_len: usize,
    ) -> FriInitialTreeProofVariable {
        let evals_proofs = num_leaves_per_oracle
            .iter()
            .map(|&num_oracle_leaves| {
                let leaves = self.read_exact(num_oracle_leaves).to_vec();
                let merkle_proof = self.read_merkle_proof(initial_merkle_proof_len);
                (leaves, merkle_proof)
            })
            .collect();
        FriInitialTreeProofVariable { evals_proofs }
    }

    fn read_fri_query_step<const D: usize>(
        &mut self,
        arity_bits: usize,
        merkle_proof_len: usize,
    ) -> FriQueryStepVariable<D> {
        FriQueryStepVariable {
            evals: self.read_vec::<ExtensionVariable<D>>(1 << arity_bits),
            merkle_proof: self.read_merkle_proof(merkle_proof_len),
        }
    }

    pub fn write_fri_proof<const D: usize>(&mut self, proof: &FriProofVariable<D>) {
        for cap in proof.commit_phase_merkle_caps.iter() {
            self.write_merkle_cap(cap);
        }

        for query_round in proof.query_round_proofs.iter() {
            self.write_fri_query_round(query_round);
        }

        self.write_poly_coeff_ext(&proof.final_poly);

        self.write(&proof.pow_witness);
    }

    pub fn write_poly_coeff_ext<const D: usize>(
        &mut self,
        coefficients: &PolynomialCoeffsExtVariable<D>,
    ) {
        self.write_slice(&coefficients.0)
    }

    pub fn write_fri_query_round<const D: usize>(
        &mut self,
        query_round: &FriQueryRoundVariable<D>,
    ) {
        self.write_fri_initial_trees_proof(&query_round.initial_trees_proof);

        for step in &query_round.steps {
            self.write_fri_query_step(step);
        }
    }

    fn write_fri_initial_trees_proof(&mut self, initial_tree_proof: &FriInitialTreeProofVariable) {
        initial_tree_proof
            .evals_proofs
            .iter()
            .for_each(|(values, merkle_proof)| {
                self.write_slice(values);
                self.write_merkle_proof(merkle_proof);
            })
    }

    fn write_fri_query_step<const D: usize>(&mut self, query_step: &FriQueryStepVariable<D>) {
        self.write_slice::<ExtensionVariable<D>>(&query_step.evals);
        self.write_merkle_proof(&query_step.merkle_proof);
    }
}

impl<L: PlonkParameters<D>, const D: usize> OutputVariableStream<L, D> {
    pub fn read_fri_proof(
        &self,
        builder: &mut CircuitBuilder<L, D>,
        num_leaves_per_oracle: &[usize],
        params: &FriParams,
    ) -> FriProofVariable<D> {
        let cap_height = params.config.cap_height;
        let num_queries = params.config.num_query_rounds;

        let commit_phase_merkle_caps = (0..params.reduction_arity_bits.len())
            .map(|_| self.read_merkle_cap(builder, cap_height))
            .collect::<Vec<_>>();

        let query_round_proofs = (0..num_queries)
            .map(|_| self.read_fri_query_round(builder, num_leaves_per_oracle, params))
            .collect::<Vec<_>>();

        let final_poly = self.read_poly_coeff_ext(builder, params.final_poly_len());
        let pow_witness = self.read::<Variable>(builder);
        FriProofVariable {
            commit_phase_merkle_caps,
            query_round_proofs,
            final_poly,
            pow_witness,
        }
    }

    pub fn read_poly_coeff_ext(
        &self,
        builder: &mut CircuitBuilder<L, D>,
        len: usize,
    ) -> PolynomialCoeffsExtVariable<D> {
        PolynomialCoeffsExtVariable(
            (0..len)
                .map(|_| self.read::<ExtensionVariable<D>>(builder))
                .collect(),
        )
    }

    pub fn read_fri_query_round(
        &self,
        builder: &mut CircuitBuilder<L, D>,
        num_leaves_per_oracle: &[usize],
        params: &FriParams,
    ) -> FriQueryRoundVariable<D> {
        let cap_height = params.config.cap_height;
        assert!(params.lde_bits() >= cap_height);
        let mut merkle_proof_len = params.lde_bits() - cap_height;

        let initial_trees_proof =
            self.read_fri_initial_trees_proof(builder, num_leaves_per_oracle, merkle_proof_len);

        let mut steps = Vec::with_capacity(params.reduction_arity_bits.len());
        for &arity_bits in &params.reduction_arity_bits {
            assert!(merkle_proof_len >= arity_bits);
            merkle_proof_len -= arity_bits;
            steps.push(self.read_fri_query_step(builder, arity_bits, merkle_proof_len));
        }

        FriQueryRoundVariable {
            initial_trees_proof,
            steps,
        }
    }

    fn read_fri_initial_trees_proof(
        &self,
        builder: &mut CircuitBuilder<L, D>,
        num_leaves_per_oracle: &[usize],
        initial_merkle_proof_len: usize,
    ) -> FriInitialTreeProofVariable {
        let evals_proofs = num_leaves_per_oracle
            .iter()
            .map(|&num_oracle_leaves| {
                let leaves = self.read_exact(builder, num_oracle_leaves).to_vec();
                let merkle_proof = self.read_merkle_proof(builder, initial_merkle_proof_len);
                (leaves, merkle_proof)
            })
            .collect();
        FriInitialTreeProofVariable { evals_proofs }
    }

    fn read_fri_query_step(
        &self,
        builder: &mut CircuitBuilder<L, D>,
        arity_bits: usize,
        merkle_proof_len: usize,
    ) -> FriQueryStepVariable<D> {
        FriQueryStepVariable {
            evals: self.read_vec::<ExtensionVariable<D>>(builder, 1 << arity_bits),
            merkle_proof: self.read_merkle_proof(builder, merkle_proof_len),
        }
    }
}

impl<L: PlonkParameters<D>, const D: usize> ValueStream<L, D> {
    pub fn read_fri_proof<H: AlgebraicHasher<L::Field>>(
        &mut self,
        num_leaves_per_oracle: &[usize],
        params: &FriParams,
    ) -> FriProof<L::Field, H, D> {
        let cap_height = params.config.cap_height;
        let num_queries = params.config.num_query_rounds;

        let commit_phase_merkle_caps = (0..params.reduction_arity_bits.len())
            .map(|_| self.read_merkle_cap(cap_height))
            .collect::<Vec<_>>();

        let query_round_proofs = (0..num_queries)
            .map(|_| self.read_fri_query_round(num_leaves_per_oracle, params))
            .collect::<Vec<_>>();

        let final_poly = self.read_poly_coeff_ext(params.final_poly_len());
        let pow_witness = self.read_value::<Variable>();
        FriProof {
            commit_phase_merkle_caps,
            query_round_proofs,
            final_poly,
            pow_witness,
        }
    }

    pub fn read_poly_coeff_ext(
        &mut self,
        len: usize,
    ) -> PolynomialCoeffs<<L::Field as Extendable<D>>::Extension> {
        PolynomialCoeffs::new((0..len).map(|_| self.read_extension()).collect())
    }

    pub fn read_fri_query_round<H: AlgebraicHasher<L::Field>>(
        &mut self,
        num_leaves_per_oracle: &[usize],
        params: &FriParams,
    ) -> FriQueryRound<L::Field, H, D> {
        let cap_height = params.config.cap_height;
        assert!(params.lde_bits() >= cap_height);
        let mut merkle_proof_len = params.lde_bits() - cap_height;

        let initial_trees_proof =
            self.read_fri_initial_trees_proof(num_leaves_per_oracle, merkle_proof_len);

        let mut steps = Vec::with_capacity(params.reduction_arity_bits.len());
        for &arity_bits in &params.reduction_arity_bits {
            assert!(merkle_proof_len >= arity_bits);
            merkle_proof_len -= arity_bits;
            steps.push(self.read_fri_query_step(arity_bits, merkle_proof_len));
        }

        FriQueryRound {
            initial_trees_proof,
            steps,
        }
    }

    fn read_fri_initial_trees_proof<H: AlgebraicHasher<L::Field>>(
        &mut self,
        num_leaves_per_oracle: &[usize],
        initial_merkle_proof_len: usize,
    ) -> FriInitialTreeProof<L::Field, H> {
        let evals_proofs = num_leaves_per_oracle
            .iter()
            .map(|&num_oracle_leaves| {
                let leaves = self.read_exact(num_oracle_leaves).to_vec();
                let merkle_proof = self.read_merkle_proof(initial_merkle_proof_len);
                (leaves, merkle_proof)
            })
            .collect();
        FriInitialTreeProof { evals_proofs }
    }

    fn read_fri_query_step<H: AlgebraicHasher<L::Field>>(
        &mut self,
        arity_bits: usize,
        merkle_proof_len: usize,
    ) -> FriQueryStep<L::Field, H, D> {
        FriQueryStep {
            evals: self.read_extension_vec(1 << arity_bits),
            merkle_proof: self.read_merkle_proof(merkle_proof_len),
        }
    }

    pub fn write_fri_proof<H: AlgebraicHasher<L::Field>>(
        &mut self,
        proof: FriProof<L::Field, H, D>,
    ) {
        let FriProof {
            commit_phase_merkle_caps,
            query_round_proofs,
            final_poly,
            pow_witness,
        } = proof;
        for cap in commit_phase_merkle_caps {
            self.write_merkle_cap(cap);
        }

        for query_round in query_round_proofs {
            self.write_fri_query_round(query_round);
        }

        self.write_poly_coeff_ext(final_poly);

        self.write_value::<Variable>(pow_witness);
    }

    pub fn write_poly_coeff_ext(
        &mut self,
        coefficients: PolynomialCoeffs<<L::Field as Extendable<D>>::Extension>,
    ) {
        self.write_extension_vec(coefficients.coeffs)
    }

    pub fn write_fri_query_round<H: AlgebraicHasher<L::Field>>(
        &mut self,
        query_round: FriQueryRound<L::Field, H, D>,
    ) {
        let FriQueryRound {
            initial_trees_proof,
            steps,
        } = query_round;
        self.write_fri_initial_trees_proof(initial_trees_proof);

        for step in steps {
            self.write_fri_query_step(step);
        }
    }

    fn write_fri_initial_trees_proof<H: AlgebraicHasher<L::Field>>(
        &mut self,
        initial_tree_proof: FriInitialTreeProof<L::Field, H>,
    ) {
        initial_tree_proof
            .evals_proofs
            .into_iter()
            .for_each(|(values, merkle_proof)| {
                self.write_slice(&values);
                self.write_merkle_proof(merkle_proof);
            })
    }

    fn write_fri_query_step<H: AlgebraicHasher<L::Field>>(
        &mut self,
        query_step: FriQueryStep<L::Field, H, D>,
    ) {
        let FriQueryStep {
            evals,
            merkle_proof,
        } = query_step;
        self.write_extension_vec(evals);
        self.write_merkle_proof(merkle_proof);
    }
}

impl From<FriInitialTreeProofVariable> for FriInitialTreeProofTarget {
    fn from(value: FriInitialTreeProofVariable) -> Self {
        Self {
            evals_proofs: value
                .evals_proofs
                .into_iter()
                .map(|(evals, merkle_proof)| {
                    (
                        evals.into_iter().map(|v| v.0).collect(),
                        merkle_proof.into(),
                    )
                })
                .collect(),
        }
    }
}

impl From<FriInitialTreeProofTarget> for FriInitialTreeProofVariable {
    fn from(value: FriInitialTreeProofTarget) -> Self {
        Self {
            evals_proofs: value
                .evals_proofs
                .into_iter()
                .map(|(evals, merkle_proof)| {
                    (
                        evals.into_iter().map(Variable).collect(),
                        merkle_proof.into(),
                    )
                })
                .collect(),
        }
    }
}

impl<const D: usize> From<FriQueryStepVariable<D>> for FriQueryStepTarget<D> {
    fn from(value: FriQueryStepVariable<D>) -> Self {
        Self {
            evals: value.evals.into_iter().map(ExtensionTarget::from).collect(),
            merkle_proof: value.merkle_proof.into(),
        }
    }
}

impl<const D: usize> From<FriQueryStepTarget<D>> for FriQueryStepVariable<D> {
    fn from(value: FriQueryStepTarget<D>) -> Self {
        Self {
            evals: value
                .evals
                .into_iter()
                .map(ExtensionVariable::from)
                .collect(),
            merkle_proof: value.merkle_proof.into(),
        }
    }
}

impl<const D: usize> From<FriQueryRoundTarget<D>> for FriQueryRoundVariable<D> {
    fn from(value: FriQueryRoundTarget<D>) -> Self {
        Self {
            initial_trees_proof: value.initial_trees_proof.into(),
            steps: value
                .steps
                .into_iter()
                .map(FriQueryStepVariable::from)
                .collect(),
        }
    }
}

impl<const D: usize> From<FriQueryRoundVariable<D>> for FriQueryRoundTarget<D> {
    fn from(value: FriQueryRoundVariable<D>) -> Self {
        Self {
            initial_trees_proof: value.initial_trees_proof.into(),
            steps: value
                .steps
                .into_iter()
                .map(FriQueryStepTarget::from)
                .collect(),
        }
    }
}

impl<const D: usize> From<FriProofTarget<D>> for FriProofVariable<D> {
    fn from(value: FriProofTarget<D>) -> Self {
        Self {
            commit_phase_merkle_caps: value
                .commit_phase_merkle_caps
                .into_iter()
                .map(MerkleCapVariable::from)
                .collect(),
            query_round_proofs: value
                .query_round_proofs
                .into_iter()
                .map(FriQueryRoundVariable::from)
                .collect(),
            final_poly: PolynomialCoeffsExtVariable::from(value.final_poly),
            pow_witness: value.pow_witness.into(),
        }
    }
}

impl<const D: usize> From<FriProofVariable<D>> for FriProofTarget<D> {
    fn from(value: FriProofVariable<D>) -> Self {
        Self {
            commit_phase_merkle_caps: value
                .commit_phase_merkle_caps
                .into_iter()
                .map(MerkleCapTarget::from)
                .collect(),
            query_round_proofs: value
                .query_round_proofs
                .into_iter()
                .map(FriQueryRoundTarget::from)
                .collect(),
            final_poly: PolynomialCoeffsExtTarget::from(value.final_poly),
            pow_witness: value.pow_witness.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use plonky2::hash::poseidon::PoseidonHash;
    use plonky2::plonk::plonk_common::salt_size;

    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_conversion() {
        let mut inner_builder = DefaultBuilder::new();
        let a = inner_builder.read::<Variable>();
        let b = inner_builder.read::<Variable>();
        let _ = inner_builder.add(a, b);
        let circuit = inner_builder.build();

        let mut builder = DefaultBuilder::new();

        let proof = builder.api.add_virtual_proof_with_pis(&circuit.data.common);
        let fri_proof = proof.proof.opening_proof;

        let fri_proof_variable = FriProofVariable::from(fri_proof.clone());
        let fri_proof_back = FriProofTarget::from(fri_proof_variable.clone());

        assert_eq!(fri_proof, fri_proof_back);
    }

    #[test]
    fn test_variable_stream() {
        let mut inner_builder = DefaultBuilder::new();
        let a = inner_builder.read::<Variable>();
        let b = inner_builder.read::<Variable>();
        let c = inner_builder.add(a, b);
        let _ = inner_builder.sub(c, b);
        let circuit = inner_builder.build();

        let mut builder = DefaultBuilder::new();

        let proof = builder.api.add_virtual_proof_with_pis(&circuit.data.common);
        let fri_proof = proof.proof.opening_proof;

        let fri_proof_variable = FriProofVariable::from(fri_proof.clone());

        let mut stream = VariableStream::new();
        stream.write_fri_proof(&fri_proof_variable);

        let common_data = &circuit.data.common;
        let config = &common_data.config;
        let fri_params = &common_data.fri_params;

        let salt = salt_size(common_data.fri_params.hiding);
        let num_leaves_per_oracle = &[
            common_data.sigmas_range().end,
            config.num_wires + salt,
            common_data.config.num_challenges * (1 + common_data.num_partial_products) + salt,
            common_data.config.num_challenges * common_data.quotient_degree_factor + salt,
        ];
        let proof_back: FriProofVariable<2> =
            stream.read_fri_proof(num_leaves_per_oracle, fri_params);

        assert_eq!(fri_proof_variable, proof_back);
    }

    #[test]
    fn test_value_stream() {
        let mut builder = DefaultBuilder::new();
        let a = builder.read::<Variable>();
        let b = builder.read::<Variable>();
        let c = builder.add(a, b);
        let _ = builder.sub(c, b);
        let circuit = builder.build();

        let mut input = circuit.input();
        input.write::<Variable>(GoldilocksField::ONE);
        input.write::<Variable>(GoldilocksField::ZERO);

        let (proof, _) = circuit.prove(&input);

        let fri_proof = proof.proof.opening_proof;

        let mut stream = ValueStream::<DefaultParameters, 2>::new();
        stream.write_fri_proof(fri_proof.clone());

        let common_data = &circuit.data.common;
        let config = &common_data.config;
        let fri_params = &common_data.fri_params;

        let salt = salt_size(common_data.fri_params.hiding);
        let num_leaves_per_oracle = &[
            common_data.sigmas_range().end,
            config.num_wires + salt,
            common_data.config.num_challenges * (1 + common_data.num_partial_products) + salt,
            common_data.config.num_challenges * common_data.quotient_degree_factor + salt,
        ];
        let proof_back: FriProof<GoldilocksField, PoseidonHash, 2> =
            stream.read_fri_proof(num_leaves_per_oracle, fri_params);

        assert_eq!(fri_proof, proof_back);
    }
}
