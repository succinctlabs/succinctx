use curta::chip::register::bit::BitRegister;
use curta::chip::register::element::ElementRegister;
use curta::chip::register::Register;
use curta::chip::uint::operations::instruction::UintInstructions;
use curta::chip::AirParameters;
use curta::machine::builder::Builder;
use curta::machine::bytes::builder::BytesBuilder;
use curta::machine::hash::sha::algorithm::SHAir;
use curta::machine::hash::sha::builder::SHABuilder;

use self::accelerator::SHAAccelerator;
use self::data::{SHAInputData, SHAInputParameters};
use self::request::SHARequest;
use self::stark::SHAStark;
use crate::prelude::*;

pub mod accelerator;
pub mod builder;
pub mod data;
pub mod digest_hint;
pub mod proof_hint;
pub mod request;
pub mod stark;

pub trait SHA<L: PlonkParameters<D>, const D: usize, const CYCLE_LEN: usize>:
    SHAir<BytesBuilder<Self::AirParameters>, CYCLE_LEN>
{
    type IntVariable: CircuitVariable<ValueType<L::Field> = Self::Integer> + Copy;
    type DigestVariable: CircuitVariable;

    type AirParameters: AirParameters<
        Field = L::Field,
        CubicParams = L::CubicParams,
        Instruction = Self::AirInstruction,
    >;
    type AirInstruction: UintInstructions;

    fn pad_circuit(
        builder: &mut CircuitBuilder<L, D>,
        input: &[ByteVariable],
    ) -> Vec<Self::IntVariable>;

    fn pad_circuit_variable_length(
        builder: &mut CircuitBuilder<L, D>,
        input: &[ByteVariable],
        length: Variable,
    ) -> Vec<Self::IntVariable>;

    fn value_to_variable(
        builder: &mut CircuitBuilder<L, D>,
        value: <Self::IntRegister as Register>::Value<Variable>,
    ) -> Self::IntVariable;

    fn digest_to_array(
        builder: &mut CircuitBuilder<L, D>,
        digest: Self::DigestVariable,
    ) -> [Self::IntVariable; 8];

    fn get_sha_data(
        builder: &mut CircuitBuilder<L, D>,
        accelerator: SHAAccelerator<Self::IntVariable>,
    ) -> SHAInputData<Self::IntVariable> {
        let mut end_bit_values = Vec::new();
        let mut current_chunk_index = 0;
        let mut digest_indices = Vec::<Variable>::new();
        let padded_chunks = accelerator
            .sha_requests
            .iter()
            .flat_map(|req| {
                let padded_chunks = match req {
                    SHARequest::Fixed(input) => Self::pad_circuit(builder, input),
                    SHARequest::Variable(_, _) => unimplemented!("TODO"),
                };
                let num_chunks = padded_chunks.len() / 16;
                digest_indices.push(builder.constant(L::Field::from_canonical_usize(
                    current_chunk_index + num_chunks - 1,
                )));
                current_chunk_index += num_chunks;
                end_bit_values.extend_from_slice(&vec![false; num_chunks - 1]);
                end_bit_values.push(true);
                padded_chunks
            })
            .collect::<Vec<_>>();

        let end_bits = builder.constant_vec(&end_bit_values);

        SHAInputData {
            padded_chunks,
            digest_bits: end_bits.clone(),
            end_bits,
            digest_indices,
            digests: accelerator.sha_responses,
        }
    }

    fn stark(parameters: SHAInputParameters) -> SHAStark<L, Self, D, CYCLE_LEN> {
        let mut builder = BytesBuilder::<Self::AirParameters>::new();

        let num_chunks = parameters.num_chunks;
        let padded_chunks = (0..num_chunks)
            .map(|_| builder.alloc_array_public::<Self::IntRegister>(16))
            .collect::<Vec<_>>();

        let end_bits = builder.alloc_array_public::<BitRegister>(num_chunks);
        let digest_bits = builder.alloc_array_public::<BitRegister>(num_chunks);
        let digest_indices = builder.alloc_array_public::<ElementRegister>(parameters.num_digests);

        let digests =
            builder.sha::<Self, CYCLE_LEN>(&padded_chunks, &end_bits, &digest_bits, digest_indices);

        let num_rows_degree = (CYCLE_LEN * num_chunks).ilog2() as usize + 1;
        let num_rows = 1 << num_rows_degree;
        let stark = builder.build::<L::CurtaConfig, D>(num_rows);

        SHAStark {
            stark,
            padded_chunks,
            end_bits,
            digest_bits,
            digest_indices,
            digests,
            num_rows,
        }
    }
}
