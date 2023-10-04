use curta::math::field::Field;
use ethers::types::Bytes;
use log::info;
use num::bigint::ToBigInt;
use num::BigInt;

use super::generators::RLPDecodeListGenerator;
use crate::debug;
use crate::frontend::ops::math::{Add, LessThanOrEqual};
use crate::frontend::vars::CircuitVariable;
use crate::prelude::{
    ArrayVariable, BoolVariable, ByteVariable, BytesVariable, CircuitBuilder, PlonkParameters,
    Variable,
};

pub fn bool_to_u32(b: bool) -> u32 {
    if b {
        1
    } else {
        0
    }
}
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
        // Long string (more than 55 bytes length)
        let len_of_str_len = (prefix - 0xB7) as usize;
        let mut str_len_bytes: Vec<u8> = input[1..1 + len_of_str_len].to_vec();
        str_len_bytes.reverse();
        let mut str_len = 0;
        for i in 0..len_of_str_len {
            str_len += str_len_bytes[i] as usize * 256_usize.pow(i as u32);
        }
        return (
            input[1 + len_of_str_len..1 + len_of_str_len + str_len].into(),
            1 + len_of_str_len + str_len,
        );
    } else {
        info!("input {:?}", input);
        panic!("Invalid prefix rlp_decode_bytes")
    }
}

pub fn rlp_decode_list_2_or_17(input: &[u8]) -> Vec<Vec<u8>> {
    info!("input {:?}", Bytes::from(input.to_vec()).to_string());
    let prefix = input[0];

    // Short list (0-55 bytes total payload)
    if prefix <= 0xF7 {
        let list_length = (prefix - 0xC0) as usize;
        // We assert that the input is simply [list_length, list_content...] and not suffixed by anything else
        assert!(input.len() == 1 + list_length);
        let (ele_1, increment) = rlp_decode_bytes(&input[1..]);
        let (ele_2, _) = rlp_decode_bytes(&input[1 + increment..]);
        vec![ele_1, ele_2]
    } else {
        info!("hi in this case");
        // TODO: check that prefix is bounded within a certain range
        let len_of_list_length = prefix - 0xF7;
        // info!("len_of_list_length {:?}", len_of_list_length);
        // TODO: figure out what to do with len_of_list_length
        let mut pos = 1 + len_of_list_length as usize;
        let mut res = vec![];
        for _ in 0..17 {
            let (ele, increment) = rlp_decode_bytes(&input[pos..]);
            info!("ele {:?}", Bytes::from(ele.clone()).to_string());
            info!("increment {:?}", increment);
            res.push(ele);
            pos += increment;
            if pos == input.len() {
                break;
            }
        }
        assert!(pos == input.len()); // Checks that we have iterated through all the input
        assert!(res.len() == 17 || res.len() == 2);
        info!("END");
        res
    }
}

/// Given `encoded` which is a RLP-encoded list, passed in as a byte array of length `M`, with "true length" `len`
pub fn decode_element_as_list<
    const ENCODING_LEN: usize,
    const LIST_LEN: usize,
    const ELEMENT_LEN: usize,
>(
    encoded: &[u8],
    len: usize,
    finish: bool,
) -> (Vec<Vec<u8>>, Vec<usize>, usize) {
    assert_eq!(encoded.len(), ENCODING_LEN);
    assert!(len <= ENCODING_LEN); // len is the "true" length of "encoded", which is padded to length `ENCODING_LEN`
    assert!(LIST_LEN == 2 || LIST_LEN == 17); // Right now we only support decoding lists of length 2 or 17

    let mut decoded_list_as_fixed = vec![vec![0u8; ELEMENT_LEN]; LIST_LEN];
    let mut decoded_list_lens = vec![0usize; LIST_LEN];
    let decoded_list_len = 0;
    if finish {
        // terminate early
        return (decoded_list_as_fixed, decoded_list_lens, decoded_list_len);
    }
    let decoded_element = rlp_decode_list_2_or_17(&encoded[..len]);
    for (i, element) in decoded_element.iter().enumerate() {
        let len: usize = element.len();
        assert!(
            len <= ELEMENT_LEN,
            "The decoded element should have length <= {ELEMENT_LEN}!"
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
                * bool_to_u32(j as u32 <= list_len);
        }
        size_accumulator += 1 + list_len;
        claim_poly += poly;
    }

    let mut encoding_poly = BigInt::default();
    for i in 3..M {
        // TODO: don't hardcode 3 here
        let idx = i - 3;
        encoding_poly += encoding[i] as u32
            * (random.pow(idx as u32))
            * bool_to_u32(idx < size_accumulator as usize);
    }

    assert!(claim_poly == encoding_poly);
}

