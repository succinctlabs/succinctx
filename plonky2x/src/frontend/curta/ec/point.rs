use curta::chip::ec::point::AffinePoint;
use curta::chip::ec::EllipticCurveParameters;

use crate::frontend::curta::field::variable::FieldVariable;
use crate::prelude::*;

#[derive(Debug, Clone, CircuitVariable)]
pub struct AffinePointVariable<E: EllipticCurveParameters> {
    pub x: FieldVariable<E::BaseField>,
    pub y: FieldVariable<E::BaseField>,
}

impl<F: RichField, E: EllipticCurveParameters> From<AffinePoint<E>>
    for AffinePointVariableValue<E, F>
{
    fn from(value: AffinePoint<E>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl<F: RichField, E: EllipticCurveParameters> From<AffinePointVariableValue<E, F>>
    for AffinePoint<E>
{
    fn from(value: AffinePointVariableValue<E, F>) -> Self {
        Self::new(value.x, value.y)
    }
}
