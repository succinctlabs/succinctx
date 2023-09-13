use plonky2::field::types::Field;
use plonky2::hash::hash_types::NUM_HASH_OUT_ELTS;
use plonky2::hash::hashing::hash_n_to_hash_no_pad;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig, Hasher};

use crate::frontend::hash::poseidon::poseidon256::{
    PoseidonHashOutVariable, PoseidonHashOutVariableValue,
};
use crate::prelude::{CircuitVariable, PlonkParameters};

pub fn compute_binary_merkle_tree_root<L: PlonkParameters<D>, V: CircuitVariable, const D: usize>(
    values: &[V::ValueType<L::Field>],
) -> PoseidonHashOutVariableValue<L::Field>
where
    <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
        AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
{
    let values = values.to_vec();

    // Calculate leafs.
    let mut leafs = Vec::new();
    for i in 0..values.len() {
        let input = V::elements::<L, D>(values[i].clone());
        let h = hash_n_to_hash_no_pad::<
            L::Field,
            <<<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher as Hasher<
                <L as PlonkParameters<D>>::Field,
            >>::Permutation,
        >(&input);
        leafs.push(h.elements);
    }

    // Pad leafs to a power of two with the zero leaf.
    let h_zero = [L::Field::ZERO; NUM_HASH_OUT_ELTS];
    while leafs.len() < leafs.len().next_power_of_two() {
        leafs.push(h_zero);
    }

    // Calculate the root.
    while leafs.len() != 1 {
        let mut tmp = Vec::new();
        for i in 0..leafs.len() / 2 {
            let left = leafs[i * 2];
            let right = leafs[i * 2 + 1];
            let mut input = Vec::new();
            input.extend(&left);
            input.extend(&right);
            let h = hash_n_to_hash_no_pad::<
                L::Field,
                <<<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher as Hasher<
                    <L as PlonkParameters<D>>::Field,
                >>::Permutation,
            >(&input);
            tmp.push(h.elements);
        }
        leafs = tmp;
    }

    PoseidonHashOutVariable::from_elements::<L, D>(&leafs[0])
}
