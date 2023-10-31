use curta::air::RAirData;
use curta::chip::ec::weierstrass::bn254::Bn254;
use curta::chip::ec::EllipticCurve;
use curta::plonky2::stark::config::StarkyConfig;
use curta::plonky2::stark::Starky;

use crate::frontend::curta::ec::aggregate::air::PKAir;
use crate::frontend::curta::ec::aggregate::hint::Bn254PKHint;
use crate::frontend::curta::ec::point::AffinePointVariable;
use crate::prelude::*;

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn bn254_aggregate(
        &mut self,
        public_keys: &[AffinePointVariable<Bn254>],
        selectors: &[BoolVariable],
    ) -> AffinePointVariable<Bn254> {
        assert_eq!(public_keys.len(), selectors.len());
        assert!(public_keys.len() < 1 << 14);

        let base = Bn254::ec_generator();
        let base_neg_value = -&base;
        let base_neg = self.constant::<AffinePointVariable<Bn254>>(base_neg_value.into());

        let num_keys_no_pad = public_keys.len() + 1;
        let num_keys_degree = num_keys_no_pad.next_power_of_two().trailing_zeros() as usize;
        let num_keys = 1 << num_keys_degree;
        let num_rows = 1 << 16;

        // Pad the public keys with the base point to make the number of keys degree a power of two.
        let public_keys = public_keys
            .iter()
            .cloned()
            .chain(
                (num_keys_no_pad..num_keys)
                    .map(|_| self.constant::<AffinePointVariable<Bn254>>(base.clone().into())),
            )
            .collect::<Vec<_>>();

        // Pad selectors.
        let selectors = selectors
            .iter()
            .copied()
            .chain((num_keys_no_pad..num_keys).map(|_| self.constant::<BoolVariable>(false)))
            .collect::<Vec<_>>();

        // Initialize the stark gadget for aggregation.
        let PKAir {
            air, aggregated_pk, ..
        } = PKAir::<L::Field, L::CubicParams, Bn254>::new(num_keys_degree);

        let stark = Starky::new(air);
        let config = StarkyConfig::<L::CurtaConfig, D>::standard_fast_config(num_rows);

        // Write the public keys and selectors to the input stream.
        let mut input_stream = VariableStream::new();
        for (pk, b) in public_keys.into_iter().zip(selectors) {
            input_stream.write::<AffinePointVariable<Bn254>>(&pk);
            input_stream.write::<BoolVariable>(&b);
        }
        input_stream.write::<AffinePointVariable<Bn254>>(&base_neg);
        input_stream.write::<BoolVariable>(&self.constant::<BoolVariable>(true));

        let hint = Bn254PKHint { num_keys_degree };
        let outputs = self.hint(input_stream, hint);

        // Read the stark proof and stark public inputs from the output stream.
        let proof = outputs.read_stark_proof(self, &stark, &config);

        // The stark proof will verify that the public inputs are range checked.
        let public_inputs = outputs.read_exact_unsafe(self, stark.air.num_public_inputs());
        self.verify_stark_proof(&config, &stark, proof.clone(), &public_inputs);

        // Read the aggregated public key from the stark public inputs.
        AffinePointVariable::read_from_stark(&aggregated_pk, &public_inputs)
    }
}

#[cfg(test)]
mod tests {

    use curta::chip::ec::point::AffinePoint;
    use curta::maybe_rayon::*;
    use num_bigint::RandBigInt;
    use rand::{thread_rng, Rng};

    use super::*;
    use crate::utils::setup_logger;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_bn254_aggregation() {
        setup_logger();

        let num_keys = 5000;

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

        let mut builder = DefaultBuilder::new();

        let public_keys = public_keys_values
            .iter()
            .map(|pk| builder.constant::<AffinePointVariable<Bn254>>(pk.clone().into()))
            .collect::<Vec<_>>();
        let selectors = selector_values
            .iter()
            .map(|b| builder.constant::<BoolVariable>(*b))
            .collect::<Vec<_>>();

        let agg_pk = builder.bn254_aggregate(&public_keys, &selectors);
        builder.write(agg_pk);

        let circuit = builder.build();
        let input = circuit.input();

        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);

        // Read the aggregated public key from the output and compare to the expected value.
        let agg_pk_output = AffinePoint::from(output.read::<AffinePointVariable<Bn254>>());

        let base_plus_agg_pk_value = public_keys_values.iter().zip(selector_values.iter()).fold(
            base.clone(),
            |agg, (pk, b)| if *b { agg.sw_add(pk) } else { agg },
        );

        assert_eq!(agg_pk_output + base, base_plus_agg_pk_value)
    }
}
