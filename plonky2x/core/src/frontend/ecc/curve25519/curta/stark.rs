use curta::chip::builder::AirBuilder;
use curta::chip::ec::edwards::ed25519::gadget::{CompressedPointGadget, CompressedPointWriter};
use curta::chip::ec::edwards::ed25519::instruction::Ed25519FpInstruction;
use curta::chip::ec::edwards::ed25519::point::CompressedPointRegister;
use curta::chip::ec::gadget::EllipticCurveWriter;
use curta::chip::ec::point::{AffinePoint, AffinePointRegister};
use curta::chip::ec::scalar::ECScalarRegister;
use curta::chip::trace::writer::{InnerWriterData, TraceWriter};
use curta::chip::AirParameters;
use curta::machine::builder::Builder;
use curta::machine::ec::builder::EllipticCurveBuilder;
use curta::machine::emulated::builder::EmulatedBuilder;
use curta::machine::emulated::proof::EmulatedStarkProof;
use curta::machine::emulated::stark::EmulatedStark;
use curve25519_dalek::edwards::CompressedEdwardsY;
use itertools::Itertools;
use log::debug;
use num_bigint::BigUint;
use plonky2::util::log2_ceil;
use plonky2::util::timing::TimingTree;

use super::air_parameters::Ed25519AirParameters;
use super::request::EcOpRequestType;
use super::{Curve, ScalarField};
use crate::frontend::curta::ec::point::{AffinePointVariable, CompressedEdwardsYVariable};
use crate::frontend::curta::proof::EmulatedStarkProofVariable;
use crate::frontend::num::nonnative::nonnative::NonNativeVariable;
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
        NonNativeVariable<ScalarField>,
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
    stark: EmulatedStark<Ed25519AirParameters<L, D>, L::CurtaConfig, D>,
    operations: Vec<Ed25519CurtaOp>,
    degree: usize,
}

impl<L: PlonkParameters<D>, const D: usize> Ed25519Stark<L, D> {
    pub fn new(request_data: &[EcOpRequestType]) -> Self {
        let mut builder = EmulatedBuilder::<Ed25519AirParameters<L, D>>::new();

        let mut scalars = vec![];
        let mut scalar_mul_points = vec![];
        let mut scalar_mul_results = vec![];
        let operations = request_data
            .iter()
            .map(|kind| {
                let air_op = Ed25519CurtaOp::new(&mut builder.api, kind);
                if let Ed25519CurtaOp::ScalarMul(scalar, point, result) = &air_op {
                    scalars.push(*scalar);
                    scalar_mul_points.push(*point);
                    scalar_mul_results.push(*result);
                }
                air_op
            })
            .collect::<Vec<_>>();

        let num_scalar_muls = scalars.len();
        let degree_log = log2_ceil(num_scalar_muls * 256);
        let degree = 1 << degree_log;
        // Constrain the scalar mul operations.
        builder.scalar_mul_batch(&scalar_mul_points, &scalars, &scalar_mul_results);

        let stark = builder.build::<L::CurtaConfig, D>(degree);

        Ed25519Stark {
            stark,
            operations,
            degree,
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
    ) -> (
        EmulatedStarkProof<L::Field, L::CurtaConfig, D>,
        Vec<L::Field>,
    ) {
        let num_rows = self.degree;
        let writer = TraceWriter::new(&self.stark.air_data, num_rows);

        debug!("Writing EC stark input");
        self.write_input(&writer, input);

        debug!("Writing EC execusion trace");
        writer.write_global_instructions(&self.stark.air_data);
        for i in 0..num_rows {
            writer.write_row_instructions(&self.stark.air_data, i);
        }

        let public_inputs: Vec<L::Field> = writer.public().unwrap().clone();

        debug!("EC stark proof generation");
        let InnerWriterData { trace, public, .. } = writer.into_inner().unwrap();
        let proof = self
            .stark
            .prove(&trace, &public, &mut TimingTree::default())
            .unwrap();

        // Verify the proof as a stark
        self.stark.verify(proof.clone(), &public).unwrap();

        debug!("EC stark proof verified");

        (proof, public_inputs)
    }

    pub fn verify_proof(
        &self,
        builder: &mut CircuitBuilder<L, D>,
        proof: EmulatedStarkProofVariable<D>,
        public_inputs: &[Variable],
        _ec_ops: &[Ed25519OpVariable],
    ) {
        builder.verify_emulated_stark_proof(&self.stark, proof, public_inputs)
    }

    pub fn read_proof_with_public_input(
        &self,
        builder: &mut CircuitBuilder<L, D>,
        output_stream: &OutputVariableStream<L, D>,
    ) -> (EmulatedStarkProofVariable<D>, Vec<Variable>) {
        let proof = output_stream.read_emulated_stark_proof(builder, &self.stark);
        let public_inputs = output_stream.read_vec(builder, self.stark.air_data.num_public_inputs);

        (proof, public_inputs)
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
