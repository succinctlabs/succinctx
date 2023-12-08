use curta::chip::register::array::ArrayRegister;
use curta::chip::register::bit::BitRegister;
use curta::chip::register::element::ElementRegister;
use curta::chip::register::Register;
use curta::chip::trace::writer::{InnerWriterData, TraceWriter};
use curta::chip::uint::register::U64Register;
use curta::chip::uint::util::u64_to_le_field_bytes;
use curta::machine::builder::Builder;
use curta::machine::bytes::builder::BytesBuilder;
use curta::machine::bytes::proof::ByteStarkProof;
use curta::machine::bytes::stark::ByteStark;
use curta::math::prelude::*;
use itertools::Itertools;
use log::debug;
use plonky2::util::log2_ceil;
use plonky2::util::timing::TimingTree;

use super::accelerator::BLAKE2BAccelerator;
use super::curta::BLAKE2BAirParameters;
use super::data::{BLAKE2BInputData, BLAKE2BInputDataValues, BLAKE2BInputParameters};
use super::request::BLAKE2BRequest;
use crate::frontend::curta::proof::ByteStarkProofVariable;
use crate::frontend::vars::EvmVariable;
use crate::prelude::{
    ByteVariable, Bytes32Variable, CircuitBuilder, CircuitVariable, PlonkParameters, U32Variable,
    U64Variable, Variable,
};

#[derive(Debug, Clone)]
pub struct BLAKE2BStark<L: PlonkParameters<D>, const D: usize> {
    pub stark: ByteStark<BLAKE2BAirParameters<L, D>, L::CurtaConfig, D>,
    pub padded_chunks: Vec<ArrayRegister<U64Register>>,
    pub t_values: ArrayRegister<U64Register>,
    pub end_bits: ArrayRegister<BitRegister>,
    pub digest_bits: ArrayRegister<BitRegister>,
    pub digest_indices: ArrayRegister<ElementRegister>,
    pub digests: Vec<ArrayRegister<U64Register>>,
    pub num_messages: ElementRegister,
    pub num_rows: usize,
}

impl<L: PlonkParameters<D>, const D: usize> BLAKE2BStark<L, D> {
    fn write_input(&self, writer: &TraceWriter<L::Field>, input: BLAKE2BInputDataValues<L, D>) {
        for (digest_index_reg, digest_index) in self
            .digest_indices
            .iter()
            .zip_eq(input.digest_indices.iter())
        {
            writer.write(&digest_index_reg, digest_index, 0);
        }

        for (digest_reg, digest_value) in self.digests.iter().zip_eq(input.digests.iter()) {
            let array: ArrayRegister<_> = *digest_reg;
            writer.write_array(&array, digest_value.map(u64_to_le_field_bytes), 0);
        }

        for (chunk, chunk_value) in self
            .padded_chunks
            .iter()
            .zip_eq(input.padded_chunks.chunks_exact(16))
        {
            println!(
                "writing chunk: {:?}",
                chunk_value
                    .iter()
                    .map(|x: &u64| u64_to_le_field_bytes::<L::Field>(*x))
                    .collect_vec()
            );
            writer.write_array(
                chunk,
                chunk_value.iter().map(|x| u64_to_le_field_bytes(*x)),
                0,
            );
        }

        for (t, t_value) in self.t_values.iter().zip_eq(input.t_values.iter()) {
            writer.write(&t, &u64_to_le_field_bytes(*t_value as u64), 0);
        }

        for (end_bit, end_bit_value) in self.end_bits.iter().zip_eq(input.end_bits) {
            writer.write(
                &end_bit,
                &L::Field::from_canonical_u8(end_bit_value as u8),
                0,
            );
        }

        for (digest_bit, digest_bit_value) in self.digest_bits.iter().zip_eq(input.digest_bits) {
            writer.write(
                &digest_bit,
                &L::Field::from_canonical_u8(digest_bit_value as u8),
                0,
            );
        }

        writer.write(
            &self.num_messages,
            &L::Field::from_canonical_u32(input.digests.len() as u32),
            0,
        );
    }

