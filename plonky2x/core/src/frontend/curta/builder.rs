use curta::chip::{AirParameters, Chip};
use curta::machine::bytes::proof::ByteStarkProofTarget;
use curta::machine::bytes::stark::ByteStark;
use curta::machine::emulated::proof::EmulatedStarkProofTarget;
use curta::machine::emulated::stark::EmulatedStark;
use curta::plonky2::stark::config::StarkyConfig;
use curta::plonky2::stark::gadget::StarkGadget;
use curta::plonky2::stark::proof::StarkProofTarget;
use curta::plonky2::stark::Starky;
use curta::plonky2::Plonky2Air;

use super::proof::{ByteStarkProofVariable, EmulatedStarkProofVariable, StarkProofVariable};
use crate::prelude::*;

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn verify_stark_proof<A: Plonky2Air<L::Field, D>>(
        &mut self,
        config: &StarkyConfig<L::CurtaConfig, D>,
        stark: &Starky<A>,
        proof: StarkProofVariable<D>,
        public_inputs: &[Variable],
    ) {
        let proof_target = StarkProofTarget::from(proof);
        let public_inputs_target = public_inputs.iter().map(|v| v.0).collect::<Vec<_>>();
        self.api
            .verify_stark_proof(config, stark, &proof_target, &public_inputs_target);
    }

    pub fn verify_byte_stark_proof<P>(
        &mut self,
        byte_stark: &ByteStark<P, L::CurtaConfig, D>,
        proof: ByteStarkProofVariable<D>,
        public_inputs: &[Variable],
    ) where
        P: AirParameters<Field = L::Field, CubicParams = L::CubicParams>,
        Chip<P>: Plonky2Air<L::Field, D>,
    {
        let proof_target = ByteStarkProofTarget::from(proof);
        let public_inputs_target = public_inputs.iter().map(|v| v.0).collect::<Vec<_>>();
        byte_stark.verify_circuit(&mut self.api, &proof_target, &public_inputs_target);
    }

    pub fn verify_emulated_stark_proof<P>(
        &mut self,
        emulated_stark: &EmulatedStark<P, L::CurtaConfig, D>,
        proof: EmulatedStarkProofVariable<D>,
        public_inputs: &[Variable],
    ) where
        P: AirParameters<Field = L::Field, CubicParams = L::CubicParams>,
        Chip<P>: Plonky2Air<L::Field, D>,
    {
        let proof_target = EmulatedStarkProofTarget::from(proof);
        let public_inputs_target = public_inputs.iter().map(|v| v.0).collect::<Vec<_>>();
        emulated_stark.verify_circuit(&mut self.api, &proof_target, &public_inputs_target);
    }
}

#[cfg(test)]
mod tests {
    use curta::chip::memory::time::Time;
    use curta::chip::register::array::ArrayRegister;
    use curta::chip::register::element::ElementRegister;
    use curta::chip::register::Register;
    use curta::chip::trace::writer::{InnerWriterData, TraceWriter};
    use curta::chip::uint::operations::instruction::UintInstruction;
    use curta::chip::uint::register::U32Register;
    use curta::chip::uint::util::u32_to_le_field_bytes;
    use curta::chip::AirParameters;
    use curta::machine::builder::Builder;
    use curta::machine::bytes::builder::BytesBuilder;
    use curta::machine::bytes::stark::ByteStark;
    use curta::math::goldilocks::cubic::GoldilocksCubicParameters;
    use curta::plonky2::stark::config::CurtaPoseidonGoldilocksConfig;
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::util::timing::TimingTree;
    use serde::{Deserialize, Serialize};

    use crate::frontend::hint::simple::hint::Hint;
    use crate::prelude::*;
    use crate::utils::setup_logger;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ByteSliceMemTest;

    impl AirParameters for ByteSliceMemTest {
        type Field = GoldilocksField;
        type CubicParams = GoldilocksCubicParameters;

        type Instruction = UintInstruction;

