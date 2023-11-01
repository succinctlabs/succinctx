use curta::chip::ec::point::{AffinePoint, AffinePointRegister};
use curta::chip::ec::EllipticCurve;
use curta::chip::register::Register;
use curve25519_dalek::edwards::CompressedEdwardsY;

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

#[derive(Debug, Clone)]
pub struct CompressedEdwardsYVariable(pub Bytes32Variable);

impl CircuitVariable for CompressedEdwardsYVariable {
    type ValueType<F: RichField> = CompressedEdwardsY;

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        Self(Bytes32Variable::init_unsafe(builder))
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        self.0.assert_is_valid(builder)
    }

    fn nb_elements() -> usize {
        Bytes32Variable::nb_elements()
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        BytesVariable::<32>::elements(*value.as_bytes())
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        CompressedEdwardsY(BytesVariable::<32>::from_elements(elements))
    }

    fn variables(&self) -> Vec<Variable> {
        self.0.variables()
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        Self(Bytes32Variable::from_variables_unsafe(variables))
    }
}
