use std::arch::is_aarch64_feature_detected;
use std::thread::current;

use curta::chip::uint::operations::add::ByteArrayAdd;
use curta::math::field::Field;
use curta::math::prelude::PrimeField64;
use itertools::Itertools;
use plonky2::hash::poseidon::PoseidonHash;
use plonky2::iop::challenger::RecursiveChallenger;
use serde::{Deserialize, Serialize};

use super::utils::{decode_padded_mpt_node, MPTNodeFixedSize};
use crate::frontend::eth::rlp::utils::{MAX_MPT_NODE_SIZE, MAX_RLP_ITEM_SIZE};
use crate::frontend::hint::simple::hint::Hint;
use crate::prelude::{
    ArrayVariable, BoolVariable, ByteVariable, CircuitBuilder, CircuitVariable, PlonkParameters,
    RichField, U32Variable, ValueStream, Variable, VariableStream,
};

#[derive(Clone, Debug, CircuitVariable)]
#[value_name(MPTValueType)]
pub struct MPTVariable {
    pub data: ArrayVariable<ArrayVariable<ByteVariable, MAX_RLP_ITEM_SIZE>, MAX_MPT_NODE_SIZE>,
    pub lens: ArrayVariable<Variable, MAX_MPT_NODE_SIZE>,
    pub len: Variable,
}

impl MPTNodeFixedSize {
    /// Convert `MPTNodeFixedSize` into `MPTValueType<F>`
    fn to_value_type<F: RichField>(&self) -> MPTValueType<F> {
        MPTValueType::<F> {
            data: self.data.iter().map(|x| x.data.to_vec()).collect_vec(),
            lens: self
                .data
                .iter()
                .map(|x| F::from_canonical_usize(x.len))
                .collect_vec(),
            len: F::from_canonical_usize(self.len),
        }
    }
}

