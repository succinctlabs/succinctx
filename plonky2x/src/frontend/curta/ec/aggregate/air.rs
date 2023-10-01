use core::marker::PhantomData;

use curta::chip::arithmetic::expression::ArithmeticExpression;
use curta::chip::builder::{AirBuilder, AirTraceData};
use curta::chip::ec::gadget::EllipticCurveGadget;
use curta::chip::ec::point::AffinePointRegister;
use curta::chip::ec::{EllipticCurve, EllipticCurveParameters};
use curta::chip::field::instruction::FpInstruction;
use curta::chip::field::register::FieldRegister;
use curta::chip::register::array::ArrayRegister;
use curta::chip::register::bit::BitRegister;
use curta::chip::register::cubic::CubicRegister;
use curta::chip::register::{Register, RegisterSerializable, RegisterSized};
use curta::chip::{AirParameters, Chip};
use curta::math::prelude::{CubicParameters, *};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PKAirParameters<F: Field, R: CubicParameters<F>, E: EllipticCurveParameters>(
    pub PhantomData<(F, R, E)>,
);

impl<F: PrimeField64, R: CubicParameters<F>, E: EllipticCurveParameters> AirParameters
    for PKAirParameters<F, R, E>
{
    type Field = F;
    type CubicParams = R;

    // TODO: specialize / implement as a function of E.
    const NUM_ARITHMETIC_COLUMNS: usize = 1000;
    const NUM_FREE_COLUMNS: usize = 9;
    const EXTENDED_COLUMNS: usize = 1527;

    type Instruction = FpInstruction<E::BaseField>;
}

#[derive(Debug, Clone)]
pub struct PKAir<F: PrimeField64, R: CubicParameters<F>, E: EllipticCurveParameters> {
    pub air: Chip<PKAirParameters<F, R, E>>,
    pub trace_data: AirTraceData<PKAirParameters<F, R, E>>,
    pub public_keys: Vec<AffinePointRegister<E>>,
    pub selectors: ArrayRegister<BitRegister>,
    pub aggregated_pk: AffinePointRegister<E>,
    pub current: AffinePointRegister<E>,
    pub flag: BitRegister,
}

impl<F: PrimeField64, R: CubicParameters<F>, E: EllipticCurve<PKAirParameters<F, R, E>>>
    PKAir<F, R, E>
{
    /// Creates a new instance of the PKAir supporting 1 << public_keys_degree public keys.
    pub fn new(public_keys_degree: usize) -> Self {
        let mut builder = AirBuilder::new();

        let num_public_keys = 1 << public_keys_degree;
        let cycle_degree = 16 - public_keys_degree;
        let cycle_size = 1 << cycle_degree;

        // Allocate public inputs.
        let public_keys = (0..num_public_keys)
            .map(|_| builder.alloc_public_ec_point())
            .collect::<Vec<_>>();
        let selectors = builder.alloc_array_public::<BitRegister>(num_public_keys);
        let aggregated_pk: AffinePointRegister<E> = builder.alloc_public_ec_point();

        // Initialize clock, bus channels, and aggregation challenges.
        let clk = builder.clock();
        let cycle = builder.cycle(cycle_degree);
        let mut bus = builder.new_bus();
        let channel_idx = bus.new_channel(&mut builder);

        let pk_challenges = builder.alloc_challenge_array::<CubicRegister>(
            1 + 2 * FieldRegister::<E::BaseField>::size_of(),
        );

        let selector_challenges =
            builder.alloc_challenge_array::<CubicRegister>(1 + BitRegister::size_of());

        // Initialize the public key aggregator and bit flag.
        let accumulator: AffinePointRegister<E> = builder.alloc_ec_point();
        let current: AffinePointRegister<E> = builder.alloc_ec_point();
        let flag = builder.alloc::<BitRegister>();

        // Connect the current ec point into the bus, depending on the cycle.
        let current_point_digest = builder.accumulate_expressions(
            &pk_challenges,
            &[clk.expr(), current.x.expr(), current.y.expr()],
        );
        builder.output_from_bus_filtered(channel_idx, current_point_digest, cycle.start_bit.expr());
        // Connect the selector into the bus, depending on the cycle.
        let selector_digest =
            builder.accumulate_expressions(&selector_challenges, &[clk.expr(), flag.expr()]);
        builder.output_from_bus_filtered(channel_idx, selector_digest, cycle.start_bit.expr());

        // Insert the public keys and selector values to the bus
        for i in 0..num_public_keys {
            let pk_digest = builder.accumulate_public_expressions(
                &pk_challenges,
                &[
                    ArithmeticExpression::from_constant(F::from_canonical_usize(i * cycle_size)),
                    public_keys[i].x.expr(),
                    public_keys[i].y.expr(),
                ],
            );
            bus.insert_global_value(&pk_digest);

            let selector_digest = builder.accumulate_public_expressions(
                &selector_challenges,
                &[
                    ArithmeticExpression::from_constant(F::from_canonical_usize(i * cycle_size)),
                    selectors.get(i).expr(),
                ],
            );
            bus.insert_global_value(&selector_digest);
        }

        // Set the accumulator in the first row to the generator.
        let generator = builder.ec_generator::<E>();
        builder.set_to_expression_first_row(&accumulator.x, generator.x.expr());
        builder.set_to_expression_first_row(&accumulator.y, generator.y.expr());

        // Flag constraints. The bus guarnatees correct values at cycle starts. for all other
        // points, we constrain the selector to zero.
        builder.assert_expression_zero(flag.expr() * cycle.start_bit.not_expr());

        // Add the accumulator to the current point
        let point_sum = builder.ec_add(&accumulator, &current);

        // Set the value of the next accumulator depending on the selector bit and the current point.

        // If the flag is set to false, the next accumulator stays the same. If the flag is set to
        // true, the next accumulator is the sum of the current point and the accumulator, except if
        // the dummy flag is set, and then the next accumulator is the current point.
        let next_point_x_expression =
            flag.not_expr() * accumulator.x.expr() + flag.expr() * point_sum.x.expr();
        let next_point_y_expression =
            flag.not_expr() * accumulator.y.expr() + flag.expr() * point_sum.y.expr();

        builder
            .set_to_expression_transition(&accumulator.x.next(), next_point_x_expression.clone());
        builder
            .set_to_expression_transition(&accumulator.y.next(), next_point_y_expression.clone());

        // Assert that current point pnly changes at the end of a cycle.
        builder.assert_expression_zero_transition(
            cycle.end_bit.not_expr() * (current.x.next().expr() - current.x.expr()),
        );
        builder.assert_expression_zero_transition(
            cycle.end_bit.not_expr() * (current.y.next().expr() - current.y.expr()),
        );

        // Set the next accumulator of the last row to the output.
        builder.assert_expression_zero_last_row(aggregated_pk.x.expr() - next_point_x_expression);
        builder.assert_expression_zero_last_row(aggregated_pk.y.expr() - next_point_y_expression);

        builder.constrain_bus(bus);
        let (air, trace_data) = builder.build();

        Self {
            air,
            trace_data,
            public_keys,
            selectors,
            aggregated_pk,
            current,
            flag,
        }
    }
}

