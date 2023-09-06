use core::marker::PhantomData;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};

use crate::prelude::{
    ArrayVariable, BoolVariable, ByteVariable, CircuitVariable, Target, Variable,
};

#[derive(Debug, Clone)]
pub struct LeGenerator<F: RichField + Extendable<D>, const D: usize> {
    pub lhs: Variable,
    pub rhs: Variable,
    pub output: BoolVariable,
    pub _phantom: PhantomData<F>,
}

// TODO: add LeGenerator to macro
impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D> for LeGenerator<F, D> {
    fn id(&self) -> String {
        "LeGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets: Vec<Target> = Vec::new();
        targets.extend(self.lhs.targets());
        targets.extend(self.rhs.targets());
        targets
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let lhs = self.lhs.get(witness).to_canonical_u64() as usize;
        let rhs = self.rhs.get(witness).to_canonical_u64() as usize;
        self.output.set(out_buffer, lhs <= rhs);
    }

    #[allow(unused_variables)]
    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        dst.write_target_vec(&self.lhs.targets())?;
        dst.write_target_vec(&self.rhs.targets())?;
        dst.write_target_vec(&self.output.targets())?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        let lhs = src.read_target_vec()?;
        let rhs = src.read_target_vec()?;
        let output = src.read_target_vec()?;
        Ok(Self {
            lhs: Variable::from_targets(&lhs),
            rhs: Variable::from_targets(&rhs),
            output: BoolVariable::from_targets(&output),
            _phantom: PhantomData,
        })
    }
}

#[derive(Debug, Clone)]
pub struct MuxGenerator<
    F: RichField + Extendable<D>,
    const D: usize,
    V: CircuitVariable,
    const N: usize,
> {
    pub input: ArrayVariable<V, N>,
    pub select: Variable,
    pub output: V,
    pub _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize, V: CircuitVariable, const N: usize>
    SimpleGenerator<F, D> for MuxGenerator<F, D, V, N>
{
    fn id(&self) -> String {
        "MuxGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets: Vec<Target> = Vec::new();
        targets.extend(self.input.targets());
        targets.extend(self.select.targets());
        targets
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let selector = self.select.get(witness).to_canonical_u64() as usize;
        self.output
            .set(out_buffer, self.input[selector].get(witness));
    }

    #[allow(unused_variables)]
    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct NibbleGenerator<F: RichField + Extendable<D>, const D: usize> {
    pub input: Vec<ByteVariable>,
    pub output: Vec<ByteVariable>,
    pub _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D> for NibbleGenerator<F, D> {
    fn id(&self) -> String {
        "NibbleGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets: Vec<Target> = Vec::new();
        targets.extend(self.input.iter().flat_map(|x| x.targets()));
        targets
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        for (i, input) in self.input.iter().enumerate() {
            let value = input.get(witness);
            let low = value & 0xf;
            let high = (value >> 4) & 0xf;
            self.output[2 * i].set(out_buffer, high);
            self.output[2 * i + 1].set(out_buffer, low);
        }
    }

    #[allow(unused_variables)]
    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct SubarrayEqualGenerator<F: RichField + Extendable<D>, const D: usize> {
    pub a: Vec<ByteVariable>,
    pub a_offset: Variable,
    pub b: Vec<ByteVariable>,
    pub b_offset: Variable,
    pub len: Variable,
    pub _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for SubarrayEqualGenerator<F, D>
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
    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
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
    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct ByteSubGenerator<F: RichField + Extendable<D>, const D: usize> {
    pub lhs: ByteVariable,
    pub rhs: ByteVariable,
    pub output: ByteVariable,
    pub _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for ByteSubGenerator<F, D>
{
    fn id(&self) -> String {
        "ByteToVariableGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets: Vec<Target> = Vec::new();
        targets.extend(self.lhs.targets());
        targets.extend(self.rhs.targets());
        targets
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let lhs = self.lhs.get(witness);
        let rhs = self.rhs.get(witness);
        self.output.set(out_buffer, lhs - rhs);
    }

    #[allow(unused_variables)]
    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct ByteToVariableGenerator<F: RichField + Extendable<D>, const D: usize> {
    pub lhs: ByteVariable,
    pub output: Variable,
    pub _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for ByteToVariableGenerator<F, D>
{
    fn id(&self) -> String {
        "ByteToVariableGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets: Vec<Target> = Vec::new();
        targets.extend(self.lhs.targets());
        targets
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let lhs = self.lhs.get(witness);
        self.output.set(out_buffer, F::from_canonical_u8(lhs));
    }

    #[allow(unused_variables)]
    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        todo!()
    }
}