static mut now_debugging: i32 = 0;

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    fn parse_list_element<const ELEMENT_LEN: usize>(
        &mut self,
        element: ArrayVariable<ByteVariable, ELEMENT_LEN>,
        len: Variable,
    ) -> (ByteVariable, Variable) {
        let prefix = element[0];
        let prefix_var = prefix.to_variable(self);

        let zero_byte = self.constant::<ByteVariable>(0);
        let zero_var = zero_byte.to_variable(self);
        let one_byte = self.constant::<ByteVariable>(1);
        let one_var = one_byte.to_variable(self);
        let max_len_55 = self.constant::<Variable>(L::Field::from_canonical_u8(55));
        let _0x7f = self.constant::<ByteVariable>(0x7F);
        let _0x7f_var = _0x7f.to_variable(self);
        let _0x80 = self.constant::<ByteVariable>(0x80);
        let _0x80_var = _0x80.to_variable(self);

        let len_eq_0 = self.is_equal(len, zero_var);
        let len_eq_1 = self.is_equal(len, one_var);
        let prx_leq_n = prefix_var.lte(_0x7f_var, self);

        let _0x80_0 = BytesVariable::<2>([_0x80, zero_byte]);
        let prx_0 = BytesVariable::<2>([prefix, zero_byte]);
        let _len_0x80 = self.add(len, _0x80_var);
        let len_0x80 =
            BytesVariable::<2>([_len_0x80.to_byte_variable(self), len.to_byte_variable(self)]);

        let len_is_leq_55_pred = len.lte(max_len_55, self);
        self.assert_is_true(len_is_leq_55_pred);

        let len_eq_1_prx = self.and(len_eq_1, prx_leq_n);
        let greater_than_0x80 = self.select(len_eq_1_prx, prx_0, len_0x80);
        let res = self.select(len_eq_0, _0x80_0, greater_than_0x80);

        (res[0], res[1].to_variable(self))
    }

    fn verify_decoded_list<
        const ENCODING_LEN: usize,
        const LIST_LEN: usize,
        const ELEMENT_LEN: usize,
    >(
        &mut self,
        list: ArrayVariable<ArrayVariable<ByteVariable, ELEMENT_LEN>, LIST_LEN>,
        lens: ArrayVariable<Variable, LIST_LEN>,
        encoding: ArrayVariable<ByteVariable, ENCODING_LEN>,
    ) {
        unsafe {
            now_debugging += 1;
            if now_debugging == 0 {
                return;
            }
            // @TODO: Accumulate the length and check the prefix (first 2 or 3 bytes of the encoding).
            let zero = self.zero();
            let one = self.one();
            let two = self.constant::<Variable>(L::Field::from_canonical_u8(2));

            let const_0xf7 = self.constant::<Variable>(L::Field::from_canonical_u8(0xf7));
            let first_byte = encoding[0].to_variable(self);
            let second_byte = encoding[1].to_variable(self);

            debug::debug(self, "first_byte".to_string(), first_byte);
            debug::debug(self, "second_byte".to_string(), second_byte);

            let is_long_list = self.gte(first_byte, const_0xf7);
            let len_if_long_list = self.sub(first_byte, const_0xf7);
            let len_if_long_list = self.mul(is_long_list.0, len_if_long_list);

            debug::debug(self, "len_if_long_list".to_string(), len_if_long_list);

            // Constrain len_if_long_list to be 0, 1 or 2.
            let is_zero = self.is_equal(len_if_long_list, zero);
            let is_one = self.is_equal(len_if_long_list, one);
            let is_two = self.is_equal(len_if_long_list, two);
            let is_zero_one = self.or(is_zero, is_one);
            let is_zero_one_two = self.or(is_zero_one, is_two);
            let _true = self._true();
            self.assert_is_equal(is_zero_one_two, _true);

            let mut start_idx = self.add(len_if_long_list, one);

            let mut encoded_decoding: Vec<ByteVariable> = Vec::new();

            for i in 0..LIST_LEN {
                let (mut start_byte, list_len) = self.parse_list_element(list[i].clone(), lens[i]);
                let start_byte_var = start_byte.to_variable(self);
                encoded_decoding.push(start_byte);
                for j in 0..ELEMENT_LEN {
                    encoded_decoding.push(list[i][j]);
                }

                debug::debug(
                    self,
                    format!("start_byte[{}]", i).to_string(),
                    start_byte_var,
                );
                debug::debug(self, format!("lens[{}]", i).to_string(), lens[i]);
                debug::debug(self, format!("list_len[{}]", i).to_string(), list_len);

                // let const_0x80 = self.constant::<Variable>(L::Field::from_canonical_u8(8));
                //let has_prefix_byte = self.lt(first_byte, const_0x80);
                //let byte_is_not_its_own_encoding = self.not(has_prefix_byte);
                // let len_is_zero = self.is_zero(lens[i]);
                // let len_is_not_zero = self.not(len_is_zero);
                // let encoded_decoding_len = self.add(list_len, len_is_not_zero.0);
                // let encoded_decoding_len =
                //     self.mul(encoded_decoding_len, byte_is_not_its_own_encoding.0);

                let first_element = list[i][0].to_variable(self);
                let first_element_is_zero = self.is_zero(first_element);
                let len_is_zero = self.is_zero(list_len);
                let should_skip = self.and(first_element_is_zero, len_is_zero);
                let should_skip_multiplier = self.not(should_skip);

                let encoded_decoding_len = self.add(list_len, one);
                let encoded_decoding_len = self.mul(encoded_decoding_len, should_skip_multiplier.0);

                debug::debug(
                    self,
                    format!("encoded_decoding_len[{}]", i).to_string(),
                    encoded_decoding_len,
                );

                self.assert_subarray_equal(
                    &encoded_decoding[..],
                    zero,
                    encoding.as_slice(),
                    start_idx,
                    encoded_decoding_len,
                );
                start_idx = self.add(start_idx, encoded_decoding_len);
                encoded_decoding.clear();
            }
        }
    }

    pub fn decode_element_as_list<
        const ENCODING_LEN: usize,
        const LIST_LEN: usize,
        const ELEMENT_LEN: usize,
    >(
        &mut self,
        encoded: ArrayVariable<ByteVariable, ENCODING_LEN>,
        len: Variable,
        finish: BoolVariable,
    ) -> (
        ArrayVariable<ArrayVariable<ByteVariable, ELEMENT_LEN>, LIST_LEN>,
        ArrayVariable<Variable, LIST_LEN>,
        Variable,
    ) {
        let generator = RLPDecodeListGenerator::new(self, encoded.clone(), len, finish);
        self.add_simple_generator(generator.clone());
        self.verify_decoded_list::<ENCODING_LEN, LIST_LEN, ELEMENT_LEN>(
            generator.decoded_list.clone(),
            generator.decoded_element_lens.clone(),
            encoded.clone(),
        );
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
    use crate::backend::circuit::DefaultParameters;
    use crate::frontend::vars::CircuitVariable;
    use crate::prelude::{DefaultBuilder, GoldilocksField};
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
    fn test_rlp_decode_list_generator() {
        let mut builder: CircuitBuilder<DefaultParameters, 2> = DefaultBuilder::new();
        type F = GoldilocksField;

        const ENCODING_LEN: usize = 600;
        const LIST_LEN: usize = 17;
        const ELEMENT_LEN: usize = 34;
        let encoding = builder.read::<ArrayVariable<ByteVariable, ENCODING_LEN>>();
        let len = builder.read::<Variable>();
        let finish = builder.read::<BoolVariable>();

        let (decoded_list, decoded_element_lens, len_decoded_list) = builder
            .decode_element_as_list::<ENCODING_LEN, LIST_LEN, ELEMENT_LEN>(
                encoding.clone(),
                len,
                finish,
            );

        builder.watch(&len_decoded_list, "len_decoded_list");
        builder.watch(&decoded_element_lens, "decoded_element_lens");
        builder.watch(&decoded_list, "decoded_list");

        let circuit = builder.mock_build();

        let mut input = circuit.input();
        let rlp_encoding: Vec<u8>  = bytes!("0xf90211a0215ead887d4da139eba306f76d765f5c4bfb03f6118ac1eb05eec3a92e1b0076a03eb28e7b61c689fae945b279f873cfdddf4e66db0be0efead563ea08bc4a269fa03025e2cce6f9c1ff09c8da516d938199c809a7f94dcd61211974aebdb85a4e56a0188d1100731419827900267bf4e6ea6d428fa5a67656e021485d1f6c89e69be6a0b281bb20061318a515afbdd02954740f069ebc75e700fde24dfbdf8c76d57119a0d8d77d917f5b7577e7e644bbc7a933632271a8daadd06a8e7e322f12dd828217a00f301190681b368db4308d1d1aa1794f85df08d4f4f646ecc4967c58fd9bd77ba0206598a4356dd50c70cfb1f0285bdb1402b7d65b61c851c095a7535bec230d5aa000959956c2148c82c207272af1ae129403d42e8173aedf44a190e85ee5fef8c3a0c88307e92c80a76e057e82755d9d67934ae040a6ec402bc156ad58dbcd2bcbc4a0e40a8e323d0b0b19d37ab6a3d110de577307c6f8efed15097dfb5551955fc770a02da2c6b12eedab6030b55d4f7df2fb52dab0ef4db292cb9b9789fa170256a11fa0d00e11cde7531fb79a315b4d81ea656b3b452fe3fe7e50af48a1ac7bf4aa6343a066625c0eb2f6609471f20857b97598ae4dfc197666ff72fe47b94e4124900683a0ace3aa5d35ba3ebbdc0abde8add5896876b25261717c0a415c92642c7889ec66a03a4931a67ae8ebc1eca9ffa711c16599b86d5286504182618d9c2da7b83f5ef780");
        let mut encoding_fixed_size = [0u8; ENCODING_LEN];
        encoding_fixed_size[..rlp_encoding.len()].copy_from_slice(&rlp_encoding);
        input.write::<ArrayVariable<ByteVariable, ENCODING_LEN>>(encoding_fixed_size.to_vec());
        input.write::<Variable>(F::from_canonical_usize(rlp_encoding.len()));
        input.write::<BoolVariable>(false);

        let (witness, _output) = circuit.mock_prove(&input);

        let len = len_decoded_list.get(&witness);
        let decoded_element_lens = decoded_element_lens.get(&witness);
        // let decoded_list = decoded_list.get(&witness);
        assert!(len == F::from_canonical_usize(17));
        for i in 0..17 {
            if i == 16 {
                assert!(decoded_element_lens[i] == F::from_canonical_usize(0));
            } else {
                assert!(decoded_element_lens[i] == F::from_canonical_usize(32));
            }
        }
    }
}
