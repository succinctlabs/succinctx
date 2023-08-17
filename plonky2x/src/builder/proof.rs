use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::plonk::proof::ProofWithPublicInputsTarget;

use super::CircuitBuilder;

// impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
//     pub fn add_virtual_proof_with_pis<G: RichField + Extendable<E>, const E: usize>(
//         &mut self,
//         common_data: &CommonCircuitData<G, E>,
//     ) -> ProofWithPublicInputsTarget<E> {
//         let proof = self.add_virtual_proof(common_data);
//         let public_inputs = self.add_virtual_targets(common_data.num_public_inputs);
//         ProofWithPublicInputsTarget {
//             proof,
//             public_inputs,
//         }
//     }

//     fn add_virtual_proof<G: RichField + Extendable<E>, const E: usize>(
//         &mut self,
//         common_data: &CommonCircuitData<G, E>,
//     ) -> ProofTarget<D> {
//         let config = &common_data.config;
//         let fri_params = &common_data.fri_params;
//         let cap_height = fri_params.config.cap_height;

//         let salt = salt_size(common_data.fri_params.hiding);
//         let num_leaves_per_oracle = &[
//             common_data.num_preprocessed_polys(),
//             config.num_wires + salt,
//             common_data.num_zs_partial_products_polys() + common_data.num_all_lookup_polys() + salt,
//             common_data.num_quotient_polys() + salt,
//         ];

//         ProofTarget {
//             wires_cap: self.add_virtual_cap(cap_height),
//             plonk_zs_partial_products_cap: self.add_virtual_cap(cap_height),
//             quotient_polys_cap: self.add_virtual_cap(cap_height),
//             openings: self.add_opening_set(common_data),
//             opening_proof: self.add_virtual_fri_proof(num_leaves_per_oracle, fri_params),
//         }
//     }
// }
