use array_macro::array;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::BoolTarget;


use super::generators::validator::BeaconValidatorGenerator;
use super::vars::{BeaconValidatorVariable, BeaconValidatorsVariable};
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::beacon::generators::validators::BeaconValidatorsRootGenerator;
use crate:: frontend::hash::sha::sha256;
use crate::frontend::vars::{Bytes32Variable, ByteVariable, BytesVariable, BoolVariable};
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

    /// SHA256 implementation for builder 
        pub fn sha(&mut self, input: &[ByteVariable]) -> Bytes32Variable {
            let input_bool: Vec<BoolTarget> = input.iter().flat_map(|byte| byte.0.iter().cloned().map(|bool_variable| BoolTarget::new_unsafe(bool_variable.0 .0))).collect();
            let hash_bool = sha256::sha256::<F,D>(&mut self.api, &input_bool);
            let hash_bytes_vec = hash_bool.chunks(8).map(|chunk| ByteVariable(array![i => BoolVariable::from(chunk[i].target); 8])).collect::<Vec<_>>();
            let mut hash_bytes_array = [ByteVariable::init(self); 32];
            hash_bytes_array.copy_from_slice(&hash_bytes_vec);
            Bytes32Variable(BytesVariable(hash_bytes_array))
        }
    
    /// SSZ Merkle Proof 
    pub fn ssz_restore_merkle_root(
        &mut self,
        leaf: Bytes32Variable,
        index: u64,
        depth: usize,
        branch: Vec<Bytes32Variable>,
    ) -> Bytes32Variable {
        let mut hasher = leaf; // Initialize the hasher with the leaf node
        for i in 0..depth { // Iterate over each level of the Merkle tree
            let (first, second) = if (index >> i) & 1 == 1 { 
                // Determine the order of hashing based on the index
                (branch[i].as_bytes(), hasher.as_bytes())
            } else {
                (hasher.as_bytes(), branch[i].as_bytes())
            };
            let data: [ByteVariable; 64] = first.iter()
                .chain(second.iter())
                .cloned()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(); // Combine the first and second slices into a single array
            hasher = self.sha(&data); // Compute the hash of the data
        }
        hasher // Return the computed Merkle root
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
}
