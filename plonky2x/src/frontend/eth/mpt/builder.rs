use core::str::Bytes;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;

// use super::generators::validator::BeaconValidatorGenerator;
// use super::vars::{BeaconValidatorVariable, BeaconValidatorsVariable};
use array_macro::array;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::generate_partial_witness;
use plonky2::iop::target::Target;

use super::generators::array::{MuxGenerator, NestedMuxGenerator};
use super::generators::keccack::Keccack256Generator;
use super::generators::math::{ByteToVariableGenerator, LeGenerator};
use super::generators::nibbles::NibbleGenerator;
use super::generators::rlp::RLPDecodeListGenerator;
use super::template::get_proof_witnesses;
use crate::builder::CircuitBuilder;
use crate::vars::{BoolVariable, ByteVariable, Bytes32Variable, CircuitVariable, Variable};

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn keccack256(&mut self, input: Bytes32Variable) -> Bytes32Variable {
        let output = self.init::<Bytes32Variable>();
        let generator: Keccack256Generator<F, D> = Keccack256Generator {
            input: input.as_slice().to_vec(),
            output,
            length: None,
            _phantom: std::marker::PhantomData::<F>,
        };
        self.add_simple_generator(&generator);
        output
    }

    pub fn keccack256_variable<const N: usize>(
        &mut self,
        input: [ByteVariable; N],
        len: Variable,
    ) -> Bytes32Variable {
        let output = self.init::<Bytes32Variable>();
        let generator: Keccack256Generator<F, D> = Keccack256Generator {
            input: input.to_vec(),
            output,
            length: Some(len),
            _phantom: std::marker::PhantomData::<F>,
        };
        self.add_simple_generator(&generator);
        output
    }
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn to_nibbles<const N: usize, const M: usize>(
        &mut self,
        input: [ByteVariable; N],
    ) -> [ByteVariable; M] {
        let mut output_vec = vec![];
        for i in 0..N {
            output_vec.push(self.init::<ByteVariable>());
            output_vec.push(self.init::<ByteVariable>());
        }
        let generator = NibbleGenerator {
            input: input.to_vec(),
            output: output_vec.clone(),
            _phantom: std::marker::PhantomData::<F>,
        };
        self.add_simple_generator(&generator);
        output_vec.try_into().unwrap()
    }

    pub fn to_nibbles_unsized(&mut self, input: &[ByteVariable]) -> Vec<ByteVariable> {
        let mut output_vec = vec![];
        for i in 0..input.len() {
            output_vec.push(self.init::<ByteVariable>());
            output_vec.push(self.init::<ByteVariable>());
        }
        let generator = NibbleGenerator {
            input: input.to_vec(),
            output: output_vec.clone(),
            _phantom: std::marker::PhantomData::<F>,
        };
        self.add_simple_generator(&generator);
        output_vec
    }

    // TODO: maybe implement this for the trait CircuitVariable
    pub fn mux<const N: usize>(
        &mut self,
        input: [ByteVariable; N],
        selector: Variable,
    ) -> ByteVariable {
        let output = self.init::<ByteVariable>();
        let generator = MuxGenerator {
            input: input.to_vec(),
            output,
            selector,
            _phantom: std::marker::PhantomData::<F>,
        };
        self.add_simple_generator(&generator);
        output
    }

    // TODO: maybe implement this for the trait CircuitVariable
    pub fn mux_nested<const N: usize>(
        &mut self,
        input: Vec<[ByteVariable; N]>,
        selector: Variable,
    ) -> [ByteVariable; N] {
        let output = array![_ => self.init::<ByteVariable>(); N];
        let generator = NestedMuxGenerator {
            input: input.to_vec(),
            output,
            selector,
            _phantom: std::marker::PhantomData::<F>,
        };
        self.add_simple_generator(&generator);
        output
    }

    pub fn le(&mut self, lhs: Variable, rhs: Variable) -> BoolVariable {
        let output = self.init::<BoolVariable>();
        let generator = LeGenerator {
            lhs,
            rhs,
            output,
            _phantom: std::marker::PhantomData::<F>,
        };
        output
    }

    pub fn byte_to_variable(&mut self, byte: ByteVariable) -> Variable {
        let output = self.init::<Variable>();
        let generator = ByteToVariableGenerator {
            byte,
            output,
            _phantom: std::marker::PhantomData::<F>,
        };
        self.add_simple_generator(&generator);
        output
    }

    pub fn assert_subarray_eq(
        &mut self,
        lhs: &[ByteVariable],
        lhs_offset: Variable,
        rhs: &[ByteVariable],
        rhs_offset: Variable,
        len: Variable,
    ) {
        // todo!();
    }
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    const PREFIX_EXTENSION_EVEN: u8 = 0;
    const PREFIX_EXTENSION_ODD: u8 = 1;
    const PREFIX_LEAF_EVEN: u8 = 2;
    const PREFIX_LEAF_ODD: u8 = 3;
    /// Get the validators for a given block root.
    /// P is the number of proof elements to be considered
    pub fn verify_mpt_proof<const L: usize, const M: usize, const P: usize>(
        &mut self,
        key: Bytes32Variable,
        proof: Box<[[ByteVariable; M]; P]>,
        len_nodes: [Variable; P],
        root: Bytes32Variable,
        value: Bytes32Variable,
    ) {
        const MAX_ELE_SIZE: usize = 34; // Maximum size of list element
        let TREE_RADIX = self.constant::<Variable>(F::from_canonical_u8(16u8));
        let BRANCH_NODE_LENGTH = self.constant::<Variable>(F::from_canonical_u8(17u8));
        let LEAF_OR_EXTENSION_NODE_LENGTH = self.constant::<Variable>(F::from_canonical_u8(2u8));
        let PREFIX_LEAF_EVEN = self.constant::<ByteVariable>(Self::PREFIX_LEAF_EVEN);
        let PREFIX_LEAF_ODD = self.constant::<ByteVariable>(Self::PREFIX_LEAF_ODD);
        let PREFIX_EXTENSION_EVEN = self.constant::<ByteVariable>(Self::PREFIX_EXTENSION_EVEN);
        let PREFIX_EXTENSION_ODD = self.constant::<ByteVariable>(Self::PREFIX_EXTENSION_ODD);
        let ONE = self.one::<Variable>();
        let TWO = self.constant::<Variable>(F::from_canonical_u8(2));
        let _64 = self.constant::<Variable>(F::from_canonical_u8(64));
        let TWO = self.constant::<Variable>(F::from_canonical_u8(2));
        let _32 = self.constant::<Variable>(F::from_canonical_u8(32));
        let _128 = self.constant::<ByteVariable>(128);

        let mut current_key_idx = self.zero::<Variable>();
        let mut finished = self._false();

        let mut current_node_id = array![_ => self.init::<ByteVariable>(); MAX_ELE_SIZE];
        for i in 0..32 {
            current_node_id[i] = root.0 .0[i]; // TODO is there a way to fix this
        }
        let hash_key = self.keccack256(key);
        let key_path = self.to_nibbles::<32, 64>(hash_key.as_slice());
        self.watch(&hash_key, format!("hash_key").as_str());
        self.watch(&root, format!("root").as_str());

        let mut current_node = proof[0];
        for i in 0..P {
            current_node = proof[i];
            let current_node_hash = self.keccack256_variable(current_node, len_nodes[i]);
            // self.watch(
            //     &current_node_hash,
            //     format!("round {}: current_node_hash", i).as_str(),
            // );

            if i == 0 {
                self.assert_eq(current_node_hash, root);
            } else {
                let first_32_bytes_eq = self.eq::<Bytes32Variable, Bytes32Variable>(
                    current_node[0..32].into(),
                    current_node_id[0..32].into(),
                );
                let hash_eq = self.eq::<Bytes32Variable, Bytes32Variable>(
                    current_node_hash,
                    current_node_id[0..32].into(),
                );
                let a = self.constant::<Variable>(F::from_canonical_u8(32u8));
                let node_len_le_32 = self.le(len_nodes[i], a);
                let case_len_le_32 = self.and(node_len_le_32, first_32_bytes_eq);
                let inter = self.not(node_len_le_32);
                let case_len_gt_32 = self.and(inter, hash_eq);
                let equality_fulfilled = self.or(case_len_le_32, case_len_gt_32);
                let checked_equality = self.or(equality_fulfilled, finished);
                let t = self._true();
                self.assert_eq(checked_equality, t);
            }

            self.watch_array(&current_node, format!("Round {} current_node", i).as_str());
            self.watch(&len_nodes[i], format!("Round {} len_nodes[i]", i).as_str());

            // Create the generators for witnessing the decoding of the node
            let rlp_decode_list_generator: RLPDecodeListGenerator<F, D, M, L, MAX_ELE_SIZE> =
                RLPDecodeListGenerator::new(self, current_node, len_nodes[i], finished);
            self.add_simple_generator(&rlp_decode_list_generator);

            let decoded_list_len = rlp_decode_list_generator.decoded_list_len;
            let decoded_element_lens = rlp_decode_list_generator.decoded_element_lens;
            let decoded_list_vec = rlp_decode_list_generator.decoded_list;

            self.watch(
                &decoded_list_len,
                format!("Round {} decoded_list_len", i).as_str(),
            );
            self.watch_array(
                &decoded_element_lens,
                format!("Round {} decoded_element_lens", i).as_str(),
            );
            // self.watch(
            //     &rlp_decode_list_generator.decoded_list[0][0],
            //     "decoded_list from generator",
            // );

            let is_branch = self.eq(decoded_list_len, BRANCH_NODE_LENGTH);
            let is_leaf = self.eq(decoded_list_len, LEAF_OR_EXTENSION_NODE_LENGTH);
            let key_terminated = self.eq(current_key_idx, _64);
            let path = self.to_nibbles_unsized(&decoded_list_vec[0]);
            let prefix = path[0];
            let prefix_leaf_even = self.eq(prefix, PREFIX_LEAF_EVEN);
            let prefix_leaf_odd = self.eq(prefix, PREFIX_LEAF_ODD);
            let prefix_extension_even = self.eq(prefix, PREFIX_EXTENSION_EVEN);
            let prefix_extension_odd = self.eq(prefix, PREFIX_EXTENSION_ODD);

            let offset_even = self.mul(prefix_extension_even.0, TWO);
            let offset_odd = self.mul(prefix_extension_odd.0, ONE);
            let offset = self.add(offset_even, offset_odd);

            let branch_key = self.mux(key_path, current_key_idx);
            let branch_key_variable: Variable = self.byte_to_variable(branch_key); // can be unsafe since nibbles are checked

            // Case 1
            let is_branch_and_key_terminated = self.and(is_branch, key_terminated);
            let case_1_value = self.mul(is_branch_and_key_terminated.0, TREE_RADIX);
            let b = self.not(key_terminated);
            let is_branch_and_key_not_terminated = self.and(is_branch, b);
            let case_2_value = self.mul(is_branch_and_key_not_terminated.0, branch_key_variable);
            let case_3_value = self.mul(is_leaf.0, ONE);

            let c = self.add(case_1_value, case_2_value);
            let updated_current_node_id_idx = self.add(c, case_3_value); // TODO: make this more concise
            self.watch(
                &updated_current_node_id_idx,
                format!("Round {} updated_current_node_id_idx", i).as_str(),
            );

            let updated_current_node_id = self.mux_nested(
                decoded_list_vec
                    .into_iter()
                    .map(|v| v.try_into().unwrap())
                    .collect::<Vec<[ByteVariable; MAX_ELE_SIZE]>>(),
                updated_current_node_id_idx,
            );

            // If finished == 1, then we should not update the current_node_id
            current_node_id =
                self.mux_nested(vec![updated_current_node_id, current_node_id], finished.0);

            let mut do_path_remainder_check = self.not(finished);
            do_path_remainder_check = self.and(do_path_remainder_check, is_leaf);
            let d = self.or(prefix_extension_even, prefix_extension_odd);
            do_path_remainder_check = self.and(do_path_remainder_check, d);

            let e = self.mul(decoded_element_lens[0], TWO);
            let f = self.mul(offset, do_path_remainder_check.0);
            let mut check_length = self.sub(e, f);
            check_length = self.mul(check_length, do_path_remainder_check.0);

            self.assert_subarray_eq(&path, offset, &key_path, current_key_idx, check_length);

            current_key_idx = self.add(current_key_idx, is_branch_and_key_not_terminated.0);
            let j = self.mul(is_leaf.0, check_length);
            current_key_idx = self.add(current_key_idx, j);

            let l = self.or(is_branch_and_key_terminated, prefix_leaf_even);
            let m = self.or(l, prefix_leaf_odd);
            finished = self.or(finished, m);
            self.watch(&finished, format!("Round {} finished", i).as_str());

            for l in 0..34 {
                self.watch(
                    &current_node_id[l],
                    format!("in loop {}: current_node_id[i]", i).as_str(),
                );
            }
        }

        let current_node_len = self.sub(current_node_id[0], _128);
        let current_node_len_as_var = self.byte_to_variable(current_node_len);
        let lhs_offset = self.sub(_32, current_node_len_as_var);
        self.assert_subarray_eq(
            &value.as_slice(),
            lhs_offset,
            &current_node_id,
            ONE,
            current_node_len_as_var,
        );
        self.watch(&value, "value");
        self.watch(&current_node_len_as_var, "At end: current_node_len_as_var");
        for i in 0..34 {
            self.watch(
                &current_node_id[i],
                format!("AT END {}: current_node_id[i]", i).as_str(),
            );
        }
    }
}

