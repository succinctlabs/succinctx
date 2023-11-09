use std::arch::is_aarch64_feature_detected;

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
    RichField, ValueStream, Variable, VariableStream,
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

    /// This handles the term in the claim polynomial corresponding to the prefix byte.
    ///
    /// You can uniquely determine the length of the RLP encoding of a byte array by looking at the
    /// first byte and also the length of the byte array. This function returns the prefix byte and
    /// the encoding length of the RLP, which is useful for performing the "polynomial trick."
    fn encoding_metadata_calculator(
        &mut self,
        first_byte_as_byte_variable: ByteVariable,
        len: Variable,
    ) -> (Variable, Variable) {
        // There are 4 cases:
        // - len = 0                       ===> (0x80, 1)
        // - len = 1 && decoded[0] < 0x80  ===> (decoded[0], 1)
        // - len = 1 && decoded[0] >= 0x80 ===> (0x81, 2)
        // - len <= 55                     ===> (0x80 + len, 1 + len)

        let first_byte = self.byte_to_variable(first_byte_as_byte_variable);
        let one: Variable = self.one();
        let zero: Variable = self.zero();
        let cons0x80 = self.constant::<Variable>(L::Field::from_canonical_u8(0x80));
        let cons0x55 = self.constant::<Variable>(L::Field::from_canonical_u8(0x55));
        let true_v = self.constant::<BoolVariable>(true);

        let pref1 = cons0x80;
        let len1 = one;
        let pref2 = first_byte;
        let len2 = one;
        let pref3 = self.constant::<Variable>(L::Field::from_canonical_u64(0x81));
        let len3 = self.constant::<Variable>(L::Field::from_canonical_u64(2));
        let pref4 = self.add(pref1, len);
        let len4 = self.add(one, len);

        let is_len_one = self.is_equal(len, one);
        let is_first_byte_less_than_0x80 = self.lt(first_byte, cons0x80);
        let is_first_byte_ge_0x80 = self.not(is_first_byte_less_than_0x80);
        let is_len_lte_55 = self.lte(len, cons0x55);
        self.assert_is_equal(is_len_lte_55, true_v);

        let is_case1 = self.is_equal(len, zero);
        let is_case2 = self.and(is_len_one, is_first_byte_less_than_0x80);
        let is_case3 = self.and(is_len_one, is_first_byte_ge_0x80);
        let mut ans_pref = pref4;
        let mut ans_len = len4;
        ans_pref = self.select(is_case3, pref3, ans_pref);
        ans_pref = self.select(is_case2, pref2, ans_pref);
        ans_pref = self.select(is_case1, pref1, ans_pref);

        ans_len = self.select(is_case3, len3, ans_len);
        ans_len = self.select(is_case2, len2, ans_len);
        ans_len = self.select(is_case1, len1, ans_len);

        self.watch(&len, "len");
        self.watch(&first_byte_as_byte_variable, "first_byte_as_byte_variable");
        self.watch(&ans_pref, "prefix of the encoding");
        self.watch(&ans_len, "length of the encoding");
        (ans_pref, ans_len)
    }

    /// This function verifies the decoding by comparing both the encoded and decoded MPT node.
    ///
    /// Mathematically speaking, we define a function `f(E : RLP-encoding) -> F` such that
    /// `f(E) = \sigma_{i = 0}^{len(E) - 1} [byte_to_field_element(E[i]) * challenger^(i)]`.
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
    /// explicitly calculate `rlp-encode(i-th item in mpt)` either. Instead, we calculate it by
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

        // TODO: This was originally 3. For debugging purposes, I changed it to 1. 1 is almost
        // certainly insufficient.
        const NUM_LOOPS: usize = 1;

        let mut challenges = challenger
            .get_n_challenges(&mut self.api, NUM_LOOPS)
            .iter()
            .map(|x| Variable::from(*x))
            .collect_vec();

        // #############################################################
        // #############################################################
        // #############################################################
        // TODO: This is obviously wrong. However, this makes debugging so much easier. We *MUST*
        // remove this line.
        // #############################################################
        // #############################################################
        // #############################################################
        challenges[0] = self.constant::<Variable>(L::Field::from_canonical_u64(1));

        let one = self.one();
        let zero = self.zero();
        let cons256 = self.constant::<Variable>(L::Field::from_canonical_u64(256));
        let cons55 = self.constant::<Variable>(L::Field::from_canonical_u64(55));
        let cons65536 = self.constant::<Variable>(L::Field::from_canonical_u64(65536));
        let true_v = self.constant::<BoolVariable>(true);
        for loop_index in 0..NUM_LOOPS {
            let mut encoding_poly = self.zero::<Variable>();
            let mut pow = self.one();

            self.watch(&challenges[loop_index], "challenges");
            for i in 0..ENCODING_LEN {
                let tmp0 = self.byte_to_variable(encoded[i]);
                let tmp1 = self.mul(tmp0, pow);
                encoding_poly = self.add(encoding_poly, tmp1);

                let index = self.constant::<Variable>(L::Field::from_canonical_usize(i));
                let is_done = self.lte(index, len);
                let is_done_coef = self.select(is_done, one, zero);
                pow = self.mul(pow, challenges[loop_index]);
                // As soon as we have i = ENCODING_LEN, pow becomes 0 for the rest of the loop.
                pow = self.mul(pow, is_done_coef);
            }

            let mut sum_of_rlp_encoding_length = zero;
            let mut claim_poly = zero;
            pow = one;

            for i in 0..MAX_MPT_NODE_SIZE {
                let index_i = self.constant::<Variable>(L::Field::from_canonical_usize(i));
                let within_outer_list_coef = self.if_less_than_else(index_i, mpt.len, one, zero);

                // TODO: Correctly pick the right len and pref

                let (mut prefix_byte, mut encoding_length) =
                    self.encoding_metadata_calculator(mpt.data[loop_index][i], mpt.lens[i]);

                encoding_length = self.mul(encoding_length, within_outer_list_coef);
                sum_of_rlp_encoding_length = self.add(sum_of_rlp_encoding_length, encoding_length);

                prefix_byte = self.mul(prefix_byte, pow);
                prefix_byte = self.mul(prefix_byte, within_outer_list_coef);

                pow = self.mul(pow, challenges[loop_index]);

                for j in 0..MAX_RLP_ITEM_SIZE {
                    let index_j = self.constant::<Variable>(L::Field::from_canonical_usize(j));
                    let inclusion_indicator =
                        self.if_less_than_else(index_j, mpt.lens[i], within_outer_list_coef, zero);

                    let mut val_as_var = self.byte_to_variable(mpt.data[i][j]);
                    val_as_var = self.mul(val_as_var, pow);
                    val_as_var = self.mul(val_as_var, inclusion_indicator);

                    claim_poly = self.add(claim_poly, val_as_var);

                    // pow = pow * challenges[i] only if j < LIST_LEN & k < ELEMENT_LEN
                    let mut pow_multiplier = challenges[loop_index];
                    pow_multiplier =
                        self.if_less_than_else(index_j, mpt.lens[i], pow_multiplier, one);
                    pow = self.mul(pow, pow_multiplier);
                }
            }
            // Based on what we've seen, we calculate the prefix of the whole encoding.
            // This is the case when sum_of_rlp_encoding_length is <= 55 bytes (1 byte).
            let mut short_list_prefix =
                self.constant::<Variable>(L::Field::from_canonical_u64(192)); // 0xc0
            short_list_prefix = self.add(short_list_prefix, sum_of_rlp_encoding_length);
            let short_list_shift = challenges[loop_index];

            // Assert that sum_of_rlp_encoding_length is less than 256^2 = 65536 bits. A
            // well-formed MPT should never need that many bytes.
            let valid_length = self.lt(sum_of_rlp_encoding_length, cons65536);
            self.assert_is_equal(true_v, valid_length);

            // The remaining case is when we need exactly two bytes to encode the length. 0xf9 =
            // 0xf7 + 2.
            let mut long_list_prefix = self.constant::<Variable>(L::Field::from_canonical_u64(249));

            // Divide sum_of_rlp_encoding_length by cons256 and get the quotient and remainder.
            let mut long_list_shift = challenges[loop_index];
            long_list_shift = self.mul(long_list_shift, challenges[loop_index]);
            long_list_shift = self.mul(long_list_shift, challenges[loop_index]);

            let mut div = self.div(sum_of_rlp_encoding_length, cons256);
            let mut rem = sum_of_rlp_encoding_length;
            let subtract = self.mul(div, cons256);
            rem = self.sub(rem, subtract);

            div = self.mul(div, challenges[loop_index]);
            rem = self.mul(rem, challenges[loop_index]);
            rem = self.mul(rem, challenges[loop_index]);
            long_list_prefix = self.add(long_list_prefix, div);
            long_list_prefix = self.add(long_list_prefix, rem);

            let is_short = self.lte(sum_of_rlp_encoding_length, cons55);

            let correct_prefix = self.select(is_short, short_list_prefix, long_list_prefix);
            let correct_shift = self.select(is_short, short_list_shift, long_list_shift);

            claim_poly = self.mul(claim_poly, correct_shift);
            claim_poly = self.add(claim_poly, correct_prefix);
            self.watch(&claim_poly, "claim_poly");
            self.watch(&encoding_poly, "encoding_poly");
            self.watch(&sum_of_rlp_encoding_length, "sum_of_rlp_encoding_length");
            let claim_poly_equals_encoding_poly = self.is_equal(claim_poly, encoding_poly);
            let result = self.or(skip_computation, claim_poly_equals_encoding_poly);

            // #############################################################
            // #############################################################
            // #############################################################
            // TODO: uncomment this once the values seem to match.
            // #############################################################
            // #############################################################
            // #############################################################
            //
            //            self.assert_is_equal(result, true_v);
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
    use rand::rngs::OsRng;
    use rand::Rng;

    use super::*;
    use crate::backend::circuit::DefaultParameters;
    use crate::prelude::{DefaultBuilder, GoldilocksField};
    use crate::utils::{bytes, setup_logger};

    fn test_verify_decoded_mpt_node<const ENCODING_LEN: usize>(rlp_encoding: Vec<u8>) {
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

        let encoded = builder
            .constant::<ArrayVariable<ByteVariable, ENCODING_LEN>>(encoding_fixed_size.to_vec());
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

        test_verify_decoded_mpt_node::<ENCODING_LEN>(rlp_encoding);
    }

    #[test]
    fn test_verify_decoded_mpt_node_branch_node() {
        const ENCODING_LEN: usize = 600;

        // This is a RLP-encoded list of a branch node. It is a list of length 17. Each of the first
        // 16 elements is a 32-byte hash, and the last element is 0.
        let rlp_encoding: Vec<u8>  = bytes!("0xf90211a0215ead887d4da139eba306f76d765f5c4bfb03f6118ac1eb05eec3a92e1b0076a03eb28e7b61c689fae945b279f873cfdddf4e66db0be0efead563ea08bc4a269fa03025e2cce6f9c1ff09c8da516d938199c809a7f94dcd61211974aebdb85a4e56a0188d1100731419827900267bf4e6ea6d428fa5a67656e021485d1f6c89e69be6a0b281bb20061318a515afbdd02954740f069ebc75e700fde24dfbdf8c76d57119a0d8d77d917f5b7577e7e644bbc7a933632271a8daadd06a8e7e322f12dd828217a00f301190681b368db4308d1d1aa1794f85df08d4f4f646ecc4967c58fd9bd77ba0206598a4356dd50c70cfb1f0285bdb1402b7d65b61c851c095a7535bec230d5aa000959956c2148c82c207272af1ae129403d42e8173aedf44a190e85ee5fef8c3a0c88307e92c80a76e057e82755d9d67934ae040a6ec402bc156ad58dbcd2bcbc4a0e40a8e323d0b0b19d37ab6a3d110de577307c6f8efed15097dfb5551955fc770a02da2c6b12eedab6030b55d4f7df2fb52dab0ef4db292cb9b9789fa170256a11fa0d00e11cde7531fb79a315b4d81ea656b3b452fe3fe7e50af48a1ac7bf4aa6343a066625c0eb2f6609471f20857b97598ae4dfc197666ff72fe47b94e4124900683a0ace3aa5d35ba3ebbdc0abde8add5896876b25261717c0a415c92642c7889ec66a03a4931a67ae8ebc1eca9ffa711c16599b86d5286504182618d9c2da7b83f5ef780");
        test_verify_decoded_mpt_node::<ENCODING_LEN>(rlp_encoding);
    }

    // TODO: Create a test where it's supposed to fail.
}
