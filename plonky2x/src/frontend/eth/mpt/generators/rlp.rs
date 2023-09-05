// // use core::marker::PhantomData;

// // use curta::math::field::PrimeField64;
// // use plonky2::field::extension::Extendable;
// // use plonky2::hash::hash_types::RichField;
// // use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
// // use plonky2::iop::target::Target;
// // use plonky2::iop::witness::PartitionWitness;
// // use plonky2::plonk::circuit_data::CommonCircuitData;
// // use plonky2::util::serialization::{Buffer, IoResult};
// // use tokio::runtime::Runtime;

// // use crate::builder::CircuitBuilder;
// // use crate::eth::beacon::vars::BeaconValidatorVariable;
// // use crate::eth::mpt::utils::rlp_decode_list_2_or_17;
// // use crate::ethutils::beacon::BeaconClient;
// // use crate::prelude::BoolVariable;
// // use crate::utils::hex;
// // use crate::vars::{ByteVariable, CircuitVariable, Variable};

// // #[derive(Debug, Clone)]
// // pub struct RLPDecodeListGenerator<
// //     F: RichField + Extendable<D>,
// //     const D: usize,
// //     const M: usize,
// //     const L: usize,
// //     const MAX_ELE_SIZE: usize,
// // > {
// //     encoding: [ByteVariable; M],
// //     length: Variable,
// //     finish: BoolVariable,
// //     pub decoded_list: Vec<Vec<ByteVariable>>,
// //     pub decoded_element_lens: Vec<Variable>,
// //     pub decoded_list_len: Variable,
// //     _phantom: PhantomData<F>,
// // }

// // impl<
// //         F: RichField + Extendable<D>,
// //         const D: usize,
// //         const M: usize,
// //         const L: usize,
// //         const MAX_ELE_SIZE: usize,
// //     > RLPDecodeListGenerator<F, D, M, L, MAX_ELE_SIZE>
// // {
// //     pub fn new(
// //         builder: &mut CircuitBuilder<F, D>,
// //         encoding: [ByteVariable; M],
// //         length: Variable,
// //         finish: BoolVariable,
// //     ) -> Self {
// //         let mut decoded_list_vec = Vec::new();
// //         let mut decoded_element_lens = Vec::new();
// //         for i in 0..L {
// //             let mut inner: Vec<ByteVariable> = Vec::new();
// //             for j in 0..MAX_ELE_SIZE {
// //                 inner.push(builder.init::<ByteVariable>());
// //             }
// //             decoded_list_vec.push(inner);
// //             decoded_element_lens.push(builder.init::<Variable>());
// //         }
// //         let decoded_list_len = builder.init::<Variable>();

// //         Self {
// //             encoding,
// //             length,
// //             finish,
// //             decoded_list: decoded_list_vec,
// //             decoded_element_lens,
// //             decoded_list_len,
// //             _phantom: PhantomData,
// //         }
// //     }
// // }

// // impl<
// //         F: RichField + Extendable<D>,
// //         const D: usize,
// //         const M: usize,
// //         const L: usize,
// //         const MAX_ELE_SIZE: usize,
// //     > SimpleGenerator<F, D> for RLPDecodeListGenerator<F, D, M, L, MAX_ELE_SIZE>
// // {
// //     fn id(&self) -> String {
// //         "RLPDecodeListGenerator".to_string()
// //     }

// fn dependencies(&self) -> Vec<Target> {
//     let mut targets: Vec<Target> = Vec::new();
//     targets.extend(
//         self.encoding
//             .iter()
//             .map(|x| x.targets())
//             .flatten()
//             .collect::<Vec<Target>>(),
//     );
//     targets.extend(self.length.targets());
//     targets.extend(self.finish.targets());
//     targets
// }

// fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
//     println!("Running RLP generator");
//     let mut decoded_list_as_fixed = [[0u8; MAX_ELE_SIZE]; L];
//     let mut decoded_list_lens = [0u8; L];
//     let mut decoded_list_len = 0;

//     let finish = self.finish.get(witness);
//     // println!("finished {}", finish);
//     if !finish {
//         let encoding = self
//             .encoding
//             .iter()
//             .map(|x| x.get(witness))
//             .collect::<Vec<_>>();
//         let length = self.length.get(witness).as_canonical_u64() as usize;
//         // println!("length {}", length);
//         // println!("encoding {:?}", encoding);
//         let decoded_element = rlp_decode_list_2_or_17(&encoding.as_slice()[..length]);

//         for (i, element) in decoded_element.iter().enumerate() {
//             let len: usize = element.len();
//             assert!(len <= MAX_ELE_SIZE, "The decoded element should have length <= MAX_ELE_SIZE, has length {} and MAX_ELE_SIZE {}!", len, MAX_ELE_SIZE);
//             decoded_list_as_fixed[i][..len].copy_from_slice(&element);
//             decoded_list_lens[i] = len as u8;
//         }
//         decoded_list_len = decoded_element.len();
//     }
//     // println!("Decoded list len: {}", decoded_list_len);
//     // println!("Decoded list: {:?}", decoded_list_as_fixed);
//     self.decoded_list.iter().enumerate().for_each(|(i, x)| {
//         x.iter()
//             .enumerate()
//             .for_each(|(j, y)| y.set(out_buffer, decoded_list_as_fixed[i][j]))
//     });
//     self.decoded_element_lens
//         .iter()
//         .enumerate()
//         .for_each(|(i, x)| x.set(out_buffer, F::from_canonical_u8(decoded_list_lens[i])));
//     self.decoded_list_len
//         .set(out_buffer, F::from_canonical_usize(decoded_list_len));
// }

