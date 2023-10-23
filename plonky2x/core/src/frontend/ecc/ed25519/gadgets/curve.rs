use curta::chip::ec::edwards::scalar_mul::generator::AffinePointTarget as CurtaAffinePointTarget;
use plonky2::field::extension::Extendable;
use plonky2::field::types::{Field, PrimeField, PrimeField64};
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::iop::witness::Witness;
use plonky2::plonk::circuit_builder::CircuitBuilder as BaseCircuitBuilder;
use plonky2::util::serialization::{Buffer, IoResult};

use crate::frontend::ecc::ed25519::curve::curve_types::{AffinePoint, Curve};
use crate::frontend::hash::bit_operations::util::biguint_to_bits_target;
use crate::frontend::num::biguint::{CircuitBuilderBiguint, WitnessBigUint};
use crate::frontend::num::nonnative::nonnative::{
    CircuitBuilderNonNative, NonNativeTarget, ReadNonNativeTarget, WriteNonNativeTarget,
};
use crate::frontend::num::nonnative::split_nonnative::CircuitBuilderSplit;
use crate::prelude::{
    BoolVariable, ByteVariable, Bytes32Variable, CircuitBuilder, CircuitVariable, PlonkParameters,
    Variable,
};
/// A Target representing an affine point on the curve `C`. We use incomplete arithmetic for efficiency,
/// so we assume these points are not zero.
#[derive(Clone, Debug, Default)]
pub struct AffinePointTarget<C: Curve> {
    pub x: NonNativeTarget<C::BaseField>,
    pub y: NonNativeTarget<C::BaseField>,
}

impl<C: Curve> CircuitVariable for AffinePointTarget<C> {
    type ValueType<F: RichField> = AffinePoint<C>;

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        Self {
            x: NonNativeTarget::init_unsafe(builder),
            y: NonNativeTarget::init_unsafe(builder),
        }
    }

    fn nb_elements() -> usize {
        NonNativeTarget::<C::BaseField>::nb_elements() * 2
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        let mut elements = Vec::new();
        elements.extend(NonNativeTarget::<C::BaseField>::elements::<F>(value.x));
        elements.extend(NonNativeTarget::<C::BaseField>::elements::<F>(value.y));
        elements
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        let nb_elements = NonNativeTarget::<C::BaseField>::nb_elements();
        let x = NonNativeTarget::<C::BaseField>::from_elements(&elements[0..nb_elements]);
        let y = NonNativeTarget::<C::BaseField>::from_elements(&elements[nb_elements..]);
        AffinePoint::nonzero(x, y)
    }

    fn variables(&self) -> Vec<Variable> {
        let mut variables = Vec::new();
        variables.extend(self.x.variables());
        variables.extend(self.y.variables());
        variables
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        let nb_variables = NonNativeTarget::<C::BaseField>::nb_elements();
        Self {
            x: NonNativeTarget::from_variables_unsafe(&variables[0..nb_variables]),
            y: NonNativeTarget::from_variables_unsafe(&variables[nb_variables..]),
        }
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        builder.api.curve_assert_valid(self);
    }
}

pub struct CompressedPointTarget {
    pub bit_targets: [BoolTarget; 256],
}

pub trait CircuitBuilderCurve<F: RichField + Extendable<D>, const D: usize> {
    fn constant_affine_point<C: Curve>(&mut self, point: AffinePoint<C>) -> AffinePointTarget<C>;

    fn connect_affine_point<C: Curve>(
        &mut self,
        lhs: &AffinePointTarget<C>,
        rhs: &AffinePointTarget<C>,
    );

    fn add_virtual_affine_point_target<C: Curve>(&mut self) -> AffinePointTarget<C>;

    fn curve_assert_valid<C: Curve>(&mut self, p: &AffinePointTarget<C>);

    fn curve_neg<C: Curve>(&mut self, p: &AffinePointTarget<C>) -> AffinePointTarget<C>;

    fn curve_conditional_neg<C: Curve>(
        &mut self,
        p: &AffinePointTarget<C>,
        b: BoolTarget,
    ) -> AffinePointTarget<C>;

    /// Add two points, which are assumed to be non-equal.
    fn curve_add<C: Curve>(
        &mut self,
        p1: &AffinePointTarget<C>,
        p2: &AffinePointTarget<C>,
    ) -> AffinePointTarget<C>;

    fn compress_point<C: Curve>(&mut self, p: &AffinePointTarget<C>) -> CompressedPointTarget;

    fn random_access_affine_point<C: Curve>(
        &mut self,
        access_index: Target,
        v: Vec<AffinePointTarget<C>>,
    ) -> AffinePointTarget<C>;

