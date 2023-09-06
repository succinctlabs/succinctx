use core::fmt::Debug;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoError, IoResult};
use serde::{Deserialize, Serialize};

use crate::frontend::vars::Variable;

pub trait Generator<F: RichField + Extendable<D>, const D: usize>:
    'static + Send + Sync + Debug + Serialize + for<'de> Deserialize<'de>
{
    fn id() -> String;

    fn dependencies(&self) -> Vec<Variable>;

    fn run(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorWrapper<G> {
    pub generator: G,
}

impl<F: RichField + Extendable<D>, const D: usize, G: Generator<F, D>> SimpleGenerator<F, D>
    for GeneratorWrapper<G>
{
    fn id(&self) -> String {
        G::id()
    }

    fn dependencies(&self) -> Vec<Target> {
        self.generator.dependencies().iter().map(|v| v.0).collect()
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        self.generator.run(witness, out_buffer);
    }

    fn serialize(&self, dst: &mut Vec<u8>, _common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        serde_json::to_writer(dst, self).map_err(|_| IoError)
    }

    fn deserialize(src: &mut Buffer, _common_data: &CommonCircuitData<F, D>) -> IoResult<Self>
    where
        Self: Sized,
    {
        serde_json::from_reader(src.bytes()).map_err(|_| IoError)
    }
}
