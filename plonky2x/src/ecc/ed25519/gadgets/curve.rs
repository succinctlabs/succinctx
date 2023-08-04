use std::marker::PhantomData;

use curta::chip::ec::edwards::scalar_mul::generator::AffinePointTarget as CurtaAffinePointTarget;
use curve25519_dalek::edwards::CompressedEdwardsY;
use num::BigUint;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::iop::witness::{PartitionWitness, Witness};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};
use plonky2::field::extension::Extendable;
use plonky2::field::types::{Field, PrimeField, Sample};

use crate::ecc::ed25519::curve::curve_types::{AffinePoint, Curve, CurveScalar};
use crate::ecc::ed25519::curve::ed25519::Ed25519;
use crate::ecc::ed25519::field::ed25519_base::Ed25519Base;
use crate::hash::bit_operations::util::biguint_to_bits_target;
use crate::num::biguint::GeneratedValuesBigUint;
use crate::num::nonnative::nonnative::{CircuitBuilderNonNative, NonNativeTarget, ReadNonNativeTarget, WriteNonNativeTarget};
use crate::num::nonnative::split_nonnative::CircuitBuilderSplit;

/// A Target representing an affine point on the curve `C`. We use incomplete arithmetic for efficiency,
/// so we assume these points are not zero.
#[derive(Clone, Debug, Default)]
pub struct AffinePointTarget<C: Curve> {
    pub x: NonNativeTarget<C::BaseField>,
    pub y: NonNativeTarget<C::BaseField>,
}

impl<C: Curve> AffinePointTarget<C> {
    pub fn to_vec(&self) -> Vec<NonNativeTarget<C::BaseField>> {
        vec![self.x.clone(), self.y.clone()]
    }
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

    fn curve_double<C: Curve>(&mut self, p: &AffinePointTarget<C>) -> AffinePointTarget<C>;

    fn curve_repeated_double<C: Curve>(
        &mut self,
        p: &AffinePointTarget<C>,
        n: usize,
    ) -> AffinePointTarget<C>;

    /// Add two points, which are assumed to be non-equal.
    fn curve_add<C: Curve>(
        &mut self,
        p1: &AffinePointTarget<C>,
        p2: &AffinePointTarget<C>,
    ) -> AffinePointTarget<C>;

    fn curve_conditional_add<C: Curve>(
        &mut self,
        p1: &AffinePointTarget<C>,
        p2: &AffinePointTarget<C>,
        b: BoolTarget,
    ) -> AffinePointTarget<C>;

    fn curve_scalar_mul<C: Curve>(
        &mut self,
        p: &AffinePointTarget<C>,
        n: &NonNativeTarget<C::ScalarField>,
    ) -> AffinePointTarget<C>;

    fn compress_point<C: Curve>(&mut self, p: &AffinePointTarget<C>) -> Vec<BoolTarget>;

    fn decompress_point<C: Curve>(&mut self, p: &[BoolTarget]) -> AffinePointTarget<C>;

    fn convert_to_curta_affine_point_target<C: Curve>(&mut self, p: &AffinePointTarget<C>) -> CurtaAffinePointTarget;

    fn convert_from_curta_affine_point_target<C: Curve>(&mut self, p: &CurtaAffinePointTarget) -> AffinePointTarget<C>;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilderCurve<F, D>
    for CircuitBuilder<F, D>
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

    fn curve_double<C: Curve>(&mut self, p: &AffinePointTarget<C>) -> AffinePointTarget<C> {
        let AffinePointTarget { x, y } = p;

        let xy = self.mul_nonnative(x, y);
        let x_squared = self.mul_nonnative(x, x);
        let y_squared = self.mul_nonnative(y, y);

        let x3_numer = self.add_nonnative(&xy, &xy);
        // the x3 denominator is a * x**2 + y**2, where a = -1
        // can be rewritten as y**2 - x**2
        let x3_denom = self.sub_nonnative(&y_squared, &x_squared);
        let x3_denom_inv = self.inv_nonnative(&x3_denom);
        let x3 = self.mul_nonnative(&x3_numer, &x3_denom_inv);

        let y3_numer = self.add_nonnative(&y_squared, &x_squared);
        // the y3 denominator is 2 - a * x**2 - y**2, where a = -1
        // can be rewritten as 2 + x**2 - y**2
        let two = self.constant_nonnative(C::BaseField::ONE.double());
        let two_plus_x_squared = self.add_nonnative(&two, &x_squared);
        let y3_denom = self.sub_nonnative(&two_plus_x_squared, &y_squared);
        let y3_denom_inv = self.inv_nonnative(&y3_denom);
        let y3 = self.mul_nonnative(&y3_numer, &y3_denom_inv);

        AffinePointTarget { x: x3, y: y3 }
    }

