use crate::builder::BuilderAPI;
use crate::vars::{ByteVariable, Variable};

pub struct SimpleSerializeAPI {
    pub api: BuilderAPI,
}

impl SimpleSerializeAPI {
    pub fn new(api: BuilderAPI) -> Self {
        Self { api }
    }

    pub fn verify_proof<const DEPTH: usize, const GINDEX: usize>(
        &mut self,
        _root: [ByteVariable; 32],
        _leaf: [ByteVariable; 32],
        _proof: [[ByteVariable; 32]; DEPTH],
    ) {
    }

    pub fn verify_proof_with_variable_gindex<const DEPTH: usize, const GINDEX: usize>(
        &mut self,
        _root: [ByteVariable; 32],
        _leaf: [ByteVariable; 32],
        _proof: [[ByteVariable; 32]; DEPTH],
        _gindex: Variable,
    ) {
    }

    pub fn restore_merkle_root<const DEPTH: usize, const GINDEX: usize>(
        &mut self,
        _leaf: [ByteVariable; 32],
        _proof: [[ByteVariable; 32]; DEPTH],
    ) -> [ByteVariable; 32] {
        let zero_byte = self.api.zero_byte();
        [zero_byte; 32]
    }

    pub fn hash_tree_root<const NB_LEAVES: usize>(
        &mut self,
        _leaves: [[ByteVariable; 32]; NB_LEAVES],
    ) -> [ByteVariable; 32] {
        let zero_byte = self.api.zero_byte();
        [zero_byte; 32]
    }
}
