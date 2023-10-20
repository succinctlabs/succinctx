use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Neg;

use curve25519_dalek::edwards::CompressedEdwardsY;
use num::{BigUint, Integer, One};
use plonky2::field::ops::Square;
use plonky2::field::types::{Field, PrimeField};
use serde::{Deserialize, Serialize};

// To avoid implementation conflicts from associated types,
// see https://github.com/rust-lang/rust/issues/20400
pub struct CurveScalar<C: Curve>(pub <C as Curve>::ScalarField);

/// A Twisted Edwards curve.
pub trait Curve: 'static + Sync + Sized + Copy + Debug {
    type BaseField: PrimeField;
    type ScalarField: PrimeField;

    const A: Self::BaseField;
    const D: Self::BaseField;

    const GENERATOR_AFFINE: AffinePoint<Self>;

    fn convert(x: Self::ScalarField) -> CurveScalar<Self> {
        CurveScalar(x)
    }

    // TODO(kevjue):  Check if this is correct!
    fn is_safe_curve() -> bool {
        // Added additional check to prevent using vulnerabilties in case a discriminant is equal to 0.
        (Self::A.cube().double().double() + Self::D.square().triple().triple().triple())
            .is_nonzero()
    }
}

/// A point on a Twisted Edwards curve, represented in affine coordinates.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct AffinePoint<C: Curve> {
    pub x: C::BaseField,
    pub y: C::BaseField,
    pub zero: bool,
}

impl<C: Curve> AffinePoint<C> {
    pub const ZERO: Self = Self {
        x: C::BaseField::ZERO,
        y: C::BaseField::ZERO,
        zero: true,
    };

    pub fn new_from_compressed_point(s: &[u8]) -> Self {
        let mut s32 = [0u8; 32];
        s32.copy_from_slice(s);
        let compressed = CompressedEdwardsY(s32);
        let point = compressed.decompress().unwrap();
        let x_biguint = BigUint::from_bytes_le(&point.get_x().as_bytes());
        let y_biguint = BigUint::from_bytes_le(&point.get_y().as_bytes());
        AffinePoint::nonzero(
            C::BaseField::from_noncanonical_biguint(x_biguint),
            C::BaseField::from_noncanonical_biguint(y_biguint),
        )
    }

    pub fn nonzero(x: C::BaseField, y: C::BaseField) -> Self {
        let point = Self { x, y, zero: false };
        debug_assert!(point.is_valid());
        point
    }

    // Taken from here:  https://github.com/warner/python-pure25519/blob/master/pure25519/basic.py#L124
    pub fn is_valid(&self) -> bool {
        let Self { x, y, zero } = *self;
        zero || C::A * x.square() + y.square() == C::BaseField::ONE + C::D * x.square() * y.square()
    }

    #[must_use]
    // Algorithm can be found here: https://en.wikipedia.org/wiki/Twisted_Edwards_curve#Doubling_on_twisted_Edwards_curves
    pub fn double(&self) -> Self {
        let AffinePoint { x: x1, y: y1, zero } = *self;

        if zero {
            return AffinePoint::ZERO;
        }

        let x_num = (x1 * y1).double(); // 2 * x1 * y1
        let x_den = C::A * x1.square() + y1.square(); // a * x1**2 + y1**2
        let x3 = x_num * x_den.inverse();

        let y_num = y1.square() - C::A * x1.square(); // y1**2 - a * x1**2
        let y_den = C::BaseField::ONE.double() - C::A * x1.square() - y1.square(); // 2 - a * x1 ** 2 - y1 ** 2
        let y3 = y_num * y_den.inverse();

        Self {
            x: x3,
            y: y3,
            zero: false,
        }
    }

    pub fn compress_point(&self) -> BigUint {
        let mut compressed_point = self.y.to_canonical_biguint();
        if self.x.to_canonical_biguint().is_odd() {
            compressed_point += BigUint::one() << 255;
        }
        compressed_point
    }
}

impl<C: Curve> PartialEq for AffinePoint<C> {
    fn eq(&self, other: &Self) -> bool {
        let AffinePoint {
            x: x1,
            y: y1,
            zero: zero1,
        } = *self;
        let AffinePoint {
            x: x2,
            y: y2,
            zero: zero2,
        } = *other;
        if zero1 || zero2 {
            return zero1 == zero2;
        }
        x1 == x2 && y1 == y2
    }
}

impl<C: Curve> Eq for AffinePoint<C> {}

impl<C: Curve> Hash for AffinePoint<C> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.zero {
            self.zero.hash(state);
        } else {
            self.x.hash(state);
            self.y.hash(state);
        }
    }
}

impl<C: Curve> Neg for AffinePoint<C> {
    type Output = AffinePoint<C>;

    fn neg(self) -> Self::Output {
        let AffinePoint { x, y, zero } = self;
        AffinePoint { x: -x, y, zero }
    }
}

pub fn base_to_scalar<C: Curve>(x: C::BaseField) -> C::ScalarField {
    C::ScalarField::from_noncanonical_biguint(x.to_canonical_biguint())
}

pub fn scalar_to_base<C: Curve>(x: C::ScalarField) -> C::BaseField {
    C::BaseField::from_noncanonical_biguint(x.to_canonical_biguint())
}
