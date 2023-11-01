use array_macro::array;
use ethers::types::{H256, U256};
use log::debug;

use super::generators::{
    BeaconAllWithdrawalsHint, BeaconBalanceBatchWitnessHint, BeaconBalanceGenerator,
    BeaconBalanceWitnessHint, BeaconBalancesGenerator, BeaconBlockRootsHint,
    BeaconExecutionPayloadHint, BeaconGraffitiHint, BeaconHeaderHint,
    BeaconHeadersFromOffsetRangeHint, BeaconHistoricalBlockHint, BeaconPartialBalancesHint,
    BeaconPartialValidatorsHint, BeaconValidatorBatchHint, BeaconValidatorGenerator,
    BeaconValidatorsHint, BeaconWithdrawalGenerator, BeaconWithdrawalsGenerator,
    CompressedBeaconValidatorBatchHint, Eth1BlockToSlotHint, CLOSE_SLOT_BLOCK_ROOT_DEPTH,
    FAR_SLOT_BLOCK_ROOT_DEPTH, FAR_SLOT_HISTORICAL_SUMMARY_DEPTH,
};
use super::vars::{
    BeaconBalancesVariable, BeaconHeaderVariable, BeaconValidatorVariable,
    BeaconValidatorsVariable, BeaconWithdrawalVariable, BeaconWithdrawalsVariable,
    CompressedBeaconValidatorVariable,
};
use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::vars::BLSPubkeyVariable;
use crate::frontend::uint::uint256::U256Variable;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::{
    Bytes32Variable, CircuitVariable, EvmVariable, SSZVariable, VariableStream,
};
use crate::prelude::{ArrayVariable, BoolVariable, ByteVariable, BytesVariable};
use crate::utils::eth::concat_g_indices;

/// The gindex for blockRoot -> validatorsRoot.
const VALIDATORS_ROOT_GINDEX: u64 = 363;

/// The gindex for blockRoot -> stateRoot.
const STATE_ROOT_GINDEX: u64 = 11;

/// The gindex for blockRoot -> balancesRoot.
const BALANCES_ROOT_GINDEX: u64 = 364;

/// The gindex for blockRoot -> withdrawalsRoot.
const WITHDRAWALS_ROOT_GINDEX: u64 = 3230;

/// The gindex for validatorsRoot -> validators[i].
const VALIDATOR_BASE_GINDEX: u64 = 1099511627776 * 2;

/// The gindex for balancesRoot -> balances[i].
const BALANCE_BASE_GINDEX: u64 = 549755813888;

/// The gindex for withdrawalsRoot -> withdrawals[i].
const WITHDRAWAL_BASE_GINDEX: u64 = 32;

/// The gindex for blockRoot -> state -> state.historicalSummaries[0].
const HISTORICAL_SUMMARIES_BASE_GINDEX: u64 = 12717129728;

/// The gindex for state.historicalSummaries[i] -> block_summary/block_roots -> block_roots[0].
const HISTORICAL_SUMMARY_BLOCK_ROOT_GINDEX: u64 = 16384;

/// The gindex for blockRoot -> state -> state.block_roots[0].
const CLOSE_SLOT_BLOCK_ROOT_GINDEX: u64 = 2924544;

/// The gindex for blockRoot -> body -> executionPayload -> blockNumber.
const EXECUTION_PAYLOAD_BLOCK_NUMBER_GINDEX: u64 = 3222;

/// The log2 of the validator registry limit.
const VALIDATOR_REGISTRY_LIMIT_LOG2: usize = 40;

/// The depth of the proof from the blockRoot -> balancesRoot.
const BALANCES_PROOF_DEPTH: usize = 8;

/// The depth of the proof from the blockRoot -> blockRoots.
const BLOCK_ROOTS_PROOF_DEPTH: usize = 8;

/// The depth of the proof from blockRoot -> graffiti.
const GRAFFITI_PROOF_DEPTH: usize = 7;

/// The gindex for stateRoot -> validators;
const VALIDATORS_GINDEX: usize = 43;

/// The gindex for stateRoot -> balances;
const BALANCES_GINDEX: usize = 44;

/// The gindex for blockRoot -> blockRoots.
const BLOCK_ROOTS_GINDEX: usize = 357;

/// The gindex for blockRoot -> graffiti.
const GRAFFITI_GINDEX: usize = 194;

/// Beacon chain constant SLOTS_PER_EPOCH.
const SLOTS_PER_EPOCH: u64 = 32;

/// Beacon chain constant SLOTS_PER_HISTORICAL_ROOT.
const SLOTS_PER_HISTORICAL_ROOT: usize = 8192;

/// Beacon chain constant CAPELLA_FORK_EPOCH (mainnet specific).
const CAPELLA_FORK_EPOCH: u64 = 194048;

