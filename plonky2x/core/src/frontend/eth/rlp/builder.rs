use curta::math::field::Field;
use curta::math::prelude::PrimeField64;
use ethers::types::Bytes;
use log::info;
use num::bigint::ToBigInt;
use num::BigInt;
use serde::{Deserialize, Serialize};

use crate::frontend::hint::simple::hint::Hint;
use crate::prelude::{
    ArrayVariable, BoolVariable, ByteVariable, CircuitBuilder, PlonkParameters, ValueStream,
    Variable, VariableStream,
};

/// Byte array of at most 32 bytes, potentially padded. This could be a hash, value,
/// rlp.encode(branch node), rlp.encode(leaf node), or rlp.encode(extension node) under the hood.
const MAX_STRING_SIZE: usize = 32;
type FixedSizeString = [u8; MAX_STRING_SIZE];

/// This represents a node in a MPT, potentially padded. This could be a null, branch, leaf, or
/// extension node. Note that a nested list (when a small node is referenced inside node), if any,
/// isn't decoded.
///
/// Why we can use FixdSizeString for the type of each element in FixedSizeMPTNode:
/// 1. For a branch node, each node is referenced by a 32-byte hash, or the node itself. We use the
///    node if and only if the len(node) < 32.
/// 2. For a leaf node, the encodedPath is an an array of up to 32 bytes because a path in Ethereum
///    is exactly 64 hex character long. Furthermore, the value is guaranteed to be up to 32 bytes.
/// 3. For an extension node, the encodedPath is the same as the leaf node. The key is keccak256(x),
///    which is exactly 32 bytes.
const MAX_NODE_SIZE: usize = 17;
type FixedSizeMPTNode = [FixedSizeString; MAX_NODE_SIZE];

/// This represents the length of each string in a node, potentially padded Since each
/// FixedSizeString in FixedSizeString is padded, we need to keep track of the "true" length of each
/// string.
type FixedSizeStringLengths = [usize; MAX_NODE_SIZE];

pub fn bool_to_u32(b: bool) -> u32 {
    if b {
        1
    } else {
        0
    }
}

/// This decodes the next byte string contained in the input. It also returns the number of bytes we
/// processed. This is useful for decoding the next string.
pub fn rlp_decode_next_string(input: &[u8]) -> (Vec<u8>, usize) {
    if input.is_empty() {
        panic!("input cannot be empty")
    }
    let prefix = input[0];
    if prefix <= 0x7F {
        // The prefix indicateas that the byte is its own RLP encoding.
        (vec![prefix], 1)
    } else if prefix == 0x80 {
        // This is the null value. In other words, the empty string.
        (vec![], 1)
    } else if prefix <= 0xB7 {
        // Prefix indicates short string containing up to 55 bytes.
        let length = (prefix - 0x80) as usize;
        let res = &input[1..1 + length];
        (res.into(), 1 + length)
    } else if prefix <= 0xBF {
        // Prefix indicates long string containing more than 55 bytes.
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
        // TODO: In some cases, a MPT node may be a nested list. So, this is not necessarily an
        // error. "When one node is referenced inside another node, what is included is
        // H(rlp.encode(x)), where H(x) = keccak256(x) if len(x) >= 32 else x and rlp.encode is the
        // RLP encoding function."
        info!("input {:?}", input);
        panic!("Prefix indicates this is a list, but we expect a string")
    }
}

