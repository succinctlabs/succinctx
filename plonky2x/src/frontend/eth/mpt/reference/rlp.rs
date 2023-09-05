use std::marker::PhantomData;

use curta::math::prelude::PrimeField64;
use num::bigint::ToBigInt;
use num::BigInt;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};

use super::utils::{is_le, is_leq};
use crate::prelude::{
    ArrayVariable, BoolVariable, ByteVariable, CircuitBuilder, CircuitVariable, Variable,
};

// Note this only decodes bytes and doesn't support long strings
pub fn rlp_decode_bytes(input: &[u8]) -> (Vec<u8>, usize) {
    let prefix = input[0];
    if prefix <= 0x7F {
        (vec![prefix], 1)
    } else if prefix == 0x80 {
        (vec![], 1) // null value
    } else if prefix <= 0xB7 {
        // Short string (0-55 bytes length)
        let length = (prefix - 0x80) as usize;
        let res = &input[1..1 + length];
        (res.into(), 1 + length)
    } else if prefix <= 0xBF {
        panic!("Long string (56+ bytes length) not supported in rlp_decode_bytes")
    } else {
        panic!("Invalid prefix rlp_decode_bytes")
    }
}

pub fn rlp_decode_list_2_or_17(input: &[u8]) -> Vec<Vec<u8>> {
    let prefix = input[0];
    // println!("input {:?}", Bytes::from(input.to_vec()).to_string());
    if prefix <= 0xF7 {
        // Short list (0-55 bytes total payload)
        let list_length = (prefix - 0xC0) as usize;
        // We assert that the input is simply [list_length, list_content...] and not suffixed by anything else
        assert!(input.len() == 1 + list_length);
        let (ele_1, increment) = rlp_decode_bytes(&input[1..]);
        let (ele_2, _) = rlp_decode_bytes(&input[1 + increment..]);
        vec![ele_1, ele_2]
    } else {
        // TODO check that prefix is bounded within a certain range
        let len_of_list_length = prefix - 0xF7;
        // println!("len_of_list_length {:?}", len_of_list_length);
        // TODO: figure out what to do with len_of_list_length
        let mut pos = 1 + len_of_list_length as usize;
        let mut res = vec![];
        for _ in 0..17 {
            let (ele, increment) = rlp_decode_bytes(&input[pos..]);
            res.push(ele);
            pos += increment;
            if pos == input.len() {
                break;
            }
        }
        assert!(pos == input.len()); // Check that we have iterated through all the input
        assert!(res.len() == 17 || res.len() == 2);
        res
    }
}

/// Given `encoded` which is a RLP-encoded list, passed in as a byte array of length `M`, with "true length" `len`
pub fn decode_element_as_list<const M: usize, const L: usize, const MAX_ELE_SIZE: usize>(
    encoded: &[u8],
    len: usize,
    finish: bool,
) -> (Vec<Vec<u8>>, Vec<usize>, usize) {
    assert_eq!(encoded.len(), M);
    assert!(len <= M);
    assert!(L == 2 || L == 17); // Right now we only support decoding lists of length 2 or 17

    let mut decoded_list_as_fixed = vec![vec![0u8; MAX_ELE_SIZE]; L];
    let mut decoded_list_lens = vec![0usize; L];
    let decoded_list_len = 0;
    if finish {
        // terminate early
        return (decoded_list_as_fixed, decoded_list_lens, decoded_list_len);
    }
    let decoded_element = rlp_decode_list_2_or_17(&encoded[..len]);
    for (i, element) in decoded_element.iter().enumerate() {
        let len: usize = element.len();
        assert!(
            len <= MAX_ELE_SIZE,
            "The decoded element should have length <= {MAX_ELE_SIZE}!"
        );
        decoded_list_as_fixed[i][..len].copy_from_slice(element);
        decoded_list_lens[i] = len;
    }
    (
        decoded_list_as_fixed,
        decoded_list_lens,
        decoded_element.len(),
    )
}

fn parse_list_element(element: [u8; 32], len: u8) -> (u32, u32) {
    let prefix = element[0];
    if len == 0 {
        (0x80, 0)
    } else if len == 1 && prefix <= 0x7F {
        (prefix as u32, 0)
    } else if len == 1 && prefix > 0x7F {
        // TODO: maybe this is the same as the below case
        (0x80 + 0x01, 1)
    } else if len <= 55 {
        (len as u32 + 0x80, len as u32)
    } else {
        panic!("Invalid length and prefix combo {} {}", len, prefix)
    }
}

// This is the vanilla implementation of the RLC trick for verifying the decoded_list
pub fn verify_decoded_list<const L: usize, const M: usize>(
    list: [[u8; 32]; L],
    lens: [u8; L],
    encoding: [u8; M],
) {
    let random = 1000_i32.to_bigint().unwrap();

    let mut size_accumulator: u32 = 0;
    let mut claim_poly = BigInt::default();
    for i in 0..L {
        let (start_byte, list_len) = parse_list_element(list[i], lens[i]);
        let mut poly = start_byte.to_bigint().unwrap() * random.pow(size_accumulator);
        for j in 0..32 {
            poly += list[i][j] as u32
                * (random.pow(1 + size_accumulator + j as u32))
                * is_leq(j as u32, list_len);
        }
        size_accumulator += 1 + list_len;
        claim_poly += poly;
    }

    let mut encoding_poly = BigInt::default();
    for i in 3..M {
        // TODO: don't hardcode 3 here
        let idx = i - 3;
        encoding_poly +=
            encoding[i] as u32 * (random.pow(idx as u32)) * is_le(idx as u32, size_accumulator);
    }

    assert!(claim_poly == encoding_poly);
}

