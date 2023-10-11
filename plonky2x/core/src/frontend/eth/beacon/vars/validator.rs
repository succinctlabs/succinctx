use std::fmt::Debug;

use plonky2::hash::hash_types::RichField;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::vars::BLSPubkeyVariable;
use crate::frontend::vars::{
    BoolVariable, Bytes32Variable, CircuitVariable, EvmVariable, SSZVariable, U256Variable,
};
use crate::prelude::{ByteVariable, Variable};
use crate::utils::eth::beacon::BeaconValidator;
use crate::utils::{bytes, bytes32, hex};

const ZERO_BYTE32: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";
const ZERO_VALIDATOR_PUBKEY: &str = "0x111111000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

#[derive(Debug, Clone, Copy)]
pub struct BeaconValidatorVariable {
    pub pubkey: BLSPubkeyVariable,
    pub withdrawal_credentials: Bytes32Variable,
    pub effective_balance: U256Variable,
    pub slashed: BoolVariable,
    pub activation_eligibility_epoch: U256Variable,
    pub activation_epoch: U256Variable,
    pub exit_epoch: U256Variable,
    pub withdrawable_epoch: U256Variable,
}

impl CircuitVariable for BeaconValidatorVariable {
    type ValueType<F: RichField> = BeaconValidator;

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        Self {
            pubkey: BLSPubkeyVariable::init_unsafe(builder),
            withdrawal_credentials: Bytes32Variable::init_unsafe(builder),
            effective_balance: U256Variable::init_unsafe(builder),
            slashed: BoolVariable::init_unsafe(builder),
            activation_eligibility_epoch: U256Variable::init_unsafe(builder),
            activation_epoch: U256Variable::init_unsafe(builder),
            exit_epoch: U256Variable::init_unsafe(builder),
            withdrawable_epoch: U256Variable::init_unsafe(builder),
        }
    }

    fn nb_elements() -> usize {
        let pubkey = BLSPubkeyVariable::nb_elements();
        let withdrawal_credentials = Bytes32Variable::nb_elements();
        let effective_balance = U256Variable::nb_elements();
        let slashed = BoolVariable::nb_elements();
        let activation_eligibility_epoch = U256Variable::nb_elements();
        let activation_epoch = U256Variable::nb_elements();
        let exit_epoch = U256Variable::nb_elements();
        let withdrawable_epoch = U256Variable::nb_elements();
        pubkey
            + withdrawal_credentials
            + effective_balance
            + slashed
            + activation_eligibility_epoch
            + activation_epoch
            + exit_epoch
            + withdrawable_epoch
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        let pubkey = BLSPubkeyVariable::elements(bytes!(value.pubkey));
        let withdrawal_credentials =
            Bytes32Variable::elements(bytes32!(value.withdrawal_credentials));
        let effective_balance = U256Variable::elements(value.effective_balance.into());
        let slashed = BoolVariable::elements(value.slashed);
        let activation_eligibility_epoch = U256Variable::elements(
            value
                .activation_eligibility_epoch
                .parse::<u64>()
                .unwrap()
                .into(),
        );
        let activation_epoch =
            U256Variable::elements(value.activation_epoch.parse::<u64>().unwrap().into());
        let exit_epoch = U256Variable::elements(value.exit_epoch.parse::<u64>().unwrap().into());
        let withdrawable_epoch =
            U256Variable::elements(value.withdrawable_epoch.parse::<u64>().unwrap().into());
        pubkey
            .into_iter()
            .chain(withdrawal_credentials)
            .chain(effective_balance)
            .chain(slashed)
            .chain(activation_eligibility_epoch)
            .chain(activation_epoch)
            .chain(exit_epoch)
            .chain(withdrawable_epoch)
            .collect()
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        let pubkey = BLSPubkeyVariable::from_elements(&elements[0..384]);
        let withdrawal_credentials = Bytes32Variable::from_elements(&elements[384..640]);
        let effective_balance = U256Variable::from_elements(&elements[640..648]);
        let slashed = BoolVariable::from_elements(&elements[648..649]);
        let activation_eligibility_epoch = U256Variable::from_elements(&elements[649..657]);
        let activation_epoch = U256Variable::from_elements(&elements[657..665]);
        let exit_epoch = U256Variable::from_elements(&elements[665..673]);
        let withdrawable_epoch = U256Variable::from_elements(&elements[673..681]);
        BeaconValidator {
            pubkey: hex!(pubkey),
            withdrawal_credentials: hex!(withdrawal_credentials),
            effective_balance: effective_balance.as_u64(),
            slashed,
            activation_eligibility_epoch: activation_eligibility_epoch.as_u64().to_string(),
            activation_epoch: activation_epoch.as_u64().to_string(),
            exit_epoch: exit_epoch.as_u64().to_string(),
            withdrawable_epoch: withdrawable_epoch.as_u64().to_string(),
        }
    }

    fn variables(&self) -> Vec<Variable> {
        let mut vars = Vec::new();
        vars.extend(self.pubkey.variables());
        vars.extend(self.withdrawal_credentials.variables());
        vars.extend(self.effective_balance.variables());
        vars.extend(self.slashed.variables());
        vars.extend(self.activation_eligibility_epoch.variables());
        vars.extend(self.activation_epoch.variables());
        vars.extend(self.exit_epoch.variables());
        vars.extend(self.withdrawable_epoch.variables());
        vars
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        let pubkey = BLSPubkeyVariable::from_variables_unsafe(&variables[0..384]);
        let withdrawal_credentials = Bytes32Variable::from_variables_unsafe(&variables[384..640]);
        let effective_balance = U256Variable::from_variables_unsafe(&variables[640..648]);
        let slashed = BoolVariable::from_variables_unsafe(&variables[648..649]);
        let activation_eligibility_epoch =
            U256Variable::from_variables_unsafe(&variables[649..657]);
        let activation_epoch = U256Variable::from_variables_unsafe(&variables[657..665]);
        let exit_epoch = U256Variable::from_variables_unsafe(&variables[665..673]);
        let withdrawable_epoch = U256Variable::from_variables_unsafe(&variables[673..681]);
        Self {
            pubkey,
            withdrawal_credentials,
            effective_balance,
            slashed,
            activation_eligibility_epoch,
            activation_epoch,
            exit_epoch,
            withdrawable_epoch,
        }
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        self.pubkey.assert_is_valid(builder);
        self.withdrawal_credentials.assert_is_valid(builder);
        self.effective_balance.assert_is_valid(builder);
        self.slashed.assert_is_valid(builder);
        self.activation_eligibility_epoch.assert_is_valid(builder);
        self.activation_epoch.assert_is_valid(builder);
        self.exit_epoch.assert_is_valid(builder);
        self.withdrawable_epoch.assert_is_valid(builder);
    }
}

