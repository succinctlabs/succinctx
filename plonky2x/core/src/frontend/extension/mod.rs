use std::collections::HashMap;

use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use starkyx::math::prelude::cubic::element::CubicElement;
use starkyx::plonky2::cubic::builder::CubicCircuitBuilder;

use crate::prelude::{
    Add, CircuitBuilder, CircuitVariable, Mul, One, PlonkParameters, Sub, Variable, Zero,
};

#[derive(Debug, Clone, Copy)]
pub struct CubicExtensionVariable {
    pub elements: [Variable; 3],
}

impl CubicExtensionVariable {
    pub fn new(a: Variable, b: Variable, c: Variable) -> Self {
        Self {
            elements: [a, b, c],
        }
    }
}

impl CircuitVariable for CubicExtensionVariable {
    type ValueType<F: RichField> = CubicElement<F>;

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        Self {
            elements: [
                Variable::init_unsafe(builder),
                Variable::init_unsafe(builder),
                Variable::init_unsafe(builder),
            ],
        }
    }

    fn variables(&self) -> Vec<Variable> {
        self.elements.to_vec()
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        assert_eq!(variables.len(), 3);
        Self {
            elements: [variables[0], variables[1], variables[2]],
        }
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(&self, _: &mut CircuitBuilder<L, D>) {
    }

    fn nb_elements() -> usize {
        3
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        value.0.to_vec()
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        assert_eq!(elements.len(), 3);
        CubicElement([elements[0], elements[1], elements[2]])
    }
}

impl Variable {
    pub fn as_cubic_extension<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) -> CubicExtensionVariable {
        let zero = builder.zero::<Variable>();
        CubicExtensionVariable::new(*self, zero, zero)
    }
}

impl From<CubicExtensionVariable> for CubicElement<Target> {
    fn from(value: CubicExtensionVariable) -> Self {
        Self([
            value.elements[0].0,
            value.elements[1].0,
            value.elements[2].0,
        ])
    }
}

impl From<CubicElement<Target>> for CubicExtensionVariable {
    fn from(value: CubicElement<Target>) -> Self {
        Self {
            elements: [value.0[0].into(), value.0[1].into(), value.0[2].into()],
        }
    }
}

impl<L: PlonkParameters<D>, const D: usize> Zero<L, D> for CubicExtensionVariable {
    fn zero(builder: &mut CircuitBuilder<L, D>) -> Self {
        Self {
            elements: [
                Variable::zero(builder),
                Variable::zero(builder),
                Variable::zero(builder),
            ],
        }
    }
}

impl<L: PlonkParameters<D>, const D: usize> One<L, D> for CubicExtensionVariable {
    fn one(builder: &mut CircuitBuilder<L, D>) -> Self {
        Self {
            elements: [
                Variable::one(builder),
                Variable::zero(builder),
                Variable::zero(builder),
            ],
        }
    }
}

impl<L: PlonkParameters<D>, const D: usize> Add<L, D> for CubicExtensionVariable {
    type Output = CubicExtensionVariable;

    fn add(self, rhs: Self, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        let mut cache = HashMap::new();
        builder
            .api
            .add_cubic(self.into(), rhs.into(), &mut cache)
            .into()
    }
}

impl<L: PlonkParameters<D>, const D: usize> Sub<L, D> for CubicExtensionVariable {
    type Output = CubicExtensionVariable;

    fn sub(self, rhs: Self, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        let mut cache = HashMap::new();
        builder
            .api
            .sub_cubic(self.into(), rhs.into(), &mut cache)
            .into()
    }
}

impl<L: PlonkParameters<D>, const D: usize> Mul<L, D> for CubicExtensionVariable {
    type Output = CubicExtensionVariable;

    fn mul(self, rhs: Self, builder: &mut CircuitBuilder<L, D>) -> Self::Output {
        let mut cache = HashMap::new();
        builder
            .api
            .mul_cubic(self.into(), rhs.into(), &mut cache)
            .into()
    }
}