#[cfg(test)]
mod tests {

    use curta::chip::ec::gadget::EllipticCurveWriter;
    use curta::chip::ec::weierstrass::bn254::Bn254;
    use curta::chip::trace::generator::ArithmeticGenerator;
    use curta::math::goldilocks::cubic::GoldilocksCubicParameters;
    use curta::maybe_rayon::*;
    use curta::plonky2::stark::config::{
        CurtaPoseidonGoldilocksConfig, PoseidonGoldilocksStarkConfig,
    };
    use curta::plonky2::stark::prover::StarkyProver;
    use curta::plonky2::stark::verifier::StarkyVerifier;
    use curta::plonky2::stark::Starky;
    use num_bigint::RandBigInt;
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::timed;
    use plonky2::util::timing::TimingTree;
    use rand::{thread_rng, Rng};

    use super::*;
    use crate::utils::setup_logger;

    #[test]
    fn test_pk_air() {
        type C = CurtaPoseidonGoldilocksConfig;
        type SC = PoseidonGoldilocksStarkConfig;
        type E = Bn254;

        setup_logger();

        let mut timing = TimingTree::new("Bn254 PK aggregation", log::Level::Debug);

        let num_keys_degree = 6;
        let num_rows = 1 << 16;
        let stride = 1 << (16 - num_keys_degree);

        let pk_air =
            PKAir::<GoldilocksField, GoldilocksCubicParameters, Bn254>::new(num_keys_degree);

        let PKAir {
            air,
            trace_data,
            public_keys,
            selectors,
            aggregated_pk,
            current,
            flag,
        } = pk_air;

        let trace_generator = ArithmeticGenerator::new(trace_data, num_rows);

        let base = E::generator();

        let writer = trace_generator.new_writer();
        let public_keys_values = timed!(
            timing,
            "Generating public keys",
            public_keys
                .par_iter()
                .map(|pk_register| {
                    let mut rng = thread_rng();
                    let sk = rng.gen_biguint(256);
                    let pk = base.sw_scalar_mul(&sk);
                    writer.write_ec_point(pk_register, &pk, 0);
                    pk
                })
                .collect::<Vec<_>>()
        );

        let mut selector_values = Vec::new();

        let mut rng = thread_rng();
        timed!(
            timing,
            "Execute trace",
            for selector_register in selectors.iter() {
                let val = rng.gen_bool(0.5);
                let selector = GoldilocksField::from_canonical_u8(val as u8);
                writer.write(&selector_register, &selector, 0);
                selector_values.push(selector);
            }
        );

        let aggregated_pk_value =
            public_keys_values
                .iter()
                .zip(selector_values.iter())
                .fold(base, |agg, (pk, b)| {
                    if *b == GoldilocksField::ONE {
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
        let config = SC::standard_fast_config(num_rows);

        let public_inputs = writer.public().unwrap().clone();

        let proof = timed!(
            timing,
            "Generate STARK proof",
            StarkyProver::<GoldilocksField, C, 2>::prove(
                &config,
                &stark,
                &trace_generator,
                &public_inputs,
            )
            .unwrap()
        );

        // Verify the proof as a stark
        StarkyVerifier::verify(&config, &stark, proof, &public_inputs).unwrap();

        timing.print();
    }
}
