use std::ops::Add;

use plonky2::field::ops::Square;
use plonky2::field::types::Field;

use crate::frontend::ecc::ed25519::curve::curve_types::{AffinePoint, Curve, ProjectivePoint};

impl<C: Curve> Add<ProjectivePoint<C>> for ProjectivePoint<C> {
    type Output = ProjectivePoint<C>;

    fn add(self, rhs: ProjectivePoint<C>) -> Self::Output {
        let ProjectivePoint {
            x: x1,
            y: y1,
            z: z1,
        } = self;
        let ProjectivePoint {
            x: x2,
            y: y2,
            z: z2,
        } = rhs;

        if z1 == C::BaseField::ZERO {
            return rhs;
        }
        if z2 == C::BaseField::ZERO {
            return self;
        }

        let x1z2 = x1 * z2;
        let y1z2 = y1 * z2;
        let x2z1 = x2 * z1;
        let y2z1 = y2 * z1;

        // Check if we're doubling or adding inverses.
        if x1z2 == x2z1 {
            if y1z2 == y2z1 {
                // TODO: inline to avoid redundant muls.
                return self.double();
            }
            if y1z2 == -y2z1 {
                return ProjectivePoint::ZERO;
            }
        }

        // From https://en.wikipedia.org/wiki/Twisted_Edwards_curve#Projective_twisted_Edwards_coordinates
        let a = z1 * z2;
        let b = a.square();
        let c = x1 * x2;
        let d = y1 * y2;
        let e = C::D * c * d;
        let f = b - e;
        let g = b + e;
        let x3 = a * f * ((x1 + y1) * (x2 + y2) - c - d);
        let y3 = a * g * (d - C::A * c); // TODO: replace (d - C::A * c) with (d + c)
        let z3 = f * g;
        ProjectivePoint::nonzero(x3, y3, z3)
    }
}

impl<C: Curve> Add<AffinePoint<C>> for ProjectivePoint<C> {
    type Output = ProjectivePoint<C>;

    fn add(self, rhs: AffinePoint<C>) -> Self::Output {
        let ProjectivePoint {
            x: x1,
            y: y1,
            z: z1,
        } = self;
        let AffinePoint {
            x: x2,
            y: y2,
            zero: zero2,
        } = rhs;

        if z1 == C::BaseField::ZERO {
            return rhs.to_projective();
        }
        if zero2 {
            return self;
        }

        let x2z1 = x2 * z1;
        let y2z1 = y2 * z1;

        // Check if we're doubling or adding inverses.
        if x1 == x2z1 {
            if y1 == y2z1 {
                // TODO: inline to avoid redundant muls.
                return self.double();
            }
            if y1 == -y2z1 {
                return ProjectivePoint::ZERO;
            }
        }

        // From https://en.wikipedia.org/wiki/Twisted_Edwards_curve#Addition_in_projective_twisted_curves
        let u = y2z1 - y1;
        let uu = u.square();
        let v = x2z1 - x1;
        let vv = v.square();
        let vvv = v * vv;
        let r = vv * x1;
        let a = uu * z1 - vvv - r.double();
        let x3 = v * a;
        let y3 = u * (r - a) - vvv * y1;
        let z3 = vvv * z1;
        ProjectivePoint::nonzero(x3, y3, z3)
    }
}

impl<C: Curve> Add<AffinePoint<C>> for AffinePoint<C> {
    type Output = AffinePoint<C>;

    fn add(self, rhs: AffinePoint<C>) -> Self::Output {
        let AffinePoint {
            x: x1,
            y: y1,
            zero: zero1,
        } = self;
        let AffinePoint {
            x: x2,
            y: y2,
            zero: zero2,
        } = rhs;

        if zero1 {
            return rhs;
        }
        if zero2 {
            return self;
        }

        // Check if we're doubling or adding inverses.
        if x1 == x2 {
            if y1 == y2 {
                return self.double();
            }
            if y1 == -y2 {
                return AffinePoint::ZERO;
            }
        }

        // From https://en.wikipedia.org/wiki/Twisted_Edwards_curve#Addition_on_twisted_Edwards_curves
        let x1y2 = x1 * y2;
        let y1x2 = y1 * x2;
        let y1y2 = y1 * y2;
        let x1x2 = x1 * x2;
        let x1x2y1y2 = x1x2 * y1y2;

        let x3_num = x1y2 + y1x2;
        let x3_den = C::BaseField::ONE + C::D * x1x2y1y2;

        let y3_num = y1y2 - C::A * x1x2;
        let y3_den = C::BaseField::ONE - C::D * x1x2y1y2;

        Self {
            x: x3_num / x3_den,
            y: y3_num / y3_den,
            zero: false,
        }
    }
}