impl SSZVariable for BeaconValidatorVariable {
    fn hash_tree_root<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> Bytes32Variable {
        let zero = builder.constant::<ByteVariable>(0);
        let one = builder.constant::<ByteVariable>(1);

        let mut pubkey_serialized = self.pubkey.0 .0.to_vec();
        pubkey_serialized.extend([zero; 16]);

        let tmp = builder.curta_sha256(&pubkey_serialized);
        let mut a1 = tmp.0 .0.to_vec();
        a1.extend(self.withdrawal_credentials.0 .0.to_vec());

        let mut a2 = self.effective_balance.encode(builder);
        a2.reverse();
        let mut slashed = vec![builder.select(self.slashed, one, zero)];
        slashed.extend([zero; 31]);
        a2.extend(slashed);

        let mut a3 = self.activation_eligibility_epoch.encode(builder);
        a3.reverse();
        let mut tmp = self.activation_epoch.encode(builder);
        tmp.reverse();
        a3.extend(&tmp);

        let mut a4 = self.exit_epoch.encode(builder);
        a4.reverse();
        let mut tmp = self.withdrawable_epoch.encode(builder);
        tmp.reverse();
        a4.extend(&tmp);

        let mut b1 = builder.curta_sha256(&a1).0 .0.to_vec();
        b1.extend(builder.curta_sha256(&a2).0 .0.to_vec());

        let mut b2 = builder.curta_sha256(&a3).0 .0.to_vec();
        b2.extend(builder.curta_sha256(&a4).0 .0.to_vec());

        let mut c1 = builder.curta_sha256(&b1).0 .0.to_vec();
        c1.extend(builder.curta_sha256(&b2).0 .0.to_vec());

        let leaf = builder.curta_sha256(&c1);
        let zero_leaf = builder.constant::<Bytes32Variable>(bytes32!(ZERO_BYTE32));
        let zero_validator_pubkey =
            builder.constant::<BLSPubkeyVariable>(bytes!(ZERO_VALIDATOR_PUBKEY));
        let is_zero_validator = builder.is_equal(self.pubkey, zero_validator_pubkey);
        builder.select(is_zero_validator, zero_leaf, leaf)
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use crate::backend::circuit::DefaultParameters;
    use crate::frontend::builder::CircuitBuilder;
    use crate::frontend::eth::beacon::vars::BeaconValidatorVariable;
    use crate::frontend::vars::{Bytes32Variable, SSZVariable};
    use crate::utils::bytes32;
    use crate::utils::eth::beacon::BeaconValidator;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_validator_hash_tree_root_1() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();
        let validator = BeaconValidator {
            pubkey: "0x1d7d6a239c32e1a82c53f9f5506d0e2bb5e4be75b5046ecb5c685544c2346a2e659203c77f9896d0783dca8c2bc7345f".to_string(),
            withdrawal_credentials: "0xc2f56d5e99cd47e06d5a7a449ed9317c843ed5056982a15fac1972eb7b1b6048".to_string(),
            effective_balance: 5,
            slashed: true,
            activation_eligibility_epoch: "3".to_string(),
            activation_epoch: "6".to_string(),
            exit_epoch: "2".to_string(),
            withdrawable_epoch: "2".to_string(),
        };
        let v = builder.constant::<BeaconValidatorVariable>(validator);
        let hash = v.hash_tree_root(&mut builder);
        let expected_hash = builder.constant::<Bytes32Variable>(bytes32!(
            "0xaf7cc4e01fcb4a0620a1c842c6040a02275438e933dc9b3280b2ec7e0b7adc9f"
        ));
        builder.assert_is_equal(hash, expected_hash);

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_validator_hash_tree_root_2() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();
        let validator = BeaconValidator {
            pubkey: "0x933ad9491b62059dd065b560d256d8957a8c402cc6e8d8ee7290ae11e8f7329267a8811c397529dac52ae1342ba58c95".to_string(),
            withdrawal_credentials: "0x0100000000000000000000000d369bb49efa5100fd3b86a9f828c55da04d2d50".to_string(),
            effective_balance: 32000000000,
            slashed: false,
            activation_eligibility_epoch: "0".to_string(),
            activation_epoch: "0".to_string(),
            exit_epoch: "18446744073709551615".to_string(),
            withdrawable_epoch: "18446744073709551615".to_string(),
        };
        let v = builder.constant::<BeaconValidatorVariable>(validator);
        let hash = v.hash_tree_root(&mut builder);
        let expected_hash = builder.constant::<Bytes32Variable>(bytes32!(
            "0x2baf4065b5d6246410518c7981e5507ce82d46d87f8099df52c396c3b62b0fd5"
        ));
        builder.assert_is_equal(hash, expected_hash);

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }
}