#[derive(Debug, Clone)]
pub struct RLPDecodeListGenerator<
    F: RichField + Extendable<D>,
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
    _phantom: PhantomData<F>,
}

impl<
        F: RichField + Extendable<D>,
        const D: usize,
        const ENCODING_LEN: usize,
        const LIST_LEN: usize,
        const ELEMENT_LEN: usize,
    > RLPDecodeListGenerator<F, D, ENCODING_LEN, LIST_LEN, ELEMENT_LEN>
{
    pub fn new(
        builder: &mut CircuitBuilder<F, D>,
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
        F: RichField + Extendable<D>,
        const D: usize,
        const ENCODING_LEN: usize,
        const LIST_LEN: usize,
        const ELEMENT_LEN: usize,
    > SimpleGenerator<F, D> for RLPDecodeListGenerator<F, D, ENCODING_LEN, LIST_LEN, ELEMENT_LEN>
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

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
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
                .map(|x| F::from_canonical_usize(*x))
                .collect(),
        );
        self.len_decoded_list
            .set(out_buffer, F::from_canonical_usize(len_decoded_list));
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

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    #[allow(unused_variables, dead_code)]
    fn decode_element_as_list<const ENCODED_LEN: usize, const LIST_LEN: usize, const M: usize>(
        &mut self,
        encoded: ArrayVariable<ByteVariable, ENCODED_LEN>,
        len: Variable,
        finish: BoolVariable,
    ) -> (
        ArrayVariable<ArrayVariable<ByteVariable, ENCODED_LEN>, LIST_LEN>,
        ArrayVariable<Variable, LIST_LEN>,
        Variable,
    ) {
        let generator = RLPDecodeListGenerator::new(self, encoded, len, finish);
        self.add_simple_generator(&generator);
        // TODO: here add verification logic constraints using `builder` to check that the decoded list is correct
        (
            generator.decoded_list,
            generator.decoded_element_lens,
            generator.len_decoded_list,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::bytes;

    #[test]
    fn test_decode_element_as_list() {
        const MAX_SIZE: usize = 17 * 32 + 20;
        let rlp_encoding: Vec<u8>  = bytes!("0xf90211a0c5becd7f8e5d47c1fe63ad9fa267d86fe0811bea0a4115aac7123b85fba2d662a03ab19202cb1de4f10fb0da8b5992c54af3dabb2312203f7477918df1393e24aea0b463eb71bcae8fa3183d0232b0d50e2400c21a0131bd48d918330e8683149b76a0d49a6c09224f74cef1286dad36a7f0e23e43e8ba4013fa386a3cda8903a3fe1ea06b6702bcfe04d3a135b786833b2748614d3aea00c728f86b2d1bbbb01b4e2311a08164a965258f9be5befcbf4de8e6cb4cd028689aad98e36ffc612b7255e4fa30a0b90309c6cb6383b2cb4cfeef9511004b705f1bca2c0556aadc2a5fe7ddf665e7a0749c3cee27e5ce85715122b76c18b7b945f1a19f507d5142445b42d50b2dd65aa0dbe35c115e9013b339743ebc2d9940158fb63b9e39f248b15ab74fade183c556a0a2b202f9b8003d73c7c84c8f7eb03298c064842382e57cecac1dfc2d5cabe2ffa02c5f8eba535bf5f18ca5aec74b51e46f219150886618c0301069dfb947006810a0dc01263a3b7c7942b5f0ac23931e0fda54fabaa3e6a58d2aca7ec65957cf8131a07d47344efa308df47f7e0e10491fa22d0564dbce634397c7748cd325fadd6b90a0cf9e45e08b8d60c68a86359adfa31c82883bb4a75b1d854392deb1f4499ba113a0081a664033eb00d5a69fc60f1f8b30e41eb643c5b9772d47301b602902b8d184a058b0bcf02a206cfa7b5f275ef09c97b4ae56abd8e9072f89dad8df1b98dfaa0280");
        let mut encoding_fixed_size = [0u8; MAX_SIZE];
        encoding_fixed_size[..rlp_encoding.len()].copy_from_slice(&rlp_encoding);

        let decoded_list = rlp_decode_list_2_or_17(&rlp_encoding);
        assert!(decoded_list.len() == 17);
        let element_lengths = decoded_list
            .iter()
            .map(|item| item.len() as u8)
            .collect::<Vec<u8>>();

        let mut decoded_list_fixed_size = [[0u8; 32]; 17];
        let mut element_lengths_fixed_size = [0u8; 17];
        for (i, item) in decoded_list.iter().enumerate() {
            let len = item.len();
            assert!(len <= 32, "The nested vector is longer than 32 bytes!");
            decoded_list_fixed_size[i][..len].copy_from_slice(item);
            element_lengths_fixed_size[i] = element_lengths[i] as u8;
        }

        // TODO: move below to a different test
        verify_decoded_list::<17, MAX_SIZE>(
            decoded_list_fixed_size,
            element_lengths_fixed_size,
            encoding_fixed_size,
        );
    }

    #[test]
    fn test_rlp_decode_generator() {}
}
