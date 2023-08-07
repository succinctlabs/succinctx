use anyhow::Result;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::Field;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

use crate::vars::{BoolVariable, Variable};

const D: usize = 2;
type C = PoseidonGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;

/// This is the API that we recommend developers use for writing circuits. It is a wrapper around
/// the basic plonky2 API.
pub struct API {
    api: CircuitBuilder<F, D>,
}

pub impl API {
    /// Creates a new builder API.
    fn new() -> Self {
        let config = CircuitConfig::<C, D>::new();
        let api = CircuitBuilder::new(config);
        Self { api }
    }

    /// Add returns res = i1 + i2.
    fn add(&self, i1: Variable, i2: Variable) -> Variable {
        Variable::from_target(self.api.add(i1.value, i2.value))
    }

    /// Add returns res = i1 + i2 + ...
    fn add(&self, values: &[Variable]) -> Variable {
        let mut acc = values[0];
        for i in 1..values.len() {
            acc = self.api.add(acc, values[i].value);
        }
        Variable::from_target(acc)
    }

    /// Sub returns res = i1 - i2.
    fn sub(&self, i1: Variable, i2: Variable) -> Variable {
        Variable::from_target(self.api.sub(i1.value, i2.value))
    }

    /// Sub returns res = i1 - i2 - ...
    fn sub(&self, values: &[Variable]) -> Variable {
        let mut acc = values[0];
        for i in 1..values.len() {
            acc = self.api.sub(acc, values[i].value);
        }
        Variable::from_target(acc)
    }

    /// Mul returns res = i1 * i2.
    fn mul(&self, i1: Variable, i2: Variable) -> Variable {
        Variable::from_target(self.api.mul(i1.value, i2.value))
    }

    /// Mul returns res = i1 * i2 * ...
    fn mul(&self, values: &[Variable]) -> Variable {
        let mut acc = values[0];
        for i in 1..values.len() {
            acc = self.api.mul(acc, values[i].value);
        }
        Variable::from_target(acc)
    }

    /// Div returns res = i1 / i2.
    fn div(&self, i1: Variable, i2: Variable) -> Variable {
        Variable::from_target(self.api.div(i1.value, i2.value))
    }

    /// Div returns res = -i1.
    fn neg(&self, i1: Variable) -> Variable {
        Variable::from_target(self.api.neg(i1.value))
    }

    /// Inverse returns res = 1 / i1.
    fn inverse(&self, i1: Variable) -> Variable {
        Variable::from_target(self.api.inverse(i1.value))
    }

    /// Select if b is true, yields i1 else yields i2.
    fn select(&self, selector: BoolVariable, i1: Variable, i2: Variable) -> Variable {
        Variable::from_target(self.api.select(selector.value.value, i1.value, i2.value))
    }

    /// Returns 1 if i1 is zero, 0 otherwise as a boolean.
    fn is_zero(&self, i1: Variable) -> BoolVariable {
        BoolVariable::from_target(self.api.is_zero(i1.value))
    }

    /// Fails if i1 != i2.
    fn assert_is_equal(&self, i1: Variable, i2: Variable) {
        self.api.connect(i1.value, i2.value);
    }
}
