// Much of this file is copied from Jump Crypto's plonky2-crypto repo:
// https://github.com/JumpCrypto/plonky2-crypto/blob/main/src/hash/keccak256.rs
use array_macro::array;
use num::BigUint;
use plonky2::field::extension::Extendable;
use plonky2::field::types::PrimeField64;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::BoolTarget;

use crate::frontend::num::biguint::{BigUintTarget, CircuitBuilderBiguint, WitnessBigUint};
use crate::frontend::num::u32::gadgets::arithmetic_u32::{CircuitBuilderU32, U32Target};
use crate::frontend::num::u32::gadgets::interleaved_u32::CircuitBuilderB32;
use crate::frontend::vars::{ByteVariable, Bytes32Variable, CircuitVariable};
use crate::prelude::{BoolVariable, BytesVariable, CircuitBuilder};

const KECCAK256_C: usize = 1600;
pub const KECCAK256_R: usize = 1088;

#[derive(Clone, Debug)]
pub struct HashInputTarget {
    pub input: BigUintTarget,
    pub blocks: Vec<BoolTarget>,
}

pub trait WitnessKeccakHandler<F:PrimeField64>:WitnessBigUint<F> {
    fn set_keccak256_input_target(&mut self, target: &HashInputTarget, input: &[u8]);
    fn set_hash_blocks_target(&mut self, target: &[BoolTarget], num_blocks: usize);
}

impl<T: WitnessBigUint<F>, F: PrimeField64> WitnessKeccakHandler<F> for T {
    fn set_keccak256_input_target(&mut self, target: &HashInputTarget, input: &[u8]) {
        let mut input_biguint: BigUint = BigUint::from_bytes_le(input);
        let input_len_bits = input.len() * 8;
        let num_actual_blocks = 1 + input_len_bits / KECCAK256_R;
        let padded_len_bits = num_actual_blocks * KECCAK256_R;

        // Padding is of form [1,0,0,..,0,0,1]
        input_biguint.set_bit(input_len_bits as u64, true);

        input_biguint.set_bit(padded_len_bits as u64 - 1, true);

        self.set_biguint_target(&target.input, &input_biguint);
        self.set_hash_blocks_target(&target.blocks, num_actual_blocks);
   }

   fn set_hash_blocks_target(&mut self, blocks: &[BoolTarget], num_blocks: usize) {
        for (i, t) in blocks.iter().enumerate() {
            self.set_bool_target(*t, i < num_blocks - 1);
        }
    }
}


fn biguint_from_le_bytes<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    bits: Vec<BoolTarget>,
) -> BigUintTarget {
    assert!(bits.len() % 32 == 0);

    let mut u32_targets = Vec::new();
    for u32_chunk in bits.chunks(32) {

        u32_targets.push(U32Target(builder.api.le_sum(u32_chunk.iter())));
    }

    BigUintTarget { limbs: u32_targets }
}

#[rustfmt::skip]
pub const KECCAKF_ROTC: [u8; 24] = [
    1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 2, 14, 27, 41, 56, 8, 25, 43, 62,
    18, 39, 61, 20, 44
];

#[rustfmt::skip]
pub const KECCAKF_PILN: [usize; 24] = [
    10, 7, 11, 17, 18, 3, 5, 16, 8, 21, 24, 4, 15, 23, 19, 13, 12, 2, 20,
    14, 22, 9, 6, 1
];

