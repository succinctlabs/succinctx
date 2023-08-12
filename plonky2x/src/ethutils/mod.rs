use crate::utils::byte_to_bits_be;

pub mod beacon;
pub mod deserialize_bigint;

pub struct Address(pub [bool; 160]);

pub struct BLSPubkey(pub [bool; 384]);

impl From<Vec<u8>> for BLSPubkey {
    fn from(item: Vec<u8>) -> Self {
        let mut a = [false; 384];
        for i in 0..48 {
            let b = byte_to_bits_be(item[i]);
            for j in 0..8 {
                a[i * 8 + j] = b[j];
            }
        }
        Self(a)
    }
}
