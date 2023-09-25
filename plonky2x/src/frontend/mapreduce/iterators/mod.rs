// mod balances;

// use crate::prelude::{CircuitBuilder, CircuitVariable, PlonkParameters};

// // TODO: get rid of batch

// trait MapReduceConfig {
//     /// The number of items we map over in a single map proof.
//     const MAP_BATCH_SIZE: usize;

//     /// The number of items we reduce over in a single reduce proof.
//     const REDUCE_BATCH_SIZE: usize;

//     /// The root of trust for the iterator.
//     type Root: CircuitVariable;

//     /// The type that the end-user interacts with in their map.
//     type Item: CircuitVariable;

//     /// The type that the backend has access to prove validity of the iterator and its items.
//     type ConstraintData: CircuitVariable;

//     /// Fetches the items from the root.
//     fn fetch_items<L: PlonkParameters<D>, const D: usize>(
//         root: Self::Root,
//         batch_idx: usize,
//         builder: &mut CircuitBuilder<L, D>,
//     ) -> Vec<Self::Item>;

//     /// The constraint that automatically gets added on every map proof.
//     fn item_constraints<L: PlonkParameters<D>, const D: usize>(
//         items: Vec<Self::Item>,
//         builder: &mut CircuitBuilder<L, D>,
//     ) -> Self::ConstraintData;

//     /// The constraint that automatically gets added on every reduce proof.
//     fn reduce_constraints<L: PlonkParameters<D>, const D: usize>(
//         constraint_data: Vec<Self::ConstraintData>,
//         builder: &mut CircuitBuilder<L, D>,
//     ) -> Self::ConstraintData;

//     /// The constraint that automatically gets added on the root proof.
//     fn root_constraint<L: PlonkParameters<D>, const D: usize>(
//         root: Self::Root,
//         constraint_data: Self::ConstraintData,
//         builder: &mut CircuitBuilder<L, D>,
//     );
// }
