// Note this only decodes bytes and doesn't support long strings
fn rlp_decode_bytes(input: &[u8]) -> (Vec<u8>, usize) {
    let prefix = input[0];
    if prefix <= 0x7F {
        return (vec![prefix], 1);
    } else if prefix == 0x80 {
        return (vec![], 1); // null value
    } else if prefix <= 0xB7 {
        // Short string (0-55 bytes length)
        let length = (prefix - 0x80) as usize;
        let res = &input[1..1 + length];
        return (res.into(), 1 + length);
    } else if prefix <= 0xBF {
        panic!("Long string (56+ bytes length) not supported in rlp_decode_bytes")
    } else {
        panic!("Invalid prefix rlp_decode_bytes")
    }
}

pub fn rlp_decode_list_2_or_17(input: &[u8]) -> Vec<Vec<u8>> {
    let prefix = input[0];
    // println!("input {:?}", Bytes::from(input.to_vec()).to_string());
    if prefix <= 0xF7 {
        // Short list (0-55 bytes total payload)
        let list_length = (prefix - 0xC0) as usize;
        // We assert that the input is simply [list_length, list_content...] and not suffixed by anything else
        assert!(input.len() == 1 + list_length);
        let (ele_1, increment) = rlp_decode_bytes(&input[1..]);
        let (ele_2, _) = rlp_decode_bytes(&input[1 + increment..]);
        return vec![ele_1, ele_2];
    } else {
        // TODO check that prefix is bounded within a certain range
        let len_of_list_length = prefix - 0xF7;
        // println!("len_of_list_length {:?}", len_of_list_length);
        // TODO: figure out what to do with len_of_list_length
        let mut pos = 1 + len_of_list_length as usize;
        let mut res = vec![];
        for _ in 0..17 {
            let (ele, increment) = rlp_decode_bytes(&input[pos..]);
            res.push(ele);
            pos += increment;
            // println!("ele {:?}", Bytes::from(ele.clone()).to_string());
            // println!("increment {:?}", increment);
            if pos == input.len() {
                break;
            }
        }
        assert!(pos == input.len()); // Check that we have iterated through all the input
        assert!(res.len() == 17 || res.len() == 2);
        return res;
    }
}

// This is simply for getting witness, we return the decoded list, the lengths of the elements in the decoded list and also the list length
pub fn witness_decoding<const M: usize, const L: usize>(
    encoded: [u8; M],
    len: u32,
    finish: u32,
) -> ([[u8; 34]; L], [u8; L], u8) {
    let mut decoded_list_as_fixed = [[0u8; 34]; L];
    let mut decoded_list_lens = [0u8; L];
    let mut decoded_list_len = 0;
    if finish == 1 {
        // terminate early
        return (decoded_list_as_fixed, decoded_list_lens, decoded_list_len);
    }
    let decoded_element = rlp_decode_list_2_or_17(&encoded[..len as usize]);
    for (i, element) in decoded_element.iter().enumerate() {
        let len: usize = element.len();
        assert!(len <= 34, "The decoded element should have length <= 34!");
        decoded_list_as_fixed[i][..len].copy_from_slice(&element);
        decoded_list_lens[i] = len as u8;
    }
    return (
        decoded_list_as_fixed,
        decoded_list_lens,
        decoded_element.len() as u8,
    );
}

fn parse_list_element(element: [u8; 32], len: u8) -> (u32, u32) {
    let prefix = element[0];
    if len == 0 {
        return (0x80, 0);
    } else if len == 1 && prefix <= 0x7F {
        return (prefix as u32, 0);
    } else if len == 1 && prefix > 0x7F {
        // TODO: maybe this is the same as the below case
        return (0x80 + 0x01, 1);
    } else if len <= 55 {
        return (len as u32 + 0x80 as u32, len as u32);
    } else {
        panic!("Invalid length and prefix combo {} {}", len, prefix)
    }
}

// This is the vanilla implementation of the RLC trick for verifying the decoded_list
pub fn verify_decoded_list<const L: usize, const M: usize>(
    list: [[u8; 32]; L],
    lens: [u8; L],
    encoding: [u8; M],
) {
    let random = 1000.to_bigint().unwrap();

    let mut size_accumulator: u32 = 0;
    let mut claim_poly = BigInt::default();
    for i in 0..L {
        let (mut start_byte, list_len) = parse_list_element(list[i], lens[i]);
        let mut poly = start_byte.to_bigint().unwrap() * random.pow(size_accumulator);
        for j in 0..32 {
            poly += list[i][j] as u32
                * (random.pow(1 + size_accumulator + j as u32))
                * is_leq(j as u32, list_len);
        }
        size_accumulator += 1 + list_len;
        claim_poly += poly;
    }

    let mut encoding_poly = BigInt::default();
    for i in 3..M {
        // TODO: don't hardcode 3 here
        let idx = i - 3;
        encoding_poly +=
            encoding[i] as u32 * (random.pow(idx as u32)) * is_le(idx as u32, size_accumulator);
    }

    assert!(claim_poly == encoding_poly);
}
