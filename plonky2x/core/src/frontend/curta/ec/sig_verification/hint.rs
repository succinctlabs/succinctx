use curta::chip::ec::gadget::EllipticCurveWriter;
use curta::chip::ec::weierstrass::bn254::Bn254;
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
pub struct Ed25519VerificationHint {
    pub num_keys_degree: usize,
}

type E = Bn254;

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for Ed25519VerificationHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        const NUM_VERIFICATIONS: usize = 256;

        let mut public_keys_values = Vec::with_capacity(NUM_VERIFICATIONS);
        let mut sigr_values = Vec::with_capacity(NUM_VERIFICATIONS);
        let mut sigs_values = Vec::with_capacity(NUM_VERIFICATIONS);
        let mut h_values = Vec::with_capacity(NUM_VERIFICATIONS);

        for _ in 0..NUM_VERIFICATIONS {
            let pk: CompressedEdwardsY = input_stream
                .read_value::<CompressedEdwardsYVariable<E>>()
                .into();
            let sig_r: CompressedEdwardsY = input_stream
                .read_value::<CompressedEdwardsYVariable<E>>()
                .into();
            let sig_s: BigUint = input_stream
                .read_value::<FieldVariable<E::ScalarField>>()
                .into();
            let h: BigUint = input_stream
                .read_value::<FieldVariable<E::ScalarField>>()
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
        } = Ed25519VerificationAir::<L::Field, L::CubicParams, E>::new();

        let trace_generator = ArithmeticGenerator::new(trace_data, 1 << 16);
        let writer = trace_generator.new_writer();

        public_keys
            .par_iter()
            .zip(public_keys_values.par_iter())
            .for_each(|(pk, pk_value)| {
                writer.write_ec_point(pk, pk_value, 0);
            });

        sigr_s
            .par_iter()
            .zip(sigr_values.par_iter())
            .for_each(|(sigr, sigr_value)| {
                writer.write_ec_point(sigr, sigr_value, 0);
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
    use curta::chip::register::Register;
    use curta::math::goldilocks::cubic::GoldilocksCubicParameters;
    use num_bigint::RandBigInt;
    use rand::{thread_rng, Rng};

    use super::*;
    use crate::frontend::curta::field::variable::FieldVariable;
    use crate::prelude::*;
    use crate::utils::setup_logger;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_pk_hint() {
        setup_logger();

        type F = GoldilocksField;
        type R = GoldilocksCubicParameters;
        type C = <DefaultParameters as PlonkParameters<2>>::CurtaConfig;

        let num_keys_degree = 6;
        let num_keys = 1 << num_keys_degree;
        let num_rows = 1 << 16;

        let mut builder = DefaultBuilder::new();

        let PKAir {
            air, aggregated_pk, ..
        } = PKAir::<F, R, Bn254>::new(num_keys_degree);

        let stark = Starky::new(air);
        let config = StarkyConfig::<C, 2>::standard_fast_config(num_rows);

        let base = Bn254::generator();
        let public_keys_values = (0..num_keys)
            .into_par_iter()
            .map(|_| {
                let mut rng = thread_rng();
                let sk = rng.gen_biguint(256);
                base.sw_scalar_mul(&sk)
            })
            .collect::<Vec<_>>();

        let mut rng = thread_rng();
        let selector_values = (0..num_keys).map(|_| rng.gen_bool(0.5)).collect::<Vec<_>>();

        let public_keys = public_keys_values
            .iter()
            .map(|pk| builder.constant::<AffinePointVariable<Bn254>>(pk.clone().into()))
            .collect::<Vec<_>>();
        let selectors = selector_values
            .iter()
            .map(|b| builder.constant::<BoolVariable>(*b))
            .collect::<Vec<_>>();

        let agg_pk_value = public_keys_values
            .iter()
            .zip(selector_values.iter())
            .fold(base, |agg, (pk, b)| if *b { agg.sw_add(pk) } else { agg });

        let mut input_stream = VariableStream::new();
        for (pk, b) in public_keys.iter().zip(selectors.iter()) {
            input_stream.write::<AffinePointVariable<Bn254>>(pk);
            input_stream.write::<BoolVariable>(b);
        }

        let hint = Bn254PKHint { num_keys_degree };
        let outputs = builder.hint(input_stream, hint);

        // Read the stark proof and stark public inputs from the output stream.
        let proof = outputs.read_stark_proof(&mut builder, &stark, &config);
        let public_inputs = outputs.read_exact_unsafe(&mut builder, stark.air.num_public_inputs());
        builder.verify_stark_proof(&config, &stark, &proof, &public_inputs);

        let aggregated_pk = AffinePointVariable::<Bn254> {
            x: FieldVariable::new(
                aggregated_pk
                    .x
                    .read_from_slice(&public_inputs)
                    .as_coefficients(),
            ),
            y: FieldVariable::new(
                aggregated_pk
                    .y
                    .read_from_slice(&public_inputs)
                    .as_coefficients(),
            ),
        };

        builder.write(aggregated_pk);

        let circuit = builder.build();
        let input = circuit.input();

        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);

        // Read the aggregated public key from the output and compare to the expected value.
        let agg_pk_output = AffinePoint::from(output.read::<AffinePointVariable<Bn254>>());
        assert_eq!(agg_pk_output, agg_pk_value);
    }
}
