use curta::chip::register::bit::BitRegister;
use curta::chip::register::element::ElementRegister;
use curta::chip::register::Register;
use curta::chip::uint::operations::instruction::UintInstructions;
use curta::chip::AirParameters;
use curta::machine::builder::Builder;
use curta::machine::bytes::builder::BytesBuilder;
use curta::machine::hash::sha::algorithm::SHAir;
use curta::machine::hash::sha::builder::SHABuilder;
use plonky2::util::log2_ceil;

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

/// An interface for a circuit that computes SHA using Curta.
pub trait SHA<L: PlonkParameters<D>, const D: usize, const CYCLE_LEN: usize>:
    SHAir<BytesBuilder<Self::AirParameters>, CYCLE_LEN>
{
    /// A `CircuitVariable` that represents the integer registers used by the hash function.
    ///
    /// For example, in `SHA256` this would be a `CircuitVariable` that represents a 32-bit integer.
    type IntVariable: CircuitVariable<ValueType<L::Field> = Self::Integer> + Copy;
    /// A `CircuitVariable` that represents the hash digest.
    type DigestVariable: CircuitVariable;

    /// The air parameters of the corresponding Curta stark.
    type AirParameters: AirParameters<
        Field = L::Field,
        CubicParams = L::CubicParams,
        Instruction = Self::AirInstruction,
    >;

    /// The air instructions of the corresponding Curta stark.
    type AirInstruction: UintInstructions;

    /// Pad an input message of fixed length.
    fn pad_circuit(
        builder: &mut CircuitBuilder<L, D>,
        input: &[ByteVariable],
    ) -> Vec<Self::IntVariable>;

    /// Pad an input message of variable length.
    fn pad_circuit_variable_length(
        builder: &mut CircuitBuilder<L, D>,
        input: &[ByteVariable],
        length: U32Variable,
    ) -> Vec<Self::IntVariable>;

    /// Convert a value of the `Self::IntRegister` type to a `Self::IntVariable`.
    ///
    /// This is used to assert compatibility between the stark and the circuit representation of the
    /// integer variables.
    fn value_to_variable(
        builder: &mut CircuitBuilder<L, D>,
        value: <Self::IntRegister as Register>::Value<Variable>,
    ) -> Self::IntVariable;

    /// Convert a value of the `Self::DigestRegister` to an array of `Self::IntVariable`s.
    fn digest_to_array(
        builder: &mut CircuitBuilder<L, D>,
        digest: Self::DigestVariable,
    ) -> [Self::IntVariable; 8];

    /// Get the input data for the stark from a `SHAAccelerator`.
    fn get_sha_data(
        builder: &mut CircuitBuilder<L, D>,
        accelerator: SHAAccelerator<Self::IntVariable>,
    ) -> SHAInputData<Self::IntVariable> {
        // Initialze the data struictures of `SHAInputData`.
        let mut end_bit_values = Vec::new();
        let mut digest_bits = Vec::new();
        let mut current_chunk_index = 0;
        let mut digest_indices = Vec::<Variable>::new();

        // Get the padded chunks from input messages, and assign the correct values of end_bit,
        // digest_bits, and digest_indices.
        //
        // `end_bit` - a bit that is true for the last chunk of a message (or the total_message
        // passed in the case of a message of variable length).
        // `digest_bit` - a bit that indicates the hash state is read into the digest list after
        // processing the chunk. For a message of fioxed length, this is the same as `end_bit`.
        // `digest_index` - the index of the digest to be read, corresponding to the location of the
        // chunk in the padded chunks.
        let padded_chunks = accelerator
            .sha_requests
            .iter()
            .flat_map(|req| {
                // For every reuqest, we read the corresponding messagem, pad it, and compute the
                // corresponding chunk index.

                // Get the padded chunks and the number of chunks in the message, depending on the
                // type of the request.
                let (padded_chunks, chunk_index) = match req {
                    SHARequest::Fixed(input) => {
                        // If the length is fixed, the chunk_index is just `number of chunks  - 1``.
                        let padded_chunks = Self::pad_circuit(builder, input);
                        let num_chunks =
                            builder.constant((padded_chunks.len() / 16 - 1).try_into().unwrap());
                        (padded_chunks, num_chunks)
                    }
                    // If the length of the massage is a variable, we read the chunk index form the
                    // request.
                    SHARequest::Variable(input, length, last_chunk) => (
                        Self::pad_circuit_variable_length(builder, input, *length),
                        *last_chunk,
                    ),
                };
                // Get the total number of chunks processed.
                let total_number_of_chunks = padded_chunks.len() / 16;
                // Store the end_bit values. The end bit indicates the end of message chunks.
                end_bit_values.extend_from_slice(&vec![false; total_number_of_chunks - 1]);
                end_bit_values.push(true);
                // The chunk index is given by the currenty index plus the chunk index we got from
                // the request.
                let current_chunk_index_variable = builder
                    .constant::<Variable>(L::Field::from_canonical_usize(current_chunk_index));
                let digest_index = builder.add(current_chunk_index_variable, chunk_index.variable);
                digest_indices.push(digest_index);
                // The digest bit is equal to zero for all chunks except the one that corresponds to
                // the `chunk_index`. We find the bits by comparing each value between 0 and the
                // total number of chunks to the `chunk_index`.
                let mut flag = builder.constant::<BoolVariable>(true);
                for j in 0..total_number_of_chunks {
                    let j_var = builder.constant::<U32Variable>(j as u32);
                    let lte = builder.lte(chunk_index, j_var);
                    let lte_times_flag = builder.and(lte, flag);
                    digest_bits.push(lte_times_flag);
                    let not_lte = builder.not(lte);
                    flag = builder.and(flag, not_lte);
                }
                // Increment the current chunk index by the total number of chunks.
                current_chunk_index += total_number_of_chunks;

                padded_chunks
            })
            .collect::<Vec<_>>();

        // Convert end_bits to variables.
        let end_bits = builder.constant_vec(&end_bit_values);

        SHAInputData {
            padded_chunks,
            digest_bits,
            end_bits,
            digest_indices,
            digests: accelerator.sha_responses,
        }
    }

    /// The Curta Stark corresponding to the input data.
    fn stark(parameters: SHAInputParameters) -> SHAStark<L, Self, D, CYCLE_LEN> {
        let mut builder = BytesBuilder::<Self::AirParameters>::new();

        let num_chunks = parameters.num_chunks;
        let padded_chunks = (0..num_chunks)
            .map(|_| builder.alloc_array_public::<Self::IntRegister>(16))
            .collect::<Vec<_>>();

        // Allocate registers for the public inputs to the Stark.
        let end_bits = builder.alloc_array_public::<BitRegister>(num_chunks);
        let digest_bits = builder.alloc_array_public::<BitRegister>(num_chunks);
        let digest_indices = builder.alloc_array_public::<ElementRegister>(parameters.num_digests);

        // Hash the padded chunks.
        let digests =
            builder.sha::<Self, CYCLE_LEN>(&padded_chunks, &end_bits, &digest_bits, digest_indices);

        // Build the stark.
        let num_rows_degree = log2_ceil(CYCLE_LEN * num_chunks);
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
