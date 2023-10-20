use curta::chip::ec::edwards::scalar_mul::generator::AffinePointTarget as CurtaAffinePointTarget;
use plonky2::field::extension::Extendable;
use plonky2::field::types::{Field, PrimeField, PrimeField64};
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::BoolTarget;
use plonky2::iop::witness::Witness;
use plonky2::plonk::circuit_builder::CircuitBuilder as BaseCircuitBuilder;
use plonky2::util::serialization::{Buffer, IoResult};

use crate::frontend::ecc::ed25519::curve::curve_types::{AffinePoint, Curve};
use crate::frontend::hash::bit_operations::util::biguint_to_bits_target;
use crate::frontend::num::biguint::WitnessBigUint;
use crate::frontend::num::nonnative::nonnative::{
    CircuitBuilderNonNative, NonNativeTarget, ReadNonNativeTarget, WriteNonNativeTarget,
};
use crate::frontend::num::nonnative::split_nonnative::CircuitBuilderSplit;
use crate::prelude::{CircuitBuilder, CircuitVariable, PlonkParameters, Variable};
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

    /// Add two points, which are assumed to be non-equal.
    fn curve_add<C: Curve>(
        &mut self,
        p1: &AffinePointTarget<C>,
        p2: &AffinePointTarget<C>,
    ) -> AffinePointTarget<C>;

    fn compress_point<C: Curve>(&mut self, p: &AffinePointTarget<C>) -> CompressedPointTarget;

    fn convert_to_curta_affine_point_target<C: Curve>(
        &mut self,
        p: &AffinePointTarget<C>,
    ) -> CurtaAffinePointTarget;

    fn convert_from_curta_affine_point_target<C: Curve>(
        &mut self,
        p: &CurtaAffinePointTarget,
    ) -> AffinePointTarget<C>;
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

    // This funciton will accept an affine point target and return
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
