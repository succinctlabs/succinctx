//! This file implements RLP decoder.
//!
//! Reference: https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/
//!
//! Implementation inspired by: https://github.com/mquandalle/ethereum-rust/blob/master/src/rlp.rs

/// An item is a string (i.e., byte array) or a list of items.
pub enum RLPItem {
    String(Vec<u8>),
    List(Vec<RLPItem>),
}

fn read_exact(it: &mut std::slice::Iter<u8>, length: usize) -> Vec<u8> {
    let mut data = vec![0; length];
    for i in 0..length {
        if let Some(&next) = it.next() {
            data[i] = next;
        } else {
            panic!("Not enough bytes to read");
        }
    }
    data
}

/// Private helper method to read the next item.
fn decode_with_iterator(it: &mut std::slice::Iter<u8>) -> RLPItem {
    match it.next() {
        Some(&byte) if byte < 0x7f => {
            // The prefix indicates that the byte has its own RLP encoding.
            RLPItem::String(vec![byte])
        }
        Some(&byte) if byte == 0x80 => {
            // The prefix indicates this is the null value.
            RLPItem::String(vec![])
        }
        Some(&byte) if byte <= 0xB7 => {
            // The byte indicates a short string containing up to 55 bytes.
            let length = (byte - 0x80) as usize;
            RLPItem::String(read_exact(it, length))
        }
        Some(&byte) if byte <= 0xBF => {
            // The byte indicates a long string containing more than 55 bytes.
            let nb_length_bytes = (byte - 0xB7) as usize;
            let length_data = read_exact(it, nb_length_bytes);

            let mut length = 0;
            for i in 0..nb_length_bytes {
                length += length_data[nb_length_bytes - i] as usize * 256_usize.pow(i as u32);
            }
            RLPItem::String(read_exact(it, length))
        }
        Some(&byte) if byte <= 0xF7 => {
            // The byte indicates a short list, where the payload is 0-55 bytes.
            let length = (byte - 0xC0) as usize;
            let mut elements = Vec::new();
            for i in 0..length {
                elements.push(decode_with_iterator(it));
            }
            RLPItem::List(elements)
        }
        Some(&byte) => {
            // The byte indicates a longer list.
            let nb_length_bytes = (byte - 0xf7) as usize;
            let length_data = read_exact(it, nb_length_bytes);

            let mut length = 0;
            for i in 0..nb_length_bytes {
                length += length_data[nb_length_bytes - i] as usize * 256_usize.pow(i as u32);
            }

            let mut elements = Vec::new();
            for i in 0..length {
                elements.push(decode_with_iterator(it));
            }
            RLPItem::List(elements)
        }
        None => {
            panic!()
        }
    }
}

pub fn decode(data: &[u8]) -> RLPItem {
    decode_with_iterator(&mut data.into_iter())
}

// TODO delete this comment for the PR
//impl Stream for RLPDecoder {
//    type Item = Result<RLPItem, DecodeError>;
//
//    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
//        // Your stream-based logic goes here to read bytes from the input stream
//    }
//}
//
///// An item is a string (i.e., byte array) or a list of items. The item assumes a fixed size.
/////
///// This item can potentially represent the following objects:
/////
///// 1. Bytes32: Usually the hash of the rlp-encoding of some data that exceeds 32 bytes.
///// 2. Branch Node (?): If the node takes less than 32 bytes to encode, it will be placed inline.
///// 3. Extension Node (?): If the node takes less than 32 bytes to encode, it will be placed inline.
///// 4. Leaf Node (?): If the node takes less than 32 bytes to ecnode, it will be placed inline.
///// 5. NULL: Represents the empty string "" or <>.
/////
//impl Stream<u8> {
//    /// Decodes the next item in the input using RLP.
//    fn rlp_decode_next_item(&mut self) -> RLPItem {
//        let prefix = self.read_exact(0)[0];
//        if prefix <= 0x7F {
//            // The prefix indicates that the byte has its own RLP encoding.
//            RLPItem::String(vec![prefix])
//        } else if prefix == 0x80 {
//            // The prefix indicates this is the null value.
//            RLPItem::String(vec![])
//        } else if prefix <= 0xB7 {
//            // The prefix indicates a short string containing up to 55 bytes.
//            let length = (prefix - 0x80) as usize;
//            RLPItem::String(self.read_exact(length).to_vec())
//        } else if prefix <= 0xBF {
//            // The prefix indicates a long string containing more than 55 bytes.
//            let nb_length_bytes = (prefix - 0xB7) as usize;
//            let mut length_bytes = self.read_exact(nb_length_bytes);
//            let mut length = 0;
//            for i in 0..nb_length_bytes {
//                length += length_bytes[nb_length_bytes - i] as usize * 256_usize.pow(i as u32);
//            }
//            RLPItem::String(self.read_exact(length).to_vec())
//        } else if prefix <= 0xF7 {
//            /// The prefix indicates a short list, where the payload is 0-55 bytes.
//            let length = (prefix - 0xC0) as usize;
//            let mut elements = Vec::new();
//            for i in 0..length {
//                elements.push(self.rlp_decode_next_item());
//            }
//            RLPItem::List(elements)
//        } else {
//            // The prefix indicates a longer list.
//            let nb_length_bytes = (prefix - 0xF7) as usize;
//            todo!()
//        }
//    }
//}
//// TODO
//// I also need to create a decoder. Use the function above to decode and return the results instead
//// of creating a stream.
//
//pub fn decode(encoded_item: &Vec<u8>) -> RLPItem {
//    todo!();
//}
//
