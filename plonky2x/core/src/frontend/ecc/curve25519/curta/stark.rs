use curve25519_dalek::edwards::CompressedEdwardsY;
use itertools::Itertools;
use log::debug;
use num_bigint::BigUint;
use plonky2::iop::target::BoolTarget;
use plonky2::util::log2_ceil;
use plonky2::util::timing::TimingTree;
use starkyx::chip::builder::AirBuilder;
use starkyx::chip::ec::edwards::ed25519::gadget::{
    CompressedPointAirWriter, CompressedPointGadget,
};
use starkyx::chip::ec::edwards::ed25519::instruction::Ed25519FpInstruction;
use starkyx::chip::ec::edwards::ed25519::params::Ed25519BaseField;
use starkyx::chip::ec::edwards::ed25519::point::CompressedPointRegister;
use starkyx::chip::ec::gadget::EllipticCurveAirWriter;
use starkyx::chip::ec::point::{AffinePoint, AffinePointRegister};
use starkyx::chip::ec::scalar::ECScalarRegister;
use starkyx::chip::ec::EllipticCurveParameters;
use starkyx::chip::field::register::FieldRegister;
use starkyx::chip::register::Register;
use starkyx::chip::trace::writer::data::AirWriterData;
use starkyx::chip::trace::writer::AirWriter;
use starkyx::chip::AirParameters;
use starkyx::machine::builder::Builder;
use starkyx::machine::ec::builder::EllipticCurveBuilder;
use starkyx::machine::emulated::builder::EmulatedBuilder;
use starkyx::machine::emulated::proof::EmulatedStarkProof;
use starkyx::machine::emulated::stark::EmulatedStark;
use starkyx::maybe_rayon::*;

use super::air_parameters::Ed25519AirParameters;
use super::request::EcOpRequestType;
use super::Curve;
use crate::frontend::curta::ec::point::{AffinePointVariable, CompressedEdwardsYVariable};
use crate::frontend::curta::field::variable::FieldVariable;
use crate::frontend::curta::proof::EmulatedStarkProofVariable;
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
    Decompress(
        CompressedPointRegister,
        AffinePointRegister<Curve>,
        FieldRegister<Ed25519BaseField>,
    ),
    IsValid(AffinePointRegister<Curve>),
}

pub enum Ed25519OpVariable {
    Add(
        AffinePointVariable<Curve>,
        AffinePointVariable<Curve>,
        AffinePointVariable<Curve>,
    ),
    ScalarMul(
        U256Variable,
        AffinePointVariable<Curve>,
        AffinePointVariable<Curve>,
    ),
    Decompress(
        Box<CompressedEdwardsYVariable>,
        AffinePointVariable<Curve>,
        FieldVariable<Ed25519BaseField>,
    ),
    IsValid(AffinePointVariable<Curve>),
}

pub enum Ed25519CurtaOpValue {
    Add(AffinePoint<Curve>, AffinePoint<Curve>, AffinePoint<Curve>),
    ScalarMul(BigUint, AffinePoint<Curve>, AffinePoint<Curve>),
    Decompress(CompressedEdwardsY, AffinePoint<Curve>),
    IsValid(AffinePoint<Curve>),
}

