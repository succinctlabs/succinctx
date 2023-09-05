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
pub struct LeGenerator<F: RichField + Extendable<D>, const D: usize> {
    pub lhs: Variable,
    pub rhs: Variable,
    pub output: BoolVariable,
    pub _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for LeGenerator<F, D>
{
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
pub struct ByteToVariableGenerator<F: RichField + Extendable<D>, const D: usize> {
    pub byte: ByteVariable,
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
        targets.extend(self.byte.targets());
        targets
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let byte = self.byte.get(witness);
        self.output.set(out_buffer, F::from_canonical_u8(byte));
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