/// A Hint structure to decode an RLP-encoded string.
///
/// The RLP-encoded string is expected to be padded to a fixed size. The fixed size should equal
/// `ENCODING_LEN` and is specified as a type parameter. The "true" length of the encoding is given
/// in the stream. `LIST_LEN` specifies the node size. (e.g., 2 for extension/leaf nodes, 17 for
/// branch nodes.) The decoded string is returned as a padded 2-dimensional byte array
/// (`MAX_RLP_ITEM_SIZE` x `LIST_LEN`).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DecodeHint<const ENCODING_LEN: usize> {}
impl<L: PlonkParameters<D>, const D: usize, const ENCODING_LEN: usize> Hint<L, D>
    for DecodeHint<ENCODING_LEN>
{
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let encoded = input_stream.read_value::<ArrayVariable<ByteVariable, ENCODING_LEN>>();
        let len = input_stream.read_value::<Variable>();
        let skip_computation = input_stream.read_value::<BoolVariable>();

        let decoded =
            decode_padded_mpt_node(&encoded, len.as_canonical_u64() as usize, skip_computation);

        output_stream.write_value::<MPTVariable>(decoded.to_value_type());
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Helper function. Equivalent to `(a < b) ? c : d`.
    fn if_less_than_else(
        &mut self,
        a: Variable,
        b: Variable,
        c: Variable,
        d: Variable,
    ) -> Variable {
        let a_lt_b = self.lt(a, b);
        self.select(a_lt_b, c, d)
    }

    /// Evaluate the polynomial constructed from seeing RLP-encode(byte_array) as a vector of
    /// coefficients and x = pow.
    ///
    /// Mathematically speaking, we define a function `f(E : RLP-encoding) -> F` such that
    /// `f(E) = \sigma_{i = 0}^{len(E) - 1} [byte_to_field_element(coefficients[i]) * pow^(i)]`.
    ///
    /// This function returns:
    /// 1. `f(RLP-encoding(E))`,
    /// 2. `pow^len(RLP-encoding(E))`, which can be seen as the "next" power,
    /// 3. `len(RLP-encoding(coefficients))`, which is necessary for calculating the prefix byte for
    ///    `RLP-encoding(mpt)`.
    ///
    /// Note that, as specified in the function name, we don't actually calculate
    /// `RLP-encoding(byte_array)`.
    fn calculate_polynomial_emulating_rlp_encoding<const ARRAY_LENGTH: usize>(
        &mut self,
        byte_array: &ArrayVariable<ByteVariable, ARRAY_LENGTH>,
        len: Variable,
        pow: Variable,
    ) -> (Variable, Variable, Variable) {
        let true_v = self.constant::<BoolVariable>(true);
        let zero = self.zero();
        let one = self.one();
        let cons55 = self.constant::<Variable>(L::Field::from_canonical_u64(55));

        // TODO: It's likely that we'll need to implement the case when the given byte string is
        // >= 56 bytes. (e.g., account state) However, for the first iteration, we will only worry
        // about the case when the byte string is <= 55 bytes.
        let len_lte_55 = self.lte(len, cons55);
        self.assert_is_equal(len_lte_55, true_v);

        // There are 2 possible outcomes of encode(byte_array):
        //
        // 1. len = 1 && byte_array[0] <  0x80  =>  {byte_array[0]}
        // 2. else                              =>  {0x80 + len, byte_array[0], byte_array[1], ...}

        let cons0x80 = self.constant::<Variable>(L::Field::from_canonical_u64(0x80));

        let first_byte_as_variable = self.byte_to_variable(byte_array[0]);
        let res_case_1 = first_byte_as_variable;
        let len1 = one;

        let mut res_case_2 = self.add(cons0x80, len);
        let len2 = self.add(len, one);

        let mut next_pow = pow;
        for i in 0..ARRAY_LENGTH {
            let index = self.constant::<Variable>(L::Field::from_canonical_usize(i));
            let mut current_term = self.byte_to_variable(byte_array[i]);
            current_term = self.mul(current_term, next_pow);
            current_term = self.if_less_than_else(index, len, current_term, zero);

            res_case_2 = self.add(res_case_2, current_term);
            let pow_multiplier = self.if_less_than_else(index, len, pow, one);
            next_pow = self.mul(next_pow, pow_multiplier);
        }

        let is_len_1 = self.is_equal(len, one);
        let is_first_variable_less_than_0x80 = self.lt(first_byte_as_variable, cons0x80);
        let is_case_1 = self.and(is_len_1, is_first_variable_less_than_0x80);

        let res_len = self.select(is_case_1, len1, len2);
        let res_pow = self.select(is_case_1, pow, next_pow);
        let res = self.select(is_case_1, res_case_1, res_case_2);

        (res, res_pow, res_len)
    }

    /// Add in the term corresponding to the prefix byte(s) of the RLP-encoding, given the sum of
    /// the item-wise polynomial.
    ///
    /// The RLP-encoding is in the form of `{ prefix, prefix_0, byte_array_0, prefix_1,
    /// byte_array_1, ... }`. And so far, we have calculated the polynomial for {prefix_0,
    /// byte_array_0, prefix_1, byte_array_1, ...}. Now we have to calculate `prefix`, and
    /// also "shift" the current polynomial. This logic isn't necessarily reusable or modular, but
    /// it's complex logic that blurs out the whole `verify_decoded_mpt_node` function, so this is
    /// a separate function.
    fn add_prefix_polynomial_and_shift(
        &mut self,
        sum_of_rlp_encoding_length: Variable,
        claim_poly: Variable,
        challenge: Variable,
    ) -> Variable {
        let true_v = self.constant::<BoolVariable>(true);
        let cons56 = self.constant::<Variable>(L::Field::from_canonical_u64(56));
        let cons256 = self.constant::<Variable>(L::Field::from_canonical_u64(256));
        let cons65536 = self.constant::<Variable>(L::Field::from_canonical_u64(65536));
        let cons0xf8 = self.constant::<Variable>(L::Field::from_canonical_u64(0xf8));

        // Assert that sum_of_rlp_encoding_length is less than 256^2 = 65536 bits. A
        // well-formed MPT should never need that many bytes.
        let valid_length = self.lt(sum_of_rlp_encoding_length, cons65536);
        self.assert_is_equal(true_v, valid_length);

        // The main idea is to convert claim_poly into `prefix_term + [appropriate power of
        // challenger] * claim_poly`. There are three cases that we work on:
        // 1.        combined length <  56    => prefix = {0xc0 + combined length}
        // 2.  56 <= combined length <  256   => prefix = {0xf8, combined length in 1 byte}
        // 3. 256 <= combined length < 65536  => prefix = {0xf9, combined length in 2 bytes}
        //
        // RLP technically allows a longer byte string, but we will not implement it, at least, for
        // now.

        // Case 1: We need 0xc0 + combined_length + claim_poly * challenge.
        let mut case_1 = self.constant::<Variable>(L::Field::from_canonical_u64(0xc0));
        case_1 = self.add(case_1, sum_of_rlp_encoding_length);
        let case_1_poly = self.mul(claim_poly, challenge);
        case_1 = self.add(case_1, case_1_poly);

        // Case 2: We need 0xf8 + combined_length * challenger + claim_poly * (challenge ^ 2).
        let mut case_2 = self.mul(sum_of_rlp_encoding_length, challenge);
        case_2 = self.add(case_2, cons0xf8);
        let mut case_2_poly = self.mul(claim_poly, challenge);
        case_2_poly = self.mul(case_2_poly, challenge);
        case_2 = self.add(case_2, case_2_poly);

        // Case 3
        //
        // Divide sum_of_rlp_encoding_length by cons256 and get the quotient and remainder, and we
        // need 0xf9 + div * challenger + rem * (challenger ^ 2) + claim_poly * (challenger ^ 3).
        let (mut div, mut rem) = self.div_rem_256(sum_of_rlp_encoding_length);
        let mut case_3 = self.constant::<Variable>(L::Field::from_canonical_u64(0xf9));

        div = self.mul(div, challenge);
        case_3 = self.add(case_3, div);

        rem = self.mul(rem, challenge);
        rem = self.mul(rem, challenge);
        case_3 = self.add(case_3, rem);

        let mut case_3_poly = self.mul(claim_poly, challenge);
        case_3_poly = self.mul(case_3_poly, challenge);
        case_3_poly = self.mul(case_3_poly, challenge);
        case_3_poly = self.mul(case_3_poly, challenge);

        case_3 = self.add(case_3, case_3_poly);

        // Pick the right one.
        let mut res = self.if_less_than_else(sum_of_rlp_encoding_length, cons256, case_2, case_3);
        res = self.if_less_than_else(sum_of_rlp_encoding_length, cons56, case_1, case_2);
        res
    }

    /// Given `a`, returns `floor(a / 256)` and `a % 256`.
    ///
    /// This only works if `floor(a / 256)` is `<= 5`. This might seem limiting, but in an MPT the
    /// encoding cannot be that long. A branch node with 16 hashes has 512 bytes, and a leaf node's
    /// path is up to 32 bytes. Even with a pessimistic assumption of having a 1000-byte value in a
    /// leaf node, the encoding is still less than 1280 bytes.
    fn div_rem_256(&mut self, a: Variable) -> (Variable, Variable) {
        let mut rem = a;
        let zero = self.zero();
        let one = self.one();
        let cons256 = self.constant::<Variable>(L::Field::from_canonical_u64(256));
        let mut div = zero;

        for _ in 0..5 {
            let can_still_subtract = self.gte(rem, cons256);
            let subtract = self.select(can_still_subtract, cons256, zero);
            let add = self.select(can_still_subtract, one, zero);

            rem = self.sub(rem, subtract);
            div = self.add(div, add);
        }

        let done = self.lt(rem, cons256);
        let true_v = self.constant::<BoolVariable>(true);
        self.assert_is_equal(done, true_v);

        (div, rem)
    }

    /// This function verifies the decoding by comparing the encoded and decoded MPT node.
    ///
    /// Mathematically speaking, we define a function `f(E : RLP-encoding) -> F` such that
    /// `f(E) = \sigma_{i = 0}^{len(E) - 1} [byte_to_field_element(E[i]) * challenger^i]`.
    ///
    /// `verify_decoded_mpt_node` then verifies that `encoded[..len] = rlp-encode(mpt)` by checking
    /// `f(encoded[..len]) = f(rlp-encode(mpt))`.
    ///
    /// `f(encoded[len])` is straightforward. We calculate `f(rlp-encode(mpt))` without explicitly
    /// encoding mpt by calculating:
    ///
    /// - `f(rlp-encode(i-th item in mpt))`` for all `i = 0..(len(mpt) - 1)``, and
    /// - `f(the prefix byte(s) of rlp-encode(mpt))`,
    ///
    /// and combining them using the appropriate power of `challenger`. Of course, we don't
    /// explicitly calculate `rlp-encode(i-th item in mpt)`, either. Instead, we calculate it by
    /// looking at the length and first byte of the `i-th item in mpt`.
    pub fn verify_decoded_mpt_node<const ENCODING_LEN: usize>(
        &mut self,
        encoded: &ArrayVariable<ByteVariable, ENCODING_LEN>,
        len: Variable,
        skip_computation: BoolVariable,
        seed: &[ByteVariable],
        mpt: &MPTVariable,
    ) {
        // TODO: Seed with 120 bits. Check if this is enough bits of security.
        const MIN_SEED_BITS: usize = 120;

        let mut seed_targets = Vec::new();
        let mut challenger = RecursiveChallenger::<L::Field, PoseidonHash, D>::new(&mut self.api);
        // TODO: Must understand the math behind this, for now, I'm just copying and pasting this from the codebase.
        // Need to get chunks of 7 since the max value of F is slightly less then 64 bits.

        let mut seed_bit_len = 0;
        for seed_chunk in seed.to_vec().chunks(7) {
            let seed_element_bits = seed_chunk
                .iter()
                .flat_map(|x| x.as_bool_targets())
                .collect_vec();
            let seed_element = self.api.le_sum(seed_element_bits.iter());
            seed_bit_len += seed_element_bits.len();
            seed_targets.push(seed_element);
        }

        assert!(seed_bit_len >= MIN_SEED_BITS);

        challenger.observe_elements(seed_targets.as_slice());

        const NUM_LOOPS: usize = 3;

        let challenges = challenger
            .get_n_challenges(&mut self.api, NUM_LOOPS)
            .iter()
            .map(|x| Variable::from(*x))
            .collect_vec();

        let one = self.one();
        let zero = self.zero();
        let cons55 = self.constant::<Variable>(L::Field::from_canonical_u64(55));
        let cons65536 = self.constant::<Variable>(L::Field::from_canonical_u64(65536));
        let true_v = self.constant::<BoolVariable>(true);
        for loop_index in 0..NUM_LOOPS {
            let mut encoding_poly = self.zero::<Variable>();
            let mut pow = self.one();

            for i in 0..ENCODING_LEN {
                let mut current_term = self.byte_to_variable(encoded[i]);
                current_term = self.mul(current_term, pow);
                // It's okay to simply add current_term as pow becomes 0 once i = ENCODING_LEN.
                encoding_poly = self.add(encoding_poly, current_term);

                let index = self.constant::<Variable>(L::Field::from_canonical_usize(i));
                let pow_multiplier =
                    self.if_less_than_else(index, len, challenges[loop_index], zero);
                pow = self.mul(pow, pow_multiplier);
            }

            let mut sum_of_rlp_encoding_length = zero;
            let mut claim_poly = zero;
            pow = one;

            for i in 0..MAX_MPT_NODE_SIZE {
                let index_i = self.constant::<Variable>(L::Field::from_canonical_usize(i));

                let (mut poly, mut next_pow, mut encoding_len) = self
                    .calculate_polynomial_emulating_rlp_encoding(
                        &mpt.data[i],
                        mpt.lens[i],
                        challenges[loop_index],
                    );

                // Shift the `poly` value by the appropriate power of `challenger`, and also check
                // if we should even include this.
                poly = self.if_less_than_else(index_i, mpt.len, poly, zero);
                poly = self.mul(poly, pow);
                claim_poly = self.add(claim_poly, poly);

                next_pow = self.if_less_than_else(index_i, mpt.len, next_pow, one);
                pow = self.mul(pow, next_pow);

                encoding_len = self.if_less_than_else(index_i, mpt.len, encoding_len, zero);
                sum_of_rlp_encoding_length = self.add(sum_of_rlp_encoding_length, encoding_len);
            }
            claim_poly = self.add_prefix_polynomial_and_shift(
                sum_of_rlp_encoding_length,
                claim_poly,
                challenges[loop_index],
            );
            self.watch(&claim_poly, "claim_poly");
            self.watch(&encoding_poly, "encoding_poly");
            let claim_poly_equals_encoding_poly = self.is_equal(claim_poly, encoding_poly);
            let result = self.or(skip_computation, claim_poly_equals_encoding_poly);

            self.assert_is_equal(result, true_v);
        }
    }

    pub fn decode_mpt_node<const ENCODING_LEN: usize, const ELEMENT_LEN: usize>(
        &mut self,
        encoded: ArrayVariable<ByteVariable, ENCODING_LEN>,
        len: Variable,
        skip_computation: BoolVariable,
    ) -> MPTVariable {
        let mut input_stream = VariableStream::new();
        input_stream.write(&encoded);
        input_stream.write(&len);
        input_stream.write(&skip_computation);

        let hint = DecodeHint::<ENCODING_LEN> {};

        let output_stream = self.hint(input_stream, hint);
        let decoded_node = output_stream.read::<MPTVariable>(self);

        let seed: &[ByteVariable] = todo!();

        self.verify_decoded_mpt_node::<ENCODING_LEN>(
            &encoded,
            len,
            skip_computation,
            seed,
            &decoded_node,
        );

        decoded_node
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use rand::rngs::OsRng;
    use rand::Rng;

    use super::*;
    use crate::backend::circuit::DefaultParameters;
    use crate::prelude::{DefaultBuilder, GoldilocksField};
    use crate::utils::{bytes, setup_logger};

    /// Passes `verify_decode_mpt_node` the given rlp-encoded string and their decoded values.
    ///
    /// `fuzzer` modifies the input to `verify_decoded_mpt_node`. If you want to test a "happy path"
    /// , simply set `fuzzer` to the identity function. If `fuzzer` modifies any meaningful value
    /// (i.e., anything other than padded 0's), the test is expected to fail. Use `#[should_panic]`
    /// to tell Rust that it's expected to fail.
    fn test_verify_decoded_mpt_node<const ENCODING_LEN: usize, F>(rlp_encoding: Vec<u8>, fuzzer: F)
    where
        F: Fn([u8; ENCODING_LEN]) -> [u8; ENCODING_LEN],
    {
        setup_logger();

        let mut builder: CircuitBuilder<DefaultParameters, 2> = DefaultBuilder::new();

        type F = GoldilocksField;
        let mut encoding_fixed_size = [0u8; ENCODING_LEN];
        encoding_fixed_size[..rlp_encoding.len()].copy_from_slice(&rlp_encoding);
        let skip_computation = false;

        let mut rng = OsRng;

        let mut seed_input = [0u8; 15];
        for elem in seed_input.iter_mut() {
            *elem = rng.gen();
        }

        let seed = seed_input
            .iter()
            .map(|x| builder.constant::<ByteVariable>(*x))
            .collect_vec();

        let mpt_node =
            decode_padded_mpt_node(&encoding_fixed_size, rlp_encoding.len(), skip_computation);

        let encoded = builder.constant::<ArrayVariable<ByteVariable, ENCODING_LEN>>(
            fuzzer(encoding_fixed_size).to_vec(),
        );
        let len = builder.constant::<Variable>(F::from_canonical_usize(rlp_encoding.len()));
        let skip_computation = builder.constant::<BoolVariable>(false);
        let mpt_node_variable = builder.constant::<MPTVariable>(mpt_node.to_value_type());

        builder.verify_decoded_mpt_node::<ENCODING_LEN>(
            &encoded,
            len,
            skip_computation,
            &seed,
            &mpt_node_variable,
        );
        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }

    #[test]
    fn test_verify_decoded_mpt_node_extension_node() {
        const ENCODING_LEN: usize = 2 * 32 + 20;

        // This is an RLP-encoded list of an extension node. The 00 in 0x006f indicates that the path
        // length is even, and the path is 6 -> f. This extension node points to a leaf node with
        // the hash starting with 0x188d11.  ["0x006f",
        // "0x188d1100731419827900267bf4e6ea6d428fa5a67656e021485d1f6c89e69be6"]
        let rlp_encoding: Vec<u8> =
            bytes!("0xe482006fa0188d1100731419827900267bf4e6ea6d428fa5a67656e021485d1f6c89e69be6");

        test_verify_decoded_mpt_node::<ENCODING_LEN, _>(rlp_encoding, |x| x);
    }

    #[test]
    fn test_verify_decoded_mpt_node_extension_node_mid_length() {
        const ENCODING_LEN: usize = 120;

        // This is an RLP-encoded list of an extension node. Both the first and second elements are
        // 32 bytes. The whole encoding is 68 bytes, and this is suitable for testing a list whose
        // length can be represented in 1 byte.
        let rlp_encoding: Vec<u8> =
            bytes!("0xf842a01111111111111111111111111111111111111111111111111111111111111111a01111111111111111111111111111111111111111111111111111111111111111");

        test_verify_decoded_mpt_node::<ENCODING_LEN, _>(rlp_encoding, |x| x);
    }

    #[test]
    fn test_verify_decoded_mpt_node_branch_node() {
        const ENCODING_LEN: usize = 600;

        // This is a RLP-encoded list of a branch node. It is a list of length 17. Each of the first
        // 16 elements is a 32-byte hash, and the last element is 0.
        let rlp_encoding: Vec<u8>  = bytes!("0xf90211a0215ead887d4da139eba306f76d765f5c4bfb03f6118ac1eb05eec3a92e1b0076a03eb28e7b61c689fae945b279f873cfdddf4e66db0be0efead563ea08bc4a269fa03025e2cce6f9c1ff09c8da516d938199c809a7f94dcd61211974aebdb85a4e56a0188d1100731419827900267bf4e6ea6d428fa5a67656e021485d1f6c89e69be6a0b281bb20061318a515afbdd02954740f069ebc75e700fde24dfbdf8c76d57119a0d8d77d917f5b7577e7e644bbc7a933632271a8daadd06a8e7e322f12dd828217a00f301190681b368db4308d1d1aa1794f85df08d4f4f646ecc4967c58fd9bd77ba0206598a4356dd50c70cfb1f0285bdb1402b7d65b61c851c095a7535bec230d5aa000959956c2148c82c207272af1ae129403d42e8173aedf44a190e85ee5fef8c3a0c88307e92c80a76e057e82755d9d67934ae040a6ec402bc156ad58dbcd2bcbc4a0e40a8e323d0b0b19d37ab6a3d110de577307c6f8efed15097dfb5551955fc770a02da2c6b12eedab6030b55d4f7df2fb52dab0ef4db292cb9b9789fa170256a11fa0d00e11cde7531fb79a315b4d81ea656b3b452fe3fe7e50af48a1ac7bf4aa6343a066625c0eb2f6609471f20857b97598ae4dfc197666ff72fe47b94e4124900683a0ace3aa5d35ba3ebbdc0abde8add5896876b25261717c0a415c92642c7889ec66a03a4931a67ae8ebc1eca9ffa711c16599b86d5286504182618d9c2da7b83f5ef780");
        test_verify_decoded_mpt_node::<ENCODING_LEN, _>(rlp_encoding, |x| x);
    }

    // TODO: Create a test where it's supposed to fail.

    #[test]
    #[should_panic]
    fn test_verify_decoded_mpt_node_branch_node_fuzzed_prefix() {
        const ENCODING_LEN: usize = 600;

        // This is a RLP-encoded list of a branch node. It is a list of length 17. Each of the first
        // 16 elements is a 32-byte hash, and the last element is 0.
        let rlp_encoding: Vec<u8>  = bytes!("0xf90211a0215ead887d4da139eba306f76d765f5c4bfb03f6118ac1eb05eec3a92e1b0076a03eb28e7b61c689fae945b279f873cfdddf4e66db0be0efead563ea08bc4a269fa03025e2cce6f9c1ff09c8da516d938199c809a7f94dcd61211974aebdb85a4e56a0188d1100731419827900267bf4e6ea6d428fa5a67656e021485d1f6c89e69be6a0b281bb20061318a515afbdd02954740f069ebc75e700fde24dfbdf8c76d57119a0d8d77d917f5b7577e7e644bbc7a933632271a8daadd06a8e7e322f12dd828217a00f301190681b368db4308d1d1aa1794f85df08d4f4f646ecc4967c58fd9bd77ba0206598a4356dd50c70cfb1f0285bdb1402b7d65b61c851c095a7535bec230d5aa000959956c2148c82c207272af1ae129403d42e8173aedf44a190e85ee5fef8c3a0c88307e92c80a76e057e82755d9d67934ae040a6ec402bc156ad58dbcd2bcbc4a0e40a8e323d0b0b19d37ab6a3d110de577307c6f8efed15097dfb5551955fc770a02da2c6b12eedab6030b55d4f7df2fb52dab0ef4db292cb9b9789fa170256a11fa0d00e11cde7531fb79a315b4d81ea656b3b452fe3fe7e50af48a1ac7bf4aa6343a066625c0eb2f6609471f20857b97598ae4dfc197666ff72fe47b94e4124900683a0ace3aa5d35ba3ebbdc0abde8add5896876b25261717c0a415c92642c7889ec66a03a4931a67ae8ebc1eca9ffa711c16599b86d5286504182618d9c2da7b83f5ef780");
        let fuzz = |x: [u8; ENCODING_LEN]| {
            let mut y = x.clone();
            y[0] = 0xe;
            y
        };
        test_verify_decoded_mpt_node::<ENCODING_LEN, _>(rlp_encoding, fuzz);
    }

    #[test]
    #[should_panic]
    fn test_verify_decoded_mpt_node_branch_node_fuzzed_body() {
        const ENCODING_LEN: usize = 600;

        // This is a RLP-encoded list of a branch node. It is a list of length 17. Each of the first
        // 16 elements is a 32-byte hash, and the last element is 0.
        let rlp_encoding: Vec<u8>  = bytes!("0xf90211a0215ead887d4da139eba306f76d765f5c4bfb03f6118ac1eb05eec3a92e1b0076a03eb28e7b61c689fae945b279f873cfdddf4e66db0be0efead563ea08bc4a269fa03025e2cce6f9c1ff09c8da516d938199c809a7f94dcd61211974aebdb85a4e56a0188d1100731419827900267bf4e6ea6d428fa5a67656e021485d1f6c89e69be6a0b281bb20061318a515afbdd02954740f069ebc75e700fde24dfbdf8c76d57119a0d8d77d917f5b7577e7e644bbc7a933632271a8daadd06a8e7e322f12dd828217a00f301190681b368db4308d1d1aa1794f85df08d4f4f646ecc4967c58fd9bd77ba0206598a4356dd50c70cfb1f0285bdb1402b7d65b61c851c095a7535bec230d5aa000959956c2148c82c207272af1ae129403d42e8173aedf44a190e85ee5fef8c3a0c88307e92c80a76e057e82755d9d67934ae040a6ec402bc156ad58dbcd2bcbc4a0e40a8e323d0b0b19d37ab6a3d110de577307c6f8efed15097dfb5551955fc770a02da2c6b12eedab6030b55d4f7df2fb52dab0ef4db292cb9b9789fa170256a11fa0d00e11cde7531fb79a315b4d81ea656b3b452fe3fe7e50af48a1ac7bf4aa6343a066625c0eb2f6609471f20857b97598ae4dfc197666ff72fe47b94e4124900683a0ace3aa5d35ba3ebbdc0abde8add5896876b25261717c0a415c92642c7889ec66a03a4931a67ae8ebc1eca9ffa711c16599b86d5286504182618d9c2da7b83f5ef780");
        let fuzz = |x: [u8; ENCODING_LEN]| {
            let mut y = x.clone();
            y[100] += 1;
            y
        };
        test_verify_decoded_mpt_node::<ENCODING_LEN, _>(rlp_encoding, fuzz);
    }

    // TODO: Create a test with a list containing a single-byte element of various values.
}
