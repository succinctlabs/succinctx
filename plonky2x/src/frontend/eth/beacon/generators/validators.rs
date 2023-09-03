use core::marker::PhantomData;

use array_macro::array;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};
use tokio::runtime::Runtime;

use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable};
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::{bytes32, hex};

const DEPTH: usize = 8;

#[derive(Debug, Clone)]
pub struct BeaconValidatorsRootGenerator<F: RichField + Extendable<D>, const D: usize> {
    client: BeaconClient,
    block_root: Bytes32Variable,
    pub validators_root: Bytes32Variable,
    pub proof: [Bytes32Variable; DEPTH],
    _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> BeaconValidatorsRootGenerator<F, D> {
    pub fn new(
        builder: &mut CircuitBuilder<F, D>,
        client: BeaconClient,
        block_root: Bytes32Variable,
    ) -> Self {
        Self {
            client,
            block_root,
            validators_root: builder.init::<Bytes32Variable>(),
            proof: array![_ => builder.init::<Bytes32Variable>(); DEPTH],
            _phantom: Default::default(),
        }
    }
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for BeaconValidatorsRootGenerator<F, D>
{
    fn id(&self) -> String {
        "BeaconValidatorsGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        self.block_root.targets()
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let block_root = self.block_root.get(witness);

        let rt = Runtime::new().expect("failed to create tokio runtime");
        let result = rt.block_on(async {
            self.client
                .get_validators_root(hex!(block_root.as_bytes()).to_string())
                .await
                .expect("failed to get validators root")
        });

        self.validators_root
            .set(out_buffer, bytes32!(result.validators_root));
        for i in 0..DEPTH {
            self.proof[i].set(out_buffer, bytes32!(result.proof[i]));
        }
    }

    #[allow(unused_variables)]
    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        dst.write_target_vec(&self.block_root.targets())?;
        dst.write_target_vec(&self.validators_root.targets())?;
        for i in 0..DEPTH {
            dst.write_target_vec(&self.proof[i].targets())?;
        }
        Ok(())
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        let block_root = Bytes32Variable::from_targets(&src.read_target_vec()?);
        let validators_root = Bytes32Variable::from_targets(&src.read_target_vec()?);
        let mut proof = Vec::new();
        for i in 0..DEPTH {
            proof.push(Bytes32Variable::from_targets(&src.read_target_vec()?));
        }
        Ok(Self {
            client: BeaconClient::new("https://beaconapi.succinct.xyz".to_string()),
            block_root,
            validators_root,
            proof: proof.try_into().unwrap(),
            _phantom: Default::default(),
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
    use crate::frontend::eth::beacon::generators::validators::BeaconValidatorsRootGenerator;
    use crate::frontend::vars::Bytes32Variable;
    use crate::utils::bytes32;
    use crate::utils::eth::beacon::BeaconClient;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_get_validators_generator() {
        dotenv::dotenv().ok();

        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let consensus_rpc = env::var("CONSENSUS_RPC_URL").unwrap();
        let client = BeaconClient::new(consensus_rpc);

        let mut builder = CircuitBuilder::<F, D>::new();
        let block_root = builder.constant::<Bytes32Variable>(bytes32!(
            "0xe6d6e23b8e07e15b98811579e5f6c36a916b749fd7146d009196beeddc4a6670"
        ));
        let generator =
            BeaconValidatorsRootGenerator::<F, D>::new(&mut builder, client, block_root);
        builder.add_simple_generator(&generator);

        let circuit = builder.build::<C>();
        let pw = PartialWitness::new();
        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }
}