mod test {
    use std::collections::HashMap;

    use ethers::providers::{Http, Middleware, Provider};
    use ethers::types::{Address, EIP1186ProofResponse, H256, U256};
    use plonky2::field::types::Field;
    use plonky2::iop::generator::generate_partial_witness;
    use plonky2::iop::witness::PartialWitness;
    use tokio::runtime::Runtime;

    use super::*;
    use crate::builder::CircuitBuilderX;
    use crate::eth::utils::{h256_to_u256_be, u256_to_h256_be};
    use crate::eth::vars::AddressVariable;
    use crate::prelude::{GoldilocksField, PoseidonGoldilocksConfig};
    use crate::utils::{address, bytes, bytes32, hex, setup_logger};

    #[test]
    fn test_mpt_verification() {
        setup_logger();

        let rpc_url = "https://eth-mainnet.g.alchemy.com/v2/V3UkTYUt0iEtxdvWVRNiqEwBsQH4tuMb";
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();

        let block_number = 17880427u64;
        let state_root =
            bytes32!("0xff90251f501c864f21d696c811af4c3aa987006916bd0e31a6c06cc612e7632e");
        let address = address!("0x55032650b14df07b85bF18A3a3eC8E0Af2e028d5");
        let location =
            bytes32!("0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5");

        // Nouns contract
        // let address = address!("0x9c8ff314c9bc7f6e59a9d9225fb22946427edc03");
        // let location = bytes32!("0x0000000000000000000000000000000000000000000000000000000000000003");

        // let get_proof_closure = || -> EIP1186ProofResponse {
        //     let rt = Runtime::new().unwrap();
        //     rt.block_on(async {
        //         provider
        //             .get_proof(address, vec![location], Some(block_number.into()))
        //             .await
        //             .unwrap()
        //     })
        // };
        // let storage_result: EIP1186ProofResponse = get_proof_closure();
        // let serialized = serde_json::to_string(&storage_result).unwrap();
        // println!("{}", serialized);

        let mut file = File::open("./src/eth/mpt/example.json").unwrap();
        let mut context = String::new();
        file.read_to_string(&mut context).unwrap();
        let storage_result: EIP1186ProofResponse = serde_json::from_str(context.as_str()).unwrap();

        let storage_proof = storage_result.storage_proof[0]
            .proof
            .iter()
            .map(|b| b.to_vec())
            .collect::<Vec<Vec<u8>>>();
        let root = storage_result.storage_hash;
        let key = storage_result.storage_proof[0].key;
        let value = storage_result.storage_proof[0].value;
        println!("root {:?} key {:?} value {:?}", root, key, value);

        let value_as_h256 = u256_to_h256_be(value);
        let (proof_as_fixed, lengths_as_fixed) = get_proof_witnesses::<600, 16>(storage_proof);

        // Define the circuit
        let mut builder = CircuitBuilderX::new();
        builder.debug(88680);

        let storage_key = builder.read::<Bytes32Variable>();
        let storage_value = builder.read::<Bytes32Variable>();
        let storage_hash = builder.read::<Bytes32Variable>();

        let mut storage_proof_vec: Vec<[ByteVariable; 600]> = vec![];
        for i in 0..16 {
            let mut v: Vec<ByteVariable> = vec![];
            for _ in 0..600 {
                v.push(builder.read::<ByteVariable>());
            }
            storage_proof_vec.push(v.try_into().unwrap());
        }
        let storage_proof: Box<[[ByteVariable; 600]; 16]> = storage_proof_vec.try_into().unwrap();
        let lengths = array![_ => builder.read::<Variable>(); 16];
        builder.verify_mpt_proof::<17, 600, 16>(
            storage_key,
            storage_proof.clone(),
            lengths,
            storage_hash,
            storage_value,
        );

        println!("Building the circuit");

        let circuit = builder.build::<PoseidonGoldilocksConfig>();

        let mut partial_witness = PartialWitness::new();
        storage_key.set(&mut partial_witness, key);
        storage_value.set(&mut partial_witness, u256_to_h256_be(value));
        storage_hash.set(&mut partial_witness, root);
        for i in 0..16 {
            for j in 0..600 {
                storage_proof[i][j].set(&mut partial_witness, proof_as_fixed[i][j]);
            }
            lengths[i].set(
                &mut partial_witness,
                GoldilocksField::from_canonical_u32(lengths_as_fixed[i]),
            );
        }

        // let watch_hash_map = HashMap::new();

        // let target = storage_hash.0.0[0].0[0].0.0;
        // match target {
        //     Target::VirtualTarget(index) => {
        //         watch_hash_map[index] = "storage_hash";
        //     }
        // }

        let prover_data = circuit.data.prover_only;
        let common_data = circuit.data.common;
        let witness = generate_partial_witness(partial_witness, &prover_data, &common_data);

        // let mut inputs = circuit.input();
        // inputs.write::<Bytes32Variable>(key);
        // inputs.write::<Bytes32Variable>(value_as_h256);
        // inputs.write::<Bytes32Variable>(root);
        // for i in 0..16 {
        //     for j in 0..600 {
        //         inputs.write::<ByteVariable>(proof_as_fixed[i][j]);
        //     }
        // }
        // for i in 0..16 {
        //     inputs.write::<Variable>(GoldilocksField::from_canonical_u32(lengths_as_fixed[i]));
        // }

        // println!("Generating a proof");
        // // Generate a proof.
        // let (proof, output) = circuit.prove(&inputs);
        // // Verify proof.
        // circuit.verify(&proof, &inputs, &output);

        // Read output.
        // let sum = output.read::<Variable>();
        // println!("{}", sum.0);

        // verified_get::<17, 600, 16>(key.to_fixed_bytes(), proof_as_fixed, root.to_fixed_bytes(), value_as_h256.to_fixed_bytes(), lengths_as_fixed);
    }
}
