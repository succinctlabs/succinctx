use core::marker::PhantomData;

use curta::math::field::PrimeField64;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::{RichField};
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};
use tokio::runtime::Runtime;
use ethers::utils::keccak256;
use ethers::types::H256;


use crate::builder::CircuitBuilder;
use crate::eth::beacon::vars::BeaconValidatorVariable;
use crate::ethutils::beacon::BeaconClient;
use crate::prelude::BoolVariable;
use crate::utils::hex;
use crate::vars::{Variable, ByteVariable, Bytes32Variable, CircuitVariable};


#[derive(Debug, Clone)]
pub struct MuxGenerator<F: RichField + Extendable<D>, const D: usize> {
    pub input: Vec<ByteVariable>,
    pub output: ByteVariable,
    pub selector: Variable,
    pub _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for MuxGenerator<F, D>
{
    fn id(&self) -> String {
        "MuxGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets: Vec<Target> = Vec::new();
        targets.extend(self.input.iter().map(|x| x.targets()).flatten().collect::<Vec<Target>>());
        targets.extend(self.selector.targets());
        targets
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let selector = self.selector.get(witness).to_canonical_u64() as usize;
        self.output.set(out_buffer, self.input[selector].get(witness));
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
            targets.extend(self.input[i].iter().map(|x| x.targets()).flatten().collect::<Vec<Target>>());
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

