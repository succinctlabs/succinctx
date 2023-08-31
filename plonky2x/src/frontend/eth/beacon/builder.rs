
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use super::generators::validator::BeaconValidatorGenerator;
use super::vars::{BeaconValidatorVariable, BeaconValidatorsVariable};
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::beacon::generators::validators::BeaconValidatorsRootGenerator;
use crate::frontend::vars::{Bytes32Variable, ByteVariable};
use crate::frontend::vars::CircuitVariable;

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

    /// Computes the expected merkle root given a leaf, branch, and deterministic index. 
    pub fn ssz_restore_merkle_root(
        &mut self,
        leaf: Bytes32Variable,
        branch: Vec<Bytes32Variable>,
        index: u64,
    ) -> Bytes32Variable {
        assert!(2u64.pow(branch.len() as u32 + 1) > index);
        let mut hasher = leaf; 
        for i in 0..branch.len() { 
            let (first, second) = if (index >> i) & 1 == 1 { 
                (branch[i].as_bytes(), hasher.as_bytes())
            } else {
                (hasher.as_bytes(), branch[i].as_bytes())
            };
            let mut data = [ByteVariable::init(self); 64];
            data[..32].copy_from_slice(&first);
            data[32..].copy_from_slice(&second); 
            hasher = self.sha(&data);
        }
        hasher
    }
    

}
    
    

#[cfg(test)]
pub(crate) mod tests {
    use std::env;

    use itertools::Itertools;
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;

    use crate::frontend::builder::CircuitBuilder;
    use crate::frontend::vars::Bytes32Variable;
    use crate::utils::eth::beacon::BeaconClient;
    use crate::utils::{bytes32, setup_logger};

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
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

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();
        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
fn test_ssz_restore_merkle_root() {
    type F = GoldilocksField;
    type C = PoseidonGoldilocksConfig;
    const D: usize = 2;

    let mut builder = CircuitBuilder::<F, D>::new();

    // Example values
    let leaf = builder.constant::<Bytes32Variable>(bytes32!("0xa1b2c3d4e5f60718291a2b3c4d5e6f708192a2b3c4d5e6f7a1b2c3d4e5f60718"));
    let index = 2;
    let branch = vec![
        builder.constant::<Bytes32Variable>(bytes32!("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")),
        builder.constant::<Bytes32Variable>(bytes32!("0xfedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321")),
    ];

    let computed_root = builder.ssz_restore_merkle_root(leaf, branch, index);

    println!("Computed root: {:?}", computed_root);

    let circuit = builder.build::<C>();
    let pw = PartialWitness::new();
    let proof = circuit.data.prove(pw).unwrap();
    circuit.data.verify(proof).unwrap();
}

}
