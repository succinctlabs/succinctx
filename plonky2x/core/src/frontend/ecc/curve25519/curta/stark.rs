use curta::air;
use curta::chip::builder::AirBuilder;
use curta::chip::ec::edwards::ed25519::gadget::{CompressedPointGadget, CompressedPointWriter};
use curta::chip::ec::edwards::ed25519::instruction::Ed25519FpInstruction;
use curta::chip::ec::edwards::ed25519::point::CompressedPointRegister;
use curta::chip::ec::gadget::EllipticCurveWriter;
use curta::chip::ec::point::{AffinePoint, AffinePointRegister};
use curta::chip::ec::scalar::ECScalarRegister;
use curta::chip::ec::EllipticCurveAir;
use curta::chip::trace::data::AirTraceData;
use curta::chip::trace::generator::ArithmeticGenerator;
use curta::chip::trace::writer::TraceWriter;
use curta::chip::{AirParameters, Chip};
use curta::machine::builder::Builder;
use curta::machine::ec::builder::EllipticCurveBuilder;
use curta::maybe_rayon::*;
use curta::plonky2::stark::config::StarkyConfig;
use curta::plonky2::stark::proof::StarkProof;
use curta::plonky2::stark::prover::StarkyProver;
use curta::plonky2::stark::verifier::StarkyVerifier;
use curta::plonky2::stark::Starky;
use curve25519_dalek::edwards::CompressedEdwardsY;
use itertools::Itertools;
use log::debug;
use num_bigint::BigUint;

use super::air_parameters::Ed25519AirParameters;
use super::request::EcOpRequestType;
use super::Curve;
use crate::frontend::curta::ec::point::{AffinePointVariable, CompressedEdwardsYVariable};
use crate::frontend::curta::proof::StarkProofVariable;
use crate::prelude::*;

pub enum Ed25519CurtaOp {
    Add(
        AffinePointRegister<Curve>,
        AffinePointRegister<Curve>,
        AffinePointRegister<Curve>,
    ),
    ScalarMul(
        ECScalarRegister<Curve>,
        AffinePointRegister<Curve>,
        AffinePointRegister<Curve>,
    ),
    Decompress(CompressedPointRegister, AffinePointRegister<Curve>),
    IsValid(AffinePointRegister<Curve>),
}

pub enum Ed25519OpVariable {
    Add(
        AffinePointVariable<Curve>,
        AffinePointVariable<Curve>,
        AffinePointVariable<Curve>,
    ),
    ScalarMul(
        BigUint,
        AffinePointVariable<Curve>,
        AffinePointVariable<Curve>,
    ),
    Decompress(Box<CompressedEdwardsYVariable>, AffinePointVariable<Curve>),
    IsValid(AffinePointVariable<Curve>),
}

pub enum Ed25519CurtaOpValue {
    Add(AffinePoint<Curve>, AffinePoint<Curve>, AffinePoint<Curve>),
    ScalarMul(BigUint, AffinePoint<Curve>, AffinePoint<Curve>),
    Decompress(CompressedEdwardsY, AffinePoint<Curve>),
    IsValid(AffinePoint<Curve>),
}

pub struct Ed25519Stark<L: PlonkParameters<D>, const D: usize> {
    config: StarkyConfig<L::CurtaConfig, D>,
    stark: Starky<Chip<Ed25519AirParameters<L, D>>>,
    trace_data: AirTraceData<Ed25519AirParameters<L, D>>,
    operations: Vec<Ed25519CurtaOp>,
}

impl<L: PlonkParameters<D>, const D: usize> Ed25519Stark<L, D> {
    pub fn new(request_data: &[EcOpRequestType]) -> Self {
        let mut builder = AirBuilder::<Ed25519AirParameters<L, D>>::new();
        builder.init_local_memory();

        let mut scalars = vec![];
        let mut scalar_mul_points = vec![];
        let mut scalar_mul_results = vec![];
        let mut operations = request_data
            .iter()
            .map(|kind| {
                let air_op = Ed25519CurtaOp::new(&mut builder, kind);
                if let Ed25519CurtaOp::ScalarMul(scalar, point, result) = &air_op {
                    scalars.push(*scalar);
                    scalar_mul_points.push(*point);
                    scalar_mul_results.push(*result);
                }
                air_op
            })
            .collect::<Vec<_>>();

        let num_scalar_muls = scalars.len();
        assert!(
            num_scalar_muls <= 256,
            "too many scalar mul requests for a single stark"
        );

        if num_scalar_muls < 256 {
            // Add a dummy scalar mul to make the number of scalar muls equals to 256.
            let generator = Curve::ec_generator_air(&mut builder);
            let mut one = vec![L::Field::ONE];
            one.resize(8, L::Field::ZERO);
            let dummy_scalar = ECScalarRegister::new(builder.constant_array(&one));
            let dummy_point = generator;
            let dummy_result = generator;
            for _ in num_scalar_muls..256 {
                operations.push(Ed25519CurtaOp::ScalarMul(
                    dummy_scalar,
                    dummy_point,
                    dummy_result,
                ));
                scalars.push(dummy_scalar);
                scalar_mul_points.push(dummy_point);
                scalar_mul_results.push(dummy_result);
            }
        }

        // Constrain the scalar mul operations.
        builder.scalar_mul_batch(&scalar_mul_points, &scalars, &scalar_mul_results);

        let (air, trace_data) = builder.build();

        let stark = Starky::new(air);
        let config = StarkyConfig::standard_fast_config(1 << 16);

        Ed25519Stark {
            config,
            stark,
            trace_data,
            operations,
        }
    }

