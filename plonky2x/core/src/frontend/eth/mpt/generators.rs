use core::marker::PhantomData;

use plonky2::field::types::PrimeField64;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};

use crate::prelude::{ByteVariable, CircuitVariable, PlonkParameters, Target, Variable};

#[derive(Debug, Clone)]
pub struct SubarrayEqualGenerator<L: PlonkParameters<D>, const D: usize> {
    pub a: Vec<ByteVariable>,
    pub a_offset: Variable,
    pub b: Vec<ByteVariable>,
    pub b_offset: Variable,
    pub len: Variable,
    pub _phantom: PhantomData<L>,
}

impl<L: PlonkParameters<D>, const D: usize> SimpleGenerator<L::Field, D>
    for SubarrayEqualGenerator<L, D>
{
    fn id(&self) -> String {
        "SubarrayEqualGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets: Vec<Target> = Vec::new();
        targets.extend(self.a.iter().flat_map(|x| x.targets()));
        targets.extend(self.b.iter().flat_map(|x| x.targets()));
        targets.extend(self.a_offset.targets());
        targets.extend(self.b_offset.targets());
        targets.extend(self.len.targets());
        targets
    }

    #[allow(unused_variables)]
    fn run_once(
        &self,
        witness: &PartitionWitness<L::Field>,
        out_buffer: &mut GeneratedValues<L::Field>,
    ) {
        let a_offset = self.a_offset.get(witness).to_canonical_u64() as usize;
        let b_offset = self.b_offset.get(witness).to_canonical_u64() as usize;
        let len = self.len.get(witness).to_canonical_u64() as usize;
        for i in 0..len {
            let a = self.a[a_offset + i].get(witness);
            let b = self.b[b_offset + i].get(witness);
            if a != b {
                panic!("SubarrayEqualGenerator failed at index {}", i);
            }
        }
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