    fn curve_repeated_double<C: Curve>(
        &mut self,
        p: &AffinePointTarget<C>,
        n: usize,
    ) -> AffinePointTarget<C> {
        let mut result = p.clone();

        for _ in 0..n {
            result = self.curve_double(&result);
        }

        result
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

    fn curve_conditional_add<C: Curve>(
        &mut self,
        p1: &AffinePointTarget<C>,
        p2: &AffinePointTarget<C>,
        b: BoolTarget,
    ) -> AffinePointTarget<C> {
        let not_b = self.not(b);
        let sum = self.curve_add(p1, p2);
        let x_if_true = self.mul_nonnative_by_bool(&sum.x, b);
        let y_if_true = self.mul_nonnative_by_bool(&sum.y, b);
        let x_if_false = self.mul_nonnative_by_bool(&p1.x, not_b);
        let y_if_false = self.mul_nonnative_by_bool(&p1.y, not_b);

        let x = self.add_nonnative(&x_if_true, &x_if_false);
        let y = self.add_nonnative(&y_if_true, &y_if_false);

        AffinePointTarget { x, y }
    }

    fn curve_scalar_mul<C: Curve>(
        &mut self,
        p: &AffinePointTarget<C>,
        n: &NonNativeTarget<C::ScalarField>,
    ) -> AffinePointTarget<C> {
        let bits = self.split_nonnative_to_bits(n);

        let rando = (CurveScalar(C::ScalarField::rand()) * C::GENERATOR_PROJECTIVE).to_affine();
        let randot = self.constant_affine_point(rando);
        // Result starts at `rando`, which is later subtracted, because we don't support arithmetic with the zero point.
        let mut result = self.add_virtual_affine_point_target();
        self.connect_affine_point(&randot, &result);

        let mut two_i_times_p = self.add_virtual_affine_point_target();
        self.connect_affine_point(p, &two_i_times_p);

        for &bit in bits.iter() {
            let not_bit = self.not(bit);

            let result_plus_2_i_p = self.curve_add(&result, &two_i_times_p);

            let new_x_if_bit = self.mul_nonnative_by_bool(&result_plus_2_i_p.x, bit);
            let new_x_if_not_bit = self.mul_nonnative_by_bool(&result.x, not_bit);
            let new_y_if_bit = self.mul_nonnative_by_bool(&result_plus_2_i_p.y, bit);
            let new_y_if_not_bit = self.mul_nonnative_by_bool(&result.y, not_bit);

            let new_x = self.add_nonnative(&new_x_if_bit, &new_x_if_not_bit);
            let new_y = self.add_nonnative(&new_y_if_bit, &new_y_if_not_bit);

            result = AffinePointTarget { x: new_x, y: new_y };

            two_i_times_p = self.curve_double(&two_i_times_p);
        }

        // Subtract off result's intial value of `rando`.
        let neg_r = self.curve_neg(&randot);
        result = self.curve_add(&result, &neg_r);

        result
    }

    // This funciton will accept an affine point target and return
    // the point in compressed form (bit vector).
    fn compress_point<C: Curve>(&mut self, p: &AffinePointTarget<C>) -> Vec<BoolTarget> {
        /*
        let mut y_bits = self.split_nonnative_to_bits(&p.y);
        let num_y_bits = y_bits.len();
        let x_bits = self.split_nonnative_to_bits(&p.x);
        y_bits[num_y_bits-1] = self.or(y_bits[num_y_bits-1].clone(), x_bits[0]);
        y_bits
        */
        let mut bits = biguint_to_bits_target::<F, D, 2>(self, &p.y.value);
        let x_bits_low_32 = self.split_le_base::<2>(p.x.value.get_limb(0).0, 32);

        let a = bits[0].target;
        let b = x_bits_low_32[0];
        // a | b = a + b - a * b
        let a_add_b = self.add(a, b);
        let ab = self.mul(a, b);
        bits[0] = BoolTarget::new_unsafe(self.sub(a_add_b, ab));
        bits
    }

    fn decompress_point<C: Curve>(&mut self, pv: &[BoolTarget]) -> AffinePointTarget<C> {
        assert_eq!(pv.len(), 256);
        let p = self.add_virtual_affine_point_target();

        self.add_simple_generator(CurvePointDecompressionGenerator::<F, D, C> {
            pv: pv.to_vec(),
            p: p.clone(),
            _phantom: PhantomData,
        });

        let pv2 = self.compress_point(&p);
        for i in 0..256 {
            self.connect(pv[i].target, pv2[i].target);
        }
        p
    }


    fn convert_to_curta_affine_point_target<C: Curve>(&mut self, p: &AffinePointTarget<C>) -> CurtaAffinePointTarget {
        let x_limbs = self.split_nonnative_to_16_bit_limbs(&p.x);
        let y_limbs = self.split_nonnative_to_16_bit_limbs(&p.y);

        assert!(x_limbs.len() == 16);
        assert!(y_limbs.len() == 16);

        CurtaAffinePointTarget{
            x: x_limbs.try_into().unwrap(),
            y: y_limbs.try_into().unwrap(),
        }
    }

    fn convert_from_curta_affine_point_target<C: Curve>(&mut self, p: &CurtaAffinePointTarget) -> AffinePointTarget<C> {
        let x = self.recombine_nonnative_16_bit_limbs(p.x.to_vec());
        let y = self.recombine_nonnative_16_bit_limbs(p.y.to_vec());

        AffinePointTarget{x, y}
    }
}

#[derive(Debug, Default)]
pub struct CurvePointDecompressionGenerator<F: RichField + Extendable<D>, const D: usize, C: Curve> {
    pv: Vec<BoolTarget>,
    p: AffinePointTarget<C>,
    _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize, C: Curve> SimpleGenerator<F, D>
    for CurvePointDecompressionGenerator<F, D, C>
{
    fn id(&self) -> String {
        "CurvePointDecompressionGenerator".to_string()
    }

    fn serialize(&self, dst: &mut Vec<u8>, _common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        dst.write_usize(self.pv.len())?;
        for target in self.pv.iter() {
            dst.write_target_bool(*target)?;
        }
        dst.write_target_affine_point(self.p.clone())
    }

    fn deserialize(src: &mut Buffer, _common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        let pv_len = src.read_usize()?;
        let pv = (0..pv_len)
            .map(|_| src.read_target_bool())
            .collect::<Result<Vec<_>, _>>()?;
        let p = src.read_target_affine_point()?;

        Ok(Self { pv, p, _phantom: PhantomData })
    }

    fn dependencies(&self) -> Vec<Target> {
        self.pv.iter().cloned().map(|l| l.target).collect()
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let mut bits = Vec::new();
        for i in 0..256 {
            bits.push(witness.get_bool_target(self.pv[i]));
        }
        let mut s: [u8; 32] = [0; 32];
        for i in 0..32 {
            for j in 0..8 {
                if bits[i * 8 + j] {
                    s[31 - i] += 1 << (7 - j);
                }
            }
        }
        let point = decompress_point(s.as_slice());

        out_buffer.set_biguint_target(&self.p.x.value, &point.x.to_canonical_biguint());
        out_buffer.set_biguint_target(&self.p.y.value, &point.y.to_canonical_biguint());
    }
}

pub fn decompress_point(s: &[u8]) -> AffinePoint<Ed25519> {
    let mut s32 = [0u8; 32];
    s32.copy_from_slice(s);
    let compressed = CompressedEdwardsY(s32);
    let point = compressed.decompress().unwrap();
    let x_biguint = BigUint::from_bytes_le(&point.get_x().as_bytes());
    let y_biguint = BigUint::from_bytes_le(&point.get_y().as_bytes());
    AffinePoint::nonzero(
        Ed25519Base::from_noncanonical_biguint(x_biguint),
        Ed25519Base::from_noncanonical_biguint(y_biguint),
    )
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
        Ok(AffinePointTarget{ x, y })
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Neg;

    use anyhow::Result;
    use num::BigUint;
    use plonky2::iop::witness::{PartialWitness, WitnessWrite};
    use plonky2::plonk::circuit_builder::CircuitBuilder;
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
    use plonky2::field::types::{Field, Sample};

    use crate::ecc::ed25519::curve::curve_types::{AffinePoint, Curve, CurveScalar};
    use crate::ecc::ed25519::curve::ed25519::Ed25519;
    use crate::ecc::ed25519::field::ed25519_base::Ed25519Base;
    use crate::ecc::ed25519::field::ed25519_scalar::Ed25519Scalar;
    use crate::ecc::ed25519::gadgets::curve::CircuitBuilderCurve;
    use crate::hash::bit_operations::util::bits_to_biguint_target;
    use crate::num::biguint::CircuitBuilderBiguint;
    use crate::num::nonnative::nonnative::CircuitBuilderNonNative;

    #[test]
    fn test_curve_point_is_valid() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_ecc_config();

        let pw = PartialWitness::new();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let g = Ed25519::GENERATOR_AFFINE;
        let g_target = builder.constant_affine_point(g);
        let neg_g_target = builder.curve_neg(&g_target);

        builder.curve_assert_valid(&g_target);
        builder.curve_assert_valid(&neg_g_target);

        let inner_data = builder.build::<C>();
        let inner_proof = inner_data.prove(pw).unwrap();

        inner_data.verify(inner_proof.clone()).unwrap();

        let mut outer_builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_ecc_config());
        let inner_proof_target = outer_builder.add_virtual_proof_with_pis(&inner_data.common);
        let inner_verifier_data = outer_builder.add_virtual_verifier_data(inner_data.common.config.fri_config.cap_height);
        outer_builder.verify_proof::<C>(&inner_proof_target, &inner_verifier_data, &inner_data.common);

