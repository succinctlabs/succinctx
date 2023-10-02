use curta::chip::ec::point::{AffinePoint, AffinePointRegister};
use curta::chip::ec::EllipticCurve;
use curta::chip::register::Register;

use crate::frontend::curta::field::variable::FieldVariable;
use crate::prelude::*;

#[derive(Debug, Clone, CircuitVariable)]
pub struct AffinePointVariable<E: EllipticCurve> {
    pub x: FieldVariable<E::BaseField>,
    pub y: FieldVariable<E::BaseField>,
}

impl<E: EllipticCurve> AffinePointVariable<E> {
    pub fn read_from_stark(register: &AffinePointRegister<E>, public_inputs: &[Variable]) -> Self {
        AffinePointVariable {
            x: FieldVariable::new(register.x.read_from_slice(public_inputs).as_coefficients()),
            y: FieldVariable::new(register.y.read_from_slice(public_inputs).as_coefficients()),
        }
    }
}

impl<F: RichField, E: EllipticCurve> From<AffinePoint<E>> for AffinePointVariableValue<E, F> {
    fn from(value: AffinePoint<E>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl<F: RichField, E: EllipticCurve> From<AffinePointVariableValue<E, F>> for AffinePoint<E> {
    fn from(value: AffinePointVariableValue<E, F>) -> Self {
        Self::new(value.x, value.y)
    }
}
