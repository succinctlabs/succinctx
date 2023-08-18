// //! This file implements variables relating to recursive proof verification in Plonky2.
// //! Some of the types in this file are a bit wonky due to the decision to not include the generic
// //! types F and D in the CircuitVariable trait. In particular, this file always assumes you are
// //! recursing to depth 2 with Poseidon, and that the field is Goldilocks.
// //! Reference: https://github.com/mir-protocol/plonky2/blob/main/plonky2/src/hash/hash_types.rs

// use array_macro::array;
// use curta::plonky2::field::Field;
// use plonky2::field::extension::Extendable;
// use plonky2::field::goldilocks_field::GoldilocksField;
// use plonky2::hash::hash_types::{HashOut, RichField};
// use plonky2::hash::merkle_tree::MerkleCap;
// use plonky2::hash::poseidon::PoseidonHash;
// use plonky2::iop::target::Target;
// use plonky2::iop::witness::{Witness, WitnessWrite};
// use plonky2::plonk::circuit_data::VerifierOnlyCircuitData;
// use plonky2::plonk::config::PoseidonGoldilocksConfig;

// use super::CircuitVariable;
// use crate::builder::CircuitBuilder;

// pub const NUM_HASH_OUT_ELTS: usize = 4;

// /// The variable version of `HashOutTarget`.
// #[derive(Default)]
// pub struct HashOutVariable {
//     pub elements: [Target; NUM_HASH_OUT_ELTS],
// }

// impl CircuitVariable for HashOutVariable {
//     type ValueType = HashOut<GoldilocksField>;

//     fn init<F: RichField + Extendable<D>, const D: usize>(
//         builder: &mut CircuitBuilder<F, D>,
//     ) -> Self {
//         Self {
//             elements: array![_ => builder.api.add_virtual_target(); 4],
//         }
//     }

//     fn constant<F: RichField + Extendable<D>, const D: usize>(
//         builder: &mut CircuitBuilder<F, D>,
//         value: Self::ValueType,
//     ) -> Self {
//         Self {
//             elements: array![i => builder.api.constant(F::from_canonical_u64(value.elements[i].0)); 4],
//         }
//     }

//     fn targets(&self) -> Vec<Target> {
//         self.elements.to_vec()
//     }

//     fn from_targets(targets: &[Target]) -> Self {
//         Self {
//             elements: [targets[0], targets[1], targets[2], targets[3]],
//         }
//     }

//     fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType {
//         HashOut {
//             elements: array![i => GoldilocksField::from_canonical_u64(
//                 witness.get_target(self.elements[i]).to_canonical_u64()
//             ); 4],
//         }
//     }

//     fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType) {
//         witness.set_target(self.elements[0], F::from_canonical_u64(value.elements[0].0));
//         witness.set_target(self.elements[1], F::from_canonical_u64(value.elements[1].0));
//         witness.set_target(self.elements[2], F::from_canonical_u64(value.elements[2].0));
//         witness.set_target(self.elements[3], F::from_canonical_u64(value.elements[3].0));
//     }
// }

// /// The variable version of `MerkleCapTarget`.
// pub struct MerkleCapVariable(pub [HashOutVariable; 4]);

// impl CircuitVariable for MerkleCapVariable {
//     type ValueType = MerkleCap<GoldilocksField, PoseidonHash>;

//     fn init<F: RichField + Extendable<D>, const D: usize>(
//         builder: &mut CircuitBuilder<F, D>,
//     ) -> Self {
//         Self(array![_ => HashOutVariable::init(builder); 4])
//     }

//     fn constant<F: RichField + Extendable<D>, const D: usize>(
//         builder: &mut CircuitBuilder<F, D>,
//         value: Self::ValueType,
//     ) -> Self {
//         Self(array![i => HashOutVariable::constant(builder, value.0[i]); 4])
//     }