    /// Generate a proof for the SHA stark given the input data.
    #[allow(clippy::type_complexity)]
    pub fn prove(
        &self,
        input: BLAKE2BInputDataValues<L, D>,
    ) -> (ByteStarkProof<L::Field, L::CurtaConfig, D>, Vec<L::Field>) {
        // Initialize a writer for the trace.
        let writer = TraceWriter::new(&self.stark.air_data, self.num_rows);
        // Write the public inputs to the trace.
        self.write_input(&writer, input);
        // Execute the air instructions.
        writer.write_global_instructions(&self.stark.air_data);
        for i in 0..self.num_rows {
            writer.write_row_instructions(&self.stark.air_data, i);
        }

        // Extract the trace and public input slice from the writer.
        let InnerWriterData { trace, public, .. } = writer.into_inner().unwrap();
        let proof = self
            .stark
            .prove(&trace, &public, &mut TimingTree::default())
            .unwrap();

        // Verify proof to make sure it's valid.
        self.stark.verify(proof.clone(), &public).unwrap();
        debug!("stark proof verified");

        (proof, public)
    }

    pub fn verify_proof(
        &self,
        builder: &mut CircuitBuilder<L, D>,
        proof: ByteStarkProofVariable<D>,
        public_inputs: &[Variable],
        blake2b_input: BLAKE2BInputData,
    ) {
        // Verify the stark proof.
        builder.verify_byte_stark_proof(&self.stark, proof, public_inputs);

        // Connect the public inputs of the stark proof to it's values.

        // Connect the padded chunks.
        for (int, register) in blake2b_input
            .padded_chunks
            .iter()
            .zip_eq(self.padded_chunks.iter().flat_map(|x| x.iter()))
        {
            let value = register.read_from_slice(public_inputs);
            let var = Self::value_to_variable(builder, value);
            builder.assert_is_equal(*int, var);
        }

        // Connect end_bits.
        for (end_bit, register) in blake2b_input.end_bits.iter().zip_eq(self.end_bits.iter()) {
            let value = register.read_from_slice(public_inputs);
            builder.assert_is_equal(end_bit.variable, value);
        }

        // Connect digest_bits.
        for (digest_bit, register) in blake2b_input
            .digest_bits
            .iter()
            .zip_eq(self.digest_bits.iter())
        {
            let value = register.read_from_slice(public_inputs);
            builder.assert_is_equal(digest_bit.variable, value);
        }

        // Connect digest_indices.
        for (digest_index, register) in blake2b_input
            .digest_indices
            .iter()
            .zip_eq(self.digest_indices.iter())
        {
            let value = register.read_from_slice(public_inputs);
            builder.assert_is_equal(*digest_index, value);
        }

        // Connect digests.
        for (digest, &register) in blake2b_input.digests.iter().zip_eq(self.digests.iter()) {
            let array: ArrayRegister<U64Register> = register;
            for (int, int_reg) in digest.iter().zip_eq(array.iter()) {
                let value = int_reg.read_from_slice(public_inputs);
                let var = Self::value_to_variable(builder, value);
                builder.watch(&var, "digest var");
                builder.assert_is_equal(*int, var);
            }
        }
    }

    fn value_to_variable(
        builder: &mut CircuitBuilder<L, D>,
        value: <U64Register as curta::chip::register::Register>::Value<Variable>,
    ) -> U64Variable {
        let low_limbs = &value[0..4];
        let high_limbs = &value[4..8];
        let mut acc_low = builder.zero::<Variable>();
        let mut acc_high = builder.zero::<Variable>();
        for (i, (low_byte, high_byte)) in low_limbs.iter().zip(high_limbs).enumerate() {
            let two_i = builder.constant::<Variable>(L::Field::from_canonical_u32(1 << (8 * i)));
            let two_i_low_byte = builder.mul(two_i, *low_byte);
            let two_i_high_byte = builder.mul(two_i, *high_byte);
            acc_low = builder.add(acc_low, two_i_low_byte);
            acc_high = builder.add(acc_high, two_i_high_byte);
        }
        let low_limb = U32Variable::from_variables_unsafe(&[acc_low]);
        let high_limb = U32Variable::from_variables_unsafe(&[acc_high]);
        U64Variable {
            limbs: [low_limb, high_limb],
        }
    }
}

