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
pub const MAX_STRING_SIZE: usize = 32;
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
pub const MAX_NODE_SIZE: usize = 17;
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

    // First, we'll calculate the polynomial that represents the given encoding. encoding_poly is
    // \sigma_{i = 0}^{encoding_len - 1} encoding[i] * random^i.

    let mut encoding_poly = BigInt::default();

    for i in 0..M {
        encoding_poly +=
            encoding[i] as u32 * (random.pow(i as u32)) * bool_to_u32(i < encoding_len);
    }

    // Now, we will calculate the polynomial that represents the node. Here, we will calculate what
    // the polynomial should be if we correctly encode this node. Due to the complexity of the
    // rlp encoding, we need to encode every string in the node _before_ we know the prefix of the
    // whole encoding. So, we will calculate the polynomial that represents the encoding of each
    // string in the node.

    let mut sum_of_rlp_encoding_length: u32 = 0;
    let mut claim_poly = BigInt::default();
    for i in 0..MAX_NODE_SIZE {
        // Calculate the prefix and the length of the encoding of the _current_ string.
        let (prefix_byte, rlp_encoding_length) = calculate_rlp_encode_metadata(node[i], lens[i]);
        let mut poly = prefix_byte.to_bigint().unwrap() * random.pow(sum_of_rlp_encoding_length);
        for j in 0..MAX_STRING_SIZE {
            poly += node[i][j] as u32
                * (random.pow(1 + sum_of_rlp_encoding_length + j as u32))
                * bool_to_u32(j < lens[i]);
        }
        sum_of_rlp_encoding_length += rlp_encoding_length * bool_to_u32(i < node_len);
        claim_poly += poly * bool_to_u32(i < node_len);
    }

    // Based on what we've seen, we calculate the prefix of the whole encoding.
    if sum_of_rlp_encoding_length <= 55 {
        // If the combined length of the encoded strings is <= 55, then the prefix is simply the
        // length of the encoding + 0xc0.

        // "Shift" the current polynomial by multiplying it by random to "make room" for the prefix.
        claim_poly *= random.clone();
        claim_poly += 0xc0 + sum_of_rlp_encoding_length;
    } else {
        // If the combined length of the encoded strings is > 55, then the prefix is [0xf7 + the
        // length of the combined length of the encoding] followed by the length of the encoding.

        // "Shift" the current polynomial by multiplying it by random.pow(3) to "make room" for the
        // prefix.
        claim_poly *= random.pow(3);
        claim_poly += 0xf9;

        // Most signficant byte.
        claim_poly += (sum_of_rlp_encoding_length / 256) * random.clone();
        // Lease siginificant byte.
        claim_poly += (sum_of_rlp_encoding_length % 256) * random.pow(2);
    }

    // TODO: Remove these debugging statements before making a PR.
    println!(
        "encoding[0] = {}, encoding[1] = {}, encoding[2] = {}, size_accumulator = {}, size_accumulator / 256 = {}, size_accumulator % 256 = {}",
        encoding[0], encoding[1], encoding[2], sum_of_rlp_encoding_length, sum_of_rlp_encoding_length / 256, sum_of_rlp_encoding_length % 256
    );
    println!(
        "claim_poly = {}, encoding_poly = {}",
        claim_poly, encoding_poly
    );

    // Now, we have both polynomials. Hopefully, they match!
    assert!(claim_poly == encoding_poly);
}

#[cfg(test)]
mod tests {}
