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

use crate::builder::CircuitBuilder;
use crate::eth::beacon::vars::BeaconValidatorVariable;
use crate::ethutils::beacon::BeaconClient;
use crate::prelude::BoolVariable;
use crate::utils::hex;
use crate::vars::{Variable, ByteVariable, CircuitVariable};

use crate::eth::mpt::utils::rlp_decode_list_2_or_17;

#[derive(Debug, Clone)]
pub struct RLPDecodeListGenerator<F: RichField + Extendable<D>, const D: usize, const M: usize, const L: usize, const MAX_ELE_SIZE: usize> {
    encoding: [ByteVariable; M],
    length: Variable,
    finish: BoolVariable,
    decoded_list: Vec<Vec<ByteVariable>>,
    decoded_element_lens: Box<[Variable; L]>,
    decoded_list_len: Variable,
    _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize, const M: usize, const L: usize, const MAX_ELE_SIZE: usize> RLPDecodeListGenerator<F, D, M, L, MAX_ELE_SIZE> {
    pub fn new(
        encoding: [ByteVariable; M],
        length: Variable,
        finish: BoolVariable,
        decoded_list: Vec<Vec<ByteVariable>>,
        decoded_element_lens: Box<[Variable; L]>,
        decoded_list_len: Variable,
    ) -> Self {
        Self {
            encoding,
            length,
            finish,
            decoded_list,
            decoded_element_lens,
            decoded_list_len,
            _phantom: PhantomData,
        }
    }
}

impl<F: RichField + Extendable<D>, const D: usize, const M: usize, const L: usize, const MAX_ELE_SIZE: usize> SimpleGenerator<F, D>
    for RLPDecodeListGenerator<F, D, M, L, MAX_ELE_SIZE>
{
    fn id(&self) -> String {
        "RLPDecodeListGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets: Vec<Target> = Vec::new();
        targets.extend(self.encoding.iter().map(|x| x.targets()).flatten().collect::<Vec<Target>>());
        targets.extend(self.length.targets());
        targets.extend(self.finish.targets());
        targets
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let mut decoded_list_as_fixed = [[0u8; MAX_ELE_SIZE]; L];
        let mut decoded_list_lens = [0u8; L];
        let mut decoded_list_len = 0;

        let finish = self.finish.get(witness);
        if !finish {
            let encoding = self.encoding.iter().map(|x| x.get(witness)).collect::<Vec<_>>();
            let length = self.length.get(witness).as_canonical_u64() as usize;
            let decoded_element = rlp_decode_list_2_or_17(&encoding.as_slice()[..length]);

            for (i, element) in decoded_element.iter().enumerate() {
                let len: usize = element.len();
                assert!(len <= MAX_ELE_SIZE, "The decoded element should have length <= MAX_ELE_SIZE, has length {} and MAX_ELE_SIZE {}!", len, MAX_ELE_SIZE);
                decoded_list_as_fixed[i][..len].copy_from_slice(&element);
                decoded_list_lens[i] = len as u8;
            }
            decoded_list_len = decoded_element.len();
        }
        println!("Decoded list len: {}", decoded_list_len);
        println!("Decoded list: {:?}", decoded_list_as_fixed);
        self.decoded_list.iter().enumerate().for_each(|(i, x)| x.iter().enumerate().for_each(|(j, y)| y.set(out_buffer, decoded_list_as_fixed[i][j])));
        self.decoded_element_lens.iter().enumerate().for_each(|(i, x)| x.set(out_buffer, F::from_canonical_u8(decoded_list_lens[i])));
        self.decoded_list_len.set(out_buffer, F::from_canonical_usize(decoded_list_len));
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