/// Beacon chain constant MAX_WITHDRAWALS_PER_PAYLOAD.
const MAX_WITHDRAWALS_PER_PAYLOAD: usize = 16;

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Get the first B validators for a given block root.
    pub fn beacon_get_partial_validators<const B: usize>(
        &mut self,
        block_root: Bytes32Variable,
    ) -> BeaconValidatorsVariable {
        let b_log2 = (B as f64).log2().ceil() as usize;
        let hint = BeaconPartialValidatorsHint::<B> {};
        let mut input_stream = VariableStream::new();
        input_stream.write(&block_root);

        let output_stream = self.hint(input_stream, hint);
        let partial_validators_root = output_stream.read::<Bytes32Variable>(self);
        let nb_branches = BALANCES_PROOF_DEPTH + (VALIDATOR_REGISTRY_LIMIT_LOG2 + 1 - b_log2);
        let mut proof = Vec::new();
        for _ in 0..nb_branches {
            proof.push(output_stream.read::<Bytes32Variable>(self));
        }

        let gindex = VALIDATORS_GINDEX * (2usize.pow(41 - b_log2 as u32));
        let gindex = concat_g_indices(&[STATE_ROOT_GINDEX as usize, gindex]);
        self.ssz_verify_proof_const(block_root, partial_validators_root, &proof, gindex as u64);
        BeaconValidatorsVariable {
            block_root,
            validators_root: partial_validators_root,
        }
    }

    /// Get the validators for a given block root.
    pub fn beacon_get_validators(
        &mut self,
        block_root: Bytes32Variable,
    ) -> BeaconValidatorsVariable {
        let mut input_stream = VariableStream::new();
        input_stream.write(&block_root);
        let hint = BeaconValidatorsHint::new();
        let output_stream = self.async_hint(input_stream, hint);

        let validators_root = output_stream.read::<Bytes32Variable>(self);
        let proof = array![_ => output_stream.read::<Bytes32Variable>(self); 8];
        self.ssz_verify_proof_const(block_root, validators_root, &proof, VALIDATORS_ROOT_GINDEX);
        BeaconValidatorsVariable {
            block_root,
            validators_root,
        }
    }

    /// Get a beacon validator from a given dynamic index.
    pub fn beacon_get_validator(
        &mut self,
        validators: BeaconValidatorsVariable,
        index: U64Variable,
    ) -> BeaconValidatorVariable {
        let generator =
            BeaconValidatorGenerator::new_with_index_variable(self, validators.block_root, index);
        self.add_simple_generator(generator.clone());
        let validator_root = self.ssz_hash_tree_root(generator.validator);
        let mut gindex = self.constant::<U64Variable>(VALIDATOR_BASE_GINDEX);
        gindex = self.add(gindex, index);
        self.ssz_verify_proof(
            validators.validators_root,
            validator_root,
            &generator.proof,
            gindex,
        );
        generator.validator
    }

    /// Witness the first B validators from a given start index.
    pub fn beacon_witness_validator_batch<const B: usize>(
        &mut self,
        balances: BeaconValidatorsVariable,
        start_idx: U64Variable,
    ) -> ArrayVariable<BeaconValidatorVariable, B> {
        let mut input_stream = VariableStream::new();
        input_stream.write(&balances.block_root);
        input_stream.write(&start_idx);
        let hint = BeaconValidatorBatchHint::<B> {};
        let output_stream = self.hint(input_stream, hint);
        output_stream.read::<ArrayVariable<BeaconValidatorVariable, B>>(self)
    }

    /// Witness the first B validators from a given start index.
    pub fn beacon_witness_compressed_validator_batch<const B: usize>(
        &mut self,
        balances: BeaconValidatorsVariable,
        start_idx: U64Variable,
    ) -> (
        Vec<Bytes32Variable>,
        ArrayVariable<CompressedBeaconValidatorVariable, B>,
    ) {
        let mut input_stream = VariableStream::new();
        input_stream.write(&balances.block_root);
        input_stream.write(&start_idx);
        let hint = CompressedBeaconValidatorBatchHint::<B> {};
        let output_stream = self.hint(input_stream, hint);
        let compressed_validators =
            output_stream.read::<ArrayVariable<CompressedBeaconValidatorVariable, B>>(self);
        let witnesses =
            output_stream.read::<ArrayVariable<ArrayVariable<Bytes32Variable, 2>, B>>(self);
        let zero = self.constant::<ByteVariable>(0);
        let mut roots = Vec::new();
        for i in 0..B {
            let compressed_validator = compressed_validators[i].clone();
            let mut pubkey_bytes = compressed_validator.pubkey.0 .0.to_vec();
            pubkey_bytes.extend([zero; 16]);
            let pubkey = self.curta_sha256(&pubkey_bytes);
            let h11 = self.curta_sha256_pair(pubkey, compressed_validator.withdrawal_credentials);
            let h12 = witnesses[i][0];
            let h21 = self.curta_sha256_pair(h11, h12);
            let h22 = witnesses[i][1];
            let h31 = self.curta_sha256_pair(h21, h22);
            roots.push(h31);
        }
        (roots, compressed_validators)
    }

    /// Get a validator from a given deterministic index.
    pub fn beacon_get_validator_const(
        &mut self,
        validators: BeaconValidatorsVariable,
        index: u64,
    ) -> BeaconValidatorVariable {
        let generator =
            BeaconValidatorGenerator::new_with_index_const(self, validators.block_root, index);
        self.add_simple_generator(generator.clone());
        let validator_root = self.ssz_hash_tree_root(generator.validator);
        let gindex = VALIDATOR_BASE_GINDEX + index;
        self.ssz_verify_proof_const(
            validators.validators_root,
            validator_root,
            &generator.proof,
            gindex,
        );
        generator.validator
    }

    /// Gets a validator from a given pubkey. Returns the validator index along with the validator
    /// data.
    pub fn beacon_get_validator_by_pubkey(
        &mut self,
        validators: BeaconValidatorsVariable,
        pubkey: BLSPubkeyVariable,
    ) -> (U64Variable, BeaconValidatorVariable) {
        let generator =
            BeaconValidatorGenerator::new_with_pubkey_variable(self, validators.block_root, pubkey);
        self.add_simple_generator(generator.clone());
        let validator_root = self.ssz_hash_tree_root(generator.validator);
        let mut gindex = self.constant::<U64Variable>(VALIDATOR_BASE_GINDEX);
        gindex = self.add(gindex, generator.validator_idx);
        self.ssz_verify_proof(
            validators.validators_root,
            validator_root,
            &generator.proof,
            gindex,
        );
        self.assert_is_equal(generator.validator.pubkey, pubkey);
        (generator.validator_idx, generator.validator)
    }

    /// Get the balances for a given block root.
    pub fn beacon_get_balances(&mut self, block_root: Bytes32Variable) -> BeaconBalancesVariable {
        let generator =
            BeaconBalancesGenerator::new(self, self.beacon_client.clone().unwrap(), block_root);
        self.add_simple_generator(generator.clone());
        self.ssz_verify_proof_const(
            block_root,
            generator.balances_root,
            &generator.proof,
            BALANCES_ROOT_GINDEX,
        );
        BeaconBalancesVariable {
            block_root,
            root: generator.balances_root,
        }
    }

    /// Get the first B balances for a given block root.
    pub fn beacon_get_partial_balances<const B: usize>(
        &mut self,
        block_root: Bytes32Variable,
    ) -> BeaconBalancesVariable {
        let b_log2 = (B as f64).log2().ceil() as usize;
        let hint = BeaconPartialBalancesHint::<B> {};
        let mut input_stream = VariableStream::new();
        input_stream.write(&block_root);

        let output_stream = self.hint(input_stream, hint);
        let partial_balances_root = output_stream.read::<Bytes32Variable>(self);
        let nb_branches = BALANCES_PROOF_DEPTH + (VALIDATOR_REGISTRY_LIMIT_LOG2 + 1 - b_log2);
        let mut proof = Vec::new();
        for _ in 0..nb_branches {
            proof.push(output_stream.read::<Bytes32Variable>(self));
        }

        let gindex = BALANCES_GINDEX * (2usize.pow(41 - b_log2 as u32));
        let gindex = concat_g_indices(&[STATE_ROOT_GINDEX as usize, gindex]);
        self.ssz_verify_proof_const(block_root, partial_balances_root, &proof, gindex as u64);
        BeaconBalancesVariable {
            block_root,
            root: partial_balances_root,
        }
    }

    /// Serializes a list of u64s into a single leaf according to the SSZ spec.
    pub fn beacon_u64s_to_leaf(&mut self, u64s: [U64Variable; 4]) -> Bytes32Variable {
        let mut leaf = self.init_unsafe::<Bytes32Variable>();
        let bytes = [
            u64s[0].encode(self),
            u64s[1].encode(self),
            u64s[2].encode(self),
            u64s[3].encode(self),
        ];
        for i in 0..8 {
            leaf.0 .0[i] = bytes[0][7 - i];
            leaf.0 .0[i + 8] = bytes[1][7 - i];
            leaf.0 .0[i + 16] = bytes[2][7 - i];
            leaf.0 .0[i + 24] = bytes[3][7 - i];
        }
        leaf
    }

    /// Get a validator balance with no constraints.
    pub fn beacon_get_balance_witness(
        &mut self,
        balances: BeaconBalancesVariable,
        index: U64Variable,
    ) -> U64Variable {
        let mut input_stream = VariableStream::new();
        input_stream.write(&balances.block_root);
        input_stream.write(&index);

        let hint = BeaconBalanceWitnessHint {};
        let output_stream = self.hint(input_stream, hint);
        output_stream.read::<U64Variable>(self)
    }

    /// Witness the first B balances from a given start index.
    pub fn beacon_witness_balance_batch<const B: usize>(
        &mut self,
        balances: BeaconBalancesVariable,
        start_idx: U64Variable,
    ) -> ArrayVariable<U64Variable, B> {
        let mut input_stream = VariableStream::new();
        input_stream.write(&balances.block_root);
        input_stream.write(&start_idx);
        let hint = BeaconBalanceBatchWitnessHint::<B> {};
        let output_stream = self.hint(input_stream, hint);
        output_stream.read::<ArrayVariable<U64Variable, B>>(self)
    }

    /// Get a validator balance from a given deterministic index.
    pub fn beacon_get_balance(
        &mut self,
        balances: BeaconBalancesVariable,
        index: U64Variable,
    ) -> U64Variable {
        let generator =
            BeaconBalanceGenerator::new_with_index_variable(self, balances.block_root, index);
        self.add_simple_generator(generator.clone());
        let mut gindex = self.constant::<U64Variable>(BALANCE_BASE_GINDEX);
        let four = self.constant::<U64Variable>(4);

        let offset = self.div(index, four);
        gindex = self.add(gindex, offset);

        self.ssz_verify_proof(
            balances.root,
            generator.balance_leaf,
            &generator.proof,
            gindex,
        );

        let index = self.rem(index, four);
        let bits = self.to_le_bits(index);
        let first_half: BytesVariable<16> =
            BytesVariable::<16>(generator.balance_leaf.0 .0[..16].try_into().unwrap());
        let second_half: BytesVariable<16> =
            BytesVariable::<16>(generator.balance_leaf.0 .0[16..].try_into().unwrap());
        let half = self.select(bits[1], second_half, first_half);
        let first_quarter: BytesVariable<8> = BytesVariable::<8>(half.0[..8].try_into().unwrap());
        let second_quarter: BytesVariable<8> = BytesVariable::<8>(half.0[8..].try_into().unwrap());
        let quarter = self.select(bits[0], second_quarter, first_quarter);

        let balance_bytes = generator.balance.encode(self);
        let quarter_bytes = quarter.0;
        for i in 0..8 {
            self.assert_is_equal(balance_bytes[7 - i], quarter_bytes[i]);
        }

        generator.balance
    }

    /// Get the withdrawals for a given block root.
    pub fn beacon_get_withdrawals(
        &mut self,
        block_root: Bytes32Variable,
    ) -> BeaconWithdrawalsVariable {
        let generator =
            BeaconWithdrawalsGenerator::new(self, self.beacon_client.clone().unwrap(), block_root);
        self.add_simple_generator(generator.clone());
        self.ssz_verify_proof_const(
            block_root,
            generator.withdrawals_root,
            &generator.proof,
            WITHDRAWALS_ROOT_GINDEX,
        );
        BeaconWithdrawalsVariable {
            block_root,
            withdrawals_root: generator.withdrawals_root,
        }
    }

    /// Get and prove all 16 withdrawal containers for a given block root.
    pub fn beacon_get_all_withdrawals(
        &mut self,
        block_root: Bytes32Variable,
    ) -> ArrayVariable<BeaconWithdrawalVariable, MAX_WITHDRAWALS_PER_PAYLOAD> {
        let withdrawals_variable = self.beacon_get_withdrawals(block_root);

        let mut withdrawals_hint_input = VariableStream::new();
        withdrawals_hint_input.write(&block_root);
        let withdrawals_hint_output =
            self.async_hint(withdrawals_hint_input, BeaconAllWithdrawalsHint {});

        let withdrawals = withdrawals_hint_output
            .read::<ArrayVariable<BeaconWithdrawalVariable, MAX_WITHDRAWALS_PER_PAYLOAD>>(self);

        let leafs = withdrawals
            .data
            .iter()
            .map(|w| w.hash_tree_root(self))
            .collect::<Vec<_>>();
        let items_root = self.ssz_hash_leafs(&leafs);

        // SSZ lists encoded as [items_root, list_length]
        // List length is u256 LE
        let mut list_length = vec![0u8; 32];
        U256::from(16).to_little_endian(&mut list_length);
        let list_length_array: [u8; 32] = list_length.try_into().unwrap();
        let list_length_variable = self.constant::<Bytes32Variable>(H256::from(list_length_array));

        let reconstructed_root = self.ssz_hash_leafs(&[items_root, list_length_variable]);
        self.assert_is_equal(withdrawals_variable.withdrawals_root, reconstructed_root);

        withdrawals
    }

    /// Get a validator withdrawal from a given index.
    pub fn beacon_get_withdrawal(
        &mut self,
        withdrawals: BeaconWithdrawalsVariable,
        idx: U64Variable,
    ) -> BeaconWithdrawalVariable {
        let generator = BeaconWithdrawalGenerator::new(
            self,
            self.beacon_client.clone().unwrap(),
            withdrawals,
            idx,
        );
        self.add_simple_generator(generator.clone());
        let mut gindex = self.constant::<U64Variable>(WITHDRAWAL_BASE_GINDEX);
        gindex = self.add(gindex, idx);
        let leaf = self.ssz_hash_tree_root(generator.withdrawal);
        self.ssz_verify_proof(withdrawals.withdrawals_root, leaf, &generator.proof, gindex);
        generator.withdrawal
    }

    /// Get block header from block root.
    pub fn beacon_get_block_header(&mut self, block_root: Bytes32Variable) -> BeaconHeaderVariable {
        let mut slot_hint_input = VariableStream::new();
        slot_hint_input.write(&block_root);
        let slot_hint_output = self.async_hint(slot_hint_input, BeaconHeaderHint {});
        let header = slot_hint_output.read::<BeaconHeaderVariable>(self);

        let restored_root = self.ssz_hash_tree_root(header);
        self.assert_is_equal(block_root, restored_root);

        header
    }

    /// Get beacon block hash from eth1 block number, then prove from source block root.
    pub fn beacon_get_block_from_eth1_block_number(
        &mut self,
        source_beacon_block_root: Bytes32Variable,
        source_slot: U64Variable,
        eth1_block_number: U256Variable,
    ) -> Bytes32Variable {
        debug!("beacon_get_block_from_eth1_block_number");
        // Witness the slot number from the eth1 block number
        let mut block_to_slot_input = VariableStream::new();
        block_to_slot_input.write(&eth1_block_number);
        let block_to_slot_output = self.async_hint(block_to_slot_input, Eth1BlockToSlotHint {});
        let slot = block_to_slot_output.read::<U64Variable>(self);

        // Prove source block root -> witnessed beacon block
        let target_root =
            self.beacon_get_historical_block(source_beacon_block_root, source_slot, slot);
        self.watch(&target_root, "target_root_in_eth1blocknumber");

        // Witness SSZ proof for target block root -> beacon body -> execution payload -> eth1 block number
        let mut beacon_block_to_eth1_number_input = VariableStream::new();
        beacon_block_to_eth1_number_input.write(&target_root);
        let beacon_block_to_eth1_number_output = self.hint(
            beacon_block_to_eth1_number_input,
            BeaconExecutionPayloadHint {},
        );
        let proof =
            beacon_block_to_eth1_number_output.read::<ArrayVariable<Bytes32Variable, 11>>(self);

        // Convert eth1 block number to leaf
        let eth1_block_number_leaf = eth1_block_number.hash_tree_root(self);

        // Verify the SSZ proof
        self.ssz_verify_proof_const(
            target_root,
            eth1_block_number_leaf,
            &proof.data,
            EXECUTION_PAYLOAD_BLOCK_NUMBER_GINDEX,
        );

        debug!("beacon_get_block_from_eth1_block_number done");
        target_root
    }

    /// Get a historical block root using state.block_roots for close slots and historical_summaries for slots > 8192 slots away.
    pub fn beacon_get_historical_block(
        &mut self,
        block_root: Bytes32Variable,
        source_slot: U64Variable,
        target_slot: U64Variable,
    ) -> Bytes32Variable {
        let mut hint_input = VariableStream::new();
        hint_input.write(&block_root);
        hint_input.write(&target_slot);
        let hint_output = self.async_hint(hint_input, BeaconHistoricalBlockHint {});

        let target_block_root = hint_output.read::<Bytes32Variable>(self);
        let close_slot_block_root_proof =
            hint_output.read::<ArrayVariable<Bytes32Variable, CLOSE_SLOT_BLOCK_ROOT_DEPTH>>(self);
        let far_slot_block_root_proof =
            hint_output.read::<ArrayVariable<Bytes32Variable, FAR_SLOT_BLOCK_ROOT_DEPTH>>(self);
        let far_slot_historical_summary_root = hint_output.read::<Bytes32Variable>(self);
        let far_slot_historical_summary_proof = hint_output
            .read::<ArrayVariable<Bytes32Variable, FAR_SLOT_HISTORICAL_SUMMARY_DEPTH>>(self);

        self.watch(
            &target_block_root,
            "target_block_root in get_historical_block",
        );

        // Use close slot logic if (source - target) < 8192
        let source_sub_target = self.sub(source_slot, target_slot);
        let slots_per_historical = self.constant::<U64Variable>(SLOTS_PER_HISTORICAL_ROOT as u64);
        let one_u64 = self.constant::<U64Variable>(1);
        let slots_per_historical_sub_one = self.sub(slots_per_historical, one_u64);
        let is_close_slot = self.lte(source_sub_target, slots_per_historical_sub_one);

        let block_roots_array_index = self.rem(target_slot, slots_per_historical);

        // Close slot logic
        let mut close_slot_block_root_gindex =
            self.constant::<U64Variable>(CLOSE_SLOT_BLOCK_ROOT_GINDEX);
        close_slot_block_root_gindex =
            self.add(close_slot_block_root_gindex, block_roots_array_index);
        let restored_close_slot_block_root = self.ssz_restore_merkle_root(
            target_block_root,
            &close_slot_block_root_proof.as_vec(),
            close_slot_block_root_gindex,
        );
        let valid_close_slot = self.is_equal(restored_close_slot_block_root, block_root);

        // Far slot logic
        let capella_slot = self.constant::<U64Variable>(CAPELLA_FORK_EPOCH * SLOTS_PER_EPOCH);
        let slots_since_capella = self.sub(target_slot, capella_slot);
        let historical_summary_array_index = self.div(slots_since_capella, slots_per_historical);
        let mut historical_summary_gindex =
            self.constant::<U64Variable>(HISTORICAL_SUMMARIES_BASE_GINDEX);
        historical_summary_gindex =
            self.add(historical_summary_gindex, historical_summary_array_index);
        let restored_far_slot_block_root = self.ssz_restore_merkle_root(
            far_slot_historical_summary_root,
            &far_slot_historical_summary_proof.as_vec(),
            historical_summary_gindex,
        );
        let valid_far_slot_block_root = self.is_equal(restored_far_slot_block_root, block_root);

        let mut far_slot_block_root_gindex =
            self.constant::<U64Variable>(HISTORICAL_SUMMARY_BLOCK_ROOT_GINDEX);
        far_slot_block_root_gindex = self.add(far_slot_block_root_gindex, block_roots_array_index);
        let restored_far_slot_historical_root = self.ssz_restore_merkle_root(
            target_block_root,
            &far_slot_block_root_proof.as_vec(),
            far_slot_block_root_gindex,
        );
        let valid_far_slot_historical_root = self.is_equal(
            restored_far_slot_historical_root,
            far_slot_historical_summary_root,
        );
        let valid_far_slot = self.and(valid_far_slot_block_root, valid_far_slot_historical_root);

        let valid = self.select(is_close_slot, valid_close_slot, valid_far_slot);

        let true_bool = self.constant::<BoolVariable>(true);
        self.assert_is_equal(valid, true_bool);

        target_block_root
    }

    pub fn beacon_get_block_roots(
        &mut self,
        block_root: Bytes32Variable,
    ) -> ArrayVariable<Bytes32Variable, SLOTS_PER_HISTORICAL_ROOT> {
        let mut input = VariableStream::new();
        input.write(&block_root);
        let output = self.hint(input, BeaconBlockRootsHint {});
        let block_roots_root = output.read::<Bytes32Variable>(self);
        let proof = output.read::<ArrayVariable<Bytes32Variable, BLOCK_ROOTS_PROOF_DEPTH>>(self);
        let block_roots =
            output.read::<ArrayVariable<Bytes32Variable, SLOTS_PER_HISTORICAL_ROOT>>(self);
        self.ssz_verify_proof_const(
            block_root,
            block_roots_root,
            proof.as_slice(),
            BLOCK_ROOTS_GINDEX as u64,
        );
        let root = self.ssz_hash_leafs(block_roots.as_slice());
        self.assert_is_equal(root, block_roots_root);
        block_roots
    }

    pub fn beacon_get_graffiti(&mut self, block_root: Bytes32Variable) -> Bytes32Variable {
        let mut input = VariableStream::new();
        input.write(&block_root);
        let output = self.hint(input, BeaconGraffitiHint {});
        let graffiti = output.read::<Bytes32Variable>(self);
        let proof = output.read::<ArrayVariable<Bytes32Variable, GRAFFITI_PROOF_DEPTH>>(self);
        self.ssz_verify_proof_const(
            block_root,
            graffiti,
            proof.as_slice(),
            GRAFFITI_GINDEX as u64,
        );
        graffiti
    }

    pub fn beacon_witness_headers_from_offset_range<const B: usize>(
        &mut self,
        end_block_root: Bytes32Variable,
        start_offset: U64Variable,
        end_offset: U64Variable,
    ) -> ArrayVariable<Bytes32Variable, B> {
        let mut input = VariableStream::new();
        input.write(&end_block_root);
        input.write(&start_offset);
        input.write(&end_offset);
        let output = self.hint(input, BeaconHeadersFromOffsetRangeHint::<B> {});
        output.read::<ArrayVariable<Bytes32Variable, B>>(self)
    }

    /// Verify a simple serialize (ssz) merkle proof with a dynamic index.
    #[allow(unused_variables)]
    pub fn ssz_verify_proof(
        &mut self,
        root: Bytes32Variable,
        leaf: Bytes32Variable,
        branch: &[Bytes32Variable],
        gindex: U64Variable,
    ) {
        let expected_root = self.ssz_restore_merkle_root(leaf, branch, gindex);
        self.assert_is_equal(root, expected_root);
    }

    /// Verify a simple serialize (ssz) merkle proof with a constant index.
    #[allow(unused_variables)]
    pub fn ssz_verify_proof_const(
        &mut self,
        root: Bytes32Variable,
        leaf: Bytes32Variable,
        branch: &[Bytes32Variable],
        gindex: u64,
    ) {
        let expected_root = self.ssz_restore_merkle_root_const(leaf, branch, gindex);
        self.assert_is_equal(root, expected_root);
    }

    /// Computes the expected merkle root given a leaf, branch, and dynamic index.
    pub fn ssz_restore_merkle_root(
        &mut self,
        leaf: Bytes32Variable,
        branch: &[Bytes32Variable],
        gindex: U64Variable,
    ) -> Bytes32Variable {
        let bits = self.to_le_bits(gindex);
        let mut hash = leaf;
        for i in 0..branch.len() {
            let left = branch[i].as_bytes();
            let right = hash.as_bytes();

            let mut data = [self.init_unsafe::<ByteVariable>(); 64];
            data[..32].copy_from_slice(&left);
            data[32..].copy_from_slice(&right);
            let case1 = self.curta_sha256(&data);

            data[..32].copy_from_slice(&right);
            data[32..].copy_from_slice(&left);
            let case2 = self.curta_sha256(&data);

            hash = self.select(bits[i], case1, case2);
        }
        hash
    }

    /// Computes the expected merkle root given a leaf, branch, and deterministic index.
    pub fn ssz_restore_merkle_root_const(
        &mut self,
        leaf: Bytes32Variable,
        branch: &[Bytes32Variable],
        gindex: u64,
    ) -> Bytes32Variable {
        let mut hash = leaf;
        for i in 0..branch.len() {
            let (first, second) = if (gindex >> i) & 1 == 1 {
                (branch[i].as_bytes(), hash.as_bytes())
            } else {
                (hash.as_bytes(), branch[i].as_bytes())
            };
            let mut data = [ByteVariable::init_unsafe(self); 64];
            data[..32].copy_from_slice(&first);
            data[32..].copy_from_slice(&second);
            hash = self.curta_sha256(&data);
        }
        hash
    }

    pub fn ssz_hash_tree_root<V: SSZVariable>(&mut self, variable: V) -> Bytes32Variable {
        variable.hash_tree_root(self)
    }

    pub fn ssz_hash_leafs(&mut self, leafs: &[Bytes32Variable]) -> Bytes32Variable {
        let mut leafs = leafs.to_vec();
        while leafs.len() != 1 {
            let mut tmp = Vec::new();
            for i in 0..leafs.len() / 2 {
                tmp.push(self.curta_sha256_pair(leafs[i * 2], leafs[i * 2 + 1]));
            }
            leafs = tmp;
        }
        leafs[0]
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::env;

    use log::debug;

    use crate::backend::circuit::DefaultParameters;
    use crate::frontend::builder::CircuitBuilder;
    use crate::frontend::eth::vars::BLSPubkeyVariable;
    use crate::frontend::uint::uint64::U64Variable;
    use crate::frontend::vars::Bytes32Variable;
    use crate::utils::eth::beacon::BeaconClient;
    use crate::utils::{bytes, bytes32};

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_beacon_get_partial_validators() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root().unwrap();

        let mut builder = CircuitBuilder::<L, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let validators = builder.beacon_get_partial_validators::<512>(block_root);
        builder.watch(&validators, "validators");

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_beacon_get_validators() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root().unwrap();

        let mut builder = CircuitBuilder::<L, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let validators = builder.beacon_get_validators(block_root);
        builder.watch(&validators, "validators");

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_beacon_get_validator() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root().unwrap();

        let mut builder = CircuitBuilder::<L, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let validators = builder.beacon_get_validators(block_root);
        let index = builder.constant::<U64Variable>(0);
        let validator = builder.beacon_get_validator(validators, index);
        let expected_validator_pubkey = builder.constant::<BLSPubkeyVariable>(bytes!(
            "0x933ad9491b62059dd065b560d256d8957a8c402cc6e8d8ee7290ae11e8f7329267a8811c397529dac52ae1342ba58c95"
        ));
        builder.assert_is_equal(validator.pubkey, expected_validator_pubkey);

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_beacon_get_validator_const() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root().unwrap();

        let mut builder = CircuitBuilder::<L, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let validators = builder.beacon_get_validators(block_root);
        let validator = builder.beacon_get_validator_const(validators, 0);
        let expected_validator_pubkey = builder.constant::<BLSPubkeyVariable>(bytes!(
            "0x933ad9491b62059dd065b560d256d8957a8c402cc6e8d8ee7290ae11e8f7329267a8811c397529dac52ae1342ba58c95"
        ));
        builder.assert_is_equal(validator.pubkey, expected_validator_pubkey);

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_beacon_get_validator_by_pubkey() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root().unwrap();

        let mut builder = CircuitBuilder::<L, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let pubkey = builder.constant::<BLSPubkeyVariable>(bytes!(
            "0x933ad9491b62059dd065b560d256d8957a8c402cc6e8d8ee7290ae11e8f7329267a8811c397529dac52ae1342ba58c95"
        ));
        let validators = builder.beacon_get_validators(block_root);
        let (_, validator) = builder.beacon_get_validator_by_pubkey(validators, pubkey);
        builder.assert_is_equal(validator.pubkey, pubkey);

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_beacon_get_validator_batch_witness() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root =
            "0x1bfb9d3eda9f16e2f50dedf079798ce218748d48024d8150a0299688bb528735";

        let mut builder = CircuitBuilder::<L, D>::new();
        builder.set_beacon_client(client);

        let zero = builder.constant::<U64Variable>(857088);
        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let validators = builder.beacon_get_validators(block_root);
        let validators = builder.beacon_witness_validator_batch::<512>(validators, zero);
        debug!("validators_len: {}", validators.len());

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_beacon_get_balances_root() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root().unwrap();

        let mut builder = CircuitBuilder::<L, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let balances = builder.beacon_get_balances(block_root);
        builder.watch(&balances, "balances");

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_beacon_get_partial_balances_root() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root().unwrap();

        let mut builder = CircuitBuilder::<L, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let balances = builder.beacon_get_partial_balances::<128>(block_root);
        builder.watch(&balances, "balances");

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_beacon_get_balance() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root().unwrap();

        let mut builder = CircuitBuilder::<L, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let balances = builder.beacon_get_balances(block_root);
        let index = builder.constant::<U64Variable>(7);
        let balance = builder.beacon_get_balance(balances, index);
        builder.watch(&balance, "balance");

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_beacon_get_withdrawals() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root().unwrap();

        let mut builder = CircuitBuilder::<L, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let withdrawals = builder.beacon_get_withdrawals(block_root);
        builder.watch(&withdrawals.withdrawals_root, "withdrawals_root");

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_beacon_get_withdrawal() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root().unwrap();

        let mut builder = CircuitBuilder::<L, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let withdrawals = builder.beacon_get_withdrawals(block_root);
        let idx = builder.constant::<U64Variable>(0);
        let withdrawal = builder.beacon_get_withdrawal(withdrawals, idx);
        builder.watch(&withdrawal, "withdrawal");

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_beacon_get_historical_block() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root().unwrap();
        let slot = client.get_finalized_slot().unwrap();
        let slot: u64 = slot.parse().unwrap();

        let mut builder = CircuitBuilder::<L, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let idx = builder.constant::<U64Variable>(slot - 100);
        let historical_block = builder.beacon_get_historical_block(block_root, slot, idx);
        builder.watch(&historical_block, "historical_block");

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_beacon_get_block_roots() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root().unwrap();

        let mut builder = CircuitBuilder::<L, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let block_roots = builder.beacon_get_block_roots(block_root);
        builder.watch(&block_roots, "block_roots");

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_beacon_get_graffiti() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root().unwrap();

        let mut builder = CircuitBuilder::<L, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let graffiti = builder.beacon_get_graffiti(block_root);
        builder.watch(&graffiti, "graffiti");

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_beacon_witness_headers_from_offset_range() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root().unwrap();

        let mut builder = CircuitBuilder::<L, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let start_offset = builder.constant::<U64Variable>(0);
        let end_offset = builder.constant::<U64Variable>(15);
        let block_roots = builder.beacon_witness_headers_from_offset_range::<16>(
            block_root,
            start_offset,
            end_offset,
        );
        for i in 0..block_roots.len() {
            builder.watch(&block_roots[i], "block_roots");
        }

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_ssz_restore_merkle_root_equal() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();

        let leaf = builder.constant::<Bytes32Variable>(bytes32!(
            "0xa1b2c3d4e5f60718291a2b3c4d5e6f708192a2b3c4d5e6f7a1b2c3d4e5f60718"
        ));
        let index = builder.constant::<U64Variable>(2);
        let branch = vec![
            builder.constant::<Bytes32Variable>(bytes32!(
                "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
            )),
            builder.constant::<Bytes32Variable>(bytes32!(
                "0xfedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321"
            )),
        ];
        let expected_root = builder.constant::<Bytes32Variable>(bytes32!(
            "0xac0757982d17231f28ac33c08f1dd7f420a60cec25bf517ac9e9b35d8543082f"
        ));

        let computed_root = builder.ssz_restore_merkle_root(leaf, &branch, index);
        builder.assert_is_equal(expected_root, computed_root);

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[should_panic]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_ssz_restore_merkle_root_unequal() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();

        let leaf = builder.constant::<Bytes32Variable>(bytes32!(
            "0xa1b2c3d4e5f60718291a2b3c4d5e6f708192a2b3c4d5e6f7a1b2c3d4e5f60718"
        ));
        let index = builder.constant::<U64Variable>(2);
        let branch = vec![
            builder.constant::<Bytes32Variable>(bytes32!(
                "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
            )),
            builder.constant::<Bytes32Variable>(bytes32!(
                "0xfedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321"
            )),
        ];
        let expected_root = builder.constant::<Bytes32Variable>(bytes32!(
            "0xbd0757982d17231f28ac33c08f1dd7f420a60cec25bf517ac9e9b35d8543082f"
        ));
        let computed_root = builder.ssz_restore_merkle_root(leaf, &branch, index);
        builder.assert_is_equal(expected_root, computed_root);

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_ssz_restore_merkle_root_const_equal() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();

        let leaf = builder.constant::<Bytes32Variable>(bytes32!(
            "0xa1b2c3d4e5f60718291a2b3c4d5e6f708192a2b3c4d5e6f7a1b2c3d4e5f60718"
        ));
        let index = 2;
        let branch = vec![
            builder.constant::<Bytes32Variable>(bytes32!(
                "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
            )),
            builder.constant::<Bytes32Variable>(bytes32!(
                "0xfedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321"
            )),
        ];
        let expected_root = builder.constant::<Bytes32Variable>(bytes32!(
            "0xac0757982d17231f28ac33c08f1dd7f420a60cec25bf517ac9e9b35d8543082f"
        ));
        let computed_root = builder.ssz_restore_merkle_root_const(leaf, &branch, index);
        builder.assert_is_equal(expected_root, computed_root);

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }

    #[test]
    #[should_panic]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_ssz_restore_merkle_root_const_unequal() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();

        let leaf = builder.constant::<Bytes32Variable>(bytes32!(
            "0xa1b2c3d4e5f60718291a2b3c4d5e6f708192a2b3c4d5e6f7a1b2c3d4e5f60718"
        ));
        let index = 2;
        let branch = vec![
            builder.constant::<Bytes32Variable>(bytes32!(
                "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
            )),
            builder.constant::<Bytes32Variable>(bytes32!(
                "0xfedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321"
            )),
        ];
        let expected_root = builder.constant::<Bytes32Variable>(bytes32!(
            "0xbd0757982d17231f28ac33c08f1dd7f420a60cec25bf517ac9e9b35d8543082f"
        ));
        let computed_root = builder.ssz_restore_merkle_root_const(leaf, &branch, index);
        builder.assert_is_equal(expected_root, computed_root);

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        circuit.test_default_serializers();
    }
}
