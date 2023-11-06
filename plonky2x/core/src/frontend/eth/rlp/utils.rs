//! This file implements fixed size utilities for RLP and MPT.
//!
//! Reference 1: https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/
//! Reference 2: https://ethereum.org/en/developers/docs/data-structures-and-encoding/patricia-merkle-trie

use num::bigint::ToBigInt;
use num::BigInt;

use super::decoder::RLPItem;
use crate::frontend::eth::rlp::decoder::decode;

pub const MAX_RLP_ITEM_SIZE: usize = 32;

/// An item is a string (i.e., byte array) or a list of items. The item assumes a fixed size.
///
/// This item can potentially represent the following objects:
///
/// 1. Bytes32: Usually the hash of the rlp-encoding of some data that exceeds 32 bytes.
/// 2. Branch Node (?): If the node takes less than 32 bytes to encode, it will be placed inline.
/// 3. Extension Node (?): If the node takes less than 32 bytes to encode, it will be placed inline.
/// 4. Leaf Node (?): If the node takes less than 32 bytes to ecnode, it will be placed inline.
/// 5. NULL: Represents the empty string "" or <>.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct RLPItemFixedSize {
    pub data: [u8; MAX_RLP_ITEM_SIZE],
    pub len: usize,
}

impl From<&RLPItem> for RLPItemFixedSize {
    fn from(item: &RLPItem) -> Self {
        // Match RLPItem and see if it's a list or a string
        match item {
            RLPItem::List(_) => {
                // This is when a node references another node directly.
                panic!("not implemented yet")
            }
            RLPItem::String(data) => {
                // Copy data into self.data.
                // If data.len() > MAX_RLP_ITEM_SIZE, panic.
                let len = data.len();
                let mut array = [0; MAX_RLP_ITEM_SIZE];
                array[..data.len()].copy_from_slice(data);

                RLPItemFixedSize { data: array, len }
            }
        }
    }
}

pub const MAX_MPT_NODE_SIZE: usize = 17;

/// A Merkle Patricia Trie node. The underlying data assumes a fixed size of up to 17 fixed size rlp
/// items.
///
/// This node can potentially represent the following objects:
///
/// 1. Branch Node: A 17-item node [v0, ..., v15, vt]
/// 2. Leaf Node: A 2-item node [encodedPath, value]
/// 3. Extension Node: A 2-item node [encodedPath, key]
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct MPTNodeFixedSize {
    pub data: [RLPItemFixedSize; MAX_MPT_NODE_SIZE],
    pub len: usize,
}

impl From<RLPItem> for MPTNodeFixedSize {
    fn from(item: RLPItem) -> Self {
        match item {
            RLPItem::String(_data) => {
                panic!("a node cannot be a string")
            }
            RLPItem::List(ls) => {
                assert!(ls.len() == 2 || ls.len() == 17);
                let mut res = [RLPItemFixedSize {
                    data: [0u8; MAX_RLP_ITEM_SIZE],
                    len: 0,
                }; MAX_MPT_NODE_SIZE];
                for (i, item) in ls.iter().enumerate() {
                    res[i] = RLPItemFixedSize::from(item);
                }
                MPTNodeFixedSize {
                    data: res,
                    len: ls.len(),
                }
            }
        }
    }
}

/// This decodes an padded encoding of MPT node and returns the padded, decoded MPT node.
///
/// The input is a tuple of:
/// - encoded: padded, RLP-encoded MPT node,
/// - len: "true length" of `encoded`,
/// - finish: a boolean indicating whether we should terminate early.
pub fn decode_padded_mpt_node<const ENCODING_LEN: usize, const LIST_LEN: usize>(
    encoded: &[u8],
    len: usize,
    finish: bool,
) -> MPTNodeFixedSize {
    assert!(len <= ENCODING_LEN); // len is the "true" length of "encoded", which is padded to length `ENCODING_LEN`
    assert!(LIST_LEN == 2 || LIST_LEN == 17); // Right now we only support decoding lists of length 2 or 17

    if finish {
        return MPTNodeFixedSize::default();
    }
    MPTNodeFixedSize::from(decode(&encoded[0..len]))
}

/// This calculates the prefix and the length of the encoding that we would get if we were to encode
/// the given string.
///
/// More specifically, the first return value is the prefix of rlp_encode(padded_string[..len]). The
/// econd return value is rlp_encode(padded_string[..len]).len(). This function is intentionally
/// kept separate from RLPItemFixedSize to make the verification code easier to convert to the
/// circuit language.
fn calculate_rlp_encode_metadata(padded_string: &RLPItemFixedSize) -> (u32, u32) {
    if padded_string.len == 0 {
        // While it may be counterintutive, rlp_encode(the empty string) = 0x80.
        (0x80, 1)
    } else if padded_string.len == 1 {
        if padded_string.data[0] < 0x80 {
            // A single byte less than 0x80 is its own RLP encoding.
            (padded_string.data[0] as u32, 1)
        } else {
            // A single byte greater than 0x80 is encoded as 0x81 + the byte.
            (0x81, 2)
        }
    } else if padded_string.len <= 55 {
        // A string of length <= 55 is encoded as (0x80 + length of the string) followed by the
        // string.
        (
            padded_string.len as u32 + 0x80,
            padded_string.len as u32 + 1,
        )
    } else {
        panic!("Invalid length {}", padded_string.len)
    }
}

