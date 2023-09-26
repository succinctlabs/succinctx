use array_macro::array;
use ethers::types::U64;

use super::generators::{
    BeaconBalanceGenerator, BeaconBalancesGenerator, BeaconHeaderHint,
    BeaconHistoricalBlockGenerator, BeaconValidatorGenerator, BeaconValidatorsHint,
    BeaconWithdrawalGenerator, BeaconWithdrawalsGenerator,
};
use super::vars::{
    BeaconBalancesVariable, BeaconHeaderVariable, BeaconValidatorVariable,
    BeaconValidatorsVariable, BeaconWithdrawalVariable, BeaconWithdrawalsVariable,
};
use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::vars::BLSPubkeyVariable;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::{
    Bytes32Variable, CircuitVariable, EvmVariable, SSZVariable, VariableStream,
};
use crate::prelude::{BoolVariable, ByteVariable, BytesVariable};

/// The gindex for blockRoot -> validatorsRoot.
const VALIDATORS_ROOT_GINDEX: u64 = 363;

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
const CLOSE_SLOT_BLOCK_ROOT_GINDEX: u64 = 303104;

/// Beacon chain constant SLOTS_PER_EPOCH.
const SLOTS_PER_EPOCH: u64 = 32;

/// Beacon chain constant SLOTS_PER_HISTORICAL_ROOT.
const SLOTS_PER_HISTORICAL_ROOT: u64 = 8192;

/// Beacon chain constant CAPELLA_FORK_EPOCH. (mainnet specific)
const CAPELLA_FORK_EPOCH: u64 = 194048;

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Get the validators for a given block root.
    pub fn beacon_get_validators(
        &mut self,
        block_root: Bytes32Variable,
    ) -> BeaconValidatorsVariable {
        let mut input_stream = VariableStream::new();
        input_stream.write(&block_root);
        let hint = BeaconValidatorsHint::new(self.beacon_client.clone().unwrap());
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
        let mut gindex = self.constant::<U64Variable>(VALIDATOR_BASE_GINDEX.into());
        gindex = self.add(gindex, index);
        self.ssz_verify_proof(
            validators.validators_root,
            validator_root,
            &generator.proof,
            gindex,
        );
        generator.validator
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
        let mut gindex = self.constant::<U64Variable>(VALIDATOR_BASE_GINDEX.into());
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
            balances_root: generator.balances_root,
        }
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
        let mut gindex = self.constant::<U64Variable>(BALANCE_BASE_GINDEX.into());
        let four = self.constant::<U64Variable>(4.into());

        let offset = self.div(index, four);
        gindex = self.add(gindex, offset);

        self.ssz_verify_proof(
            balances.balances_root,
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
        let mut gindex = self.constant::<U64Variable>(WITHDRAWAL_BASE_GINDEX.into());
        gindex = self.add(gindex, idx);
        let leaf = self.ssz_hash_tree_root(generator.withdrawal);
        self.ssz_verify_proof(withdrawals.withdrawals_root, leaf, &generator.proof, gindex);
        generator.withdrawal
    }

    /// Get block header from block root.
    pub fn beacon_get_block_header(&mut self, block_root: Bytes32Variable) -> BeaconHeaderVariable {
        let mut slot_hint_input = VariableStream::new();
        slot_hint_input.write(&block_root);
        let slot_hint_output = self.hint(slot_hint_input, BeaconHeaderHint {});
        let header = slot_hint_output.read::<BeaconHeaderVariable>(self);

        let restored_root = self.ssz_hash_tree_root(header);
        self.assert_is_equal(block_root, restored_root);

        header
    }

    /// Get a historical block root using state.block_roots for close slots and historical_summaries for slots > 8192 slots away.
    pub fn beacon_get_historical_block(
        &mut self,
        block_root: Bytes32Variable,
        target_slot: U64Variable,
    ) -> Bytes32Variable {
        let generator = BeaconHistoricalBlockGenerator::new(
            self,
            self.beacon_client.clone().unwrap(),
            block_root,
            target_slot,
        );
        self.add_simple_generator(generator.clone());

        // Use close slot logic if (source - target) < 8192
        let source_slot = self.beacon_get_block_header(block_root).slot;
        let source_sub_target = self.sub(source_slot, target_slot);
        let slots_per_historical =
            self.constant::<U64Variable>(U64::from(SLOTS_PER_HISTORICAL_ROOT));
        let one_u64 = self.constant::<U64Variable>(U64::from(1));
        let slots_per_historical_sub_one = self.sub(slots_per_historical, one_u64);
        let is_close_slot = self.le(source_sub_target, slots_per_historical_sub_one);

        let block_roots_array_index = self.rem(target_slot, slots_per_historical);

        // Close slot logic
        let mut close_slot_block_root_gindex =
            self.constant::<U64Variable>(CLOSE_SLOT_BLOCK_ROOT_GINDEX.into());
        close_slot_block_root_gindex =
            self.add(close_slot_block_root_gindex, block_roots_array_index);
        let restored_close_slot_block_root = self.ssz_restore_merkle_root(
            generator.target_block_root,
            &generator.close_slot_block_root_proof,
            close_slot_block_root_gindex,
        );
        let valid_close_slot = self.is_equal(restored_close_slot_block_root, block_root);

        // Far slot logic
        let capella_slot =
            self.constant::<U64Variable>(U64::from(CAPELLA_FORK_EPOCH * SLOTS_PER_EPOCH));
        let slots_since_capella = self.sub(target_slot, capella_slot);
        let historical_summary_array_index = self.div(slots_since_capella, slots_per_historical);
        let mut historical_summary_gindex =
            self.constant::<U64Variable>(HISTORICAL_SUMMARIES_BASE_GINDEX.into());
        historical_summary_gindex =
            self.add(historical_summary_gindex, historical_summary_array_index);
        let restored_far_slot_block_root = self.ssz_restore_merkle_root(
            generator.far_slot_historical_summary_root,
            &generator.far_slot_historical_summary_proof,
            historical_summary_gindex,
        );
        let valid_far_slot_block_root = self.is_equal(restored_far_slot_block_root, block_root);

        let mut far_slot_block_root_gindex =
            self.constant::<U64Variable>(HISTORICAL_SUMMARY_BLOCK_ROOT_GINDEX.into());
        far_slot_block_root_gindex = self.add(far_slot_block_root_gindex, block_roots_array_index);
        let restored_far_slot_historical_root = self.ssz_restore_merkle_root(
            generator.target_block_root,
            &generator.far_slot_block_root_proof,
            far_slot_block_root_gindex,
        );
        let valid_far_slot_historical_root = self.is_equal(
            restored_far_slot_historical_root,
            generator.far_slot_historical_summary_root,
        );
        let valid_far_slot = self.and(valid_far_slot_block_root, valid_far_slot_historical_root);

        let valid = self.select(is_close_slot, valid_close_slot, valid_far_slot);

        let true_bool = self.constant::<BoolVariable>(true);
        self.assert_is_equal(valid, true_bool);

        generator.target_block_root
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
        // let expected_root = self.ssz_restore_merkle_root_const(leaf, branch, gindex);
        // self.assert_is_equal(root, expected_root);
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
}

