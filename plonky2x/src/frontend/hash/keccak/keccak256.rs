use core::marker::PhantomData;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::{RichField};
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};
use ethers::utils::keccak256;
use ethers::types::H256;


use crate::frontend::vars::{Variable, ByteVariable, Bytes32Variable, CircuitVariable};

#[derive(Debug, Clone)]
pub struct Keccack256Generator<F: RichField + Extendable<D>, const D: usize> {
    pub input: Vec<ByteVariable>,
    pub output: Bytes32Variable,
    pub length: Option<Variable>,
    pub _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for Keccack256Generator<F, D>
{
    fn id(&self) -> String {
        "Keccack256Generator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets: Vec<Target> = Vec::new();
        targets.extend(self.input.iter().flat_map(|x| x.targets()).collect::<Vec<Target>>());
        if let Some(length) = self.length {
            targets.extend(length.targets());
        }
        targets
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        println!("Running keccak256 generator");
        let mut length = self.input.len();
        if let Some(length_variable) = self.length {
            length = length_variable.get(witness).to_canonical_u64() as usize;
        }
        let input: Vec<u8> = self.input.iter().map(|x| x.get(witness)).collect();
        let result = keccak256(&input[..length]);
        self.output.set(out_buffer, H256::from_slice(&result[..]));
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