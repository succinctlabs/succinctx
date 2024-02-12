pub use curve25519_dalek::edwards::CompressedEdwardsY;
use starkyx::chip::ec::point::{AffinePoint, AffinePointRegister};
use starkyx::chip::ec::EllipticCurve;
use starkyx::chip::register::Register;

use crate::frontend::curta::field::variable::FieldVariable;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AffinePointVariable<E: EllipticCurve> {
    pub x: FieldVariable<E::BaseField>,
    pub y: FieldVariable<E::BaseField>,
}

impl<E: EllipticCurve> CircuitVariable for AffinePointVariable<E> {
    type ValueType<F: RichField> = AffinePoint<E>;

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        Self {
            x: FieldVariable::init_unsafe(builder),
            y: FieldVariable::init_unsafe(builder),
        }
    }

    fn variables(&self) -> Vec<Variable> {
        let mut variables = self.x.variables();
        variables.extend(self.y.variables());
        variables
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        let len = variables.len();
        assert!(len % 2 == 0);
        let x = FieldVariable::from_variables_unsafe(&variables[..len / 2]);
        let y = FieldVariable::from_variables_unsafe(&variables[len / 2..]);
        Self { x, y }
    }

    // This function will verify that the inner elements are valid FieldVariables.  It will NOT
    // verify that the point is a valid EC point.
    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        self.x.assert_is_valid(builder);
        self.y.assert_is_valid(builder);
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        let mut elements = FieldVariable::<E::BaseField>::elements::<F>(value.x);
        elements.extend(FieldVariable::<E::BaseField>::elements::<F>(value.y));
        elements
    }

    fn nb_elements() -> usize {
        FieldVariable::<E::BaseField>::nb_elements() * 2
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        let len = elements.len();
        assert!(len % 2 == 0);
        let x = FieldVariable::<E::BaseField>::from_elements(&elements[..len / 2]);
        let y = FieldVariable::<E::BaseField>::from_elements(&elements[len / 2..]);
        AffinePoint::new(x, y)
    }
}

impl<E: EllipticCurve> AffinePointVariable<E> {
    pub fn read_from_stark(register: &AffinePointRegister<E>, public_inputs: &[Variable]) -> Self {
        AffinePointVariable {
            x: FieldVariable::new(register.x.read_from_slice(public_inputs).as_coefficients()),
            y: FieldVariable::new(register.y.read_from_slice(public_inputs).as_coefficients()),
        }
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

    // This function will verify that the inner element is a valid Bytes32Variable.  It will NOT
    // verify that the decompressed point is a valid EC point.
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