/// A Curta stark for proving EC operations.
///
/// The Curta stark consists of a range check table to prove elements are between 0 and 2^16 - 1.
/// These range checks are used to constrain EC operations in the following way:
///    - EC Add, decompress, and is_valid operations are done on public inputs and using the AIR
///      table only for range checks.
///    - Scalar mul operations are done in the AIR table, with each scalae mul taking 256 rows.
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

    pub fn write_input(
        &self,
        writer: &mut impl AirWriter<Field = L::Field>,
        input: &[Ed25519CurtaOpValue],
    ) {
        self.operations
            .iter()
            .zip(input.iter())
            .for_each(|(op, op_value)| match &op {
                Ed25519CurtaOp::Add(a, b, _) => {
                    if let Ed25519CurtaOpValue::Add(a_val, b_val, _) = &op_value {
                        writer.write_ec_point(a, a_val);
                        writer.write_ec_point(b, b_val);
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
                            writer.write(&limb_reg, &L::Field::from_canonical_u32(limb));
                        }
                        writer.write_ec_point(point, point_val);
                        writer.write_ec_point(result, result_val);
                    } else {
                        panic!("invalid input");
                    }
                }
                Ed25519CurtaOp::Decompress(compressed_point, _, _) => {
                    if let Ed25519CurtaOpValue::Decompress(compressed_point_val, _) = &op_value {
                        writer.write_ec_compressed_point(compressed_point, compressed_point_val);
                    } else {
                        panic!("invalid input");
                    }
                }
                Ed25519CurtaOp::IsValid(point) => {
                    if let Ed25519CurtaOpValue::IsValid(point_val) = &op_value {
                        writer.write_ec_point(point, point_val);
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

        let mut writer_data = AirWriterData::new(&self.stark.air_data, num_rows);

        debug!("Writing EC stark input");
        let mut writer = writer_data.public_writer();
        self.write_input(&mut writer, input);

        debug!("Writing EC execution trace");
        self.stark.air_data.write_global_instructions(&mut writer);
        writer_data.chunks_par(256).for_each(|mut chunk| {
            for i in 0..256 {
                let mut writer = chunk.window_writer(i);
                self.stark.air_data.write_trace_instructions(&mut writer);
            }
        });

        debug!("EC stark proof generation");
        let (trace, public) = (writer_data.trace, writer_data.public);
        let proof = self
            .stark
            .prove(&trace, &public, &mut TimingTree::default())
            .unwrap();

        // Verify the proof as a stark
        self.stark.verify(proof.clone(), &public).unwrap();

        debug!("EC stark proof verified");

        (proof, public)
    }

    pub fn verify_proof(
        &self,
        builder: &mut CircuitBuilder<L, D>,
        proof: EmulatedStarkProofVariable<D>,
        public_inputs: &[Variable],
        ec_ops: &[Ed25519OpVariable],
    ) {
        // Verify the stark proof in the circuit.
        builder.verify_emulated_stark_proof(&self.stark, proof, public_inputs);

        // Assert consistency between the public inputs to the stark and the circuit data.

        for (curta_op, op) in self.operations.iter().zip_eq(ec_ops.iter()) {
            match (curta_op, op) {
                (
                    Ed25519CurtaOp::Add(a, b, result),
                    Ed25519OpVariable::Add(a_var, b_var, result_var),
                ) => {
                    Self::assert_point_equal(builder, a, a_var, public_inputs);
                    Self::assert_point_equal(builder, b, b_var, public_inputs);
                    Self::assert_point_equal(builder, result, result_var, public_inputs);
                }
                (
                    Ed25519CurtaOp::ScalarMul(scalar, point, result),
                    Ed25519OpVariable::ScalarMul(scalar_var, point_var, result_var),
                ) => {
                    Self::assert_scalar_equal(builder, scalar, scalar_var, public_inputs);
                    Self::assert_point_equal(builder, point, point_var, public_inputs);
                    Self::assert_point_equal(builder, result, result_var, public_inputs);
                }
                (
                    Ed25519CurtaOp::Decompress(compressed_point, result, pos_sqrt),
                    Ed25519OpVariable::Decompress(compressed_point_var, result_var, pos_sqrt_var),
                ) => {
                    Self::assert_compressed_point_equal(
                        builder,
                        compressed_point,
                        compressed_point_var,
                        public_inputs,
                    );
                    Self::assert_point_equal(builder, result, result_var, public_inputs);
                    Self::assert_field_element_equal(
                        builder,
                        pos_sqrt,
                        pos_sqrt_var,
                        public_inputs,
                    );
                }
                (Ed25519CurtaOp::IsValid(point), Ed25519OpVariable::IsValid(point_var)) => {
                    Self::assert_point_equal(builder, point, point_var, public_inputs);
                }
                _ => panic!("invalid operation"),
            }
        }
    }

    fn assert_compressed_point_equal(
        builder: &mut CircuitBuilder<L, D>,
        c: &CompressedPointRegister,
        c_var: &CompressedEdwardsYVariable,
        public_inputs: &[Variable],
    ) {
        let sign = c.sign.read_from_slice(public_inputs);
        let y = c.y.read_from_slice(public_inputs);

        let c_bytes = c_var.0.as_bytes();
        let sign_var = c_var.0.as_bytes()[31].as_le_bits().last().copied().unwrap();
        builder.assert_is_equal(sign, sign_var.variable);

        let mut y_bytes = c_bytes;
        // And with 255 because `ByteVariable` is internally big endian.
        let b_255 = builder.constant::<ByteVariable>(0b01111111);
        y_bytes[31] = builder.and(y_bytes[31], b_255);

        let y_var_bits = y_bytes
            .into_iter()
            .flat_map(|b| b.as_le_bits())
            .collect::<Vec<_>>();

        let y_var_limbs = y_var_bits
            .chunks_exact(16)
            .map(|chunk| {
                let le_targets = chunk
                    .iter()
                    .map(|x| BoolTarget::new_unsafe(x.variables()[0].0));
                Variable::from(builder.api.le_sum(le_targets))
            })
            .collect::<Vec<_>>();

        for (limb, var_limb) in y.coefficients().iter().zip(y_var_limbs) {
            builder.assert_is_equal(*limb, var_limb);
        }
    }

    fn assert_scalar_equal(
        builder: &mut CircuitBuilder<L, D>,
        s: &ECScalarRegister<Curve>,
        s_var: &U256Variable,
        public_inputs: &[Variable],
    ) {
        for (limb, var_limb) in s.limbs.iter().zip(s_var.limbs.iter()) {
            let limb = limb.read_from_slice(public_inputs);
            builder.assert_is_equal(limb, var_limb.variable);
        }
    }

    fn assert_field_element_equal(
        builder: &mut CircuitBuilder<L, D>,
        element: &FieldRegister<<Curve as EllipticCurveParameters>::BaseField>,
        element_var: &FieldVariable<<Curve as EllipticCurveParameters>::BaseField>,
        public_inputs: &[Variable],
    ) {
        let element = FieldVariable::from_variables_unsafe(
            element.read_from_slice(public_inputs).coefficients(),
        );
        builder.assert_is_equal(element, element_var.clone());
    }

    fn assert_point_equal(
        builder: &mut CircuitBuilder<L, D>,
        a: &AffinePointRegister<Curve>,
        a_var: &AffinePointVariable<Curve>,
        public_inputs: &[Variable],
    ) {
        Self::assert_field_element_equal(builder, &a.x, &a_var.x, public_inputs);
        Self::assert_field_element_equal(builder, &a.y, &a_var.y, public_inputs);
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
                let (result, pos_square_root) = builder.ed25519_decompress(&compressed_point);
                Self::Decompress(compressed_point, result, pos_square_root)
            }
            EcOpRequestType::IsValid => {
                let point = builder.alloc_public_ec_point();
                builder.ed_assert_valid(&point);
                Self::IsValid(point)
            }
        }
    }
}
