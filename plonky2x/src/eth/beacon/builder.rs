use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use super::generators::validator::BeaconValidatorGenerator;
use super::vars::{BeaconValidatorVariable, BeaconValidatorsVariable};
use crate::builder::CircuitBuilder;
use crate::eth::beacon::generators::validators::BeaconValidatorsRootGenerator;
use crate::vars::Bytes32Variable;

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

    /// Get a validator from a given index.
    pub fn get_beacon_validator_from_u64(
        &mut self,
        validators: BeaconValidatorsVariable,
        index: u64,
    ) -> BeaconValidatorVariable {
        let generator = BeaconValidatorGenerator::new(
            self,
            validators.block_root,
            validators.validators_root,
            index,
        );
        self.add_simple_generator(&generator);
        generator.validator
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::env;

    use itertools::Itertools;
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;

    use crate::builder::CircuitBuilder;
    use crate::ethutils::beacon::BeaconClient;
    use crate::utils::{bytes32, setup_logger};
    use crate::vars::Bytes32Variable;

    #[test]
    fn test_get_validator_generator() {
        setup_logger();
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

        let balances = (0..4)
            .map(|i| {
                let validator = builder.get_beacon_validator_from_u64(validators, i);
                validator.effective_balance
            })
            .collect_vec();
        println!("balances: {:?}", balances);

        let data = builder.build::<C>();
        let pw = PartialWitness::new();
        let proof = data.prove(pw).unwrap();
        data.verify(proof).unwrap();
    }
}
