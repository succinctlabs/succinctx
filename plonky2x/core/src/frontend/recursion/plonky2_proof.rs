use crate::frontend::recursion::extension::ExtensionVariable;
use crate::frontend::recursion::fri::proof::FriProofVariable;
use crate::frontend::recursion::hash::MerkleCapVariable;
use crate::prelude::Variable;
pub struct ProofWithPublicInputsVariable<const D: usize> {
    proof: ProofVariable<D>, // Add missing generic argument
    public_inputs: Vec<Variable>,
}

pub struct ProofVariable<const D: usize> {
    wires_cap: MerkleCapVariable,
    plonk_zs_partial_products_cap: MerkleCapVariable,
    quotient_polys_cap: MerkleCapVariable,
    openings: OpeningSetVariable<D>,
    opening_proof: FriProofVariable<D>,
}

pub struct OpeningSetVariable<const D: usize> {
    pub constants: Vec<ExtensionVariable<D>>,
    pub plonk_sigmas: Vec<ExtensionVariable<D>>,
    pub wires: Vec<ExtensionVariable<D>>,
    pub plonk_zs: Vec<ExtensionVariable<D>>,
    pub plonk_zs_next: Vec<ExtensionVariable<D>>,
    pub partial_products: Vec<ExtensionVariable<D>>,
    pub quotient_polys: Vec<ExtensionVariable<D>>,
}
