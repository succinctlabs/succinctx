pub macro bytes32($hex_literal:expr) {
    $hex_literal.parse::<ethers::types::H256>().unwrap()
}

pub macro address($hex_literal:expr) {
    $hex_literal.parse::<ethers::types::Address>().unwrap()
}

// pub fn le_bits_to_bytes<const N: usize>(input: [bool; N*8]) -> [u8; N] {
//     let mut output = [0; N];
//     for i in 0..N {
//         for j in 0..8 {
//             if input[i * 8 + j] {
//                 output[i] |= 1 << j;
//             }
//         }
//     }
//     output
// }

pub fn bits_to_bytes<const N: usize>(input: [bool; N]) -> [u8; N / 8] {
    let mut output = [0u8; N / 8];

    for (i, chunk) in input.chunks(8).enumerate() {
        for (j, &bit) in chunk.iter().enumerate() {
            if bit {
                output[i] |= 1 << j;
            }
        }
    }

    output
}

pub fn byte_to_bits_le(input: u8) -> [bool; 8] {
    let mut bits = [false; 8];
    for i in 0..8 {
        bits[i] = (input & (1 << i)) != 0;
    }
    bits
}

pub fn byte_to_bits_be(input: u8) -> [bool; 8] {
    let mut bits = [false; 8];
    for i in 0..8 {
        bits[7 - i] = (input & (1 << i)) != 0;
    }
    bits
}

pub fn le_bits_to_bytes<const N: usize>(input: &[bool]) -> [u8; N] {
    let mut output = [0; N];
    for i in 0..N {
        for j in 0..8 {
            if input[i * 8 + j] {
                output[i] |= 1 << j;
            }
        }
    }
    output
}
