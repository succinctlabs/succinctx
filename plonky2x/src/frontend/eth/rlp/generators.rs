use core::marker::PhantomData;

use curta::math::field::PrimeField64;
use plonky2::field::types::Field;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};

use super::builder::decode_element_as_list;
use crate::prelude::{
    ArrayVariable, BoolVariable, ByteVariable, CircuitBuilder, CircuitVariable, PlonkParameters,
    Variable,
};

#[derive(Debug, Clone)]
pub struct RLPDecodeListGenerator<
    L: PlonkParameters<D>,
    const D: usize,
    const ENCODING_LEN: usize,
    const LIST_LEN: usize,
    const ELEMENT_LEN: usize,
> {
    encoding: ArrayVariable<ByteVariable, ENCODING_LEN>,
    length: Variable,
    finish: BoolVariable,
    pub decoded_list: ArrayVariable<ArrayVariable<ByteVariable, ELEMENT_LEN>, LIST_LEN>,
    pub decoded_element_lens: ArrayVariable<Variable, LIST_LEN>,
    pub len_decoded_list: Variable,
    _phantom: PhantomData<L>,
}

impl<
        L: PlonkParameters<D>,
        const D: usize,
        const ENCODING_LEN: usize,
        const LIST_LEN: usize,
        const ELEMENT_LEN: usize,
    > RLPDecodeListGenerator<L, D, ENCODING_LEN, LIST_LEN, ELEMENT_LEN>
{
    pub fn new(
        builder: &mut CircuitBuilder<L, D>,
        encoding: ArrayVariable<ByteVariable, ENCODING_LEN>,
        length: Variable,
        finish: BoolVariable,
    ) -> Self {
        let decoded_list =
            builder.init::<ArrayVariable<ArrayVariable<ByteVariable, ELEMENT_LEN>, LIST_LEN>>();
        let decoded_element_lens = builder.init::<ArrayVariable<Variable, LIST_LEN>>();
        let len_decoded_list = builder.init::<Variable>();
        Self {
            encoding,
            length,
            finish,
            decoded_list,
            decoded_element_lens,
            len_decoded_list,
            _phantom: PhantomData,
        }
    }
}

impl<
        L: PlonkParameters<D>,
        const D: usize,
        const ENCODING_LEN: usize,
        const LIST_LEN: usize,
        const ELEMENT_LEN: usize,
    > SimpleGenerator<L::Field, D>
    for RLPDecodeListGenerator<L, D, ENCODING_LEN, LIST_LEN, ELEMENT_LEN>
{
    fn id(&self) -> String {
        "RLPDecodeListGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets: Vec<Target> = Vec::new();
        targets.extend(self.encoding.targets());
        targets.extend(self.length.targets());
        targets.extend(self.finish.targets());
        targets
    }

    fn run_once(
        &self,
        witness: &PartitionWitness<L::Field>,
        out_buffer: &mut GeneratedValues<L::Field>,
    ) {
        let finish = self.finish.get(witness);
        let encoding = self.encoding.get(witness);
        let length = self.length.get(witness).as_canonical_u64() as usize;
        let (decoded_list, decoded_list_lens, len_decoded_list) =
            decode_element_as_list::<ENCODING_LEN, LIST_LEN, ELEMENT_LEN>(
                &encoding, length, finish,
            );
        self.decoded_list.set(out_buffer, decoded_list);
        self.decoded_element_lens.set(
            out_buffer,
            decoded_list_lens
                .iter()
                .map(|x| L::Field::from_canonical_usize(*x))
                .collect(),
        );
        self.len_decoded_list
            .set(out_buffer, L::Field::from_canonical_usize(len_decoded_list));
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
