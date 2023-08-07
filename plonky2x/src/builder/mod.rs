mod bool;

use plonky2::iop::target::BoolTarget;
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

impl API {
    /// Creates a new builder API.
    fn new() -> Self {
        let config = CircuitConfig::standard_recursion_config();
        let api = CircuitBuilder::new(config);
        Self { api }
    }

    /// Add returns res = i1 + i2.
    fn add(&mut self, i1: Variable, i2: Variable) -> Variable {
        Variable::from_target(self.api.add(i1.value, i2.value))
    }

    /// Add returns res = i1 + i2 + ...
    fn add_many(&mut self, values: &[Variable]) -> Variable {
        let mut acc = values[0].value;
        for i in 1..values.len() {
            acc = self.api.add(acc, values[i].value);
        }
        Variable::from_target(acc)
    }

    /// Sub returns res = i1 - i2.
    fn sub(&mut self, i1: Variable, i2: Variable) -> Variable {
        Variable::from_target(self.api.sub(i1.value, i2.value))
    }

    /// Sub returns res = i1 - i2 - ...
    fn sub_many(&mut self, values: &[Variable]) -> Variable {
        let mut acc = values[0].value;
        for i in 1..values.len() {
            acc = self.api.sub(acc, values[i].value);
        }
        Variable::from_target(acc)
    }

    /// Mul returns res = i1 * i2.
    fn mul(&mut self, i1: Variable, i2: Variable) -> Variable {
        Variable::from_target(self.api.mul(i1.value, i2.value))
    }

    /// Mul returns res = i1 * i2 * ...
    fn mul_many(&mut self, values: &[Variable]) -> Variable {
        let mut acc = values[0].value;
        for i in 1..values.len() {
            acc = self.api.mul(acc, values[i].value);
        }
        Variable::from_target(acc)
    }

    /// Div returns res = i1 / i2.
    fn div(&mut self, i1: Variable, i2: Variable) -> Variable {
        Variable::from_target(self.api.div(i1.value, i2.value))
    }

    /// Div returns res = -i1.
    fn neg(&mut self, i1: Variable) -> Variable {
        Variable::from_target(self.api.neg(i1.value))
    }

    /// Inverse returns res = 1 / i1.
    fn inverse(&mut self, i1: Variable) -> Variable {
        Variable::from_target(self.api.inverse(i1.value))
    }

    /// Select if b is true, yields i1 else yields i2.
    fn select(&mut self, selector: BoolVariable, i1: Variable, i2: Variable) -> Variable {
        Variable::from_target(self.api.select(
            BoolTarget::new_unsafe(selector.value),
            i1.value,
            i2.value,
        ))
    }

    /// Returns 1 if i1 is zero, 0 otherwise as a boolean.
    fn is_zero(&mut self, i1: Variable) -> BoolVariable {
        let zero = self.api.zero();
        BoolVariable::from_target(self.api.is_equal(i1.value, zero).target)
    }

    /// Fails if i1 != i2.
    fn assert_is_equal(&mut self, i1: Variable, i2: Variable) {
        self.api.connect(i1.value, i2.value);
    }
}

impl Default for API {
    fn default() -> Self {
        Self::new()
    }
}
