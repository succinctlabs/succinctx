//! This file implements RLP decoder.
//!
//! Reference: https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/

/// An item is a string (i.e., byte array) or a list of items.
#[derive(PartialEq, Debug)]
pub enum RLPItem {
    String(Vec<u8>),
    List(Vec<RLPItem>),
}

/// Private helper struct to iterate over a vector.
///
/// This struct is used to read the next item in the RLP encoding. This is very similar to a regular
/// built-in iterator, but it also keeps track of the current index. Unfortunately, it appears that
/// there is no built-in Rust class that returns the next item and also returns the index of the
/// item. The index of the item is crucial when reading the elements of a list as we only know
/// how many _bytes_ to read, but not how many _items_ to read.
struct VecIterator {
    data: Vec<u8>,
    index: usize,
}

impl VecIterator {
    pub fn new(data: Vec<u8>) -> Self {
        VecIterator { data, index: 0 }
    }
    pub fn next(&mut self) -> Option<u8> {
        if self.index < self.data.len() {
            let item = self.data[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
    pub fn next_chunk(&mut self, length: usize) -> Vec<u8> {
        let mut data = vec![0; length];
        for i in 0..length {
            if let Some(next) = self.next() {
                data[i] = next;
            } else {
                panic!("Not enough bytes to read");
            }
        }
        data
    }
    pub fn next_index(&mut self) -> usize {
        self.index
    }
}

/// Private helper method to read the next item.
fn decode_with_iterator(it: &mut VecIterator) -> RLPItem {
    match it.next() {
        Some(byte) if byte < 0x7f => {
            // The prefix indicates that the byte has its own RLP encoding.
            RLPItem::String(vec![byte])
        }
        Some(byte) if byte <= 0xB7 => {
            // The byte indicates a short string containing up to 55 bytes.
            let length = (byte - 0x80) as usize;
            RLPItem::String(it.next_chunk(length))
        }
        Some(byte) if byte <= 0xBF => {
            // The byte indicates a long string containing more than 55 bytes.
            let nb_length_bytes = (byte - 0xB7) as usize;
            let length_data = it.next_chunk(nb_length_bytes);

            // Convert the length data to a usize.
            let length = length_data
                .iter()
                .rev()
                .enumerate()
                .fold(0, |acc, (i, x)| acc + ((*x as usize) << (8 * i as u32)));
            RLPItem::String(it.next_chunk(length))
        }
        Some(byte) if byte <= 0xF7 => {
            // The byte indicates a short list, where the payload is 0-55 bytes.
            let length = (byte - 0xC0) as usize;

            // Here, we need to process length _bytes_, not length _items_.
            let next_index = it.next_index();
            let mut elements = Vec::new();
            while it.next_index() < next_index + length {
                elements.push(decode_with_iterator(it));
            }
            RLPItem::List(elements)
        }
        Some(byte) => {
            // The byte indicates a longer list.
            let nb_length_bytes = (byte - 0xf7) as usize;
            let length_data = it.next_chunk(nb_length_bytes);

            // Convert the length data to a usize.
            let length = length_data
                .iter()
                .rev()
                .enumerate()
                .fold(0, |acc, (i, x)| acc + ((*x as usize) << (8 * i as u32)));

            // Here, we need to process length _bytes_, not length _items_.
            let next_index = it.next_index();
            let mut elements = Vec::new();
            while it.next_index() < next_index + length {
                elements.push(decode_with_iterator(it));
            }
            RLPItem::List(elements)
        }
        None => {
            panic!("unexpectedly ran out of bytes to read")
        }
    }
}

pub fn decode(data: &[u8]) -> RLPItem {
    let mut vec_it = VecIterator::new(data.to_vec());
    decode_with_iterator(&mut vec_it)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::bytes;

    fn test_decoder(encoding: Vec<u8>, exp: RLPItem) {
        let decoded = decode(&encoding);
        assert_eq!(decoded, exp);
    }

    #[test]
    fn test_simple_list() {
        let encoding: Vec<u8> = bytes!("0x82006f");
        let exp = RLPItem::String(vec![0x00, 0x6f]);
        test_decoder(encoding, exp)
    }

    #[test]
    fn test_empty_string() {
        let encoding: Vec<u8> = bytes!("0x80");
        let exp = RLPItem::String(vec![]);
        test_decoder(encoding, exp)
    }

    #[test]
    fn test_empty_list() {
        let encoding: Vec<u8> = bytes!("0xc0");
        let exp = RLPItem::List(vec![]);
        test_decoder(encoding, exp)
    }

    #[test]
    fn test_single_hash() {
        let encoding: Vec<u8> =
            bytes!("0xa04cfa7808badb1e62513ba42486f21240f696a9ffc6d598273d226cc5b30bfe28");
        let exp = RLPItem::String(bytes!(
            "0x4cfa7808badb1e62513ba42486f21240f696a9ffc6d598273d226cc5b30bfe28"
        ));
        test_decoder(encoding, exp)
    }

    #[test]
    fn test_list_hash() {
        // A list of 3 hashes.
        let encoding: Vec<u8> =
            bytes!("0xf863a011fa7808badb1e62513ba42486f21240f696a9ffc6d598273d226cc5b30bfe28a022fa7808badb1e62513ba42486f21240f696a9ffc6d598273d226cc5b30bfe28a033fa7808badb1e62513ba42486f21240f696a9ffc6d598273d226cc5b30bfe28");
        let exp = RLPItem::List(vec![
            RLPItem::String(bytes!(
                "0x11fa7808badb1e62513ba42486f21240f696a9ffc6d598273d226cc5b30bfe28"
            )),
            RLPItem::String(bytes!(
                "0x22fa7808badb1e62513ba42486f21240f696a9ffc6d598273d226cc5b30bfe28"
            )),
            RLPItem::String(bytes!(
                "0x33fa7808badb1e62513ba42486f21240f696a9ffc6d598273d226cc5b30bfe28"
            )),
        ]);
        test_decoder(encoding, exp)
    }

    #[test]
    fn test_branch_node() {
        // A list of 17 hashes, most of them are empty.
        let encoding: Vec<u8> =
            bytes!("0xf851808080808080808080808080a035d937961d73f8a0eea9ae41b2f4cbb73c1d2c0666ea35f1ae05c43b5896b1098080a0b286218777cc1883b08227a900f3b4b876e52de06e342560852a263838d4c8a280");
        let mut exp: Vec<RLPItem> = vec![];
        for i in 0..17 {
            if i == 12 {
                exp.push(RLPItem::String(bytes!(
                    "0x35d937961d73f8a0eea9ae41b2f4cbb73c1d2c0666ea35f1ae05c43b5896b109"
                )));
            } else if i == 15 {
                exp.push(RLPItem::String(bytes!(
                    "0xb286218777cc1883b08227a900f3b4b876e52de06e342560852a263838d4c8a2"
                )));
            } else {
                exp.push(RLPItem::String(vec![]));
            }
        }
        test_decoder(encoding, RLPItem::List(exp))
    }

    #[test]
    fn test_nested_list() {
        let encoding: Vec<u8> = bytes!("0xc801c502c2030405c0");
        // The original value is ["0x01", ["0x02", ["0x03", "0x04"], "0x05"], []].
        let mut exp: Vec<RLPItem> = vec![];
        exp.push(RLPItem::String(bytes!("0x01")));
        exp.push(RLPItem::List(vec![
            RLPItem::String(bytes!("0x02")),
            RLPItem::List(vec![
                RLPItem::String(bytes!("0x03")),
                RLPItem::String(bytes!("0x04")),
            ]),
            RLPItem::String(bytes!("0x05")),
        ]));
        exp.push(RLPItem::List(vec![]));

        test_decoder(encoding, RLPItem::List(exp))
    }
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