    fn convert_to_curta_affine_point_target<C: Curve>(
        &mut self,
        p: &AffinePointTarget<C>,
    ) -> CurtaAffinePointTarget;

    fn convert_from_curta_affine_point_target<C: Curve>(
        &mut self,
        p: &CurtaAffinePointTarget,
    ) -> AffinePointTarget<C>;

    fn is_equal_affine_point<C: Curve>(
        &mut self,
        a: &AffinePointTarget<C>,
        b: &AffinePointTarget<C>,
    ) -> BoolTarget;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilderCurve<F, D>
    for BaseCircuitBuilder<F, D>
{
    fn constant_affine_point<C: Curve>(&mut self, point: AffinePoint<C>) -> AffinePointTarget<C> {
        debug_assert!(!point.zero);
        AffinePointTarget {
            x: self.constant_nonnative(point.x),
            y: self.constant_nonnative(point.y),
        }
    }

    fn connect_affine_point<C: Curve>(
        &mut self,
        lhs: &AffinePointTarget<C>,
        rhs: &AffinePointTarget<C>,
    ) {
        self.connect_nonnative(&lhs.x, &rhs.x);
        self.connect_nonnative(&lhs.y, &rhs.y);
    }

    fn add_virtual_affine_point_target<C: Curve>(&mut self) -> AffinePointTarget<C> {
        let x = self.add_virtual_nonnative_target();
        let y = self.add_virtual_nonnative_target();

        AffinePointTarget { x, y }
    }

    fn curve_assert_valid<C: Curve>(&mut self, p: &AffinePointTarget<C>) {
        // ed25519 has the following parameters
        // Equation: a * x ** 2 + y ** 2 = 1 + d * x ** 2 * y ** 2
        // a is -1, so the above equation can be rewritten as
        // y ** 2 = 1 + d * x ** 2 * y ** 2 + x ** 2
        let d = self.constant_nonnative(C::D);
        let one = self.constant_nonnative(C::BaseField::ONE);

        let y_squared = self.mul_nonnative(&p.y, &p.y);
        let x_squared = self.mul_nonnative(&p.x, &p.x);
        let x_squared_times_y_squared = self.mul_nonnative(&x_squared, &y_squared);
        let d_x_squared_times_y_squared = self.mul_nonnative(&d, &x_squared_times_y_squared);
        let d_x_squared_times_y_squared_plus_x_sqaured =
            self.add_nonnative(&d_x_squared_times_y_squared, &x_squared);
        let rhs = self.add_nonnative(&one, &d_x_squared_times_y_squared_plus_x_sqaured);

        self.connect_nonnative(&y_squared, &rhs);
    }

    fn curve_neg<C: Curve>(&mut self, p: &AffinePointTarget<C>) -> AffinePointTarget<C> {
        let neg_x = self.neg_nonnative(&p.x);
        AffinePointTarget {
            x: neg_x,
            y: p.y.clone(),
        }
    }

    fn curve_conditional_neg<C: Curve>(
        &mut self,
        p: &AffinePointTarget<C>,
        b: BoolTarget,
    ) -> AffinePointTarget<C> {
        AffinePointTarget {
            x: self.nonnative_conditional_neg(&p.x, b),
            y: p.y.clone(),
        }
    }

    fn curve_add<C: Curve>(
        &mut self,
        p1: &AffinePointTarget<C>,
        p2: &AffinePointTarget<C>,
    ) -> AffinePointTarget<C> {
        let AffinePointTarget { x: x1, y: y1 } = p1;
        let AffinePointTarget { x: x2, y: y2 } = p2;

        let d = self.constant_nonnative(C::D);
        let one = self.constant_nonnative(C::BaseField::ONE);

        let x1y2 = self.mul_nonnative(x1, y2);
        let y1x2 = self.mul_nonnative(y1, x2);
        let y1y2 = self.mul_nonnative(y1, y2);
        let x1x2 = self.mul_nonnative(x1, x2);
        let x1x2y1y2 = self.mul_nonnative(&x1x2, &y1y2);
        let dx1x2y1y2 = self.mul_nonnative(&d, &x1x2y1y2);

        let x3_num = self.add_nonnative(&x1y2, &y1x2);
        let x3_den = self.add_nonnative(&one, &dx1x2y1y2);
        let x3_den_inv = self.inv_nonnative(&x3_den);
        let x3 = self.mul_nonnative(&x3_num, &x3_den_inv);

        // y3 numerator is y1y2 - ax1x2 where a = -1
        // can be rewritten as y1y2 + x1x2
        let y3_num = self.add_nonnative(&y1y2, &x1x2);
        let y3_den = self.sub_nonnative(&one, &dx1x2y1y2);
        let y3_den_inv = self.inv_nonnative(&y3_den);
        let y3 = self.mul_nonnative(&y3_num, &y3_den_inv);

        AffinePointTarget { x: x3, y: y3 }
    }

    // This function will accept an affine point target and return
    // the point in compressed form (bit vector).
    fn compress_point<C: Curve>(&mut self, p: &AffinePointTarget<C>) -> CompressedPointTarget {
        let mut bits = biguint_to_bits_target::<F, D>(self, &p.y.value);
        let x_bits_low_32 = self.split_le_base::<2>(p.x.value.get_limb(0).0, 32);

        let a = bits[0].target;
        let b = x_bits_low_32[0];
        // a | b = a + b - a * b
        let a_add_b = self.add(a, b);
        let ab = self.mul(a, b);
        bits[0] = BoolTarget::new_unsafe(self.sub(a_add_b, ab));
        CompressedPointTarget {
            bit_targets: bits.try_into().unwrap(),
        }
    }

    fn random_access_affine_point<C: Curve>(
        &mut self,
        access_index: Target,
        v: Vec<AffinePointTarget<C>>,
    ) -> AffinePointTarget<C> {
        AffinePointTarget {
            x: self.random_access_nonnative(
                access_index,
                v.iter().map(|p| p.x.clone()).collect::<Vec<_>>(),
            ),
            y: self.random_access_nonnative(
                access_index,
                v.iter().map(|p| p.y.clone()).collect::<Vec<_>>(),
            ),
        }
    }

    fn convert_to_curta_affine_point_target<C: Curve>(
        &mut self,
        p: &AffinePointTarget<C>,
    ) -> CurtaAffinePointTarget {
        let x_limbs = self.split_nonnative_to_16_bit_limbs(&p.x);
        let y_limbs = self.split_nonnative_to_16_bit_limbs(&p.y);

        assert!(x_limbs.len() == 16);
        assert!(y_limbs.len() == 16);

        CurtaAffinePointTarget {
            x: x_limbs.try_into().unwrap(),
            y: y_limbs.try_into().unwrap(),
        }
    }

    fn convert_from_curta_affine_point_target<C: Curve>(
        &mut self,
        p: &CurtaAffinePointTarget,
    ) -> AffinePointTarget<C> {
        let x = self.recombine_nonnative_16_bit_limbs(p.x.to_vec());
        let y = self.recombine_nonnative_16_bit_limbs(p.y.to_vec());

        AffinePointTarget { x, y }
    }

    fn is_equal_affine_point<C: Curve>(
        &mut self,
        a: &AffinePointTarget<C>,
        b: &AffinePointTarget<C>,
    ) -> BoolTarget {
        let a_x_biguint = self.nonnative_to_canonical_biguint(&a.x);
        let b_x_biguint = self.nonnative_to_canonical_biguint(&b.x);
        let x_equal = self.is_equal_biguint(&a_x_biguint, &b_x_biguint);

        let a_y_biguint = self.nonnative_to_canonical_biguint(&a.y);
        let b_y_biguint = self.nonnative_to_canonical_biguint(&b.y);
        let y_equal = self.is_equal_biguint(&a_y_biguint, &b_y_biguint);

        self.and(x_equal, y_equal)
    }
}

pub struct CompressedPointVariable(pub Bytes32Variable);

pub trait CircuitBuilderCurveGadget<L: PlonkParameters<D>, const D: usize> {
    fn compress_point<C: Curve>(&mut self, p: &AffinePointTarget<C>) -> CompressedPointVariable;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilderCurveGadget<L, D>
    for CircuitBuilder<L, D>
{
    fn compress_point<C: Curve>(&mut self, p: &AffinePointTarget<C>) -> CompressedPointVariable {
        let bool_targets = self.api.compress_point(p).bit_targets;
        let bool_variables = bool_targets
            .iter()
            .map(|b| BoolVariable::from(*b))
            .collect::<Vec<_>>();
        let mut byte_variables = bool_variables
            .chunks(8)
            .map(|chunk| ByteVariable(chunk.try_into().unwrap()))
            .collect::<Vec<_>>();
        // Flip from little endian to big endian.
        byte_variables.reverse();
        CompressedPointVariable(Bytes32Variable::from(byte_variables.as_slice()))
    }
}

pub trait WitnessAffinePoint<F: PrimeField64>: Witness<F> {
    fn get_affine_point_target<C: Curve>(&self, target: AffinePointTarget<C>) -> AffinePoint<C>;
    fn set_affine_point_target<C: Curve>(
        &mut self,
        target: &AffinePointTarget<C>,
        value: &AffinePoint<C>,
    );
}

impl<T: Witness<F>, F: PrimeField64> WitnessAffinePoint<F> for T {
    fn get_affine_point_target<C: Curve>(&self, target: AffinePointTarget<C>) -> AffinePoint<C> {
        let x_biguint =
            C::BaseField::from_noncanonical_biguint(self.get_biguint_target(target.x.value));
        let y_biguint =
            C::BaseField::from_noncanonical_biguint(self.get_biguint_target(target.y.value));
        AffinePoint::nonzero(x_biguint, y_biguint)
    }

