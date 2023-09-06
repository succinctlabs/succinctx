use core::marker::PhantomData;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};

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
        todo!()
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        todo!()
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
pub struct NestedMuxGenerator<F: RichField + Extendable<D>, const D: usize, const N: usize> {
    pub input: Vec<[ByteVariable; N]>,
    pub output: [ByteVariable; N],
    pub selector: Variable,
    pub _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize, const N: usize> SimpleGenerator<F, D>
    for NestedMuxGenerator<F, D, N>
{
    fn id(&self) -> String {
        "MuxGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets: Vec<Target> = Vec::new();
        for i in 0..self.input.len() {
            targets.extend(
                self.input[i]
                    .iter()
                    .map(|x| x.targets())
                    .flatten()
                    .collect::<Vec<Target>>(),
            );
        }
        targets.extend(self.selector.targets());
        targets
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let selector = self.selector.get(witness).to_canonical_u64() as usize;
        for i in 0..N {
            self.output[i].set(out_buffer, self.input[selector][i].get(witness));
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