// //     #[allow(unused_variables)]
// //     fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
// //         todo!()
// //     }

// //     #[allow(unused_variables)]
// //     fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
// //         todo!()
// //     }
// // }

// // pub mod tests {
// //     use std::collections::HashMap;
// //     use std::fs::File;
// //     use std::io::Read;

// //     use array_macro::array;
// //     use curta::math::field::Field;
// //     use ethers::providers::{Http, Middleware, Provider};
// //     use ethers::types::{Address, EIP1186ProofResponse, H256, U256};
// //     use plonky2::iop::generator::generate_partial_witness;
// //     use plonky2::iop::witness::PartialWitness;
// //     use tokio::runtime::Runtime;

// //     use super::*;
// //     use crate::builder::{CircuitBuilder, CircuitBuilderX};
// //     use crate::eth::mpt::template::get_proof_witnesses;
// //     use crate::eth::utils::{h256_to_u256_be, u256_to_h256_be};
// //     use crate::eth::vars::AddressVariable;
// //     use crate::prelude::{BytesVariable, GoldilocksField, PoseidonGoldilocksConfig, Variable};
// //     use crate::utils::{address, bytes, bytes32, hex, setup_logger};

// //     impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
// //         fn rlp<const M: usize>(&mut self, current_node: [ByteVariable; M], node_length: Variable) {
// //             self.watch_array(&current_node, format!("current_node").as_str());
// //             self.watch(&node_length, format!("len_nodes[i]").as_str());
// //             let finished = self._false();

// //             // Create the generators for witnessing the decoding of the node
// //             // const L: usize, const M: usize, const P: usize
// //             // <34, 600, 16>

// //             let rlp_decode_list_generator: RLPDecodeListGenerator<GoldilocksField, 2, 600, 17, 34> =
// //                 RLPDecodeListGenerator::new(self, current_node, node_length, finished);
// //             self.add_simple_generator(&rlp_decode_list_generator);

// //             let decoded_list_len = rlp_decode_list_generator.decoded_list_len;
// //             let decoded_element_lens = rlp_decode_list_generator.decoded_element_lens;
// //             let decoded_list_vec = rlp_decode_list_generator.decoded_list;

// //             self.watch(&decoded_list_len, format!("decoded_list_len").as_str());
// //             self.watch_array(
// //                 &decoded_element_lens,
// //                 format!("decoded_element_lens").as_str(),
// //             );
// //         }
// //     }

// //     #[test]
// //     fn test_rlp_generator() {
// //         setup_logger();

// //         let mut file = File::open("./src/eth/mpt/example.json").unwrap();
// //         let mut context = String::new();
// //         file.read_to_string(&mut context).unwrap();
// //         let storage_result: EIP1186ProofResponse = serde_json::from_str(context.as_str()).unwrap();

// //         let storage_proof = storage_result.storage_proof[0]
// //             .proof
// //             .iter()
// //             .map(|b| b.to_vec())
// //             .collect::<Vec<Vec<u8>>>();
// //         let root = storage_result.storage_hash;
// //         let key = storage_result.storage_proof[0].key;
// //         let value = storage_result.storage_proof[0].value;
// //         println!("root {:?} key {:?} value {:?}", root, key, value);

// //         let value_as_h256 = u256_to_h256_be(value);
// //         let (proof_as_fixed, lengths_as_fixed) = get_proof_witnesses::<600, 16>(storage_proof);

// //         // Define the circuit
// //         let mut builder = CircuitBuilderX::new();
// //         let current_node = array![_ => builder.read::<ByteVariable>(); 600];
// //         let node_length = builder.read::<Variable>();
// //         builder.rlp(current_node, node_length);
// //         println!("Building the circuit");

// //         let circuit = builder.build::<PoseidonGoldilocksConfig>();

// //         let mut partial_witness = PartialWitness::new();
// //         node_length.set(
// //             &mut partial_witness,
// //             GoldilocksField::from_canonical_u32(lengths_as_fixed[3]),
// //         );
// //         for j in 0..600 {
// //             current_node[j].set(&mut partial_witness, proof_as_fixed[3][j]);
// //         }

// //         let prover_data = circuit.data.prover_only;
// //         let common_data = circuit.data.common;
// //         let witness = generate_partial_witness(partial_witness, &prover_data, &common_data);

// //         // let mut inputs = circuit.input();
// //         // inputs.write::<Bytes32Variable>(key);
// //         // inputs.write::<Bytes32Variable>(value_as_h256);
// //         // inputs.write::<Bytes32Variable>(root);
// //         // for i in 0..16 {
// //         //     for j in 0..600 {
// //         //         inputs.write::<ByteVariable>(proof_as_fixed[i][j]);
// //         //     }
// //         // }
// //         // for i in 0..16 {
// //         //     inputs.write::<Variable>(GoldilocksField::from_canonical_u32(lengths_as_fixed[i]));
// //         // }

// //         // println!("Generating a proof");
// //         // // Generate a proof.
// //         // let (proof, output) = circuit.prove(&inputs);
// //         // // Verify proof.
// //         // circuit.verify(&proof, &inputs, &output);

// //         // Read output.
// //         // let sum = output.read::<Variable>();
// //         // println!("{}", sum.0);

// //         // verified_get::<17, 600, 16>(key.to_fixed_bytes(), proof_as_fixed, root.to_fixed_bytes(), value_as_h256.to_fixed_bytes(), lengths_as_fixed);
// //     }
// // }
