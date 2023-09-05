use core::marker::PhantomData;

use curta::math::field::PrimeField64;
use ethers::types::H256;
use ethers::utils::keccak256;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};
use tokio::runtime::Runtime;

use crate::builder::CircuitBuilder;
use crate::eth::beacon::vars::BeaconValidatorVariable;
use crate::ethutils::beacon::BeaconClient;
use crate::prelude::BoolVariable;
use crate::utils::hex;
use crate::vars::{ByteVariable, Bytes32Variable, CircuitVariable, Variable};

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
        targets.extend(
            self.input
                .iter()
                .map(|x| x.targets())
                .flatten()
                .collect::<Vec<Target>>(),
        );
        targets
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        println!("In run_once of nibble generator");
        for i in 0..self.input.len() {
            let byte = self.input[i].get(witness);
            // println!("byte {:?}", byte);
            let nibble1 = (byte >> 4);
            let nibble2 = byte & 0x0f;
            self.output[i * 2].set(out_buffer, nibble1);
            self.output[i * 2 + 1].set(out_buffer, nibble2);
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
