// use plonky2::field::extension::Extendable;
// use plonky2::field::goldilocks_field::GoldilocksField;
// use plonky2::hash::hash_types::RichField;
// use plonky2::plonk::circuit_data::CommonCircuitData;
// use plonky2::plonk::plonk_common::salt_size;
// use plonky2::plonk::proof::{ProofTarget, ProofWithPublicInputsTarget};

// use super::CircuitBuilder;

// impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
//     /// Copy of https://github.com/mir-protocol/plonky2/blob/main/plonky2/src/recursion/recursive_verifier.rs#L134
//     /// with modifications to enforce that we always use the same recursion config..
//     pub fn add_virtual_proof_with_pis(
//         &mut self,
//         common_data: &CommonCircuitData<GoldilocksField, 2>,
//     ) -> ProofWithPublicInputsTarget<2> {
//         let proof = self.add_virtual_proof(common_data);
//         let public_inputs = self.api.add_virtual_targets(common_data.num_public_inputs);
//         ProofWithPublicInputsTarget {
//             proof,
//             public_inputs,
//         }
//     }

//     /// Copy of https://github.com/mir-protocol/plonky2/blob/main/plonky2/src/recursion/recursive_verifier.rs#L146
//     /// with modifications to enforce that we always use the same recursion config and to expose
//     /// private methods.
//     fn add_virtual_proof(
//         &mut self,
//         common_data: &CommonCircuitData<GoldilocksField, 2>,
//     ) -> ProofTarget<2> {
//         let config = &common_data.config;
//         let fri_params = &common_data.fri_params;
//         let cap_height = fri_params.config.cap_height;

//         let salt = salt_size(common_data.fri_params.hiding);

//         // Calculating the result of private methods.
//         let num_preprocessed_polys = common_data.sigmas_range().end;
//         let num_zs_partial_products_polys =
//             common_data.config.num_challenges * (1 + common_data.num_partial_products);
//         let num_all_lookup_polys = common_data.config.num_challenges * common_data.num_lookup_polys;
//         let num_quotient_polys =
//             common_data.config.num_challenges * common_data.quotient_degree_factor;

//         let num_leaves_per_oracle = &[
//             num_preprocessed_polys,
//             config.num_wires + salt,
//             num_zs_partial_products_polys + num_all_lookup_polys + salt,
//             num_quotient_polys + salt,
//         ];

//         ProofTarget {
//             wires_cap: self.api.add_virtual_cap(cap_height),
//             plonk_zs_partial_products_cap: self.api.add_virtual_cap(cap_height),
//             quotient_polys_cap: self.api.add_virtual_cap(cap_height),
//             openings: self.api.add_opening_set(common_data),
//             opening_proof: self
//                 .api
//                 .add_virtual_fri_proof(num_leaves_per_oracle, fri_params),
//         }
//     }
// }