        let outer_data = outer_builder.build::<C>();

        let mut outer_pw = PartialWitness::new();
        outer_pw.set_proof_with_pis_target(&inner_proof_target, &inner_proof);
        outer_pw.set_verifier_data_target(&inner_verifier_data, &inner_data.verifier_only);

        let outer_proof = outer_data.prove(outer_pw).unwrap();
        outer_data.verify(outer_proof)

    }

    #[test]
    #[should_panic]
    fn test_curve_point_is_not_valid() {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_ecc_config();

        let pw = PartialWitness::new();
        let mut builder = CircuitBuilder::<F, D>::new(config);

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
    fn test_curve_double() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_ecc_config();

        let pw = PartialWitness::new();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let g = Ed25519::GENERATOR_AFFINE;
        let g_target = builder.constant_affine_point(g);
        let neg_g_target = builder.curve_neg(&g_target);

        let double_g = g.double();
        let double_g_expected = builder.constant_affine_point(double_g);
        builder.curve_assert_valid(&double_g_expected);

        let double_neg_g = (-g).double();
        let double_neg_g_expected = builder.constant_affine_point(double_neg_g);
        builder.curve_assert_valid(&double_neg_g_expected);

        let double_g_actual = builder.curve_double(&g_target);
        let double_neg_g_actual = builder.curve_double(&neg_g_target);
        builder.curve_assert_valid(&double_g_actual);
        builder.curve_assert_valid(&double_neg_g_actual);

        builder.connect_affine_point(&double_g_expected, &double_g_actual);
        builder.connect_affine_point(&double_neg_g_expected, &double_neg_g_actual);

        dbg!(builder.num_gates());
        let inner_data = builder.build::<C>();
        let inner_proof = inner_data.prove(pw).unwrap();
        inner_data.verify(inner_proof.clone()).unwrap();


        let mut outer_builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_ecc_config());
        let inner_proof_target = outer_builder.add_virtual_proof_with_pis(&inner_data.common);
        let inner_verifier_data = outer_builder.add_virtual_verifier_data(inner_data.common.config.fri_config.cap_height);
        outer_builder.verify_proof::<C>(&inner_proof_target, &inner_verifier_data, &inner_data.common);