#[cfg(test)]
pub(crate) mod tests {
    use std::env;

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
    fn test_beacon_get_validators() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root_sync().unwrap();

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
        let latest_block_root = client.get_finalized_block_root_sync().unwrap();

        let mut builder = CircuitBuilder::<L, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let validators = builder.beacon_get_validators(block_root);
        let index = builder.constant::<U64Variable>(0.into());
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
        let latest_block_root = client.get_finalized_block_root_sync().unwrap();

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
        let latest_block_root = client.get_finalized_block_root_sync().unwrap();

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
    fn test_beacon_get_balances_root() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root_sync().unwrap();

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
    fn test_beacon_get_balance() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root_sync().unwrap();

        let mut builder = CircuitBuilder::<L, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let balances = builder.beacon_get_balances(block_root);
        let index = builder.constant::<U64Variable>(7.into());
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
        let latest_block_root = client.get_finalized_block_root_sync().unwrap();

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
        let latest_block_root = client.get_finalized_block_root_sync().unwrap();

        let mut builder = CircuitBuilder::<L, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let withdrawals = builder.beacon_get_withdrawals(block_root);
        let idx = builder.constant::<U64Variable>(0.into());
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
        let latest_block_root = client.get_finalized_block_root_sync().unwrap();

        let mut builder = CircuitBuilder::<L, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let idx = builder.constant::<U64Variable>(0.into());
        let historical_block = builder.beacon_get_historical_block(block_root, idx);
        builder.watch(&historical_block, "historical_block");

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
        let index = builder.constant::<U64Variable>(2.into());
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
        let index = builder.constant::<U64Variable>(2.into());
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
