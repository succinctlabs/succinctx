use curta::chip::ec::edwards::ed25519::gadget::CompressedPointWriter;
use curta::chip::ec::edwards::ed25519::params::Ed25519BaseField;
use curta::chip::trace::generator::ArithmeticGenerator;
use curta::maybe_rayon::*;
use curta::plonky2::stark::config::StarkyConfig;
use curta::plonky2::stark::prover::StarkyProver;
use curta::plonky2::stark::verifier::StarkyVerifier;
use curta::plonky2::stark::Starky;
use curve25519_dalek::edwards::CompressedEdwardsY;
use log::debug;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

use crate::frontend::curta::ec::point::CompressedEdwardsYVariable;
use crate::frontend::curta::ec::sig_verification::air::Ed25519VerificationAir;
use crate::frontend::curta::field::variable::FieldVariable;
use crate::frontend::hint::simple::hint::Hint;
use crate::prelude::{PlonkParameters, ValueStream};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ed25519VerificationHint {}

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for Ed25519VerificationHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        const NUM_VERIFICATIONS: usize = 256;

        let mut public_keys_values = Vec::with_capacity(NUM_VERIFICATIONS);
        let mut sigr_values = Vec::with_capacity(NUM_VERIFICATIONS);
        let mut sigs_values = Vec::with_capacity(NUM_VERIFICATIONS);
        let mut h_values = Vec::with_capacity(NUM_VERIFICATIONS);

        for _ in 0..NUM_VERIFICATIONS {
            let pk: CompressedEdwardsY = input_stream
                .read_value::<CompressedEdwardsYVariable>()
                .into();
            let sig_r: CompressedEdwardsY = input_stream
                .read_value::<CompressedEdwardsYVariable>()
                .into();
            let sig_s: BigUint = input_stream
                .read_value::<FieldVariable<Ed25519BaseField>>()
                .into();
            let h: BigUint = input_stream
                .read_value::<FieldVariable<Ed25519BaseField>>()
                .into();
            public_keys_values.push(pk);
            sigr_values.push(sig_r);
            sigs_values.push(sig_s);
            h_values.push(h);
        }

        let Ed25519VerificationAir {
            air,
            trace_data,
            public_keys,
            sigr_s,
            sigs_s,
            h_s,
        } = Ed25519VerificationAir::<L::Field, L::CubicParams>::new();

        let trace_generator = ArithmeticGenerator::new(trace_data, 1 << 16);
        let writer = trace_generator.new_writer();

        public_keys
            .par_iter()
            .zip(public_keys_values.par_iter())
            .for_each(|(pk, pk_value)| {
                writer.write_ec_compressed_point(pk, pk_value, 0);
            });

        sigr_s
            .par_iter()
            .zip(sigr_values.par_iter())
            .for_each(|(sigr, sigr_value)| {
                writer.write_ec_compressed_point(sigr, sigr_value, 0);
            });

        sigs_s
            .par_iter()
            .zip(sigs_values.par_iter())
            .for_each(|(sigs, sigs_value)| {
                writer.write(sigs, sigs_value, 0);
            });

        h_s.par_iter()
            .zip(h_values.par_iter())
            .for_each(|(h, h_value)| {
                writer.write(h, h_value, 0);
            });

        writer.write_global_instructions(&trace_generator.air_data);

        let stark = Starky::new(air);
        let config = StarkyConfig::standard_fast_config(1 << 16);

        let public_inputs: Vec<L::Field> = writer.public().unwrap().clone();

        let proof = StarkyProver::<L::Field, L::CurtaConfig, D>::prove(
            &config,
            &stark,
            &trace_generator,
            &public_inputs,
        )
        .unwrap();
        debug!("Generated proof");

        // Verify the proof to make sure it's valid.
        StarkyVerifier::verify(&config, &stark, proof.clone(), &public_inputs).unwrap();

        // Return the aggregated public key and the proof.
        output_stream.write_stark_proof(proof);
        output_stream.write_slice(&public_inputs);
    }
}

#[cfg(test)]
mod tests {

    use curta::air::RAirData;
    use curta::math::goldilocks::cubic::GoldilocksCubicParameters;

    use super::*;
    use crate::frontend::curta::field::variable::FieldVariable;
    use crate::prelude::*;
    use crate::utils::setup_logger;

    const NUM_SIGS: usize = 1;

    pub const H: [&str; NUM_SIGS] =
        ["9f81b8e0cf40cb8ec8f0bdcf8d4f7a9b56002e7e04b1ffe9790d27974519eb06"];

    pub const SIGS: [&str; NUM_SIGS] = [
        "186953830bb9c1f18b8ed096eef919bf6409d1921b8dc698fb39ca8135c2575f0b6feb4f0383b70f5156a99cd8210ec516bd1d702bdc8444ed3172b5f42fb008"];

    pub const PUB_KEYS: [&str; NUM_SIGS] =
        ["02f80695f0a4a2308246c88134b2de759e347d527189742dd42e98724bd5a9bc"];

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_ed25519_verification_hint() {
        setup_logger();

        type F = GoldilocksField;
        type R = GoldilocksCubicParameters;
        type C = <DefaultParameters as PlonkParameters<2>>::CurtaConfig;

        let num_rows = 1 << 16;
        const NUM_VERIFICATIONS: usize = 256;

        let mut builder = DefaultBuilder::new();

        let Ed25519VerificationAir { air, .. } = Ed25519VerificationAir::<F, R>::new();

        let stark = Starky::new(air);
        let config = StarkyConfig::<C, 2>::standard_fast_config(num_rows);

        let mut input_stream = VariableStream::new();
        for i in 0..NUM_VERIFICATIONS {
            let compressed_p_bytes = hex::decode(PUB_KEYS[i % NUM_SIGS]).unwrap();
            let compressed_p = builder.constant::<CompressedEdwardsYVariable>(CompressedEdwardsY(
                compressed_p_bytes.try_into().unwrap(),
            ));

            let sig_bytes = hex::decode(SIGS[i % NUM_SIGS]).unwrap();
            let sig_r = builder.constant::<CompressedEdwardsYVariable>(CompressedEdwardsY(
                sig_bytes[0..32].try_into().unwrap(),
            ));

            let sig_s_biguint = builder.constant::<FieldVariable<Ed25519BaseField>>(
                BigUint::from_bytes_le(sig_bytes[32..64].try_into().unwrap()),
            );

            let h_biguint = builder.constant::<FieldVariable<Ed25519BaseField>>(
                BigUint::from_bytes_le(&hex::decode(H[i % NUM_SIGS]).unwrap()),
            );

            input_stream.write::<CompressedEdwardsYVariable>(&compressed_p);
            input_stream.write::<CompressedEdwardsYVariable>(&sig_r);
            input_stream.write::<FieldVariable<Ed25519BaseField>>(&sig_s_biguint);
            input_stream.write::<FieldVariable<Ed25519BaseField>>(&h_biguint);
        }

        let hint = Ed25519VerificationHint {};
        let outputs = builder.hint(input_stream, hint);

        // Read the stark proof and stark public inputs from the output stream.
        let proof = outputs.read_stark_proof(&mut builder, &stark, &config);
        let public_inputs = outputs.read_exact_unsafe(&mut builder, stark.air.num_public_inputs());
        builder.verify_stark_proof(&config, &stark, &proof, &public_inputs);

        let circuit = builder.build();
        let input = circuit.input();

        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }
}