/// The Curta Stark corresponding to the input data.
pub fn stark<L: PlonkParameters<D>, const D: usize>(
    parameters: BLAKE2BInputParameters,
) -> BLAKE2BStark<L, D> {
    println!("parameters are {:?}", parameters);

    let mut builder = BytesBuilder::<BLAKE2BAirParameters<L, D>>::new();

    let num_chunks = parameters.num_chunks;
    let padded_chunks = (0..num_chunks)
        .map(|_| builder.alloc_array_public::<U64Register>(16))
        .collect::<Vec<_>>();

    // Allocate registers for the public inputs to the Stark.
    let t_values = builder.alloc_array_public::<U64Register>(num_chunks);
    let end_bits = builder.alloc_array_public::<BitRegister>(num_chunks);
    let digest_bits = builder.alloc_array_public::<BitRegister>(num_chunks);
    let digest_indices = builder.alloc_array_public::<ElementRegister>(parameters.num_digests);
    let num_messages = builder.alloc_public::<ElementRegister>();

    // Hash the padded chunks.
    let digests = builder.blake2b(
        &padded_chunks,
        &t_values,
        &end_bits,
        &digest_bits,
        &digest_indices,
        &num_messages,
    );

    // Build the stark.
    let num_rows_degree = log2_ceil(96 * num_chunks);
    let num_rows = 1 << num_rows_degree;
    let stark = builder.build::<L::CurtaConfig, D>(num_rows);

    BLAKE2BStark {
        stark,
        padded_chunks,
        t_values,
        end_bits,
        digest_bits,
        digest_indices,
        digests,
        num_messages,
        num_rows,
    }
}