pub fn rlp_decode_mpt_node(input: &[u8]) -> Vec<Vec<u8>> {
    info!("input {:?}", Bytes::from(input.to_vec()).to_string());
    let prefix = input[0];

    if prefix < 0xC0 {
        panic!("Invalid prefix, MPT node must be a list")
    } else if prefix <= 0xF7 {
        // Short list (0-55 bytes total payload)
        let list_length = (prefix - 0xC0) as usize;
        // We assert that the input is simply [list_length, list_content...] and not suffixed by anything else
        assert!(input.len() == 1 + list_length);
        let (ele_1, increment) = rlp_decode_next_string(&input[1..]);
        let (ele_2, _) = rlp_decode_next_string(&input[1 + increment..]);
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
            let (decoded_string, num_bytes_processed) = rlp_decode_next_string(&input[pos..]);
            info!(
                "decoded_string {:?}",
                Bytes::from(decoded_string.clone()).to_string()
            );
            info!("{:?} bytes processed", num_bytes_processed);
            res.push(decoded_string);
            pos += num_bytes_processed;
            if pos >= input.len() {
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
/// This decodes a padded, RLP encoded MPT node.
///
/// The input is a tuple of:
/// - encoded: padded, RLP-encoded MPT node,
/// - len: "true length" of `encoded`,
/// - finish: a boolean indicating whether we should terminate early.
///
/// The output is a tuple of:
/// - Padded decoded node,
/// - Lengths of each string in the decoded node,
/// - Length of the decoded node.
pub fn decode_padded_mpt_node<const ENCODING_LEN: usize, const LIST_LEN: usize>(
    encoded: &[u8],
    len: usize,
    finish: bool,
) -> (Vec<Vec<u8>>, Vec<usize>, usize) {
    assert_eq!(encoded.len(), ENCODING_LEN);
    assert!(len <= ENCODING_LEN); // len is the "true" length of "encoded", which is padded to length `ENCODING_LEN`
    assert!(LIST_LEN == 2 || LIST_LEN == 17); // Right now we only support decoding lists of length 2 or 17

    let mut decoded_node_fixed_size = vec![vec![0u8; MAX_STRING_SIZE]; LIST_LEN];
    let mut decoded_node_lens = vec![0usize; LIST_LEN];
    let decoded_list_len = 0;
    if finish {
        // terminate early
        return (decoded_node_fixed_size, decoded_node_lens, decoded_list_len);
    }
    let decoded_node = rlp_decode_mpt_node(&encoded[..len]);
    for (i, string) in decoded_node.iter().enumerate() {
        let len: usize = string.len();
        assert!(
            len <= MAX_STRING_SIZE,
            "The decoded string should have length <= {MAX_STRING_SIZE}!"
        );
        decoded_node_fixed_size[i][..len].copy_from_slice(string);
        decoded_node_lens[i] = len;
    }
    (
        decoded_node_fixed_size,
        decoded_node_lens,
        decoded_node.len(),
    )
}

/// This calculates the prefix and the length of the encoding if we were to encode the given string.
/// More specifically, the first return value is the prefix of rlp_encode(padded_string[..len]). The
/// second return value is rlp_encode(padded_string[..len]).len().
fn calculate_rlp_encode_metadata(padded_string: FixedSizeString, len: usize) -> (u32, u32) {
    if len == 0 {
        // While it may be counterintutive, rlp_encode(the empty string) = 0x80.
        (0x80, 1)
    } else if len == 1 {
        if padded_string[0] < 0x80 {
            // A single byte less than 0x80 is its own RLP encoding.
            (padded_string[0] as u32, 1)
        } else {
            // A single byte greater than 0x80 is encoded as 0x81 + the byte.
            (0x81, 2)
        }
    } else if len <= 55 {
        // A string of length <= 55 is encoded as (0x80 + length of the string) followed by the
        // string.
        (len as u32 + 0x80, len as u32 + 1)
    } else {
        panic!("Invalid length {}", len)
    }
}

// This is the vanilla implementation of the RLC trick for verifying the decoded_list
pub fn verify_decoded_list<const M: usize>(
    node: FixedSizeMPTNode,
    lens: FixedSizeStringLengths,
    node_len: usize,
    encoding: [u8; M],
    encoding_len: usize,
) {
    let random = 1000_i32.to_bigint().unwrap();

    let mut size_accumulator: u32 = 0;
    let mut claim_poly = BigInt::default();
    for i in 0..MAX_NODE_SIZE {
        let (prefix_byte, rlp_encoding_length) = calculate_rlp_encode_metadata(node[i], lens[i]);
        let mut poly = prefix_byte.to_bigint().unwrap() * random.pow(size_accumulator);
        for j in 0..MAX_STRING_SIZE {
            poly += node[i][j] as u32
                * (random.pow(1 + size_accumulator + j as u32))
                * bool_to_u32(j < lens[i]);
        }
        size_accumulator += rlp_encoding_length * bool_to_u32(i < node_len);
        claim_poly += poly * bool_to_u32(i < node_len);
    }

    // Based on what we've seen, this is what the prefix of the whole encoding should be.
    //
    // Note: For this first version, we assume that the combined length of all the encoded items is
    // >= 56. More specifically, rlp_encode(node[0]).len() + rlp_encode(node[1]).len() + ... +
    // rlp_encode(node[node_len - 1]).len() >= 56. This means:
    // 1. The prefix of `0xf7 + length in bytes of the combined length = 0xf7 + 2 = 0xf9`.
    // 2. 0xf9 is followed by the combined length of all the encoded items.
    claim_poly += 0xf9 * random.pow(size_accumulator);
    claim_poly += (size_accumulator / 256) * random.pow(size_accumulator + 1);
    claim_poly += (size_accumulator % 256) * random.pow(size_accumulator + 2);

    let mut encoding_poly = BigInt::default();
    for i in 3..M {
        // TODO: Don't hardcode 3 here. To understand why we have 3 here, see the comments above
        // about 0xf9.
        let idx = i - 3;
        encoding_poly +=
            encoding[i] as u32 * (random.pow(idx as u32)) * bool_to_u32(idx < encoding_len);
    }

    // Stick the linear combination of the 3-byte prefix to the accumulator. Again, to understand
    // why we have 3 here, see the comments above about 0xf9.
    encoding_poly += encoding[0] * random.pow(size_accumulator);
    encoding_poly += encoding[1] * random.pow(size_accumulator + 1);
    encoding_poly += encoding[2] * random.pow(size_accumulator + 2);

    println!(
        "encoding[0] = {}, encoding[1] = {}, encoding[2] = {}, size_accumulator = {}, size_accumulator / 16 = {}, size_accumulator % 16 = {}",
        encoding[0], encoding[1], encoding[2], size_accumulator, size_accumulator / 16, size_accumulator % 16
    );
    println!(
        "claim_poly = {}, encoding_poly = {}",
        claim_poly, encoding_poly
    );
    assert!(claim_poly == encoding_poly);
}

/// TODO: Consider removing LIST_LEN and using MAX_STRING_SIZE instead.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DecodeHint<const ENCODING_LEN: usize, const LIST_LEN: usize> {}
impl<L: PlonkParameters<D>, const D: usize, const ENCODING_LEN: usize, const LIST_LEN: usize>
    Hint<L, D> for DecodeHint<ENCODING_LEN, LIST_LEN>
{
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let encoded = input_stream.read_value::<ArrayVariable<ByteVariable, ENCODING_LEN>>();
        let len = input_stream.read_value::<Variable>();
        let finish = input_stream.read_value::<BoolVariable>();

        let (decoded_list, decoded_list_lens, len_decoded_list) =
            decode_padded_mpt_node::<ENCODING_LEN, LIST_LEN>(
                &encoded,
                len.as_canonical_u64() as usize,
                finish,
            );

        output_stream
            .write_value::<ArrayVariable<ArrayVariable<ByteVariable, MAX_STRING_SIZE>, LIST_LEN>>(
                decoded_list,
            );
        output_stream.write_value::<ArrayVariable<Variable, LIST_LEN>>(
            decoded_list_lens
                .iter()
                .map(|x| L::Field::from_canonical_usize(*x))
                .collect::<Vec<_>>(),
        );
        output_stream.write_value::<Variable>(L::Field::from_canonical_usize(len_decoded_list));
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
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
        let mut input_stream = VariableStream::new();
        input_stream.write(&encoded);
        input_stream.write(&len);
        input_stream.write(&finish);

        let hint = DecodeHint::<ENCODING_LEN, LIST_LEN> {};

        let output_stream = self.hint(input_stream, hint);
        let decoded_list = output_stream
            .read::<ArrayVariable<ArrayVariable<ByteVariable, ELEMENT_LEN>, LIST_LEN>>(self);
        let decoded_element_lens = output_stream.read::<ArrayVariable<Variable, LIST_LEN>>(self);
        let len_decoded_list = output_stream.read::<Variable>(self);

        // TODO: here add verification logic constraints using `builder` to check that the decoded list is correct

        (decoded_list, decoded_element_lens, len_decoded_list)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::backend::circuit::DefaultParameters;
    use crate::prelude::{DefaultBuilder, GoldilocksField};
    use crate::utils::bytes;

    #[test]
    fn test_decode_element_as_list() {
        const MAX_SIZE: usize = 17 * 32 + 20;
        let rlp_encoding: Vec<u8>  = bytes!("0xf90211a0c5becd7f8e5d47c1fe63ad9fa267d86fe0811bea0a4115aac7123b85fba2d662a03ab19202cb1de4f10fb0da8b5992c54af3dabb2312203f7477918df1393e24aea0b463eb71bcae8fa3183d0232b0d50e2400c21a0131bd48d918330e8683149b76a0d49a6c09224f74cef1286dad36a7f0e23e43e8ba4013fa386a3cda8903a3fe1ea06b6702bcfe04d3a135b786833b2748614d3aea00c728f86b2d1bbbb01b4e2311a08164a965258f9be5befcbf4de8e6cb4cd028689aad98e36ffc612b7255e4fa30a0b90309c6cb6383b2cb4cfeef9511004b705f1bca2c0556aadc2a5fe7ddf665e7a0749c3cee27e5ce85715122b76c18b7b945f1a19f507d5142445b42d50b2dd65aa0dbe35c115e9013b339743ebc2d9940158fb63b9e39f248b15ab74fade183c556a0a2b202f9b8003d73c7c84c8f7eb03298c064842382e57cecac1dfc2d5cabe2ffa02c5f8eba535bf5f18ca5aec74b51e46f219150886618c0301069dfb947006810a0dc01263a3b7c7942b5f0ac23931e0fda54fabaa3e6a58d2aca7ec65957cf8131a07d47344efa308df47f7e0e10491fa22d0564dbce634397c7748cd325fadd6b90a0cf9e45e08b8d60c68a86359adfa31c82883bb4a75b1d854392deb1f4499ba113a0081a664033eb00d5a69fc60f1f8b30e41eb643c5b9772d47301b602902b8d184a058b0bcf02a206cfa7b5f275ef09c97b4ae56abd8e9072f89dad8df1b98dfaa0280");
        let mut encoding_fixed_size = [0u8; MAX_SIZE];
        encoding_fixed_size[..rlp_encoding.len()].copy_from_slice(&rlp_encoding);

        let decoded_list = rlp_decode_mpt_node(&rlp_encoding);
        assert!(decoded_list.len() == 17);
        let string_lengths = decoded_list
            .iter()
            .map(|item| item.len() as u8)
            .collect::<Vec<u8>>();

        let mut decoded_node_fixed_size: FixedSizeMPTNode = [[0u8; MAX_STRING_SIZE]; MAX_NODE_SIZE];
        let mut string_lengths_fixed_size: FixedSizeStringLengths = [0 as usize; 17];
        for (i, item) in decoded_list.iter().enumerate() {
            let len = item.len();
            assert!(len <= 32, "The nested vector is longer than 32 bytes!");
            decoded_node_fixed_size[i][..len].copy_from_slice(item);
            string_lengths_fixed_size[i] = string_lengths[i] as usize;
        }

        // TODO: move below to a different test
        verify_decoded_list::<MAX_SIZE>(
            decoded_node_fixed_size,
            string_lengths_fixed_size,
            17,
            encoding_fixed_size,
            rlp_encoding.len(),
        );
    }

    #[test]
    fn test_decode_element_as_list_wrong_prefix() {
        const MAX_SIZE: usize = 17 * 32 + 20;
        let rlp_encoding: Vec<u8>  = bytes!("0xf90211a0c5becd7f8e5d47c1fe63ad9fa267d86fe0811bea0a4115aac7123b85fba2d662a03ab19202cb1de4f10fb0da8b5992c54af3dabb2312203f7477918df1393e24aea0b463eb71bcae8fa3183d0232b0d50e2400c21a0131bd48d918330e8683149b76a0d49a6c09224f74cef1286dad36a7f0e23e43e8ba4013fa386a3cda8903a3fe1ea06b6702bcfe04d3a135b786833b2748614d3aea00c728f86b2d1bbbb01b4e2311a08164a965258f9be5befcbf4de8e6cb4cd028689aad98e36ffc612b7255e4fa30a0b90309c6cb6383b2cb4cfeef9511004b705f1bca2c0556aadc2a5fe7ddf665e7a0749c3cee27e5ce85715122b76c18b7b945f1a19f507d5142445b42d50b2dd65aa0dbe35c115e9013b339743ebc2d9940158fb63b9e39f248b15ab74fade183c556a0a2b202f9b8003d73c7c84c8f7eb03298c064842382e57cecac1dfc2d5cabe2ffa02c5f8eba535bf5f18ca5aec74b51e46f219150886618c0301069dfb947006810a0dc01263a3b7c7942b5f0ac23931e0fda54fabaa3e6a58d2aca7ec65957cf8131a07d47344efa308df47f7e0e10491fa22d0564dbce634397c7748cd325fadd6b90a0cf9e45e08b8d60c68a86359adfa31c82883bb4a75b1d854392deb1f4499ba113a0081a664033eb00d5a69fc60f1f8b30e41eb643c5b9772d47301b602902b8d184a058b0bcf02a206cfa7b5f275ef09c97b4ae56abd8e9072f89dad8df1b98dfaa0280");
        let mut encoding_fixed_size = [0u8; MAX_SIZE];
        encoding_fixed_size[..rlp_encoding.len()].copy_from_slice(&rlp_encoding);

        let decoded_list = rlp_decode_mpt_node(&rlp_encoding);
        assert!(decoded_list.len() == 17);
        let string_lengths = decoded_list
            .iter()
            .map(|item| item.len() as u8)
            .collect::<Vec<u8>>();

        let mut decoded_node_fixed_size: FixedSizeMPTNode = [[0u8; MAX_STRING_SIZE]; MAX_NODE_SIZE];
        let mut string_lengths_fixed_size: FixedSizeStringLengths = [0 as usize; 17];
        for (i, item) in decoded_list.iter().enumerate() {
            let len = item.len();
            assert!(len <= 32, "The nested vector is longer than 32 bytes!");
            decoded_node_fixed_size[i][..len].copy_from_slice(item);
            string_lengths_fixed_size[i] = string_lengths[i] as usize;
        }

        println!("encoding_fixed_size[0] = {}", encoding_fixed_size[0]);
        // Change the prefix of the rlp encoding. We want verify_decoded_list to detect it and fail
        // even if the remaining encoding is correct.
        encoding_fixed_size[0] = 0x80;
        println!("encoding_fixed_size[0] is now {}", encoding_fixed_size[0]);

        // This should NOT pass.
        let result = std::panic::catch_unwind(|| {
            verify_decoded_list::<MAX_SIZE>(
                decoded_node_fixed_size,
                string_lengths_fixed_size,
                17,
                encoding_fixed_size,
                rlp_encoding.len(),
            );
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_short_encoding() {
        const MAX_SIZE: usize = 2 * 32 + 20;
        // This is a RLP-encoded list of an extension node. The 00 in 0x006f indicates that the path
        // length is even, and the path is 6 -> f. This extension node points to a leaf node with
        // the hash starting with 0x188d11.  ["0x006f",
        // "0x188d1100731419827900267bf4e6ea6d428fa5a67656e021485d1f6c89e69be6"]
        let rlp_encoding: Vec<u8> =
            bytes!("0xe482006fa0188d1100731419827900267bf4e6ea6d428fa5a67656e021485d1f6c89e69be6");
        let mut encoding_fixed_size = [0u8; MAX_SIZE];
        encoding_fixed_size[..rlp_encoding.len()].copy_from_slice(&rlp_encoding);

        let decoded_list = rlp_decode_mpt_node(&rlp_encoding);
        assert!(decoded_list.len() == 2);
        let string_lengths = decoded_list
            .iter()
            .map(|item| item.len() as u8)
            .collect::<Vec<u8>>();

        let mut decoded_node_fixed_size: FixedSizeMPTNode = [[0u8; MAX_STRING_SIZE]; MAX_NODE_SIZE];
        let mut string_lengths_fixed_size: FixedSizeStringLengths = [0; 17];
        for (i, item) in decoded_list.iter().enumerate() {
            let len = item.len();
            assert!(len <= 32, "The nested vector is longer than 32 bytes!");
            decoded_node_fixed_size[i][..len].copy_from_slice(item);
            string_lengths_fixed_size[i] = string_lengths[i] as usize;
        }

        verify_decoded_list::<MAX_SIZE>(
            decoded_node_fixed_size,
            string_lengths_fixed_size,
            2,
            encoding_fixed_size,
            rlp_encoding.len(),
        );
    }

    #[test]

    fn test_rlp_decode_hint() {
        let mut builder: CircuitBuilder<DefaultParameters, 2> = DefaultBuilder::new();

        type F = GoldilocksField;
        const ENCODING_LEN: usize = 600;
        const LIST_LEN: usize = 17;

        let hint: DecodeHint<ENCODING_LEN, LIST_LEN> = DecodeHint::<ENCODING_LEN, LIST_LEN> {};
        let encoded = builder.read::<ArrayVariable<ByteVariable, ENCODING_LEN>>();
        let len = builder.read::<Variable>();
        let finish = builder.read::<BoolVariable>();
        let mut input_stream = VariableStream::new();
        input_stream.write(&encoded);
        input_stream.write(&len);
        input_stream.write(&finish);
        let output_stream = builder.hint(input_stream, hint);
        let decoded_list = output_stream
            .read::<ArrayVariable<ArrayVariable<ByteVariable, MAX_STRING_SIZE>, LIST_LEN>>(
                &mut builder,
            );
        let decoded_element_lens =
            output_stream.read::<ArrayVariable<Variable, LIST_LEN>>(&mut builder);
        let len_decoded_list = output_stream.read::<Variable>(&mut builder);

        builder.write(decoded_list);
        builder.write(decoded_element_lens);
        builder.write(len_decoded_list);

        let circuit = builder.build();
        let mut input = circuit.input();

        // This is a RLP-encoded list of length 17. Each of the first 16 elements is a 32-byte hash,
        // and the last element is 0.
        let rlp_encoding: Vec<u8>  = bytes!("0xf90211a0215ead887d4da139eba306f76d765f5c4bfb03f6118ac1eb05eec3a92e1b0076a03eb28e7b61c689fae945b279f873cfdddf4e66db0be0efead563ea08bc4a269fa03025e2cce6f9c1ff09c8da516d938199c809a7f94dcd61211974aebdb85a4e56a0188d1100731419827900267bf4e6ea6d428fa5a67656e021485d1f6c89e69be6a0b281bb20061318a515afbdd02954740f069ebc75e700fde24dfbdf8c76d57119a0d8d77d917f5b7577e7e644bbc7a933632271a8daadd06a8e7e322f12dd828217a00f301190681b368db4308d1d1aa1794f85df08d4f4f646ecc4967c58fd9bd77ba0206598a4356dd50c70cfb1f0285bdb1402b7d65b61c851c095a7535bec230d5aa000959956c2148c82c207272af1ae129403d42e8173aedf44a190e85ee5fef8c3a0c88307e92c80a76e057e82755d9d67934ae040a6ec402bc156ad58dbcd2bcbc4a0e40a8e323d0b0b19d37ab6a3d110de577307c6f8efed15097dfb5551955fc770a02da2c6b12eedab6030b55d4f7df2fb52dab0ef4db292cb9b9789fa170256a11fa0d00e11cde7531fb79a315b4d81ea656b3b452fe3fe7e50af48a1ac7bf4aa6343a066625c0eb2f6609471f20857b97598ae4dfc197666ff72fe47b94e4124900683a0ace3aa5d35ba3ebbdc0abde8add5896876b25261717c0a415c92642c7889ec66a03a4931a67ae8ebc1eca9ffa711c16599b86d5286504182618d9c2da7b83f5ef780");
        let mut encoding_fixed_size = [0u8; ENCODING_LEN];
        encoding_fixed_size[..rlp_encoding.len()].copy_from_slice(&rlp_encoding);
        let finish = false;
        input.write::<ArrayVariable<ByteVariable, ENCODING_LEN>>(encoding_fixed_size.to_vec());
        input.write::<Variable>(F::from_canonical_usize(rlp_encoding.len()));
        input.write::<BoolVariable>(finish);

        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);

        let decoded_list_out =
            output.read::<ArrayVariable<ArrayVariable<ByteVariable, MAX_STRING_SIZE>, LIST_LEN>>();
        let decoded_element_lens_out = output.read::<ArrayVariable<Variable, LIST_LEN>>();
        let len_decoded_list_out = output.read::<Variable>();

        let (decoded_list_exp, decoded_list_lens_exp, len_decoded_list_exp) =
            decode_padded_mpt_node::<ENCODING_LEN, LIST_LEN>(
                &encoding_fixed_size,
                rlp_encoding.len(),
                finish,
            );

        assert_eq!(
            len_decoded_list_out,
            F::from_canonical_usize(len_decoded_list_exp)
        );
        assert_eq!(decoded_list_out.len(), LIST_LEN);
        assert_eq!(len_decoded_list_out, F::from_canonical_usize(LIST_LEN));

        for i in 0..LIST_LEN {
            assert_eq!(decoded_list_out[i], decoded_list_exp[i]);
            assert_eq!(
                decoded_element_lens_out[i],
                F::from_canonical_usize(decoded_list_lens_exp[i])
            );
        }
    }
    #[test]

    fn test_rlp_decode_hint_short_encoding() {
        let mut builder: CircuitBuilder<DefaultParameters, 2> = DefaultBuilder::new();

        type F = GoldilocksField;
        const ENCODING_LEN: usize = 600;
        const LIST_LEN: usize = 2;

        let hint: DecodeHint<ENCODING_LEN, LIST_LEN> = DecodeHint::<ENCODING_LEN, LIST_LEN> {};
        let encoded = builder.read::<ArrayVariable<ByteVariable, ENCODING_LEN>>();
        let len = builder.read::<Variable>();
        let finish = builder.read::<BoolVariable>();
        let mut input_stream = VariableStream::new();
        input_stream.write(&encoded);
        input_stream.write(&len);
        input_stream.write(&finish);
        let output_stream = builder.hint(input_stream, hint);
        let decoded_list = output_stream
            .read::<ArrayVariable<ArrayVariable<ByteVariable, MAX_STRING_SIZE>, LIST_LEN>>(
                &mut builder,
            );
        let decoded_element_lens =
            output_stream.read::<ArrayVariable<Variable, LIST_LEN>>(&mut builder);
        let len_decoded_list = output_stream.read::<Variable>(&mut builder);

        builder.write(decoded_list);
        builder.write(decoded_element_lens);
        builder.write(len_decoded_list);

        let circuit = builder.build();
        let mut input = circuit.input();

        // This is a RLP-encoded list of an extension node. The 00 in 0x006f indicates that the path
        // length is even, and the path is 6 -> f. This extension node points to a leaf node with
        // the hash starting with 0x188d11.  ["0x006f",
        // "0x188d1100731419827900267bf4e6ea6d428fa5a67656e021485d1f6c89e69be6"]
        let rlp_encoding: Vec<u8> =
            bytes!("0xe482006fa0188d1100731419827900267bf4e6ea6d428fa5a67656e021485d1f6c89e69be6");
        let mut encoding_fixed_size = [0u8; ENCODING_LEN];
        encoding_fixed_size[..rlp_encoding.len()].copy_from_slice(&rlp_encoding);
        let finish = false;
        input.write::<ArrayVariable<ByteVariable, ENCODING_LEN>>(encoding_fixed_size.to_vec());
        input.write::<Variable>(F::from_canonical_usize(rlp_encoding.len()));
        input.write::<BoolVariable>(finish);

        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);

        let decoded_list_out =
            output.read::<ArrayVariable<ArrayVariable<ByteVariable, MAX_STRING_SIZE>, LIST_LEN>>();
        let decoded_element_lens_out = output.read::<ArrayVariable<Variable, LIST_LEN>>();
        let len_decoded_list_out = output.read::<Variable>();

        let (decoded_list_exp, decoded_list_lens_exp, len_decoded_list_exp) =
            decode_padded_mpt_node::<ENCODING_LEN, LIST_LEN>(
                &encoding_fixed_size,
                rlp_encoding.len(),
                finish,
            );

        assert_eq!(
            len_decoded_list_out,
            F::from_canonical_usize(len_decoded_list_exp)
        );
        assert_eq!(decoded_list_out.len(), LIST_LEN);
        assert_eq!(len_decoded_list_out, F::from_canonical_usize(LIST_LEN));

        for i in 0..LIST_LEN {
            assert_eq!(decoded_list_out[i], decoded_list_exp[i]);
            assert_eq!(
                decoded_element_lens_out[i],
                F::from_canonical_usize(decoded_list_lens_exp[i])
            );
        }
    }
}
