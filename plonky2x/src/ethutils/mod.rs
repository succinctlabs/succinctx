pub mod beacon;

pub struct Address(pub [u8; 20]);

pub struct BLSPubkey(pub [u8; 48]);

impl From<Vec<u8>> for BLSPubkey {
    fn from(item: Vec<u8>) -> Self {
        let mut bytes = [0u8; 48];
        bytes.copy_from_slice(&item);
        Self(bytes)
    }
}
