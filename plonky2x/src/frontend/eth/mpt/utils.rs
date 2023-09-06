use ethers::types::Bytes;
use ethers::utils::keccak256;

pub fn print_vec(a: &[u8]) -> String {
    let bytes = Bytes::from_iter(a.iter());
    bytes.to_string()
}

pub fn print_vecs(a: Vec<Vec<u8>>) {
    for i in 0..a.len() {
        println!("{} {:?}", i, print_vec(&a[i]));
    }
}

pub fn keccack_variable<const M: usize>(input: [u8; M], len: u32) -> [u8; 32] {
    keccak256(&input[..len as usize])
}

pub fn to_nibbles(data: &[u8]) -> Vec<u8> {
    let mut nibbles = Vec::with_capacity(data.len() * 2);
    for byte in data {
        // High nibble (upper 4 bits)
        nibbles.push(byte >> 4);
        // Low nibble (lower 4 bits)
        nibbles.push(byte & 0x0F);
    }
    nibbles
}

pub fn assert_bytes_equal(a: &[u8], b: &[u8]) {
    assert!(a.len() == b.len());
    for i in 0..a.len() {
        assert!(a[i] == b[i]);
    }
}

pub fn is_bytes32_eq(a: [u8; 32], b: [u8; 32]) -> u32 {
    for i in 0..32 {
        if a[i] != b[i] {
            return 0;
        }
    }
    1
}

pub fn is_eq(a: u8, b: usize) -> u32 {
    if a == b as u8 {
        1
    } else {
        0
    }
}

pub fn is_leq(x: u32, y: u32) -> u32 {
    if x <= y {
        1
    } else {
        0
    }
}

pub fn is_le(x: u32, y: u32) -> u32 {
    if x < y {
        1
    } else {
        0
    }
}

pub fn mux(a: &[u8], sel: u8) -> u8 {
    a[sel as usize]
}

pub fn mux_nested<const N: usize>(a: Vec<Vec<u8>>, sel: u8) -> [u8; N] {
    a[sel as usize].clone().try_into().unwrap()
}
