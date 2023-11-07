use core::marker::PhantomData;
use std::env;

use async_trait::async_trait;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};
use serde::{Deserialize, Serialize};

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::hint::asynchronous::hint::AsyncHint;
use crate::frontend::vars::{Bytes32Variable, CircuitVariable, ValueStream};
use crate::utils::eth::beacon::BeaconClient;
use crate::utils::{bytes32, hex};

pub(crate) const DEPTH: usize = 8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconValidatorsHint {}

impl BeaconValidatorsHint {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for BeaconValidatorsHint {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<L: PlonkParameters<D>, const D: usize> AsyncHint<L, D> for BeaconValidatorsHint {
    async fn hint(
        &self,
        input_stream: &mut ValueStream<L, D>,
        output_stream: &mut ValueStream<L, D>,
    ) {
        let block_root = input_stream.read_value::<Bytes32Variable>();

        let result = BeaconClient::new(env::var("CONSENSUS_RPC_1").unwrap())
            .get_validators_root(hex!(block_root.as_bytes()).to_string())
            .expect("failed to get validators root");

        // write root
        output_stream.write_value::<Bytes32Variable>(bytes32!(result.validators_root));

        // write proof
        for i in 0..DEPTH {
            output_stream.write_value::<Bytes32Variable>(bytes32!(result.proof[i]));
        }
    }
}

#[derive(Debug, Clone)]
pub struct BeaconValidatorsGenerator<L: PlonkParameters<D>, const D: usize> {
    block_root: Bytes32Variable,
    pub validators_root: Bytes32Variable,
    pub proof: Vec<Bytes32Variable>,
    _phantom: PhantomData<L>,
}

impl<L: PlonkParameters<D>, const D: usize> BeaconValidatorsGenerator<L, D> {
    pub fn new(builder: &mut CircuitBuilder<L, D>, block_root: Bytes32Variable) -> Self {
        Self {
            block_root,
            validators_root: builder.init::<Bytes32Variable>(),
            proof: (0..DEPTH)
                .map(|_| builder.init::<Bytes32Variable>())
                .collect::<Vec<_>>(),
            _phantom: Default::default(),
        }
    }

    pub fn id() -> String {
        "BeaconValidatorsGenerator".to_string()
    }
}

impl<L: PlonkParameters<D>, const D: usize> SimpleGenerator<L::Field, D>
    for BeaconValidatorsGenerator<L, D>
{
    fn id(&self) -> String {
        Self::id()
    }

    fn dependencies(&self) -> Vec<Target> {
        self.block_root.targets()
    }

    fn run_once(
        &self,
        witness: &PartitionWitness<L::Field>,
        out_buffer: &mut GeneratedValues<L::Field>,
    ) {
        let block_root = self.block_root.get(witness);

        let result = BeaconClient::new(env::var("CONSENSUS_RPC_1").unwrap())
            .get_validators_root(hex!(block_root.as_bytes()).to_string())
            .expect("failed to get validators root");

        self.validators_root
            .set(out_buffer, bytes32!(result.validators_root));
        for i in 0..DEPTH {
            self.proof[i].set(out_buffer, bytes32!(result.proof[i]));
        }
    }

    #[allow(unused_variables)]
    fn serialize(
        &self,
        dst: &mut Vec<u8>,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<()> {
        dst.write_target_vec(&self.block_root.targets())?;
        dst.write_target_vec(&self.validators_root.targets())?;
        for i in 0..DEPTH {
            dst.write_target_vec(&self.proof[i].targets())?;
        }
        Ok(())
    }

    #[allow(unused_variables)]
    fn deserialize(
        src: &mut Buffer,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<Self> {
        let block_root = Bytes32Variable::from_targets(&src.read_target_vec()?);
        let validators_root = Bytes32Variable::from_targets(&src.read_target_vec()?);
        let mut proof = Vec::new();
        for i in 0..DEPTH {
            proof.push(Bytes32Variable::from_targets(&src.read_target_vec()?));
        }
        Ok(Self {
            block_root,
            validators_root,
            proof,
            _phantom: Default::default(),
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use plonky2::iop::witness::PartialWitness;

    use crate::backend::circuit::DefaultParameters;
    use crate::frontend::builder::CircuitBuilder;
    use crate::frontend::eth::beacon::generators::validators::BeaconValidatorsGenerator;
    use crate::frontend::vars::Bytes32Variable;
    use crate::utils::bytes32;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_get_validators_generator() {
        dotenv::dotenv().ok();

        let mut builder = CircuitBuilder::<L, D>::new();
        let block_root = builder.constant::<Bytes32Variable>(bytes32!(
            "0xe6d6e23b8e07e15b98811579e5f6c36a916b749fd7146d009196beeddc4a6670"
        ));
        let generator = BeaconValidatorsGenerator::<L, D>::new(&mut builder, block_root);
        builder.add_simple_generator(generator);

        let circuit = builder.build();
        let pw = PartialWitness::new();
        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }
}
