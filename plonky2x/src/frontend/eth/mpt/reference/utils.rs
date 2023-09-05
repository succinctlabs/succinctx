pub fn assert_bytes_equal(a: &[u8], b: &[u8]) {
    assert!(a.len() == b.len());
    for i in 0..a.len() {
        assert!(a[i] == b[i]);
    }
}

pub fn print_vec(a: &[u8]) -> String {
    let bytes = Bytes::from_iter(a.iter());
    bytes.to_string()
}

pub fn print_vecs(a: Vec<Vec<u8>>) {
    for i in 0..a.len() {
        println!("{} {:?}", i, print_vec(&a[i]));
    }
}

fn to_nibbles(data: &[u8]) -> Vec<u8> {
    let mut nibbles = Vec::with_capacity(data.len() * 2);
    for byte in data {
        // High nibble (upper 4 bits)
        nibbles.push(byte >> 4);
        // Low nibble (lower 4 bits)
        nibbles.push(byte & 0x0F);
    }
    nibbles
}

pub fn to_sized_nibbles(bytes: [u8; 32]) -> [u8; 64] {
    let mut nibbles = [0u8; 64];
    let mut i = 0;
    for byte in bytes {
        nibbles[i] = byte >> 4;
        nibbles[i + 1] = byte & 0x0F;
        i += 2;
    }
    nibbles
}

pub fn to_nibbles<const N: usize>(bytes: [u8; N]) -> [u8; 2 * N] {
    let mut nibbles = [0u8; 2 * N];
    let mut i = 0;
    for byte in bytes {
        nibbles[i] = byte >> 4;
        nibbles[i + 1] = byte & 0x0F;
        i += 2;
    }
    nibbles
}

pub fn is_bytes32_eq(a: [u8; 32], b: [u8; 32]) -> u32 {
    for i in 0..32 {
        if a[i] != b[i] {
            return 0;
        }
    }
    return 1;
}

// TODO: have to implement the constrained version of this
pub fn keccack_variable<const M: usize>(input: [u8; M], len: u32) -> [u8; 32] {
    return keccak256(&input[..len as usize]);
}

pub fn is_eq(a: u8, b: usize) -> u32 {
    if a == b as u8 {
        return 1;
    } else {
        return 0;
    }
}

pub fn mux<const N: usize>(a: [u8; N], sel: u8) -> u8 {
    return a[sel as usize];
}

pub fn mux_nested<const N: usize, const M: usize>(a: [[u8; N]; M], sel: u8) -> [u8; N] {
    return a[sel as usize];
}

// Below everything would be implemented as constraints on the builder

pub fn is_leq(x: u32, y: u32) -> u32 {
    if x <= y {
        return 1;
    } else {
        return 0;
    }
}

pub fn is_le(x: u32, y: u32) -> u32 {
    if x < y {
        return 1;
    } else {
        return 0;
    }
}