        const NUM_ARITHMETIC_COLUMNS: usize = 0;
        const NUM_FREE_COLUMNS: usize = 18;
        const EXTENDED_COLUMNS: usize = 24;
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ByteMemHint {
        stark: ByteStark<ByteSliceMemTest, CurtaPoseidonGoldilocksConfig, 2>,
        a_init: ArrayRegister<U32Register>,
        a_final: U32Register,
        num_rows: usize,
    }

    impl Hint<DefaultParameters, 2> for ByteMemHint {
        fn hint(
            &self,
            input_stream: &mut ValueStream<DefaultParameters, 2>,
            output_stream: &mut ValueStream<DefaultParameters, 2>,
        ) {
            let a_init = input_stream
                .read_value::<ArrayVariable<U32Variable, 4>>()
                .into_iter()
                .map(u32_to_le_field_bytes);

            let writer = TraceWriter::new(&self.stark.air_data, self.num_rows);

            writer.write_array(&self.a_init, a_init, 0);
            writer.write_global_instructions(&self.stark.air_data);
            for i in 0..self.num_rows {
                writer.write_row_instructions(&self.stark.air_data, i);
            }

            let InnerWriterData { trace, public, .. } = writer.into_inner().unwrap();
            let proof = self
                .stark
                .prove(&trace, &public, &mut TimingTree::default())
                .unwrap();

            self.stark.verify(proof.clone(), &public).unwrap();

            output_stream.write_byte_stark_proof(proof);
            output_stream.write_slice(&public);
        }
    }

    #[test]
    fn test_byte_slice_memory_multi_stark() {
        setup_logger();
        let mut air_builder = BytesBuilder::<ByteSliceMemTest>::new();

        let a_init = air_builder.alloc_array_public::<U32Register>(4);

        let num_rows = 1 << 5;

        let a_ptr = air_builder.initialize_slice::<U32Register>(&a_init, &Time::zero(), None);

        let num_rows_reg = air_builder
            .api
            .constant::<ElementRegister>(&GoldilocksField::from_canonical_usize(num_rows));
        air_builder.store(
            &a_ptr.get(1),
            a_init.get(1),
            &Time::zero(),
            Some(num_rows_reg),
        );

        let clk = Time::from_element(air_builder.clk);
        let zero = air_builder.constant::<ElementRegister>(&GoldilocksField::ZERO);

        let a_0 = a_ptr.get_at(zero);
        let zero_trace = air_builder.alloc::<ElementRegister>();
        air_builder.set_to_expression(&zero_trace, GoldilocksField::ZERO.into());
        let a_0_trace = a_ptr.get_at(zero_trace);
        let a = air_builder.load(&a_0_trace, &clk);
        let b = air_builder.load(&a_ptr.get(1), &Time::zero());
        let c = air_builder.and(&a, &b);
        air_builder.store(&a_0_trace, c, &clk.advance(), None);

        let a_final = air_builder.api.alloc_public::<U32Register>();

        air_builder.free(&a_0, a_final, &Time::constant(num_rows));
        air_builder.set_to_expression_last_row(&a_final, c.expr());

        for (i, a) in a_init.iter().enumerate().skip(1) {
            air_builder.free(&a_ptr.get(i), a, &Time::zero());
        }

        let stark = air_builder.build::<CurtaPoseidonGoldilocksConfig, 2>(num_rows);

        let hint = ByteMemHint {
            stark: stark.clone(),
            a_init,
            a_final,
            num_rows,
        };

        let mut builder = DefaultBuilder::new();

        let a_init = builder.read::<ArrayVariable<U32Variable, 4>>();

        let mut input_stream = VariableStream::new();
        input_stream.write(&a_init);

        let output_stream = builder.hint(input_stream, hint);

        let proof = output_stream.read_byte_stark_proof(&mut builder, &stark);
        let num_public_inputs = stark.air_data.num_public_inputs;
        let public_inputs = output_stream.read_vec::<Variable>(&mut builder, num_public_inputs);
        builder.verify_byte_stark_proof(&stark, proof, &public_inputs);

        let circuit = builder.build();
        let mut input = circuit.input();

        input.write::<ArrayVariable<U32Variable, 4>>(vec![0, 1, 2, 3]);

        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }
}
