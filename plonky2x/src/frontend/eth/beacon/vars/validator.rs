use std::fmt::Debug;

use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::vars::BLSPubkeyVariable;
use crate::frontend::vars::{
    BoolVariable, Bytes32Variable, CircuitVariable, EvmVariable, SSZVariable, U256Variable,
};
use crate::prelude::{ByteVariable, Variable};
use crate::utils::eth::beacon::BeaconValidator;
use crate::utils::{bytes, bytes32, hex};

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

    fn init<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>) -> Self {
        Self {
            pubkey: BLSPubkeyVariable::init(builder),
            withdrawal_credentials: Bytes32Variable::init(builder),
            effective_balance: U256Variable::init(builder),
            slashed: BoolVariable::init(builder),
            activation_eligibility_epoch: U256Variable::init(builder),
            activation_epoch: U256Variable::init(builder),
            exit_epoch: U256Variable::init(builder),
            withdrawable_epoch: U256Variable::init(builder),
        }
    }

    #[allow(unused_variables)]
    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Self::ValueType<L::Field>,
    ) -> Self {
        Self {
            pubkey: BLSPubkeyVariable::constant(builder, bytes!(value.pubkey)),
            withdrawal_credentials: Bytes32Variable::constant(
                builder,
                bytes32!(value.withdrawal_credentials),
            ),
            effective_balance: U256Variable::constant(builder, value.effective_balance.into()),
            slashed: BoolVariable::constant(builder, value.slashed),
            activation_eligibility_epoch: U256Variable::constant(
                builder,
                value.activation_eligibility_epoch.into(),
            ),
            activation_epoch: U256Variable::constant(builder, value.activation_epoch.into()),
            exit_epoch: U256Variable::constant(
                builder,
                value.exit_epoch.parse::<u64>().unwrap().into(),
            ),
            withdrawable_epoch: U256Variable::constant(
                builder,
                value.withdrawable_epoch.parse::<u64>().unwrap().into(),
            ),
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

    fn from_variables(variables: &[Variable]) -> Self {
        let pubkey = BLSPubkeyVariable::from_variables(&variables[0..384]);
        let withdrawal_credentials = Bytes32Variable::from_variables(&variables[384..640]);
        let effective_balance = U256Variable::from_variables(&variables[640..648]);
        let slashed = BoolVariable::from_variables(&variables[648..649]);
        let activation_eligibility_epoch = U256Variable::from_variables(&variables[649..657]);
        let activation_epoch = U256Variable::from_variables(&variables[657..665]);
        let exit_epoch = U256Variable::from_variables(&variables[665..673]);
        let withdrawable_epoch = U256Variable::from_variables(&variables[673..681]);
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

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        BeaconValidator {
            pubkey: hex!(self.pubkey.get(witness)),
            withdrawal_credentials: hex!(self.withdrawal_credentials.get(witness)),
            effective_balance: self.effective_balance.get(witness).as_u64(),
            slashed: self.slashed.get(witness),
            activation_eligibility_epoch: self.activation_eligibility_epoch.get(witness).as_u64(),
            activation_epoch: self.activation_epoch.get(witness).as_u64(),
            exit_epoch: self.exit_epoch.get(witness).as_u64().to_string(),
            withdrawable_epoch: self.withdrawable_epoch.get(witness).as_u64().to_string(),
        }
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.pubkey.set(witness, bytes!(value.pubkey));
        self.withdrawal_credentials
            .set(witness, bytes32!(value.withdrawal_credentials));
        self.effective_balance
            .set(witness, value.effective_balance.into());
        self.slashed.set(witness, value.slashed);
        self.activation_eligibility_epoch
            .set(witness, value.activation_eligibility_epoch.into());
        self.activation_epoch
            .set(witness, value.activation_epoch.into());
        self.exit_epoch
            .set(witness, value.exit_epoch.parse::<u64>().unwrap().into());
        self.withdrawable_epoch.set(
            witness,
            value.withdrawable_epoch.parse::<u64>().unwrap().into(),
        );
    }
}

impl SSZVariable for BeaconValidatorVariable {
    fn hash_tree_root<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> Bytes32Variable {
        // Reference: https://www.ssz.dev/sszexplorer

        let zero = builder.constant::<ByteVariable>(0);
        let one = builder.constant::<ByteVariable>(1);

        let mut pubkey_serialized = self.pubkey.0 .0.to_vec();
        pubkey_serialized.extend([zero; 16]);

        let tmp = builder.sha256(&pubkey_serialized);
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

        let mut b1 = builder.sha256(&a1).0 .0.to_vec();
        b1.extend(builder.sha256(&a2).0 .0.to_vec());

        let mut b2 = builder.sha256(&a3).0 .0.to_vec();
        b2.extend(builder.sha256(&a4).0 .0.to_vec());

        let mut c1 = builder.sha256(&b1).0 .0.to_vec();
        c1.extend(builder.sha256(&b2).0 .0.to_vec());

        builder.sha256(&c1)
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
    fn test_validator_hash_tree_root_1() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();
        let validator = BeaconValidator {
            pubkey: "0x1d7d6a239c32e1a82c53f9f5506d0e2bb5e4be75b5046ecb5c685544c2346a2e659203c77f9896d0783dca8c2bc7345f".to_string(),
            withdrawal_credentials: "0xc2f56d5e99cd47e06d5a7a449ed9317c843ed5056982a15fac1972eb7b1b6048".to_string(),
            effective_balance: 5,
            slashed: true,
            activation_eligibility_epoch: 3,
            activation_epoch: 6,
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
        let input = circuit.inputs();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    #[test]
    fn test_validator_hash_tree_root_2() {
        env_logger::try_init().unwrap_or_default();
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();
        let validator = BeaconValidator {
            pubkey: "0x933ad9491b62059dd065b560d256d8957a8c402cc6e8d8ee7290ae11e8f7329267a8811c397529dac52ae1342ba58c95".to_string(),
            withdrawal_credentials: "0x0100000000000000000000000d369bb49efa5100fd3b86a9f828c55da04d2d50".to_string(),
            effective_balance: 32000000000,
            slashed: false,
            activation_eligibility_epoch: 0,
            activation_epoch: 0,
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
        let input = circuit.inputs();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }
}
