use core::any::{Any, TypeId};
use core::fmt::Debug;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoError, IoResult, Write};
use serde::{Deserialize, Serialize};

use crate::frontend::vars::Variable;
use crate::prelude::CircuitBuilder;

pub trait Generator<F: RichField + Extendable<D>, const D: usize>:
    'static + Send + Clone + Sync + Debug + Any + Serialize + for<'de> Deserialize<'de>
{
    fn inputs(&self) -> Vec<Variable>;

    fn run(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>);
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn add_generator<G: Generator<F, D> + 'static>(&mut self, generator: G) {
        let generator = GeneratorWrapper { generator };
        self.add_simple_generator(generator);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorWrapper<G> {
    pub generator: G,
}

impl<F: RichField + Extendable<D>, const D: usize, G: Generator<F, D>> SimpleGenerator<F, D>
    for GeneratorWrapper<G>
{
    fn id(&self) -> String {
        format!(
            "Generator, name: {:?}, id: {:?}",
            core::any::type_name::<G>(),
            TypeId::of::<G>()
        )
        .to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        self.generator.inputs().iter().map(|v| v.0).collect()
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        self.generator.run(witness, out_buffer);
    }

    fn serialize(&self, dst: &mut Vec<u8>, _common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        let bytes = bincode::serialize(self).map_err(|_| IoError)?;
        dst.write_all(&bytes)
    }

    fn deserialize(src: &mut Buffer, _common_data: &CommonCircuitData<F, D>) -> IoResult<Self>
    where
        Self: Sized,
    {
        bincode::deserialize(src.bytes()).map_err(|_| IoError)
    }
}
