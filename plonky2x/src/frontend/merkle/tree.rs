use ethers::types::H256;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

// #[derive(Clone, Debug, CircuitVariable)]
// #[value_name(MerkleInclusionProof)]
// pub struct MerkleInclusionProofVariable<const PROOF_DEPTH: usize, const LEAF_SIZE_BYTES: usize> {
//     pub aunts: ArrayVariable<Bytes32Variable, PROOF_DEPTH>,
//     pub path_indices: ArrayVariable<BoolVariable, PROOF_DEPTH>,
//     pub enc_leaf: BytesVariable<LEAF_SIZE_BYTES>,
// }

/// The leaf, and it's corresponding proof and path indices against the header.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionProof {
    pub leaf: Vec<u8>,
    // Path and proof should have a fixed length of HEADER_PROOF_DEPTH.
    pub path: Vec<bool>,
    pub proof: Vec<H256>,
}

#[derive(Clone, Debug)]
pub struct MerkleInclusionProofVariable<const PROOF_DEPTH: usize, const LEAF_SIZE_BYTES: usize> {
    pub aunts: ArrayVariable<Bytes32Variable, PROOF_DEPTH>,
    pub path_indices: ArrayVariable<BoolVariable, PROOF_DEPTH>,
    pub leaf: BytesVariable<LEAF_SIZE_BYTES>,
}

impl<const PROOF_DEPTH: usize, const LEAF_SIZE_BYTES: usize> CircuitVariable
    for MerkleInclusionProofVariable<PROOF_DEPTH, LEAF_SIZE_BYTES>
{
    type ValueType<F: RichField> = InclusionProof;

    fn init<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>) -> Self {
        Self {
            aunts: ArrayVariable::<Bytes32Variable, PROOF_DEPTH>::init(builder),
            path_indices: ArrayVariable::<BoolVariable, PROOF_DEPTH>::init(builder),
            leaf: BytesVariable::<LEAF_SIZE_BYTES>::init(builder),
        }
    }

    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Self::ValueType<L::Field>,
    ) -> Self {
        Self {
            aunts: ArrayVariable::<Bytes32Variable, PROOF_DEPTH>::constant(builder, value.proof),
            path_indices: ArrayVariable::<BoolVariable, PROOF_DEPTH>::constant(builder, value.path),
            leaf: BytesVariable::<LEAF_SIZE_BYTES>::constant(
                builder,
                value.leaf.try_into().unwrap(),
            ),
        }
    }

    fn variables(&self) -> Vec<Variable> {
        let mut vars = Vec::new();
        vars.extend(self.aunts.variables());
        vars.extend(self.path_indices.variables());
        vars.extend(self.leaf.variables());
        vars
    }

    fn from_variables(variables: &[Variable]) -> Self {
        let num_elements = ArrayVariable::<Bytes32Variable, PROOF_DEPTH>::nb_elements();
        let aunts = ArrayVariable::<Bytes32Variable, PROOF_DEPTH>::from_variables(
            &variables[0..num_elements],
        );
        let mut offset = num_elements;
        let num_elements = ArrayVariable::<BoolVariable, PROOF_DEPTH>::nb_elements();
        let path_indices = ArrayVariable::<BoolVariable, PROOF_DEPTH>::from_variables(
            &variables[offset..offset + num_elements],
        );
        offset += num_elements;
        let leaf = BytesVariable::<LEAF_SIZE_BYTES>::from_variables(
            &variables[offset..offset + BytesVariable::<LEAF_SIZE_BYTES>::nb_elements()],
        );
        Self {
            aunts,
            path_indices,
            leaf,
        }
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        InclusionProof {
            proof: self.aunts.get(witness),
            path: self.path_indices.get(witness),
            leaf: self.leaf.get(witness).to_vec(),
        }
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.aunts.set(witness, value.proof);
        self.path_indices.set(witness, value.path);
        self.leaf.set(witness, value.leaf.try_into().unwrap());
    }
}
