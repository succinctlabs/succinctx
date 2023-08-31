use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use super::generators::balance::BeaconValidatorBalanceGenerator;
use super::generators::validator::BeaconValidatorGenerator;
use super::vars::{BeaconValidatorVariable, BeaconValidatorsVariable};
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::beacon::generators::validators::BeaconValidatorsRootGenerator;
use crate::frontend::uint::uint256::U256Variable;
use crate::frontend::vars::Bytes32Variable;
use crate::prelude::Variable;

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    /// Get the validators for a given block root.
    pub fn get_beacon_validators(
        &mut self,
        block_root: Bytes32Variable,
    ) -> BeaconValidatorsVariable {
        let generator = BeaconValidatorsRootGenerator::new(
            self,
            self.beacon_client.clone().unwrap(),
            block_root,
        );
        self.add_simple_generator(&generator);
        BeaconValidatorsVariable {
            block_root,
            validators_root: generator.validators_root,
        }
    }

    /// Get a beacon validator from a given dynamic index.
    pub fn get_beacon_validator(
        &mut self,
        validators: BeaconValidatorsVariable,
        index: Variable,
    ) -> BeaconValidatorVariable {
        let generator = BeaconValidatorGenerator::new(
            self,
            validators.block_root,
            validators.validators_root,
            None,
            Some(index),
        );
        self.add_simple_generator(&generator);
        generator.validator
    }

    /// Get a validator from a given deterministic index.
    pub fn get_beacon_validator_from_u64(
        &mut self,
        validators: BeaconValidatorsVariable,
        index: u64,
    ) -> BeaconValidatorVariable {
        let generator = BeaconValidatorGenerator::new(
            self,
            validators.block_root,
            validators.validators_root,
            Some(index),
            None,
        );
        self.add_simple_generator(&generator);
        generator.validator
    }

    /// Get a validator balance from a given deterministic index.
    pub fn get_beacon_validator_balance(
        &mut self,
        validators: BeaconValidatorsVariable,
        index: Variable,
    ) -> U256Variable {
        let generator = BeaconValidatorBalanceGenerator::new(
            self,
            validators.block_root,
            validators.validators_root,
            None,
            Some(index),
        );
        self.add_simple_generator(&generator);
        generator.balance
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::env;

    use curta::math::prelude::Field;
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;

    use crate::frontend::builder::CircuitBuilder;
    use crate::frontend::vars::Bytes32Variable;
    use crate::prelude::Variable;
    use crate::utils::bytes32;
    use crate::utils::eth::beacon::BeaconClient;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_get_validator_generator() {
        env_logger::init();
        dotenv::dotenv().ok();

        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let consensus_rpc = env::var("CONSENSUS_RPC_URL").unwrap();
        let client = BeaconClient::new(consensus_rpc);

        let mut builder = CircuitBuilder::<F, D>::new();

        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(
            "0xe6d6e23b8e07e15b98811579e5f6c36a916b749fd7146d009196beeddc4a6670"
        ));
        let validators = builder.get_beacon_validators(block_root);

        (0..1).for_each(|i| {
            let validator = builder.get_beacon_validator_from_u64(validators, i);
        });

        (0..1).for_each(|i| {
            let idx = builder.constant::<Variable>(F::from_canonical_u64(i));
            let validator = builder.get_beacon_validator(validators, idx);
            let balance = builder.get_beacon_validator_balance(validators, idx);
        });

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();
        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }
}