/// This is the vanilla implementation of the RLC trick for verifying the decoded_list
pub fn verify_decoded_list<const M: usize>(
    node: MPTNodeFixedSize,
    encoding: [u8; M],
    encoding_len: usize,
) {
    let random = 1000_i32.to_bigint().unwrap();

    // First, we'll calculate the polynomial that represents the given encoding. encoding_poly is
    // \sigma_{i = 0}^{encoding_len - 1} encoding[i] * random^i.

    let mut encoding_poly = BigInt::default();

    for i in 0..M {
        encoding_poly += encoding[i] as u32 * (random.pow(i as u32)) * ((i < encoding_len) as u32);
    }

    // Now, we will calculate the polynomial that represents the node. Here, we will calculate what
    // the polynomial should be if we correctly encode this node. Due to the complexity of the
    // rlp encoding, we need to encode every string in the node _before_ we know the prefix of the
    // whole encoding. So, we will calculate the polynomial that represents the encoding of each
    // string in the node.

    let mut sum_of_rlp_encoding_length: u32 = 0;
    let mut claim_poly = BigInt::default();
    for i in 0..MAX_MPT_NODE_SIZE {
        // Calculate the prefix and the length of the encoding of the _current_ string.
        let (prefix_byte, rlp_encoding_length) = calculate_rlp_encode_metadata(&node.data[i]);
        let mut poly = prefix_byte.to_bigint().unwrap() * random.pow(sum_of_rlp_encoding_length);
        for j in 0..MAX_RLP_ITEM_SIZE {
            poly += node.data[i].data[j] as u32
                * (random.pow(1 + sum_of_rlp_encoding_length + j as u32))
                * ((j < node.data[i].len) as u32);
        }
        sum_of_rlp_encoding_length += rlp_encoding_length * ((i < node.len) as u32);
        claim_poly += poly * ((i < node.len) as u32);
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

    assert!(claim_poly == encoding_poly);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::bytes;

    const MAX_SIZE_SHORT_ENCODING: usize = 2 * 32 + 20;
    fn set_up_short_encoding() -> (Vec<u8>, [u8; MAX_SIZE_SHORT_ENCODING], MPTNodeFixedSize) {
        // This is a RLP-encoded list of an extension node. The 00 in 0x006f indicates that the path
        // length is even, and the path is 6 -> f. This extension node points to a leaf node with
        // the hash starting with 0x188d11.  ["0x006f",
        // "0x188d1100731419827900267bf4e6ea6d428fa5a67656e021485d1f6c89e69be6"]
        let rlp_encoding: Vec<u8> =
            bytes!("0xe482006fa0188d1100731419827900267bf4e6ea6d428fa5a67656e021485d1f6c89e69be6");
        let mut encoding_fixed_size = [0u8; MAX_SIZE_SHORT_ENCODING];
        encoding_fixed_size[..rlp_encoding.len()].copy_from_slice(&rlp_encoding);

        let mut decoded_node_fixed_size: MPTNodeFixedSize = MPTNodeFixedSize::default();

        decoded_node_fixed_size.data[0].data[..2].copy_from_slice(rlp_encoding[2..4].as_ref());
        decoded_node_fixed_size.data[0].len = 2;

        decoded_node_fixed_size.data[1].data[..32].copy_from_slice(rlp_encoding[5..].as_ref());
        decoded_node_fixed_size.data[1].len = 32;

        decoded_node_fixed_size.len = 2;

        (rlp_encoding, encoding_fixed_size, decoded_node_fixed_size)
    }

    #[test]
    fn test_rlp_decode_mpt_node_short_encoding() {
        let (rlp_encoding, rlp_encoding_fixed_size, decoded_node_fixed_size_exp) =
            set_up_short_encoding();

        let decoded_node_fixed_size_out = decode_padded_mpt_node::<MAX_SIZE_SHORT_ENCODING, 2>(
            &rlp_encoding_fixed_size,
            rlp_encoding.len(),
            false,
        );
        assert_eq!(decoded_node_fixed_size_out, decoded_node_fixed_size_exp);
    }

    #[test]
    fn test_verify_decode_list_short_encoding() {
        let (rlp_encoding, rlp_encoding_fixed_size, decoded_node_fixed_size_exp) =
            set_up_short_encoding();

        verify_decoded_list::<MAX_SIZE_SHORT_ENCODING>(
            decoded_node_fixed_size_exp,
            rlp_encoding_fixed_size,
            rlp_encoding.len(),
        );
    }

    #[test]
    fn test_verify_decode_list_short_encoding_wrong_prefix() {
        let (rlp_encoding, mut rlp_encoding_fixed_size, decoded_node_fixed_size_exp) =
            set_up_short_encoding();

        rlp_encoding_fixed_size[0] = 0x80;

        // We expect the verifier to fail as the encoding and decoding don't match.
        let result = std::panic::catch_unwind(|| {
            verify_decoded_list::<MAX_SIZE_SHORT_ENCODING>(
                decoded_node_fixed_size_exp,
                rlp_encoding_fixed_size,
                rlp_encoding.len(),
            );
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_decode_list_short_encoding_wrong_encoding_body() {
        let (rlp_encoding, mut rlp_encoding_fixed_size, decoded_node_fixed_size_exp) =
            set_up_short_encoding();

        rlp_encoding_fixed_size[30] += 0x1;

        // We expect the verifier to fail as the encoding and decoding don't match.
        let result = std::panic::catch_unwind(|| {
            verify_decoded_list::<MAX_SIZE_SHORT_ENCODING>(
                decoded_node_fixed_size_exp,
                rlp_encoding_fixed_size,
                rlp_encoding.len(),
            );
        });
        assert!(result.is_err());
    }
}