#[rustfmt::skip]
pub const KECCAKF_RNDC: [[u32; 2]; 24] = [
    [0x00000001, 0x00000000], [0x00008082, 0x00000000],
    [0x0000808A, 0x80000000], [0x80008000, 0x80000000],
    [0x0000808B, 0x00000000], [0x80000001, 0x00000000],
    [0x80008081, 0x80000000], [0x00008009, 0x80000000],
    [0x0000008A, 0x00000000], [0x00000088, 0x00000000],
    [0x80008009, 0x00000000], [0x8000000A, 0x00000000],
    [0x8000808B, 0x00000000], [0x0000008B, 0x80000000],
    [0x00008089, 0x80000000], [0x00008003, 0x80000000],
    [0x00008002, 0x80000000], [0x00000080, 0x80000000],
    [0x0000800A, 0x00000000], [0x8000000A, 0x80000000],
    [0x80008081, 0x80000000], [0x00008080, 0x80000000],
    [0x80000001, 0x00000000], [0x80008008, 0x80000000],
];

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {

    fn _hash_keccak256_f1600(&mut self, s: &mut [[U32Target; 2]; 25]) {
        let zero = self.api.zero_u32();
        let mut bc = [[zero; 2]; 5];

        let mut keccakf_rndc = Vec::new();
        for item in &KECCAKF_RNDC {
            keccakf_rndc.push([
                self.api.constant_u32(item[0]),
                self.api.constant_u32(item[1]),
            ]);
        }

        // for round in 0..24 {
        for rndc in keccakf_rndc.iter().take(24) {
            // Theta
            for i in 0..5 {
                bc[i] = self.api.unsafe_xor_many_u64(&[
                    s[i],
                    s[i + 5],
                    s[i + 10],
                    s[i + 15],
                    s[i + 20],
                ]);
            }

            for i in 0..5 {
                let t1 = self.api.lrot_u64(&bc[(i + 1) % 5], 1);
                let t2 = self.api.xor_u64(&bc[(i + 4) % 5], &t1);
                for j in 0..5 {
                    s[5 * j + i] = self.api.xor_u64(&s[5 * j + i], &t2);
                }
            }

            // Rho Pi
            let mut t = s[1];
            for i in 0..24 {
                let j = KECCAKF_PILN[i];
                let tmp = s[j];
                s[j] = self.api.lrot_u64(&t, KECCAKF_ROTC[i]);
                t = tmp;
            }

            // Chi
            for j in 0..5 {
                for i in 0..5 {
                    bc[i] = s[5 * j + i];
                }
                for i in 0..5 {
                    let t1 = self.api.not_u64(&bc[(i + 1) % 5]);
                    let t2 = self.api.and_u64(&bc[(i + 2) % 5], &t1);
                    s[5 * j + i] = self.api.xor_u64(&s[5 * j + i], &t2);
                }
            }

            // Iota
            s[0] = self.api.xor_u64(&s[0], rndc);
        }
    }

    pub fn keccak256(&mut self, input: &[ByteVariable]) -> Bytes32Variable {
        let hash_input_target = self.get_processed_inputs(input);

        let chunks_len = KECCAK256_R / 64;
        let zero = self.api.zero_u32();
        let mut state = [[zero; 2]; KECCAK256_C / 64];
        let mut next_state = [[zero; 2]; KECCAK256_C / 64];

        // Sponge function: absorb & squeeze. 
        // Absorb:
        // First block. xor = use input as initial state.
        for (i, s) in state.iter_mut().enumerate().take(chunks_len) {
            s[0] = hash_input_target.input.limbs[2 * i];
            s[1] = hash_input_target.input.limbs[2 * i + 1];
        }
        
        //Permute.
        self._hash_keccak256_f1600(&mut state);

        // Other blocks.
        for (k, target_block) in hash_input_target.blocks.iter().enumerate() {
            // Xor.
            let input_start = (k + 1) * chunks_len * 2;
            for (i, s) in state.iter().enumerate() {
                if i < chunks_len {
                    next_state[i][0] = self
                        .api
                        .xor_u32(s[0], hash_input_target.input.limbs[input_start + i * 2]);
                    next_state[i][1] = self
                        .api
                        .xor_u32(s[1], hash_input_target.input.limbs[input_start + i * 2 + 1]);
                } else {
                    next_state[i][0] = s[0];
                    next_state[i][1] = s[1];
                }
            }
            //Permute.
            self._hash_keccak256_f1600(&mut next_state);

            // Conditionally set old or new state, depending if block needs to be processed
            // basically a MUX wiring the state.
            for (i, s) in next_state.iter().enumerate() {
                state[i] =  self.api.conditional_u64(s, &state[i], *target_block);
            }
        }

        let output: BigUintTarget = self.api.add_virtual_biguint_target(8);

        // Absorb, or squueze.
        let output_len: usize = output.num_limbs();
        for (i, s) in state.iter().enumerate().take(output_len / 2) {
            self.api.connect_u32(s[0], output.limbs[2 * i]);
            self.api.connect_u32(s[1], output.limbs[2 * i + 1]);
        }

        assert!(output.num_limbs() == 8);

        // Convert the bigint's u32 limbs into 8 bytes.
        let hash_bytes_vec = output
        .limbs
        .iter()
        .flat_map(|chunk| {
            let bit_list = self.api.split_le(chunk.0, 32);

            let hash_byte_vec = bit_list
            .chunks(8)
            .map(|chunk| ByteVariable(array![i => BoolVariable::from(chunk[7 - i].target); 8])) // Reverse bits. Underlying ByteVariable is of Big-Endian format.
            .collect::<Vec<_>>();

            hash_byte_vec
        })
        .collect::<Vec<_>>();

        let mut hash_bytes_array = [ByteVariable::init(self); 32];
        hash_bytes_array.copy_from_slice(&hash_bytes_vec);

        Bytes32Variable(BytesVariable(hash_bytes_array))

    }

    pub fn get_processed_inputs(&mut self, input: &[ByteVariable]) -> HashInputTarget {
        let input_len_bits = input.len() * 8;

        let num_actual_blocks = 1 + input_len_bits / KECCAK256_R;


        //Seeing assert being used in some circuits. Shouldn't the below be constrained instead of asserted?
        assert!(num_actual_blocks < 5);

        let padded_len_bits = num_actual_blocks * KECCAK256_R;

        let mut blocks: Vec<BoolTarget> = Vec::new();
        for _ in 0..(num_actual_blocks - 1) {
            blocks.push(self.api.add_virtual_bool_target_unsafe());
        }

        let bool_targets = self.get_padded_bool_array(input, padded_len_bits);
        
        let input_biguint = biguint_from_le_bytes(self, bool_targets);

        HashInputTarget {
            input: input_biguint,
            blocks
        }


    }

    fn get_padded_bool_array(&mut self, input: &[ByteVariable], padded_len_bits: usize) -> Vec<BoolTarget> {
        //Convert to bits, of little-endian format.: ByteVariable being represented as BE.
        let mut bits = Vec::new();
        for byte in input.iter() {
            bits.extend(byte.to_le_bits());
        }

        // Add the padding of form [1,0,0,..,0,0,1].
        bits.push(BoolVariable::constant(self, true));
        for _ in bits.len()..padded_len_bits - 1 {
            bits.push(BoolVariable::constant(self, false));
        }
        bits.push(BoolVariable::constant(self, true));

        bits
            .into_iter()
            .map(|b| BoolTarget::new_unsafe(b.0 .0))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig, KeccakGoldilocksConfig};
    use plonky2::iop::witness::{PartialWitness};
    use hex::decode;

    use super::*;
    use crate::prelude::CircuitBuilder;

    use crate::utils::{bytes32, setup_logger};

    use log::debug;

    //Circuit logic replication.
    fn bytes_to_bool_vec(bytes: &[u8]) -> Vec<bool> {
        let mut bits = Vec::new();
        for &byte in bytes {
            for i in 0..8 {
                bits.push((byte >> i) & 1 == 1);
            }
        }
        bits
    }

    fn biguint_from_le_bytes_test(bits: Vec<bool>) -> Vec<u32> {
        assert!(bits.len() % 32 == 0);
    
        // Convert to chunks of u32 in little-endian
        let mut u32_values = Vec::new();
        for chunk in bits.chunks(32) {
            let mut value: u32 = 0;
            for (index, bit) in chunk.iter().enumerate() {
                if *bit {
                    value |= 1 << index;
                }
            }
            u32_values.push(value);
        }
    
        u32_values
    }

    fn vec_u32_to_string_base10(u32_values: &[u32]) -> String {
        let mut result: u128 = 0; // Using u128 to handle overflow
        for &value in u32_values.iter().rev() { // Assuming little-endian
            result = (result << 32) | value as u128;
        }
        result.to_string()
    }

    #[test]
    fn test_keccak256_empty() {
        setup_logger();

        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let word_byte = b"";

        let word = [ByteVariable::init(&mut builder); 0]; 

        let str_repr = "0";

        let input_biguint_lib: BigUint = BigUint::from_bytes_le(word_byte);

        debug!("biguint_test : {:?}", str_repr);
        debug!("input_biguint : {:?}", input_biguint_lib);

        let hash_input_target = builder.get_processed_inputs(&word);
        let digest_target = builder.keccak256(&word);
        
        builder.watch(&digest_target, "digest_target");

        let digest = builder.constant::<Bytes32Variable>(bytes32!(
            "0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"
        ));

        builder.assert_is_equal(digest_target, digest);

        let circuit = builder.build::<C>();
        let input = circuit.input();
        let (_, _) = circuit.prove(&input);

        // test circuit
        let mut pw: PartialWitness<GoldilocksField> = PartialWitness::new();

        pw.set_keccak256_input_target(&hash_input_target, word_byte);

        let proof= circuit.data.prove(pw).unwrap();
        assert!(circuit.data.verify(proof).is_ok());
    }
   
    #[test]
    fn test_keccak256_short() {
        setup_logger();

        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let word_byte = bytes32!(
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        );
        
        let word= builder.constant::<Bytes32Variable>(word_byte);

        let bool_vec = bytes_to_bool_vec(word_byte.as_bytes());
        let biguint_test = biguint_from_le_bytes_test(bool_vec);
        let str_repr = vec_u32_to_string_base10(&biguint_test);

        let input_biguint_lib: BigUint = BigUint::from_bytes_le(word_byte.as_bytes());

        debug!("biguint_test : {:?}", str_repr);
        debug!("input_biguint : {:?}", input_biguint_lib);

        let hash_input_target = builder.get_processed_inputs(&word.0 .0);
        let digest_target = builder.keccak256(&word.0 .0);

        builder.watch(&digest_target, "digest_target");

        let digest = builder.constant::<Bytes32Variable>(bytes32!(
            "0x290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563"
        ));

        builder.assert_is_equal(digest_target, digest);

        let circuit = builder.build::<C>();
        let input = circuit.input();
        let (_, _) = circuit.prove(&input);

        // test circuit
        let mut pw: PartialWitness<GoldilocksField> = PartialWitness::new();

        pw.set_keccak256_input_target(&hash_input_target, word_byte.as_bytes());

        let proof= circuit.data.prove(pw).unwrap();
        assert!(circuit.data.verify(proof).is_ok());
    }

    #[test]
    fn test_keccak256_long() {
        setup_logger();

        type F = <C as GenericConfig<D>>::F;
        type C = KeccakGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let msg = decode("f9015180a060f3bdb593359880987612345e1234509871234509876123049518762a705ff4581e01928374657483921029384756478010192837465b99537f24e8a00e7a0006f459182f07dba16a06776a7e6abd64250488ed106c0fbd66ee338b7ce59ae967714ce43ecd5a3de97a0f8d6740520928d0e540bf439f1c214ce434f349e4c9b71bb9fcce14144a48914a0f31b2b9570033a103b8a4c0db8debbff2cf8dc4eb2ed31fa292d41c7adf13dc980808080a016a530127910d9d4a89450f0c9dc075545441126b222396eb28e30c73c01c8a9a05d9eb59dae800d3f8cfe8efdfa86776fc7f3b09dfc5b2f537b2c2abda9787755a0bcdc8744035201f5d8d9bd0f440887a40d8cafc3f986f20ce").unwrap();

        let targets = msg
        .iter()
        .map(|&byte| builder.constant::<ByteVariable>(byte))
        .collect::<Vec<_>>();

        let hash_input_target = builder.get_processed_inputs(&targets[..]);
        let digest_target = builder.keccak256(&targets[..]);
        let num_gates = builder.api.num_gates();

        builder.watch(&digest_target, "digest_target");

        debug!("num_gates: {:?}", num_gates);

        let digest = builder.constant::<Bytes32Variable>(bytes32!(
            "0xef0a5c7a1dfaa6a396e9163c4849e53f5d0b919f6de0b923cf41d321eedec03c"
        ));

        builder.assert_is_equal(digest_target, digest);

        debug!("msg.as slice: {:?}", msg.as_slice().len());

        let circuit = builder.build::<C>();

        // test circuit
        let mut pw= PartialWitness::new();

        pw.set_keccak256_input_target(&hash_input_target, msg.as_slice());

        let proof= circuit.data.prove(pw).unwrap();
        assert!(circuit.data.verify(proof).is_ok());
    }

    #[test]
    #[should_panic]
    fn test_keccak256_failure() {
        setup_logger();

        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let word_byte = bytes32!(
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        );

        let word = builder.constant::<Bytes32Variable>(word_byte);
  
        let bool_vec = bytes_to_bool_vec(word_byte.as_bytes());
        let biguint_test = biguint_from_le_bytes_test(bool_vec);
        let str_repr = vec_u32_to_string_base10(&biguint_test);

        let input_biguint_lib: BigUint = BigUint::from_bytes_le(word_byte.as_bytes());

        debug!("biguint_test : {:?}", str_repr);
        debug!("input_biguint : {:?}", input_biguint_lib);

        let hash_input_target = builder.get_processed_inputs(&word.0 .0);
        let digest_target = builder.keccak256(&word.0 .0);

        let digest = builder.constant::<Bytes32Variable>(bytes32!(
            "0x290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e564"
        ));

        builder.watch(&digest_target, "digest_target");
        builder.assert_is_equal(digest_target, digest);

        let circuit = builder.build::<C>();
        let input = circuit.input();
        let (_, _) = circuit.prove(&input);

        // test circuit
        let mut pw: PartialWitness<GoldilocksField> = PartialWitness::new();
        pw.set_keccak256_input_target(&hash_input_target, word_byte.as_bytes());

        let proof= circuit.data.prove(pw).unwrap();
        assert!(circuit.data.verify(proof).is_ok());
    }
}