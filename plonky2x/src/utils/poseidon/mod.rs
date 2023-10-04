use plonky2::hash::hashing::hash_n_to_hash_no_pad;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig, Hasher};

use crate::frontend::hash::poseidon::poseidon256::{
    PoseidonHashOutVariable, PoseidonHashOutVariableValue,
};
use crate::prelude::{CircuitVariable, PlonkParameters};

pub fn mapreduce_merkle_tree_root<
    L: PlonkParameters<D>,
    Input: CircuitVariable,
    const B: usize,
    const D: usize,
>(
    inputs: &[Input::ValueType<L::Field>],
) -> PoseidonHashOutVariableValue<L::Field>
where
    <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
        AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
{
    assert_eq!(inputs.len() % B, 0, "inputs length must be a multiple of B");
    let inputs = inputs.to_vec();

    // Calculate leafs.
    let mut leafs = Vec::new();
    for i in 0..inputs.len() / B {
        let mut input = Vec::new();
        for j in 0..B {
            input.extend(Input::elements::<L::Field>(inputs[i * B + j].clone()));
        }
        let h = hash_n_to_hash_no_pad::<
            L::Field,
            <<<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher as Hasher<
                <L as PlonkParameters<D>>::Field,
            >>::Permutation,
        >(&input);
        leafs.push(h.elements);
    }

    assert!(
        leafs.len().is_power_of_two(),
        "leafs length must be a power of two"
    );

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

    PoseidonHashOutVariable::from_elements::<L::Field>(&leafs[0])
}
