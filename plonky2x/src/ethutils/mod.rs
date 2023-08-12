pub mod beacon;
pub mod deserialize_bigint;

pub struct Address(pub [bool; 160]);

pub struct BLSPubkey(pub [bool; 384]);

impl From<Vec<u8>> for BLSPubkey {
    fn from(item: Vec<u8>) -> Self {
        let mut bytes = [false; 384];
        Self(bytes)
    }
}