        let outer_data = outer_builder.build::<C>();

        let mut outer_pw = PartialWitness::new();
        outer_pw.set_proof_with_pis_target(&inner_proof_target, &inner_proof);
        outer_pw.set_verifier_data_target(&inner_verifier_data, &inner_data.verifier_only);

        let outer_proof = outer_data.prove(outer_pw).unwrap();
    
        outer_data.verify(outer_proof)


    }

    #[test]
    fn test_curve_add() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_ecc_config();

        let pw = PartialWitness::new();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let g = Ed25519::GENERATOR_AFFINE;
        let double_g = g.double();
        let g_plus_2g = g + double_g;
        let g_plus_2g_expected = builder.constant_affine_point(g_plus_2g);
        builder.curve_assert_valid(&g_plus_2g_expected);

        let g_target = builder.constant_affine_point(g);
        let double_g_target = builder.curve_double(&g_target);
        let g_plus_2g_actual = builder.curve_add(&g_target, &double_g_target);
        builder.curve_assert_valid(&g_plus_2g_actual);

        builder.connect_affine_point(&g_plus_2g_expected, &g_plus_2g_actual);

        dbg!(builder.num_gates());
        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();

        data.verify(proof)
        
    }

    #[test]
    fn test_curve_conditional_add() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_ecc_config();

        let pw = PartialWitness::new();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let g = Ed25519::GENERATOR_AFFINE;
        let double_g = g.double();
        let g_plus_2g = g + double_g;
        let g_plus_2g_expected = builder.constant_affine_point(g_plus_2g);

        let g_expected = builder.constant_affine_point(g);
        let double_g_target = builder.curve_double(&g_expected);
        let t = builder._true();
        let f = builder._false();
        let g_plus_2g_actual = builder.curve_conditional_add(&g_expected, &double_g_target, t);
        let g_actual = builder.curve_conditional_add(&g_expected, &double_g_target, f);

        builder.connect_affine_point(&g_plus_2g_expected, &g_plus_2g_actual);
        builder.connect_affine_point(&g_expected, &g_actual);

        let inner_data = builder.build::<C>();
        let inner_proof = inner_data.prove(pw).unwrap();
        inner_data.verify(inner_proof.clone()).unwrap();



        let mut outer_builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_ecc_config());
        let inner_proof_target = outer_builder.add_virtual_proof_with_pis(&inner_data.common);
        let inner_verifier_data = outer_builder.add_virtual_verifier_data(inner_data.common.config.fri_config.cap_height);
        outer_builder.verify_proof::<C>(&inner_proof_target, &inner_verifier_data, &inner_data.common);

        let outer_data = outer_builder.build::<C>();

        let mut outer_pw = PartialWitness::new();
        outer_pw.set_proof_with_pis_target(&inner_proof_target, &inner_proof);
        outer_pw.set_verifier_data_target(&inner_verifier_data, &inner_data.verifier_only);

        let outer_proof = outer_data.prove(outer_pw).unwrap();
    
        outer_data.verify(outer_proof)

    }

    #[test]
    fn test_curve_mul() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_ecc_config();

        let pw = PartialWitness::new();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let g = Ed25519::GENERATOR_PROJECTIVE.to_affine();
        let five = Ed25519Scalar::from_canonical_usize(5);
        let neg_five = five.neg();
        let neg_five_scalar = CurveScalar::<Ed25519>(neg_five);
        let neg_five_g = (neg_five_scalar * g.to_projective()).to_affine();
        let neg_five_g_expected = builder.constant_affine_point(neg_five_g);
        builder.curve_assert_valid(&neg_five_g_expected);

        let g_target = builder.constant_affine_point(g);
        let neg_five_target = builder.constant_nonnative(neg_five);
        let neg_five_g_actual = builder.curve_scalar_mul(&g_target, &neg_five_target);
        builder.curve_assert_valid(&neg_five_g_actual);

        builder.connect_affine_point(&neg_five_g_expected, &neg_five_g_actual);

        let inner_data = builder.build::<C>();
        let inner_proof = inner_data.prove(pw).unwrap();

        inner_data.verify(inner_proof.clone()).unwrap();




        let mut outer_builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_ecc_config());
        let inner_proof_target = outer_builder.add_virtual_proof_with_pis(&inner_data.common);
        let inner_verifier_data = outer_builder.add_virtual_verifier_data(inner_data.common.config.fri_config.cap_height);
        outer_builder.verify_proof::<C>(&inner_proof_target, &inner_verifier_data, &inner_data.common);

        let outer_data = outer_builder.build::<C>();

        let mut outer_pw = PartialWitness::new();
        outer_pw.set_proof_with_pis_target(&inner_proof_target, &inner_proof);
        outer_pw.set_verifier_data_target(&inner_verifier_data, &inner_data.verifier_only);

        let outer_proof = outer_data.prove(outer_pw).unwrap();
    
        outer_data.verify(outer_proof)

    }

    #[test]
    #[ignore]
    fn test_curve_random() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_ecc_config();

        let pw = PartialWitness::new();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let rando =
            (CurveScalar(Ed25519Scalar::rand()) * Ed25519::GENERATOR_PROJECTIVE).to_affine();
        let randot = builder.constant_affine_point(rando);

        let two_target = builder.constant_nonnative(Ed25519Scalar::TWO);
        let randot_doubled = builder.curve_double(&randot);
        let randot_times_two = builder.curve_scalar_mul(&randot, &two_target);
        builder.connect_affine_point(&randot_doubled, &randot_times_two);

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();

        data.verify(proof)
    }

    // from: https://github.com/polymerdao/plonky2-ed25519/blob/main/src/gadgets/curve.rs#L589
    #[test]
    fn test_point_compress_decompress() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_ecc_config();

        let pw = PartialWitness::new();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let rando =
            (CurveScalar(Ed25519Scalar::rand()) * Ed25519::GENERATOR_PROJECTIVE).to_affine();
        assert!(rando.is_valid());

        let randot = builder.constant_affine_point(rando);

        let rando_compressed = builder.compress_point(&randot);
        let rando_decompressed = builder.decompress_point(&rando_compressed);

        builder.connect_affine_point(&randot, &rando_decompressed);

        let inner_data = builder.build::<C>();
        let inner_proof = inner_data.prove(pw).unwrap();
        inner_data.verify(inner_proof.clone()).unwrap();



        let mut outer_builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_ecc_config());
        let inner_proof_target = outer_builder.add_virtual_proof_with_pis(&inner_data.common);
        let inner_verifier_data = outer_builder.add_virtual_verifier_data(inner_data.common.config.fri_config.cap_height);
        outer_builder.verify_proof::<C>(&inner_proof_target, &inner_verifier_data, &inner_data.common);

        let outer_data = outer_builder.build::<C>();

        let mut outer_pw = PartialWitness::new();
        outer_pw.set_proof_with_pis_target(&inner_proof_target, &inner_proof);
        outer_pw.set_verifier_data_target(&inner_verifier_data, &inner_data.verifier_only);

        let outer_proof = outer_data.prove(outer_pw).unwrap();
    
        outer_data.verify(outer_proof)

    }

    #[test]
    #[ignore]
    // FIXME: failing
    fn test_compress_point() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_ecc_config();

        let pw = PartialWitness::new();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        /*let priv_key = Ed25519Scalar::from_canonical_usize(5);
        let g = Ed25519::GENERATOR_AFFINE;
        let pub_key = (CurveScalar(priv_key) * g.to_projective()).to_affine();
        let pub_key_t = builder.constant_affine_point(pub_key);

        println!("pub_key point is ({}, {})", pub_key.x, pub_key.y);
        */

        let sig_r_x_biguint = BigUint::parse_bytes(
            b"34429777554096177233623231228348362084988839912431844356123812156003444176586",
            10,
        )
        .unwrap();
        let sig_r_x = Ed25519Base::from_noncanonical_biguint(sig_r_x_biguint);
        let sig_r_y_biguint = BigUint::parse_bytes(
            b"22119998304038584770835958502800813263484475345060077692632207186036243708011",
            10,
        )
        .unwrap();
        let sig_r_y = Ed25519Base::from_noncanonical_biguint(sig_r_y_biguint);
        let pub_key = AffinePoint::<Ed25519> {
            x: sig_r_x,
            y: sig_r_y,
            zero: false,
        };
        let pub_key_t = builder.constant_affine_point(pub_key);

        /*
        >>> import pure25519.basic
        >>> privateKey = 5
        >>> pkBytes = privateKey.to_bytes(32, byteorder="little")
        >>> pkScalar = pure25519.basic.bytes_to_clamped_scalar(pkBytes)
        >>> pubKey = pure25519.basic.Base.scalarmult(pkScalar)
        >>> pubKey.to_bytes().hex()
        'edc876d6831fd2105d0b4389ca2e283166469289146e2ce06faefe98b22548df'
        */

        //let pub_key_compressed = BigUint::parse_bytes(b"df4825b298feae6fe02c6e148992466631282eca89430b5d10d21f83d676c8ed", 16).unwrap();
        let pub_key_compressed = BigUint::parse_bytes(
            b"30e779b1a01719b15aeab33b736949ece8ed46276b09b37880dec2bb411a386b",
            16,
        )
        .unwrap();
        let pub_key_compressed_t = builder.constant_biguint(&pub_key_compressed);
        let mut pub_key_compressed_actual = builder.compress_point(&pub_key_t);

        // TODO: This is really hacky.  bits_to_biguint_target expects the bit vector to be in big-endian format.
        pub_key_compressed_actual.reverse();
        let pub_key_compressed_actual_bigint =
            bits_to_biguint_target(&mut builder, pub_key_compressed_actual);

        builder.curve_assert_valid(&pub_key_t);
        builder.connect_biguint(&pub_key_compressed_actual_bigint, &pub_key_compressed_t);

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();

        data.verify(proof)
    }
}