    fn set_affine_point_target<C: Curve>(
        &mut self,
        target: &AffinePointTarget<C>,
        value: &AffinePoint<C>,
    ) {
        assert!(
            value.is_valid() && !value.zero,
            "Point is not on curve or is zero"
        );
        self.set_biguint_target(&target.x.value, &value.x.to_canonical_biguint());
        self.set_biguint_target(&target.y.value, &value.y.to_canonical_biguint());
    }
}

pub trait WriteAffinePoint {
    fn write_target_affine_point<C: Curve>(&mut self, x: AffinePointTarget<C>) -> IoResult<()>;
}

impl WriteAffinePoint for Vec<u8> {
    #[inline]
    fn write_target_affine_point<C: Curve>(&mut self, point: AffinePointTarget<C>) -> IoResult<()> {
        self.write_target_nonnative(point.x)?;
        self.write_target_nonnative(point.y)
    }
}

pub trait ReadAffinePoint {
    fn read_target_affine_point<C: Curve>(&mut self) -> IoResult<AffinePointTarget<C>>;
}

impl ReadAffinePoint for Buffer<'_> {
    #[inline]
    fn read_target_affine_point<C: Curve>(&mut self) -> IoResult<AffinePointTarget<C>> {
        let x = self.read_target_nonnative()?;
        let y = self.read_target_nonnative()?;
        Ok(AffinePointTarget { x, y })
    }
}

#[cfg(test)]
mod tests {

