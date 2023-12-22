use core::fmt::Debug;

use curta::chip::register::array::ArrayRegister;
use curta::chip::register::bit::BitRegister;
use curta::chip::register::element::ElementRegister;
use curta::chip::register::Register;
use curta::chip::uint::operations::instruction::UintInstructions;
use curta::chip::AirParameters;
use curta::machine::builder::Builder;
use curta::machine::bytes::builder::BytesBuilder;
use curta::machine::hash::{HashDigest, HashIntConversion};
use plonky2::util::log2_ceil;
use serde::de::DeserializeOwned;
use serde::Serialize;

use self::accelerator::HashAccelerator;
use self::data::{HashInputData, HashInputParameters};
use self::request::HashRequest;
use self::stark::HashStark;
use crate::prelude::*;

pub mod accelerator;
pub mod builder;
pub mod data;
pub mod digest_hint;
pub mod proof_hint;
pub mod request;
pub mod stark;

/// An interface for a circuit that computes a hash using Curta.
pub trait Hash<
    L: PlonkParameters<D>,
    const D: usize,
    const CYCLE_LEN: usize,
    const HAS_T_VALUES: bool,
    const DIGEST_LEN: usize,
>:
    HashIntConversion<BytesBuilder<Self::AirParameters>>
    + HashDigest<BytesBuilder<Self::AirParameters>>
    + Debug
    + Clone
    + 'static
    + Serialize
    + DeserializeOwned
    + Send
    + Sync
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

    /// Convert a value of usize type to a `Self::IntVariable`.
    fn usize_to_variable(builder: &mut CircuitBuilder<L, D>, value: usize) -> Self::IntVariable;

    /// Convert a value of the `Self::DigestRegister` to an array of `Self::IntVariable`s.
    fn digest_to_array(
        builder: &mut CircuitBuilder<L, D>,
        digest: Self::DigestVariable,
    ) -> [Self::IntVariable; DIGEST_LEN];

    /// Get the input data for the stark from a `SHAAccelerator`.
    fn get_hash_data(
        builder: &mut CircuitBuilder<L, D>,
        accelerator: HashAccelerator<Self::IntVariable, DIGEST_LEN>,
    ) -> HashInputData<Self::IntVariable, DIGEST_LEN> {
        // Initialze the data struictures of `SHAInputData`.
        let mut t_values: Option<Vec<_>> = None;
        if HAS_T_VALUES {
            t_values = Some(Vec::new());
        }
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
            .hash_requests
            .iter()
            .flat_map(|req| {
                // For every request, we read the corresponding messagem, pad it, and compute the
                // corresponding chunk index.

                // Get the padded chunks and the number of chunks in the message, depending on the
                // type of the request.
                let (padded_chunks, length, last_chunk_index) = match req {
                    HashRequest::Fixed(input) => {
                        // If the length is fixed, the chunk_index is just `number of chunks  - 1``.
                        let padded_chunks = Self::pad_circuit(builder, input);
                        let num_chunks =
                            builder.constant((padded_chunks.len() / 16 - 1).try_into().unwrap());
                        let length = builder.constant::<U32Variable>(input.len() as u32);
                        (padded_chunks, length, num_chunks)
                    }
                    // If the length of the message is a variable, we read the chunk index from the
                    // request.
                    HashRequest::Variable(input, length, last_chunk) => (
                        Self::pad_circuit_variable_length(builder, input, *length),
                        *length,
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
                let digest_index =
                    builder.add(current_chunk_index_variable, last_chunk_index.variable);
                digest_indices.push(digest_index);
                // The digest bit is equal to zero for all chunks except the one that corresponds to
                // the `chunk_index`. We find the bits by comparing each value between 0 and the
                // total number of chunks to the `chunk_index`.
                let mut t_var = builder.constant::<U32Variable>(0);
                let chunk_size = builder.constant::<U32Variable>(128);
                for j in 0..total_number_of_chunks {
                    if HAS_T_VALUES {
                        t_var = builder.add(t_var, chunk_size);
                    }
                    let j_var = builder.constant::<U32Variable>(j as u32);
                    let at_digest_chunk = builder.is_equal(j_var, last_chunk_index);
                    digest_bits.push(at_digest_chunk);

                    t_var = builder.select(at_digest_chunk, length, t_var);
                    if HAS_T_VALUES {
                        t_values.unwrap().push(t_var);
                    }
                }
                // Increment the current chunk index by the total number of chunks.
                current_chunk_index += total_number_of_chunks;

                padded_chunks
            })
            .collect::<Vec<_>>();

        // Convert end_bits to variables.
        let end_bits = builder.constant_vec(&end_bit_values);

        HashInputData {
            padded_chunks,
            t_values,
            digest_bits,
            end_bits,
            digest_indices,
            digests: accelerator.hash_responses,
        }
    }

    /// The Curta Stark corresponding to the input data.
    fn stark(
        parameters: HashInputParameters,
    ) -> HashStark<L, Self, D, CYCLE_LEN, HAS_T_VALUES, DIGEST_LEN> {
        let mut builder = Self::curta_builder();

        let num_chunks = parameters.num_chunks;
        let padded_chunks = (0..num_chunks)
            .map(|_| builder.alloc_array_public::<Self::IntRegister>(16))
            .collect::<Vec<_>>();

        let mut t_values = None;
        if HAS_T_VALUES {
            // Allocate registers for the t-values.
            t_values = Some(
                (0..num_chunks)
                    .map(|_| builder.alloc_public::<Self::IntRegister>())
                    .collect::<Vec<_>>(),
            );
        }

        let end_bits = builder.alloc_array_public::<BitRegister>(num_chunks);
        let digest_bits = builder.alloc_array_public::<BitRegister>(num_chunks);
        let digest_indices = builder.alloc_array_public::<ElementRegister>(parameters.num_digests);

        // Hash the padded chunks.
        let digests = Self::hash_circuit(
            builder,
            padded_chunks,
            t_values,
            &end_bits,
            &digest_bits,
            &digest_indices,
            num_messages,
        );

        // Build the stark.
        let num_rows_degree = log2_ceil(CYCLE_LEN * num_chunks);
        let num_rows = 1 << num_rows_degree;
        let stark = builder.build::<L::CurtaConfig, D>(num_rows);

        HashStark {
            stark,
            padded_chunks,
            t_values,
            end_bits,
            digest_bits,
            digest_indices,
            digests,
            num_rows,
        }
    }

    fn hash(message: Vec<u8>) -> [Self::Integer; DIGEST_LEN];

    fn curta_builder() -> BytesBuilder<Self::AirParameters> {
        BytesBuilder::<Self::AirParameters>::new()
    }

    fn hash_circuit(
        builder: &mut BytesBuilder<Self::AirParameters>,
        padded_chunks: &[ArrayRegister<Self::IntRegister>],
        t_values: Option<&ArrayRegister<Self::IntRegister>>,
        end_bits: &ArrayRegister<BitRegister>,
        digest_bits: &ArrayRegister<BitRegister>,
        digest_indices: &ArrayRegister<ElementRegister>,
        num_messages: &ElementRegister,
    ) -> Vec<Self::DigestRegister>;
}