    pub fn write_input(&self, writer: &TraceWriter<L::Field>, input: &[Ed25519CurtaOpValue]) {
        self.operations
            .iter()
            .zip(input.iter())
            .for_each(|(op, op_value)| match &op {
                Ed25519CurtaOp::Add(a, b, _) => {
                    if let Ed25519CurtaOpValue::Add(a_val, b_val, _) = &op_value {
                        writer.write_ec_point(a, a_val, 0);
                        writer.write_ec_point(b, b_val, 0);
                    } else {
                        panic!("invalid input");
                    }
                }
                Ed25519CurtaOp::ScalarMul(scalar, point, result) => {
                    if let Ed25519CurtaOpValue::ScalarMul(scalar_val, point_val, result_val) =
                        &op_value
                    {
                        let mut limb_values = scalar_val.to_u32_digits();
                        limb_values.resize(8, 0);
                        for (limb_reg, limb) in scalar.limbs.iter().zip_eq(limb_values) {
                            writer.write(&limb_reg, &L::Field::from_canonical_u32(limb), 0);
                        }
                        writer.write_ec_point(point, point_val, 0);
                        writer.write_ec_point(result, result_val, 0);
                    } else {
                        panic!("invalid input");
                    }
                }
                Ed25519CurtaOp::Decompress(compressed_point, _) => {
                    if let Ed25519CurtaOpValue::Decompress(compressed_point_val, _) = &op_value {
                        writer.write_ec_compressed_point(compressed_point, compressed_point_val, 0);
                    } else {
                        panic!("invalid input");
                    }
                }
                Ed25519CurtaOp::IsValid(point) => {
                    if let Ed25519CurtaOpValue::IsValid(point_val) = &op_value {
                        writer.write_ec_point(point, point_val, 0);
                    } else {
                        panic!("invalid input");
                    }
                }
            });
    }

    #[allow(clippy::type_complexity)]
    pub fn prove(
        &self,
        input: &[Ed25519CurtaOpValue],
    ) -> (StarkProof<L::Field, L::CurtaConfig, D>, Vec<L::Field>) {
        let num_rows = 1 << 16;
        let generator = ArithmeticGenerator::<Ed25519AirParameters<L, D>>::new(
            self.trace_data.clone(),
            num_rows,
        );
        let writer = generator.new_writer();

        debug!("Writing EC stark input");
        self.write_input(&writer, input);

        debug!("Writing EC execusion trace");
        writer.write_global_instructions(&generator.air_data);
        for i in 0..num_rows {
            writer.write_row_instructions(&generator.air_data, i);
        }

        let public_inputs: Vec<L::Field> = writer.public().unwrap().clone();

        debug!("EC stark proof generation");
        let proof = StarkyProver::<L::Field, L::CurtaConfig, D>::prove(
            &self.config,
            &self.stark,
            &generator,
            &public_inputs,
        )
        .unwrap();

        // Verify the proof as a stark
        StarkyVerifier::verify(&self.config, &self.stark, proof.clone(), &public_inputs).unwrap();
        debug!("EC stark proof verified");

        (proof, public_inputs)
    }

    pub fn verify_proof(
        &self,
        builder: &mut CircuitBuilder<L, D>,
        proof: StarkProofVariable<D>,
        public_inputs: &[Variable],
        _ec_ops: &[Ed25519OpVariable],
    ) {
        builder.verify_stark_proof(&self.config, &self.stark, proof, public_inputs)
    }
}

impl Ed25519CurtaOp {
    pub fn new<L: AirParameters<Instruction = Ed25519FpInstruction>>(
        builder: &mut AirBuilder<L>,
        reuest_type: &EcOpRequestType,
    ) -> Self {
        match reuest_type {
            EcOpRequestType::Add => {
                let a = builder.alloc_public_ec_point();
                let b = builder.alloc_public_ec_point();
                let result = builder.add(a, b);
                Self::Add(a, b, result)
            }
            EcOpRequestType::ScalarMul => {
                let point = builder.alloc_public_ec_point();
                let scalar = ECScalarRegister::new(builder.alloc_array_public(8));
                let result = builder.alloc_public_ec_point();
                Self::ScalarMul(scalar, point, result)
            }
            EcOpRequestType::Decompress => {
                let compressed_point = builder.alloc_public_ec_compressed_point();
                let result = builder.ed25519_decompress(&compressed_point);
                Self::Decompress(compressed_point, result)
            }
            EcOpRequestType::IsValid => {
                let point = builder.alloc_public_ec_point();
                Self::IsValid(point)
            }
        }
    }
}
