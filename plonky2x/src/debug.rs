use std::marker::PhantomData;

use plonky2::field::types::PrimeField64;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};

use crate::frontend::vars::{CircuitVariable, Variable};
use crate::prelude::{CircuitBuilder, PlonkParameters};

#[derive(Debug, Clone)]
pub struct DebugGenerator<L: PlonkParameters<D>, const D: usize> {
    format: String,
    variable: Variable,
    _phantom: PhantomData<L>,
}

impl<L: PlonkParameters<D>, const D: usize> DebugGenerator<L, D> {
    pub fn new(builder: &mut CircuitBuilder<L, D>, format: String, variable: Variable) -> Self {
        Self {
            format,
            variable,
            _phantom: PhantomData,
        }
    }
}

impl<L: PlonkParameters<D>, const D: usize> SimpleGenerator<L::Field, D> for DebugGenerator<L, D> {
    fn id(&self) -> String {
        "DebugGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets: Vec<Target> = Vec::new();
        targets.extend(self.variable.targets());
        targets
    }

    #[allow(unused_variables)]
    fn run_once(
        &self,
        witness: &PartitionWitness<L::Field>,
        out_buffer: &mut GeneratedValues<L::Field>,
    ) {
        let value = self.variable.get(witness).to_canonical_u64();
        println!("{} = {}", self.format, value);
    }

    #[allow(unused_variables)]
    fn serialize(
        &self,
        dst: &mut Vec<u8>,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    fn deserialize(
        src: &mut Buffer,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<Self> {
        todo!()
    }
}

pub fn debug<L: PlonkParameters<D>, const D: usize>(
    builder: &mut CircuitBuilder<L, D>,
    format: String,
    variable: Variable,
) {
    let generator = DebugGenerator::new(builder, format, variable);
    builder.add_simple_generator(generator.clone());
}
