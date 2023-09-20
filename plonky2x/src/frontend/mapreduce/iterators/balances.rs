use plonky2::iop::witness::{Witness, WitnessWrite};

use super::MapReduceIteratorConfig;
use crate::frontend::uint::uint64::U64Variable;
use crate::prelude::{
    Bytes32Variable, CircuitBuilder, CircuitVariable, PlonkParameters, RichField, Variable,
};

// #[derive(Debug, Clone, Default)]
// struct BeaconBalancesIterator;

// #[derive(Debug, Clone, CircuitVariable)]
// struct BeaconBalancesConstraintData {
//     root: Bytes32Variable,
// }

// impl MapReduceIteratorConfig for BeaconBalancesIterator {
//     const MAP_BATCH_SIZE: usize = 2048;

//     const REDUCE_BATCH_SIZE: usize = 2;

//     type Root = Bytes32Variable;

//     type Item = U64Variable;

//     type ConstraintData = BeaconBalancesConstraintData;

//     fn fetch_items<L: PlonkParameters<D>, const D: usize>(
//         root: Self::Root,
//         batch_idx: usize,
//         builder: &mut CircuitBuilder<L, D>,
//     ) -> [Self::Item; Self::MAP_BATCH_SIZE] {
//         let idx = batch_idx * Self::MAP_BATCH_SIZE;
//         let balances = builder.beacon_get_balance_batch_witness(root, idx);
//         balances
//     }

//     fn map_constraint<L: PlonkParameters<D>, const D: usize>(
//         items: [Self::Item; Self::MAP_BATCH_SIZE],
//         builder: &mut CircuitBuilder<L, D>,
//     ) -> Self::ConstraintData {
//         let mut leafs = Vec::new();
//         for chunk in items.chunks(4) {
//             let leaf = builder.beacon_u64s_to_leaf(chunk);
//             leafs.push(leaf);
//         }

//         while leafs.len() != 1 {
//             let mut tmp = Vec::new();
//             for i in 0..leafs.len() / 2 {
//                 tmp.push(builder.curta_sha256_pair(leafs[i * 2], leafs[i * 2 + 1]));
//             }
//             leafs = tmp;
//         }

//         Self::ConstraintData { root: leafs[0] }
//     }

//     fn reduce_constraint<L: PlonkParameters<D>, const D: usize>(
//         constraint_datas: [Self::ConstraintData; Self::REDUCE_BATCH_SIZE],
//         builder: &mut CircuitBuilder<L, D>,
//     ) -> Self::ConstraintData {
//         let root = builder.sha256_pair(constraint_datas[0].root, constraint_datas[1].root);
//         Self::ConstraintData { root }
//     }

//     fn root_constraint<L: PlonkParameters<D>, const D: usize>(
//         root: Self::Root,
//         constraint_data: Self::ConstraintData,
//         builder: &mut CircuitBuilder<L, D>,
//     ) {
//         builder.assert_is_equal(root, constraint_data.root);
//     }
// }
