//! This file implements fixed size utilities for RLP and MPT.
//!
//! Reference 1: https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/
//! Reference 2: https://ethereum.org/en/developers/docs/data-structures-and-encoding/patricia-merkle-trie

use num::bigint::ToBigInt;
use num::BigInt;

use crate::utils::stream::Stream;

/// An item is a string (i.e., byte array) or a list of items. The item assumes a fixed size.
///
/// This item can potentially represent the following objects:
///
/// 1. Bytes32: Usually the hash of the rlp-encoding of some data that exceeds 32 bytes.
/// 2. Branch Node (?): If the node takes less than 32 bytes to encode, it will be placed inline.
/// 3. Extension Node (?): If the node takes less than 32 bytes to encode, it will be placed inline.
/// 4. Leaf Node (?): If the node takes less than 32 bytes to ecnode, it will be placed inline.
/// 5. NULL: Represents the empty string "" or <>.
pub struct RLPItemFixedSize {
    pub data: [u8; 32],
    pub len: usize,
}

/// A Merkle Patricia Trie node. The underlying data assumes a fixed size of up to 17 fixed size rlp
/// items.
///
/// This node can potentially represent the following objects:
///
/// 1. Branch Node: A 17-item node [v0, ..., v15, vt]
/// 2. Leaf Node: A 2-item node [encodedPath, value]
/// 3. Extension Node: A 2-item node [encodedPath, key]
pub struct MPTNodeFixedSize {
    pub data: [RLPItemFixedSize; 17],
    pub len: usize,
}

// TODO:??
// rlp_item.to_fixed_size();
// rlp_item.to_mpt_node_fixed_size();

// pub fn rlp_decode_mpt_node(input: &[u8]) -> Vec<Vec<u8>> {
//     info!("input {:?}", Bytes::from(input.to_vec()).to_string());
//     let prefix = input[0];

//     if prefix < 0xC0 {
//         panic!("Invalid prefix, MPT node must be a list")
//     } else if prefix <= 0xF7 {
//         // Short list (0-55 bytes total payload)
//         let list_length = (prefix - 0xC0) as usize;
//         // We assert that the input is simply [list_length, list_content...] and not suffixed by anything else
//         assert!(input.len() == 1 + list_length);
//         let (ele_1, increment) = rlp_decode_next_string(&input[1..]);
//         let (ele_2, _) = rlp_decode_next_string(&input[1 + increment..]);
//         vec![ele_1, ele_2]
//     } else {
//         info!("hi in this case");
//         // TODO: check that prefix is bounded within a certain range
//         let len_of_list_length = prefix - 0xF7;
//         // info!("len_of_list_length {:?}", len_of_list_length);
//         // TODO: figure out what to do with len_of_list_length
//         let mut pos = 1 + len_of_list_length as usize;
//         let mut res = vec![];
//         for _ in 0..17 {
//             let (decoded_string, num_bytes_processed) = rlp_decode_next_string(&input[pos..]);
//             info!(
//                 "decoded_string {:?}",
//                 Bytes::from(decoded_string.clone()).to_string()
//             );
//             info!("{:?} bytes processed", num_bytes_processed);
//             res.push(decoded_string);
//             pos += num_bytes_processed;
//             if pos >= input.len() {
//                 break;
//             }
//         }
//         assert!(pos == input.len()); // Checks that we have iterated through all the input
//         assert!(res.len() == 17 || res.len() == 2);
//         info!("END");
//         res
//     }
// }

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

/// This calculates the prefix and the length of the encoding that we would get if we were to encode
/// the given string. More specifically, the first return value is the prefix of
/// rlp_encode(padded_string[..len]). The second return value is
/// rlp_encode(padded_string[..len]).len().
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
        encoding_poly += encoding[i] as u32 * (random.pow(i as u32)) * ((i < encoding_len) as u32);
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
                * ((j < lens[i]) as u32);
        }
        sum_of_rlp_encoding_length += rlp_encoding_length * ((i < node_len) as u32);
        claim_poly += poly * ((i < node_len) as u32);
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
    fn set_up_short_encoding() -> (
        Vec<u8>,
        [u8; MAX_SIZE_SHORT_ENCODING],
        FixedSizeMPTNode,
        FixedSizeStringLengths,
    ) {
        // This is a RLP-encoded list of an extension node. The 00 in 0x006f indicates that the path
        // length is even, and the path is 6 -> f. This extension node points to a leaf node with
        // the hash starting with 0x188d11.  ["0x006f",
        // "0x188d1100731419827900267bf4e6ea6d428fa5a67656e021485d1f6c89e69be6"]
        let rlp_encoding: Vec<u8> =
            bytes!("0xe482006fa0188d1100731419827900267bf4e6ea6d428fa5a67656e021485d1f6c89e69be6");
        let mut encoding_fixed_size = [0u8; MAX_SIZE_SHORT_ENCODING];
        encoding_fixed_size[..rlp_encoding.len()].copy_from_slice(&rlp_encoding);

        let mut decoded_node_fixed_size: FixedSizeMPTNode = [[0u8; MAX_STRING_SIZE]; MAX_NODE_SIZE];
        let mut string_lengths_fixed_size: FixedSizeStringLengths = [0; 17];

        decoded_node_fixed_size[0][..2].copy_from_slice(rlp_encoding[2..4].as_ref());
        string_lengths_fixed_size[0] = 2;

        decoded_node_fixed_size[1][..32].copy_from_slice(rlp_encoding[5..].as_ref());
        string_lengths_fixed_size[1] = 32;
        (
            rlp_encoding,
            encoding_fixed_size,
            decoded_node_fixed_size,
            string_lengths_fixed_size,
        )
    }

    #[test]
    fn test_rlp_decode_mpt_node_short_encoding() {
        let (rlp_encoding, _, decoded_node_fixed_size, string_lengths_fixed_size) =
            set_up_short_encoding();

        let decoded_node = rlp_decode_mpt_node(&rlp_encoding);
        assert_eq!(decoded_node.len(), 2);
        for i in 0..2 {
            assert_eq!(
                decoded_node[i].as_slice(),
                &decoded_node_fixed_size[i][..string_lengths_fixed_size[i]]
            );
        }
    }

    #[test]
    fn test_verify_decode_list_short_encoding() {
        let (rlp_encoding, encoding_fixed_size, decoded_node_fixed_size, string_lengths_fixed_size) =
            set_up_short_encoding();

        verify_decoded_list::<MAX_SIZE_SHORT_ENCODING>(
            decoded_node_fixed_size,
            string_lengths_fixed_size,
            2,
            encoding_fixed_size,
            rlp_encoding.len(),
        );
    }

    #[test]
    fn test_verify_decode_list_short_encoding_wrong_prefix() {
        let (
            rlp_encoding,
            mut encoding_fixed_size,
            decoded_node_fixed_size,
            string_lengths_fixed_size,
        ) = set_up_short_encoding();

        encoding_fixed_size[0] = 0x80;

        // We expect the verifier to fail as the encoding is incorrect.
        let result = std::panic::catch_unwind(|| {
            verify_decoded_list::<MAX_SIZE_SHORT_ENCODING>(
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
    fn test_rlp_decode_mpt_node_and_verify_decoded_list() {
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
            17,
            encoding_fixed_size,
            rlp_encoding.len(),
        );
    }
}