//     fn targets(&self) -> Vec<Target> {
//         let mut targets = Vec::new();
//         for i in 0..4 {
//             targets.extend(self.0[i].targets());
//         }
//         targets
//     }

//     fn from_targets(targets: &[Target]) -> Self {
//         let mut ptr = 0;
//         let a = HashOutVariable::from_targets(&targets[ptr..ptr + NUM_HASH_OUT_ELTS]);
//         ptr += NUM_HASH_OUT_ELTS;
//         let b = HashOutVariable::from_targets(&targets[ptr..ptr + NUM_HASH_OUT_ELTS]);
//         ptr += NUM_HASH_OUT_ELTS;
//         let c = HashOutVariable::from_targets(&targets[ptr..ptr + NUM_HASH_OUT_ELTS]);
//         ptr += NUM_HASH_OUT_ELTS;
//         let d = HashOutVariable::from_targets(&targets[ptr..ptr + NUM_HASH_OUT_ELTS]);
//         Self([a, b, c, d])
//     }

//     fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType {
//         MerkleCap(vec![
//             self.0[0].value(witness),
//             self.0[1].value(witness),
//             self.0[2].value(witness),
//             self.0[3].value(witness),
//         ])
//     }

//     fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType) {
//         self.0[0].set(witness, value.0[0]);
//         self.0[1].set(witness, value.0[1]);
//         self.0[2].set(witness, value.0[2]);
//         self.0[3].set(witness, value.0[3]);
//     }
// }

// /// The variable version of `VerifierCircuitTarget`.
// pub struct VerifierCircuitVariable {
//     /// A commitment to each constant polynomial and each permutation polynomial.
//     pub constants_sigmas_cap: MerkleCapVariable,

//     /// A digest of the "circuit" (i.e. the instance, minus public inputs), which can be used to
//     /// seed Fiat-Shamir.
//     pub circuit_digest: HashOutVariable,
// }

// impl CircuitVariable for VerifierCircuitVariable {
//     type ValueType = VerifierOnlyCircuitData<PoseidonGoldilocksConfig, 2>;

//     fn init<F: RichField + Extendable<D>, const D: usize>(
//         builder: &mut CircuitBuilder<F, D>,
//     ) -> Self {
//         Self {
//             constants_sigmas_cap: MerkleCapVariable::init(builder),
//             circuit_digest: HashOutVariable::init(builder),
//         }
//     }

//     fn constant<F: RichField + Extendable<D>, const D: usize>(
//         builder: &mut CircuitBuilder<F, D>,
//         value: Self::ValueType,
//     ) -> Self {
//         Self {
//             constants_sigmas_cap: MerkleCapVariable::constant(builder, value.constants_sigmas_cap),
//             circuit_digest: HashOutVariable::constant(builder, value.circuit_digest),
//         }
//     }

//     fn targets(&self) -> Vec<Target> {
//         let mut targets = Vec::new();
//         targets.extend(self.constants_sigmas_cap.targets());
//         targets.extend(self.circuit_digest.targets());
//         targets
//     }

//     fn from_targets(targets: &[Target]) -> Self {
//         let mut ptr = 0;
//         let constants_sigmas_cap =
//             MerkleCapVariable::from_targets(&targets[ptr..ptr + 4 * NUM_HASH_OUT_ELTS]);
//         ptr += 4 * NUM_HASH_OUT_ELTS;
//         let circuit_digest = HashOutVariable::from_targets(&targets[ptr..ptr + NUM_HASH_OUT_ELTS]);
//         Self {
//             constants_sigmas_cap,
//             circuit_digest,
//         }
//     }

//     fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType {
//         VerifierOnlyCircuitData {
//             constants_sigmas_cap: self.constants_sigmas_cap.value(witness),
//             circuit_digest: self.circuit_digest.value(witness),
//         }
//     }

//     fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType) {
//         self.constants_sigmas_cap
//             .set(witness, value.constants_sigmas_cap);
//         self.circuit_digest.set(witness, value.circuit_digest);
//     }
// }
