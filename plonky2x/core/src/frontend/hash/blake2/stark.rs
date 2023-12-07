use curta::chip::register::array::ArrayRegister;
use curta::chip::register::bit::BitRegister;
use curta::chip::register::element::ElementRegister;
use curta::chip::register::Register;
use curta::chip::trace::writer::{InnerWriterData, TraceWriter};
use curta::chip::uint::register::U64Register;
use curta::chip::uint::util::u64_to_le_field_bytes;
use curta::chip::{AirParameters, Chip};
use curta::machine::bytes::proof::ByteStarkProof;
use curta::machine::bytes::stark::ByteStark;
use curta::machine::hash::blake::blake2b::register::BLAKE2BDigestRegister;
use curta::math::prelude::*;
use curta::plonky2::Plonky2Air;
use itertools::Itertools;
use log::debug;
use plonky2::util::timing::TimingTree;

use super::data::{BLAKE2BInputData, BLAKE2BInputDataValues};
use crate::frontend::curta::proof::ByteStarkProofVariable;
use crate::prelude::{
    CircuitBuilder, CircuitVariable, PlonkParameters, U32Variable, U64Variable, Variable,
};

#[derive(Debug, Clone)]
pub struct BLAKE2BStark<L: PlonkParameters<D>, A: AirParameters, const D: usize> {
    pub stark: ByteStark<A, L::CurtaConfig, D>,
    pub padded_chunks: Vec<ArrayRegister<U64Register>>,
    pub t_values: Vec<U64Register>,
    pub end_bits: ArrayRegister<BitRegister>,
    pub digest_bits: ArrayRegister<BitRegister>,
    pub digest_indices: ArrayRegister<ElementRegister>,
    pub digests: Vec<BLAKE2BDigestRegister>,
    pub num_rows: usize,
}

impl<
        L: PlonkParameters<D>,
        A: AirParameters<Field = L::Field, CubicParams = L::CubicParams>,
        const D: usize,
    > BLAKE2BStark<L, A, D>
where
    Chip<A>: Plonky2Air<L::Field, D>,
{
    fn write_input(&self, writer: &TraceWriter<L::Field>, input: BLAKE2BInputDataValues<L, D>) {
        for (digest_index_reg, digest_index) in
            self.digest_indices.iter().zip(input.digest_indices.iter())
        {
            writer.write(&digest_index_reg, digest_index, 0);
        }

        for (digest_reg, digest_value) in self.digests.iter().zip(input.digests.iter()) {
            let array: ArrayRegister<_> = (*digest_reg).into();
            writer.write_array(&array, digest_value.map(u64_to_le_field_bytes), 0);
        }

        for (chunk, chunk_value) in self
            .padded_chunks
            .iter()
            .zip(input.padded_chunks.chunks_exact(16))
        {
            writer.write_array(
                chunk,
                chunk_value.iter().map(|x| u64_to_le_field_bytes(*x)),
                0,
            );
        }

        for (t, t_value) in self.t_values.iter().zip(input.t_values.iter()) {
            writer.write(t, &u64_to_le_field_bytes(*t_value), 0);
        }

        for (end_bit, end_bit_value) in self.end_bits.iter().zip(input.end_bits) {
            writer.write(
                &end_bit,
                &L::Field::from_canonical_u8(end_bit_value as u8),
                0,
            );
        }

        for (digest_bit, digest_bit_value) in self.digest_bits.iter().zip(input.digest_bits) {
            writer.write(
                &digest_bit,
                &L::Field::from_canonical_u8(digest_bit_value as u8),
                0,
            );
        }
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
            let array: ArrayRegister<U64Register> = register.into();
            for (int, int_reg) in digest.iter().zip_eq(array.iter()) {
                let value = int_reg.read_from_slice(public_inputs);
                let var = Self::value_to_variable(builder, value);
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