    use std::env;

    use plonky2::field::types::{Field, Sample};
    use plonky2::iop::witness::{PartialWitness, WitnessWrite};
    use plonky2::plonk::circuit_builder::CircuitBuilder as BaseCircuitBuilder;
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

    use crate::frontend::ecc::ed25519::curve::curve_types::{AffinePoint, Curve, CurveScalar};
    use crate::frontend::ecc::ed25519::curve::ed25519::Ed25519;
    use crate::frontend::ecc::ed25519::field::ed25519_base::Ed25519Base;
    use crate::frontend::ecc::ed25519::field::ed25519_scalar::Ed25519Scalar;
    use crate::frontend::ecc::ed25519::gadgets::curve::{
        AffinePointTarget, CircuitBuilderCurve, CircuitBuilderCurveGadget,
    };
    use crate::frontend::hash::bit_operations::util::biguint_to_bits_target;
    use crate::frontend::num::biguint::CircuitBuilderBiguint;
    use crate::prelude::{Bytes32Variable, DefaultBuilder};

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_curve_point_is_valid() {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_ecc_config();

        let pw = PartialWitness::new();
        let mut builder = BaseCircuitBuilder::<F, D>::new(config);

        let g = Ed25519::GENERATOR_AFFINE;
        let g_target = builder.constant_affine_point(g);
        let neg_g_target = builder.curve_neg(&g_target);

        builder.curve_assert_valid(&g_target);
        builder.curve_assert_valid(&neg_g_target);

        let inner_data = builder.build::<C>();
        let inner_proof = inner_data.prove(pw).unwrap();

        inner_data.verify(inner_proof.clone()).unwrap();

        let mut outer_builder =
            BaseCircuitBuilder::<F, D>::new(CircuitConfig::standard_ecc_config());
        let inner_proof_target = outer_builder.add_virtual_proof_with_pis(&inner_data.common);
        let inner_verifier_data =
            outer_builder.add_virtual_verifier_data(inner_data.common.config.fri_config.cap_height);
        outer_builder.verify_proof::<C>(
            &inner_proof_target,
            &inner_verifier_data,
            &inner_data.common,
        );

        let outer_data = outer_builder.build::<C>();

        let mut outer_pw = PartialWitness::new();
        outer_pw.set_proof_with_pis_target(&inner_proof_target, &inner_proof);
        outer_pw.set_verifier_data_target(&inner_verifier_data, &inner_data.verifier_only);

        let outer_proof = outer_data.prove(outer_pw).unwrap();
        outer_data.verify(outer_proof).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_curve_point_is_not_valid() {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_ecc_config();

        let pw = PartialWitness::new();
        let mut builder = BaseCircuitBuilder::<F, D>::new(config);

        let g = Ed25519::GENERATOR_AFFINE;
        let not_g = AffinePoint::<Ed25519> {
            x: g.x,
            y: g.y + Ed25519Base::ONE,
            zero: g.zero,
        };
        let not_g_target = builder.constant_affine_point(not_g);

        builder.curve_assert_valid(&not_g_target);

        let inner_data = builder.build::<C>();
        let inner_proof = inner_data.prove(pw).unwrap();

        inner_data.verify(inner_proof).unwrap();
    }

    #[test]
    fn test_curve_add() {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_ecc_config();

        let pw = PartialWitness::new();
        let mut builder = BaseCircuitBuilder::<F, D>::new(config);

        let g = Ed25519::GENERATOR_AFFINE;
        let double_g = g.double();
        let g_plus_2g = g + double_g;
        let g_plus_2g_expected = builder.constant_affine_point(g_plus_2g);
        builder.curve_assert_valid(&g_plus_2g_expected);

        let g_target = builder.constant_affine_point(g);
        let double_g_target = builder.curve_add(&g_target, &g_target);
        let g_plus_2g_actual = builder.curve_add(&g_target, &double_g_target);
        builder.curve_assert_valid(&g_plus_2g_actual);

        builder.connect_affine_point(&g_plus_2g_expected, &g_plus_2g_actual);

        dbg!(builder.num_gates());
        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();

        data.verify(proof).unwrap();
    }

    #[test]
    fn test_curve_random() {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_ecc_config();

        let pw = PartialWitness::new();
        let mut builder = BaseCircuitBuilder::<F, D>::new(config);

        let rando =
            (CurveScalar(Ed25519Scalar::rand()) * Ed25519::GENERATOR_PROJECTIVE).to_affine();
        let rando_doubled = rando.double();
        let expected_rando_doubled = builder.constant_affine_point(rando_doubled);

        let randot = builder.constant_affine_point(rando);
        let randot_doubled = builder.curve_add(&randot, &randot);
        builder.connect_affine_point(&randot_doubled, &expected_rando_doubled);

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();

        data.verify(proof).unwrap();
    }

    #[test]
    fn test_compress_point() {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_ecc_config();

        let pw = PartialWitness::new();
        let mut builder = BaseCircuitBuilder::<F, D>::new(config);

        let priv_key = Ed25519Scalar::from_canonical_usize(5);
        let g = Ed25519::GENERATOR_AFFINE;
        let pub_key_affine = (CurveScalar(priv_key) * g.to_projective()).to_affine();
        let pub_key_affine_t = builder.constant_affine_point(pub_key_affine);

        let pub_key_compressed = pub_key_affine.compress_point();
        let expected_pub_key_compressed_t = builder.constant_biguint(&pub_key_compressed);
        let expected_bits_t = biguint_to_bits_target(&mut builder, &expected_pub_key_compressed_t);

        builder.curve_assert_valid(&pub_key_affine_t);
        let pub_key_keycompressed_t = builder.compress_point(&pub_key_affine_t);
        assert!(pub_key_keycompressed_t.bit_targets.len() == expected_bits_t.len());
        for (i, bit) in pub_key_keycompressed_t.bit_targets.iter().enumerate() {
            builder.connect(bit.target, expected_bits_t[i].target);
        }

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();

        data.verify(proof).unwrap();
    }

    #[test]
    fn test_compress_point_variable() {
        env::set_var("RUST_LOG", "debug");

        type Curve = Ed25519;

        let mut builder = DefaultBuilder::new();

        let pubkey_point = builder.read::<AffinePointTarget<Curve>>();

        let compressed_bytes = builder.compress_point(&pubkey_point);

        builder.write(compressed_bytes.0);

        let circuit = builder.build();
        let mut input = circuit.input();

        let pubkey = "de25aec935b10f657b43fa97e5a8d4e523bdb0f9972605f0b064eff7b17048ba";
        let pubkey_bytes = hex::decode(pubkey).unwrap();

        let pub_key_uncompressed: AffinePoint<Curve> =
            AffinePoint::new_from_compressed_point(&pubkey_bytes);

        input.write::<AffinePointTarget<Curve>>(pub_key_uncompressed);

        let (proof, mut output) = circuit.prove(&input);

        circuit.verify(&proof, &input, &output);

        let computed_pubkey = output.read::<Bytes32Variable>();

        assert_eq!(computed_pubkey.0.to_vec(), pubkey_bytes);
    }
}
