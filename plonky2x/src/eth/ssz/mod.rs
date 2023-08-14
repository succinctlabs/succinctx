use crate::builder::CircuitBuilder;
use crate::vars::{Bytes32Variable, Variable};

pub struct SimpleSerializeAPI {
    pub api: CircuitBuilder,
}

impl SimpleSerializeAPI {
    pub fn new(api: CircuitBuilder) -> Self {
        Self { api }
    }

    pub fn verify_proof<const DEPTH: usize, const GINDEX: usize>(
        &mut self,
        _root: Bytes32Variable,
        _leaf: Bytes32Variable,
        _proof: [Bytes32Variable; DEPTH],
    ) {
        todo!()
    }

    pub fn verify_proof_with_variable_gindex<const DEPTH: usize, const GINDEX: usize>(
        &mut self,
        _root: Bytes32Variable,
        _leaf: Bytes32Variable,
        _proof: [Bytes32Variable; DEPTH],
        _gindex: Variable,
    ) {
        todo!()
    }

    pub fn restore_merkle_root<const DEPTH: usize, const GINDEX: usize>(
        &mut self,
        _leaf: Bytes32Variable,
        _proof: [Bytes32Variable; DEPTH],
    ) -> Bytes32Variable {
        todo!()
    }

    pub fn hash_tree_root<const NB_LEAVES: usize>(
        &mut self,
        _leaves: [Bytes32Variable; NB_LEAVES],
    ) -> Bytes32Variable {
        todo!()
    }
}
