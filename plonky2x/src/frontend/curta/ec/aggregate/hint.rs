use curta::chip::ec::gadget::EllipticCurveWriter;
use curta::chip::ec::point::AffinePoint;
use curta::chip::ec::weierstrass::bn254::Bn254;
use curta::chip::trace::generator::ArithmeticGenerator;
use curta::maybe_rayon::*;
use curta::plonky2::stark::config::StarkyConfig;
use curta::plonky2::stark::prover::StarkyProver;
use curta::plonky2::stark::verifier::StarkyVerifier;
use curta::plonky2::stark::Starky;
use log::debug;
use serde::{Deserialize, Serialize};

use super::air::PKAir;
use crate::frontend::curta::ec::point::AffinePointVariable;
use crate::frontend::hint::simple::hint::Hint;
use crate::prelude::{PlonkParameters, ValueStream, *};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bn254PKHint {
    num_keys_degree: usize,
}

type E = Bn254;

impl<L: PlonkParameters<D>, const D: usize> Hint<L, D> for Bn254PKHint {
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let num_keys = 1 << self.num_keys_degree;
        let num_rows = 1 << 16;
        let stride = 1 << (16 - self.num_keys_degree);

        let mut public_keys_values = Vec::with_capacity(num_keys);
        let mut selector_values = Vec::with_capacity(num_keys);
        for _ in 0..num_keys {
            let pk: AffinePoint<E> = input_stream.read_value::<AffinePointVariable<E>>().into();
            let selector = input_stream.read_value::<BoolVariable>();
            public_keys_values.push(pk);
            selector_values.push(L::Field::from_canonical_u8(selector as u8));
        }

        let PKAir {
            air,
            trace_data,
            public_keys,
            selectors,
            aggregated_pk,
            current,
            flag,
        } = PKAir::<L::Field, L::CubicParams, E>::new(self.num_keys_degree);

        let trace_generator = ArithmeticGenerator::new(trace_data, num_rows);
        let writer = trace_generator.new_writer();

        public_keys
            .par_iter()
            .zip(public_keys_values.par_iter())
            .for_each(|(pk, pk_value)| {
                writer.write_ec_point(pk, pk_value, 0);
            });

        selectors
            .iter()
            .zip(selector_values.iter())
            .for_each(|(s, b)| {
                writer.write(&s, b, 0);
            });

        let base = E::generator();
        let aggregated_pk_value =
            public_keys_values
                .iter()
                .zip(selector_values.iter())
                .fold(base, |agg, (pk, b)| {
                    if *b == L::Field::ONE {
                        agg.sw_add(pk)
                    } else {
                        agg
                    }
                });
        writer.write_ec_point(&aggregated_pk, &aggregated_pk_value, 0);

        writer.write_global_instructions(&trace_generator.air_data);
        (0..num_rows).for_each(|i| {
            if i % stride == 0 {
                let k = i / stride;
                writer.write(&flag, &selector_values[k], i);
            }
            writer.write_ec_point(&current, &public_keys_values[i / stride], i);
            writer.write_row_instructions(&trace_generator.air_data, i);
        });

        let stark = Starky::new(air);
        let config = StarkyConfig::standard_fast_config(num_rows);

        let public_inputs = writer.public().unwrap().clone();

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
        output_stream.write_value::<AffinePointVariable<E>>(aggregated_pk_value.into());
        output_stream.write_stark_proof(proof);
        output_stream.write_slice(&public_inputs);
    }
}

#[cfg(test)]
mod tests {

    use curta::math::goldilocks::cubic::GoldilocksCubicParameters;
    use num_bigint::RandBigInt;
    use rand::{thread_rng, Rng};

    use super::*;
    use crate::prelude::*;
    use crate::utils::setup_logger;

    #[test]
    fn test_pk_hint() {
        setup_logger();

        type L = DefaultParameters;
        type F = GoldilocksField;
        type R = GoldilocksCubicParameters;
        type C = <DefaultParameters as PlonkParameters<2>>::CurtaConfig;

        let num_keys_degree = 6;
        let num_keys = 1 << num_keys_degree;
        let num_rows = 1 << 16;

        let mut builder = DefaultBuilder::new();

        let air = PKAir::<F, R, Bn254>::new(num_keys_degree).air;

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

        let mut input_stream = VariableStream::new();
        for (pk, b) in public_keys.iter().zip(selectors.iter()) {
            input_stream.write::<AffinePointVariable<Bn254>>(pk);
            input_stream.write::<BoolVariable>(b);
        }

        let hint = Bn254PKHint { num_keys_degree };
        let outputs = builder.hint(input_stream, hint);

        let aggregated_pk = outputs.read::<AffinePointVariable<Bn254>>(&mut builder);
        let proof = outputs.read_stark_proof(&mut builder, &stark, &config);
        let public_inputs = outputs.read_exact(&mut builder, stark.air.num_public_values);
        builder.verify_stark_proof(&config, &stark, &proof, &public_inputs);

        let circuit = builder.build();
        let input = circuit.input();

        let proof = circuit.prove(&input);
    }
}
