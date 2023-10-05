use core::marker::PhantomData;

use curta::chip::builder::AirBuilder;
use curta::chip::hash::sha::sha256::generator::{SHA256AirParameters, SHA256StarkData};
use curta::chip::hash::sha::sha256::SHA256PublicData;
use curta::chip::trace::generator::ArithmeticGenerator;
use curta::plonky2::stark::config::StarkyConfig;
use curta::plonky2::stark::prover::StarkyProver;
use curta::plonky2::stark::Starky;
use plonky2::field::types::PrimeField64;
use serde::{Deserialize, Serialize};

use crate::frontend::hint::simple::hint::Hint;
use crate::prelude::{PlonkParameters, ValueStream, *};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sha256ProofHint<L: PlonkParameters<D>, const D: usize> {
    pub num_messages: usize,
    pub _phantom: PhantomData<L>,
}

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for Sha256ProofHint<L, D> {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let SHA256StarkData {
            stark,
            table,
            trace_generator,
            config,
            gadget,
        } = Self::stark_data();

        let padded_messages = input_stream
            .read_exact(1024 * 64)
            .iter()
            .map(|b| b.to_canonical_u64() as u8)
            .collect::<Vec<_>>();

        let chunk_sizes = input_stream.read_vec::<Variable>(self.num_messages);

        let message_chunks = chunk_sizes.iter().scan(0, |idx, size| {
            let size = size.to_canonical_u64() as usize;
            let chunk = padded_messages[*idx..*idx + 64 * size].to_vec();
            *idx += 64 * size;
            Some(chunk)
        });

        // Write trace values
        let num_rows = 1 << 16;
        let writer = trace_generator.new_writer();
        table.write_table_entries(&writer);
        let sha_public_values = gadget.write(message_chunks, &writer);
        for i in 0..num_rows {
            writer.write_row_instructions(&trace_generator.air_data, i);
        }

        let public_inputs: Vec<_> = writer.public().unwrap().clone();

        let proof = StarkyProver::<L::Field, L::CurtaConfig, D>::prove(
            &config,
            &stark,
            &trace_generator,
            &public_inputs,
        )
        .unwrap();

        output_stream.write_stark_proof(proof);

        let SHA256PublicData {
            public_w,
            hash_state,
            ..
        } = sha_public_values;

        output_stream.write_slice(&public_w.into_iter().flatten().collect::<Vec<_>>());

        output_stream.write_slice(&hash_state.into_iter().flatten().collect::<Vec<_>>());
    }
}

impl<L: PlonkParameters<D>, const D: usize> Sha256ProofHint<L, D> {
    pub fn stark_data() -> SHA256StarkData<L::Field, L::CubicParams, L::CurtaConfig, D> {
        let mut air_builder = AirBuilder::<SHA256AirParameters<L::Field, L::CubicParams>>::new();
        let clk = air_builder.clock();

        let (mut operations, table) = air_builder.byte_operations();

        let mut bus = air_builder.new_bus();
        let channel_idx = bus.new_channel(&mut air_builder);

        let gadget =
            air_builder.process_sha_256_batch(&clk, &mut bus, channel_idx, &mut operations);

        air_builder.register_byte_lookup(operations, &table);
        air_builder.constrain_bus(bus);

        let (air, trace_data) = air_builder.build();

        let num_rows = 1 << 16;
        let stark = Starky::new(air);
        let config = StarkyConfig::<L::CurtaConfig, D>::standard_fast_config(num_rows);

        let trace_generator =
            ArithmeticGenerator::<SHA256AirParameters<L::Field, L::CubicParams>>::new(
                trace_data, num_rows,
            );

        SHA256StarkData {
            stark,
            table,
            trace_generator,
            config,
            gadget,
        }
    }
}
