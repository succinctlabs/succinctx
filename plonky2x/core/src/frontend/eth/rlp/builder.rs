use curta::math::field::Field;
use curta::math::prelude::PrimeField64;
use serde::{Deserialize, Serialize};

use super::utils::decode_padded_mpt_node;
use crate::frontend::eth::rlp::utils::MAX_STRING_SIZE;
use crate::frontend::hint::simple::hint::Hint;
use crate::prelude::{
    ArrayVariable, BoolVariable, ByteVariable, CircuitBuilder, PlonkParameters, ValueStream,
    Variable, VariableStream,
};

/// TODO: Consider removing LIST_LEN and using MAX_STRING_SIZE instead.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DecodeHint<const ENCODING_LEN: usize, const LIST_LEN: usize> {}
impl<L: PlonkParameters<D>, const D: usize, const ENCODING_LEN: usize, const LIST_LEN: usize>
    Hint<L, D> for DecodeHint<ENCODING_LEN, LIST_LEN>
{
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let encoded = input_stream.read_value::<ArrayVariable<ByteVariable, ENCODING_LEN>>();
        let len = input_stream.read_value::<Variable>();
        let finish = input_stream.read_value::<BoolVariable>();

        let (decoded_list, decoded_list_lens, len_decoded_list) =
            decode_padded_mpt_node::<ENCODING_LEN, LIST_LEN>(
                &encoded,
                len.as_canonical_u64() as usize,
                finish,
            );

        output_stream
            .write_value::<ArrayVariable<ArrayVariable<ByteVariable, MAX_STRING_SIZE>, LIST_LEN>>(
                decoded_list,
            );
        output_stream.write_value::<ArrayVariable<Variable, LIST_LEN>>(
            decoded_list_lens
                .iter()
                .map(|x| L::Field::from_canonical_usize(*x))
                .collect::<Vec<_>>(),
        );
        output_stream.write_value::<Variable>(L::Field::from_canonical_usize(len_decoded_list));
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn decode_element_as_list<
        const ENCODING_LEN: usize,
        const LIST_LEN: usize,
        const ELEMENT_LEN: usize,
    >(
        &mut self,
        encoded: ArrayVariable<ByteVariable, ENCODING_LEN>,
        len: Variable,
        finish: BoolVariable,
    ) -> (
        ArrayVariable<ArrayVariable<ByteVariable, ELEMENT_LEN>, LIST_LEN>,
        ArrayVariable<Variable, LIST_LEN>,
        Variable,
    ) {
        let mut input_stream = VariableStream::new();
        input_stream.write(&encoded);
        input_stream.write(&len);
        input_stream.write(&finish);

        let hint = DecodeHint::<ENCODING_LEN, LIST_LEN> {};

        let output_stream = self.hint(input_stream, hint);
        let decoded_list = output_stream
            .read::<ArrayVariable<ArrayVariable<ByteVariable, ELEMENT_LEN>, LIST_LEN>>(self);
        let decoded_element_lens = output_stream.read::<ArrayVariable<Variable, LIST_LEN>>(self);
        let len_decoded_list = output_stream.read::<Variable>(self);

        // TODO: here add verification logic constraints using `builder` to check that the decoded list is correct

        (decoded_list, decoded_element_lens, len_decoded_list)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::backend::circuit::DefaultParameters;
    use crate::prelude::{DefaultBuilder, GoldilocksField};
    use crate::utils::bytes;

    #[test]

    fn test_rlp_decode_hint() {
        let mut builder: CircuitBuilder<DefaultParameters, 2> = DefaultBuilder::new();

        type F = GoldilocksField;
        const ENCODING_LEN: usize = 600;
        const LIST_LEN: usize = 17;

        let hint: DecodeHint<ENCODING_LEN, LIST_LEN> = DecodeHint::<ENCODING_LEN, LIST_LEN> {};
        let encoded = builder.read::<ArrayVariable<ByteVariable, ENCODING_LEN>>();
        let len = builder.read::<Variable>();
        let finish = builder.read::<BoolVariable>();
        let mut input_stream = VariableStream::new();
        input_stream.write(&encoded);
        input_stream.write(&len);
        input_stream.write(&finish);
        let output_stream = builder.hint(input_stream, hint);
        let decoded_list = output_stream
            .read::<ArrayVariable<ArrayVariable<ByteVariable, MAX_STRING_SIZE>, LIST_LEN>>(
                &mut builder,
            );
        let decoded_element_lens =
            output_stream.read::<ArrayVariable<Variable, LIST_LEN>>(&mut builder);
        let len_decoded_list = output_stream.read::<Variable>(&mut builder);

        builder.write(decoded_list);
        builder.write(decoded_element_lens);
        builder.write(len_decoded_list);

        let circuit = builder.build();
        let mut input = circuit.input();

        // This is a RLP-encoded list of length 17. Each of the first 16 elements is a 32-byte hash,
        // and the last element is 0.
        let rlp_encoding: Vec<u8>  = bytes!("0xf90211a0215ead887d4da139eba306f76d765f5c4bfb03f6118ac1eb05eec3a92e1b0076a03eb28e7b61c689fae945b279f873cfdddf4e66db0be0efead563ea08bc4a269fa03025e2cce6f9c1ff09c8da516d938199c809a7f94dcd61211974aebdb85a4e56a0188d1100731419827900267bf4e6ea6d428fa5a67656e021485d1f6c89e69be6a0b281bb20061318a515afbdd02954740f069ebc75e700fde24dfbdf8c76d57119a0d8d77d917f5b7577e7e644bbc7a933632271a8daadd06a8e7e322f12dd828217a00f301190681b368db4308d1d1aa1794f85df08d4f4f646ecc4967c58fd9bd77ba0206598a4356dd50c70cfb1f0285bdb1402b7d65b61c851c095a7535bec230d5aa000959956c2148c82c207272af1ae129403d42e8173aedf44a190e85ee5fef8c3a0c88307e92c80a76e057e82755d9d67934ae040a6ec402bc156ad58dbcd2bcbc4a0e40a8e323d0b0b19d37ab6a3d110de577307c6f8efed15097dfb5551955fc770a02da2c6b12eedab6030b55d4f7df2fb52dab0ef4db292cb9b9789fa170256a11fa0d00e11cde7531fb79a315b4d81ea656b3b452fe3fe7e50af48a1ac7bf4aa6343a066625c0eb2f6609471f20857b97598ae4dfc197666ff72fe47b94e4124900683a0ace3aa5d35ba3ebbdc0abde8add5896876b25261717c0a415c92642c7889ec66a03a4931a67ae8ebc1eca9ffa711c16599b86d5286504182618d9c2da7b83f5ef780");
        let mut encoding_fixed_size = [0u8; ENCODING_LEN];
        encoding_fixed_size[..rlp_encoding.len()].copy_from_slice(&rlp_encoding);
        let finish = false;
        input.write::<ArrayVariable<ByteVariable, ENCODING_LEN>>(encoding_fixed_size.to_vec());
        input.write::<Variable>(F::from_canonical_usize(rlp_encoding.len()));
        input.write::<BoolVariable>(finish);

        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);

        let decoded_list_out =
            output.read::<ArrayVariable<ArrayVariable<ByteVariable, MAX_STRING_SIZE>, LIST_LEN>>();
        let decoded_element_lens_out = output.read::<ArrayVariable<Variable, LIST_LEN>>();
        let len_decoded_list_out = output.read::<Variable>();

        let (decoded_list_exp, decoded_list_lens_exp, len_decoded_list_exp) =
            decode_padded_mpt_node::<ENCODING_LEN, LIST_LEN>(
                &encoding_fixed_size,
                rlp_encoding.len(),
                finish,
            );

        assert_eq!(
            len_decoded_list_out,
            F::from_canonical_usize(len_decoded_list_exp)
        );
        assert_eq!(decoded_list_out.len(), LIST_LEN);
        assert_eq!(len_decoded_list_out, F::from_canonical_usize(LIST_LEN));

        for i in 0..LIST_LEN {
            assert_eq!(decoded_list_out[i], decoded_list_exp[i]);
            assert_eq!(
                decoded_element_lens_out[i],
                F::from_canonical_usize(decoded_list_lens_exp[i])
            );
        }
    }
    #[test]

    fn test_rlp_decode_hint_short_encoding() {
        let mut builder: CircuitBuilder<DefaultParameters, 2> = DefaultBuilder::new();

        type F = GoldilocksField;
        const ENCODING_LEN: usize = 600;
        const LIST_LEN: usize = 2;

        let hint: DecodeHint<ENCODING_LEN, LIST_LEN> = DecodeHint::<ENCODING_LEN, LIST_LEN> {};
        let encoded = builder.read::<ArrayVariable<ByteVariable, ENCODING_LEN>>();
        let len = builder.read::<Variable>();
        let finish = builder.read::<BoolVariable>();
        let mut input_stream = VariableStream::new();
        input_stream.write(&encoded);
        input_stream.write(&len);
        input_stream.write(&finish);
        let output_stream = builder.hint(input_stream, hint);
        let decoded_list = output_stream
            .read::<ArrayVariable<ArrayVariable<ByteVariable, MAX_STRING_SIZE>, LIST_LEN>>(
                &mut builder,
            );
        let decoded_element_lens =
            output_stream.read::<ArrayVariable<Variable, LIST_LEN>>(&mut builder);
        let len_decoded_list = output_stream.read::<Variable>(&mut builder);

        builder.write(decoded_list);
        builder.write(decoded_element_lens);
        builder.write(len_decoded_list);

        let circuit = builder.build();
        let mut input = circuit.input();

        // This is a RLP-encoded list of an extension node. The 00 in 0x006f indicates that the path
        // length is even, and the path is 6 -> f. This extension node points to a leaf node with
        // the hash starting with 0x188d11.  ["0x006f",
        // "0x188d1100731419827900267bf4e6ea6d428fa5a67656e021485d1f6c89e69be6"]
        let rlp_encoding: Vec<u8> =
            bytes!("0xe482006fa0188d1100731419827900267bf4e6ea6d428fa5a67656e021485d1f6c89e69be6");
        let mut encoding_fixed_size = [0u8; ENCODING_LEN];
        encoding_fixed_size[..rlp_encoding.len()].copy_from_slice(&rlp_encoding);
        let finish = false;
        input.write::<ArrayVariable<ByteVariable, ENCODING_LEN>>(encoding_fixed_size.to_vec());
        input.write::<Variable>(F::from_canonical_usize(rlp_encoding.len()));
        input.write::<BoolVariable>(finish);

        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);

        let decoded_list_out =
            output.read::<ArrayVariable<ArrayVariable<ByteVariable, MAX_STRING_SIZE>, LIST_LEN>>();
        let decoded_element_lens_out = output.read::<ArrayVariable<Variable, LIST_LEN>>();
        let len_decoded_list_out = output.read::<Variable>();

        let (decoded_list_exp, decoded_list_lens_exp, len_decoded_list_exp) =
            decode_padded_mpt_node::<ENCODING_LEN, LIST_LEN>(
                &encoding_fixed_size,
                rlp_encoding.len(),
                finish,
            );

        assert_eq!(
            len_decoded_list_out,
            F::from_canonical_usize(len_decoded_list_exp)
        );
        assert_eq!(decoded_list_out.len(), LIST_LEN);
        assert_eq!(len_decoded_list_out, F::from_canonical_usize(LIST_LEN));

        for i in 0..LIST_LEN {
            assert_eq!(decoded_list_out[i], decoded_list_exp[i]);
            assert_eq!(
                decoded_element_lens_out[i],
                F::from_canonical_usize(decoded_list_lens_exp[i])
            );
        }
    }
}
