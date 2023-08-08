pub fn le_bits_to_bytes<const N: usize>(input: [bool; N]) -> [u8; N / 8] {
    let mut output = [0; N / 8];
    for i in 0..N {
        for j in 0..8 {
            if input[i * 8 + j] {
                output[i] |= 1 << j;
            }
        }
    }
    output
}

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

pub fn bytes_to_bits<const N: usize>(input: [u8; N / 8]) -> [bool; N] {
    let mut output = [false; N];

    for (i, &byte) in input.iter().enumerate() {
        for j in 0..8 {
            output[i * 8 + j] = (byte & (1 << j)) != 0;
        }
    }

    output
}