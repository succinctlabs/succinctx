use core::marker::PhantomData;

use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};

#[derive(Debug, Clone)]
pub struct LeGenerator<F: RichField + Extendable<D>, const D: usize> {
    pub lhs: Variable,
    pub rhs: Variable,
    pub output: BoolVariable,
    pub _phantom: PhantomData<F>,
}

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

// #[derive(Debug, Clone)]
// pub struct ByteSubGenerator<F: RichField + Extendable<D>, const D: usize> {
//     pub lhs: ByteVariable,
//     pub rhs: ByteVariable,
//     pub output: ByteVariable,
//     pub _phantom: PhantomData<F>,
// }

// impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
//     for ByteSubGenerator<F, D>
// {
//     fn id(&self) -> String {
//         "ByteToVariableGenerator".to_string()
//     }

//     fn dependencies(&self) -> Vec<Target> {
//         let mut targets: Vec<Target> = Vec::new();
//         targets.extend(self.lhs.targets());
//         targets.extend(self.rhs.targets());
//         targets
//     }

//     fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
//         let lhs = self.lhs.get(witness);
//         let rhs = self.rhs.get(witness);
//         self.output.set(out_buffer, lhs - rhs);
//     }

//     #[allow(unused_variables)]
//     fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
//         todo!()
//     }

//     #[allow(unused_variables)]
//     fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
//         todo!()
//     }
// }
