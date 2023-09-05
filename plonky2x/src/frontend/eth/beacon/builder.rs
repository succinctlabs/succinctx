use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use super::generators::balance::BeaconBalanceGenerator;
use super::generators::balances::BeaconBalancesGenerator;
use super::generators::validator::BeaconValidatorGenerator;
use super::vars::{BeaconBalancesVariable, BeaconValidatorVariable, BeaconValidatorsVariable};
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::beacon::generators::validators::BeaconValidatorsGenerator;
use crate::frontend::eth::vars::BLSPubkeyVariable;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::{
    ByteVariable, Bytes32Variable, CircuitVariable, EvmVariable, SSZVariable,
};
use crate::prelude::BytesVariable;

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    /// Get the validators for a given block root.
    pub fn beacon_get_validators(
        &mut self,
        block_root: Bytes32Variable,
    ) -> BeaconValidatorsVariable {
        let generator =
            BeaconValidatorsGenerator::new(self, self.beacon_client.clone().unwrap(), block_root);
        self.add_simple_generator(&generator);
        let gindex = 363u64;
        self.ssz_verify_proof_const(
            block_root,
            generator.validators_root,
            &generator.proof,
            gindex,
        );
        BeaconValidatorsVariable {
            block_root,
            validators_root: generator.validators_root,
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
        self.add_simple_generator(&generator);
        let validator_root = self.ssz_hash_tree_root(generator.validator);
        let mut gindex = self.constant::<U64Variable>((1099511627776u64 * 2).into());
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
        self.add_simple_generator(&generator);
        let validator_root = self.ssz_hash_tree_root(generator.validator);
        let gindex = 1099511627776 * 2 + index;
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
        self.add_simple_generator(&generator);
        let validator_root = self.ssz_hash_tree_root(generator.validator);
        let mut gindex = self.constant::<U64Variable>((1099511627776u64 * 2).into());
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
        self.add_simple_generator(&generator);
        let gindex = 364u64;
        self.ssz_verify_proof_const(
            block_root,
            generator.balances_root,
            &generator.proof,
            gindex,
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
        self.add_simple_generator(&generator);
        let mut gindex = self.constant::<U64Variable>((549755813888u64).into());
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
        let bits = self.to_be_bits(index);
        let first_half: BytesVariable<16> =
            BytesVariable::<16>(generator.balance_leaf.0 .0[..16].try_into().unwrap());
        let second_half: BytesVariable<16> =
            BytesVariable::<16>(generator.balance_leaf.0 .0[16..].try_into().unwrap());
        let half = self.select(bits[0], second_half, first_half);
        let first_quarter: BytesVariable<8> = BytesVariable::<8>(half.0[..8].try_into().unwrap());
        let second_quarter: BytesVariable<8> = BytesVariable::<8>(half.0[8..].try_into().unwrap());
        let quarter = self.select(bits[1], second_quarter, first_quarter);

        let balance_bytes = generator.balance.encode(self);
        let quarter_bytes = quarter.0;
        for i in 0..8 {
            self.assert_is_equal(balance_bytes[7 - i], quarter_bytes[i]);
        }

        generator.balance
    }

    /// Verify a simple serialize (ssz) merkle proof with a dynamic index.
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

            let mut data = [self.init::<ByteVariable>(); 64];
            data[..32].copy_from_slice(&left);
            data[32..].copy_from_slice(&right);
            let case1 = self.sha256(&data);

            data[..32].copy_from_slice(&right);
            data[32..].copy_from_slice(&left);
            let case2 = self.sha256(&data);

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
            let mut data = [ByteVariable::init(self); 64];
            data[..32].copy_from_slice(&first);
            data[32..].copy_from_slice(&second);
            hash = self.sha256(&data);
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

    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;

    use crate::frontend::builder::CircuitBuilder;
    use crate::frontend::eth::vars::BLSPubkeyVariable;
    use crate::frontend::uint::uint64::U64Variable;
    use crate::frontend::vars::Bytes32Variable;
    use crate::utils::eth::beacon::BeaconClient;
    use crate::utils::{bytes, bytes32};

    type F = GoldilocksField;
    type C = PoseidonGoldilocksConfig;
    const D: usize = 2;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_get_validators() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root_sync().unwrap();

        let mut builder = CircuitBuilder::<F, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let validators = builder.beacon_get_validators(block_root);
        let expected_validators_root = builder.constant::<Bytes32Variable>(bytes32!(
            "0x117c8ce619123b5ded4bc150731335cacd41d5b291770cb35812e56db76f408c"
        ));
        builder.assert_is_equal(validators.validators_root, expected_validators_root);

        let circuit = builder.build::<C>();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_get_validator() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root_sync().unwrap();

        let mut builder = CircuitBuilder::<F, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let validators = builder.beacon_get_validators(block_root);
        let index = builder.constant::<U64Variable>(0.into());
        let validator = builder.beacon_get_validator(validators, index);
        let expected_validator_pubkey = builder.constant::<BLSPubkeyVariable>(bytes!(
            "0x933ad9491b62059dd065b560d256d8957a8c402cc6e8d8ee7290ae11e8f7329267a8811c397529dac52ae1342ba58c95"
        ));
        builder.assert_is_equal(validator.pubkey, expected_validator_pubkey);

        let circuit = builder.build::<C>();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_get_validator_const() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root_sync().unwrap();

        let mut builder = CircuitBuilder::<F, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let validators = builder.beacon_get_validators(block_root);
        let validator = builder.beacon_get_validator_const(validators, 0);
        let expected_validator_pubkey = builder.constant::<BLSPubkeyVariable>(bytes!(
            "0x933ad9491b62059dd065b560d256d8957a8c402cc6e8d8ee7290ae11e8f7329267a8811c397529dac52ae1342ba58c95"
        ));
        builder.assert_is_equal(validator.pubkey, expected_validator_pubkey);

        let circuit = builder.build::<C>();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_get_validator_by_pubkey() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root_sync().unwrap();

        let mut builder = CircuitBuilder::<F, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let pubkey = builder.constant::<BLSPubkeyVariable>(bytes!(
            "0x933ad9491b62059dd065b560d256d8957a8c402cc6e8d8ee7290ae11e8f7329267a8811c397529dac52ae1342ba58c95"
        ));
        let validators = builder.beacon_get_validators(block_root);
        let (_, validator) = builder.beacon_get_validator_by_pubkey(validators, pubkey);
        builder.assert_is_equal(validator.pubkey, pubkey);

        let circuit = builder.build::<C>();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_get_balances_root() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root_sync().unwrap();

        let mut builder = CircuitBuilder::<F, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let balances = builder.beacon_get_balances(block_root);
        builder.watch(&balances, "balances");

        let circuit = builder.build::<C>();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_get_balance() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let consensus_rpc = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(consensus_rpc);
        let latest_block_root = client.get_finalized_block_root_sync().unwrap();

        let mut builder = CircuitBuilder::<F, D>::new();
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(latest_block_root));
        let balances = builder.beacon_get_balances(block_root);
        let index = builder.constant::<U64Variable>(0.into());
        let balance = builder.beacon_get_balance(balances, index);
        builder.watch(&balance, "balance");

        let circuit = builder.build::<C>();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    #[test]
    fn test_ssz_restore_merkle_root_equal() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<F, D>::new();

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

        let circuit = builder.build::<C>();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    #[test]
    #[should_panic]
    fn test_ssz_restore_merkle_root_unequal() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<F, D>::new();

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

        let circuit = builder.build::<C>();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    #[test]
    fn test_ssz_restore_merkle_root_const_equal() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<F, D>::new();

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

        let circuit = builder.build::<C>();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    #[test]
    #[should_panic]
    fn test_ssz_restore_merkle_root_const_unequal() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<F, D>::new();

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

        let circuit = builder.build::<C>();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }
}
