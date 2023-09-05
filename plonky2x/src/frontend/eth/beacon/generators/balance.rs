use core::marker::PhantomData;

use curta::math::prelude::PrimeField64;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};
use tokio::runtime::Runtime;

use crate::frontend::builder::CircuitBuilder;
use crate::frontend::uint::uint256::U256Variable;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable};
use crate::prelude::Variable;
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::hex;

#[derive(Debug, Clone)]
pub struct BeaconValidatorBalanceGenerator<F: RichField + Extendable<D>, const D: usize> {
    client: BeaconClient,
    block_root: Bytes32Variable,
    validators_root: Bytes32Variable,
    deterministic_idx: Option<u64>,
    dynamic_idx: Option<Variable>,
    pub balance: U256Variable,
    _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> BeaconValidatorBalanceGenerator<F, D> {
    pub fn new(
        builder: &mut CircuitBuilder<F, D>,
        block_root: Bytes32Variable,
        validators_root: Bytes32Variable,
        deterministic_idx: Option<u64>,
        dynamic_idx: Option<Variable>,
    ) -> Self {
        Self {
            client: builder.beacon_client.clone().unwrap(),
            block_root,
            validators_root,
            deterministic_idx,
            dynamic_idx,
            balance: builder.init::<U256Variable>(),
            _phantom: PhantomData,
        }
    }
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for BeaconValidatorBalanceGenerator<F, D>
{
    fn id(&self) -> String {
        "BeaconValidatorBalanceGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets = Vec::new();
        targets.extend(self.block_root.targets());
        targets.extend(self.validators_root.targets());
        if self.dynamic_idx.is_some() {
            targets.extend(self.dynamic_idx.unwrap().targets());
        }
        targets
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let block_root = self.block_root.get(witness);
        let rt = Runtime::new().expect("failed to create tokio runtime");
        let result = rt.block_on(async {
            if self.deterministic_idx.is_some() {
                self.client
                    .get_validator_balance(hex!(block_root), self.deterministic_idx.unwrap())
                    .await
                    .expect("failed to get validator")
            } else {
                let idx = self.dynamic_idx.unwrap().get(witness).as_canonical_u64();
                self.client
                    .get_validator_balance(hex!(block_root), idx)
                    .await
                    .expect("failed to get validator")
            }
        });
        self.balance.set(out_buffer, result);
    }

    #[allow(unused_variables)]
    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        dst.write_target_vec(&self.block_root.targets())?;
        dst.write_target_vec(&self.validators_root.targets())?;
        dst.write_bool(self.deterministic_idx.is_some())?;
        if self.deterministic_idx.is_some() {
            dst.write_usize(self.deterministic_idx.unwrap() as usize)?;
        } else {
            dst.write_target_vec(&self.dynamic_idx.unwrap().targets())?;
        }
        dst.write_target_vec(&self.balance.targets())?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        let block_root = Bytes32Variable::from_targets(&src.read_target_vec()?);
        let validators_root = Bytes32Variable::from_targets(&src.read_target_vec()?);
        let is_deterministic = src.read_bool()?;
        let deterministic_idx = if is_deterministic {
            Some(src.read_usize()? as u64)
        } else {
            None
        };
        let dynamic_idx = if is_deterministic {
            None
        } else {
            Some(Variable::from_targets(&src.read_target_vec()?))
        };
        let balance = U256Variable::from_targets(&src.read_target_vec()?);
        Ok(Self {
            client: BeaconClient::new("https://beaconapi.succinct.xyz".to_string()),
            block_root,
            validators_root,
            deterministic_idx,
            dynamic_idx,
            balance,
            _phantom: PhantomData,
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::env;

    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;

    use crate::frontend::builder::CircuitBuilder;
    use crate::frontend::eth::beacon::generators::validator::BeaconValidatorGenerator;
    use crate::frontend::vars::Bytes32Variable;
    use crate::utils::bytes32;
    use crate::utils::eth::beacon::BeaconClient;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_get_validator_generator() {
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
        let generator = BeaconValidatorGenerator::new(
            &mut builder,
            validators.block_root,
            validators.validators_root,
            Some(0),
            None,
        );
        builder.add_simple_generator(&generator);

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();
        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }
}