/// Get the input data for the stark from a `BLAKE2BAccelerator`.
pub(crate) fn get_blake2b_data<L: PlonkParameters<D>, const D: usize>(
    builder: &mut CircuitBuilder<L, D>,
    accelerator: BLAKE2BAccelerator,
) -> BLAKE2BInputData {
    // Initialze the data struictures of `BLAKE2BInputData`.
    let mut t_values = Vec::new();
    let mut end_bit_values = Vec::new();
    let mut digest_bits = Vec::new();
    let mut current_chunk_index = 0;
    let mut digest_indices = Vec::<Variable>::new();

    // Get the padded chunks from input messages, and assign the correct values of t_values, end_bit,
    // digest_bits, and digest_indices.
    //
    // `t_values` - the t values for each chunk.
    // `end_bit` - a bit that is true for the last chunk of a message (or the total_message
    // passed in the case of a message of variable length).
    // `digest_bit` - a bit that indicates the hash state is read into the digest list after
    // processing the chunk. For a message of fioxed length, this is the same as `end_bit`.
    // `digest_index` - the index of the digest to be read, corresponding to the location of the
    // chunk in the padded chunks.
    let padded_chunks = accelerator
        .blake2b_requests
        .iter()
        .flat_map(|req| {
            // For every request, we read the corresponding messagem, pad it, and compute the
            // corresponding chunk index.

            // Get the padded chunks and the number of chunks in the message, depending on the
            // type of the request.
            let (padded_chunks, length, last_chunk_index) = match req {
                BLAKE2BRequest::Fixed(input) => {
                    // If the length is fixed, the chunk_index is just `number of chunks  - 1``.
                    let padded_chunks = pad_blake2b_circuit(builder, input);
                    let num_chunks =
                        builder.constant((padded_chunks.len() / 16 - 1).try_into().unwrap());
                    let length = builder.constant::<U32Variable>(input.len() as u32);
                    (padded_chunks, length, num_chunks)
                }
                // If the length of the message is a variable, we read the chunk index from the
                // request.
                BLAKE2BRequest::Variable(input, length, last_chunk) => {
                    (pad_blake2b_circuit(builder, input), *length, *last_chunk)
                }
            };

            builder.watch(&last_chunk_index, "last chunk index");

            // Get the total number of chunks processed.
            let total_number_of_chunks = padded_chunks.len() / 16;
            // Store the end_bit values. The end bit indicates the end of message chunks.
            end_bit_values.extend_from_slice(&vec![false; total_number_of_chunks - 1]);
            end_bit_values.push(true);
            // The chunk index is given by the currenty index plus the chunk index we got from
            // the request.
            let current_chunk_index_variable =
                builder.constant::<Variable>(L::Field::from_canonical_usize(current_chunk_index));
            builder.watch(
                &current_chunk_index_variable,
                "current_chunk_index_variable",
            );
            let digest_index = builder.add(current_chunk_index_variable, last_chunk_index.variable);
            digest_indices.push(digest_index);
            // The digest bit is equal to zero for all chunks except the one that corresponds to
            // the `chunk_index`. We find the bits by comparing each value between 0 and the
            // total number of chunks to the `chunk_index`.
            let mut t_var = builder.constant::<U32Variable>(0);
            let chunk_size = builder.constant::<U32Variable>(128);
            for j in 0..total_number_of_chunks {
                t_var = builder.add(t_var, chunk_size);
                let j_var = builder.constant::<U32Variable>(j as u32);
                let at_digest_chunk = builder.is_equal(j_var, last_chunk_index);
                digest_bits.push(at_digest_chunk);

                t_var = builder.select(at_digest_chunk, length, t_var);
                t_values.push(t_var);
            }
            // Increment the current chunk index by the total number of chunks.
            current_chunk_index += total_number_of_chunks;

            padded_chunks
        })
        .collect::<Vec<_>>();

    // Convert end_bits to variables.
    let end_bits = builder.constant_vec(&end_bit_values);

    BLAKE2BInputData {
        padded_chunks,
        t_values,
        digest_bits,
        end_bits,
        digest_indices,
        digests: accelerator.blake2b_responses,
    }
}

// Need to make the message 128 byte aligned
fn pad_blake2b_circuit<L: PlonkParameters<D>, const D: usize>(
    builder: &mut CircuitBuilder<L, D>,
    input: &[ByteVariable],
) -> Vec<U64Variable> {
    let num_pad_bytes = 128 - (input.len() % 128);

    let mut padded_message = Vec::new();
    padded_message.extend_from_slice(input);

    for _ in 0..num_pad_bytes {
        padded_message.push(builder.zero());
    }

    padded_message
        .chunks_exact(8)
        .map(|bytes| {
            let mut bytes_copy = Vec::new();
            bytes_copy.extend_from_slice(bytes);
            bytes_copy.reverse();
            U64Variable::decode(builder, &bytes_copy)
        })
        .collect()
}

pub(crate) fn digest_to_array<L: PlonkParameters<D>, const D: usize>(
    builder: &mut CircuitBuilder<L, D>,
    digest: Bytes32Variable,
) -> [U64Variable; 4] {
    digest
        .as_bytes()
        .chunks_exact(8)
        .map(|x| {
            let mut x_copy = Vec::new();
            x_copy.extend_from_slice(x);
            x_copy.reverse();
            U64Variable::decode(builder, &x_copy)
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

pub(crate) fn compute_blake2b_last_chunk_index<L: PlonkParameters<D>, const D: usize>(
    builder: &mut CircuitBuilder<L, D>,
    input_byte_length: U32Variable,
) -> U32Variable {
    let chunk_size = builder.constant::<U32Variable>(128);
    builder.div(input_byte_length, chunk_size)
}
