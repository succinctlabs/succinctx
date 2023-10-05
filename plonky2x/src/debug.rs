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
    pub string: Vec<String>,
    pub value: Vec<Variable>,
    _phantom: PhantomData<L>,
}

impl<L: PlonkParameters<D>, const D: usize> DebugGenerator<L, D> {
    pub fn new(
        _builder: &mut CircuitBuilder<L, D>,
        string: Vec<String>,
        value: Vec<Variable>,
    ) -> Self {
        Self {
            string,
            value,
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
        for v in &self.value {
            targets.extend(v.targets());
        }
        targets
    }

    #[allow(unused_variables)]
    fn run_once(
        &self,
        witness: &PartitionWitness<L::Field>,
        out_buffer: &mut GeneratedValues<L::Field>,
    ) {
        for i in 0..self.value.len() {
            println!(
                "{}: {}",
                self.string[i],
                self.value[i].get(witness).to_canonical_u64()
            );
        }
        println!();
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
    format: Vec<String>,
    variable: Vec<Variable>,
) {
    let generator: DebugGenerator<L, D> = DebugGenerator::new(builder, format, variable);
    builder.add_simple_generator(generator.clone());
}
