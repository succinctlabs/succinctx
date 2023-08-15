use crate::{builder::BuilderAPI, vars::ByteVariable};

pub struct RecursiveLengthPrefixAPI {
    pub api: BuilderAPI,
}

impl RecursiveLengthPrefixAPI {
    pub fn new(api: BuilderAPI) -> Self {
        Self { api }
    }

    pub fn constrain_rlp_encoding_bytes(self, claimed_bytes: Vec<ByteVariable>, encoding: Vec<ByteVariable>, len_claimed_bytes: usize) {
        // https://github.com/ethereum-optimism/optimism/blob/6e041bcd9d678a0ea2bb92cfddf9716f8ae2336c/packages/contracts-bedrock/src/libraries/rlp/RLPWriter.sol#L13
        assert!(claimed_bytes.len() == len_claimed_bytes);
        if len_claimed_bytes == 1 {  
            assert!(encoding.len() == 1);
            self.api.assert_equal(claimed_bytes[0], encoding[0]);
            self.api.assert_le(claimed_bytes[0], ByteVariable.constant(128));
        } else {
            if len_claimed_bytes < 56 {
                assert!(encoding.len() == 1 + len_claimed_bytes);
                self.api.assert_equal(encoding[0], len_claimed_bytes + 128);
                for i in 0..len_claimed_bytes {
                    self.api.assert_equal(claimed_bytes[i], encoding[i+1])
                }
            } else {
                self.api.assert_ge(encoding[0], 128+55);
                let x = self.api.sub(encoding[0], 128 + 55);
                let claimed_x = log_256(len_claimed_bytes);
                self.api.assert_is_equal(x, claimed_x);
                for i in 0..claimed_x {
                    self.api.assert_equal(claimed_bytes[i], encoding[i + claimed_x + 1])
                }
                assert!(encoding.len() == 1 + claimed_x + len_claimed_bytes);
            }
    
        }
    }
}




#[cfg(test)]
mod tests {
    use core::ops::Add;

    use anyhow::Result;
    use ethers::types::{Address, H256, U256};
    use plonky2::iop::witness::{PartialWitness, WitnessWrite};
    use plonky2::plonk::circuit_builder::CircuitBuilder;
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
    use subtle_encoding::hex::decode;
    use ethers::utils::rlp::{RlpStream};

    use crate::eth::utils::{u256_to_h256_be, h256_to_u256_be};

    use super::*;

    #[test]
    fn test_rlp_decode() {
        let mut stream = RlpStream::new();
        stream.begin_list(1);
        let h256 =
            "0xff90251f501c864f21d696c811af4c3aa987006916bd0e31a6c06cc612e7632e"
                .parse::<H256>()
                .unwrap();
        let u256 = h256_to_u256_be(h256);
        stream.append::<U256>(&u256.into());
        let encoding = stream.out().freeze();
        println!("encoding {:x?}", encoding);


    }
}
