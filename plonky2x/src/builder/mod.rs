mod boolean;
mod byte;

use plonky2::iop::target::BoolTarget;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::{CircuitConfig, CircuitData};
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

use crate::vars::{BoolVariable, Variable};

const D: usize = 2;
type C = PoseidonGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;

/// This is the API that we recommend developers use for writing circuits. It is a wrapper around
/// the basic plonky2 API.
pub struct BuilderAPI {
    pub api: CircuitBuilder<F, D>,
}

impl BuilderAPI {
    /// Creates a new API for building circuits.
    pub fn new() -> Self {
        let config = CircuitConfig::standard_recursion_config();
        let api = CircuitBuilder::new(config);
        Self { api }
    }

    /// Build the circuit.
    pub fn build(self) -> CircuitData<F, C, D> {
        self.api.build()
    }

    /// Add returns res = i1 + i2.
    pub fn add(&mut self, i1: Variable, i2: Variable) -> Variable {
        self.api.add(i1.0, i2.0).into()
    }

    /// Add returns res = i1 + i2 + ...
    pub fn add_many(&mut self, values: &[Variable]) -> Variable {
        let mut acc = values[0].0;
        for i in 1..values.len() {
            acc = self.api.add(acc, values[i].0);
        }
        acc.into()
    }

    /// Sub returns res = i1 - i2.
    pub fn sub(&mut self, i1: Variable, i2: Variable) -> Variable {
        self.api.sub(i1.0, i2.0).into()
    }

    /// Sub returns res = i1 - i2 - ...
    pub fn sub_many(&mut self, values: &[Variable]) -> Variable {
        let mut acc = values[0].0;
        for i in 1..values.len() {
            acc = self.api.sub(acc, values[i].0);
        }
        acc.into()
    }

    /// Mul returns res = i1 * i2.
    pub fn mul(&mut self, i1: Variable, i2: Variable) -> Variable {
        self.api.mul(i1.0, i2.0).into()
    }

    /// Mul returns res = i1 * i2 * ...
    pub fn mul_many(&mut self, values: &[Variable]) -> Variable {
        let mut acc = values[0].0;
        for i in 1..values.len() {
            acc = self.api.mul(acc, values[i].0);
        }
        acc.into()
    }

    /// Div returns res = i1 / i2.
    pub fn div(&mut self, i1: Variable, i2: Variable) -> Variable {
        self.api.div(i1.0, i2.0).into()
    }

    /// Div returns res = -i1.
    pub fn neg(&mut self, i1: Variable) -> Variable {
        self.api.neg(i1.0).into()
    }

    /// Inverse returns res = 1 / i1.
    pub fn inverse(&mut self, i1: Variable) -> Variable {
        self.api.inverse(i1.0).into()
    }

    /// Select if b is true, yields i1 else yields i2.
    pub fn select(&mut self, selector: BoolVariable, i1: Variable, i2: Variable) -> Variable {
        self.api
            .select(BoolTarget::new_unsafe(selector.0 .0), i1.0, i2.0)
            .into()
    }

    /// Returns 1 if i1 is zero, 0 otherwise as a boolean.
    pub fn is_zero(&mut self, i1: Variable) -> BoolVariable {
        let zero = self.api.zero();
        self.api.is_equal(i1.0, zero).into()
    }

    /// Fails if i1 != i2.
    pub fn assert_is_equal(&mut self, i1: Variable, i2: Variable) {
        self.api.connect(i1.0, i2.0);
    }

    pub fn zero(&mut self) -> Variable {
        Variable::from_target(self.api.zero())
    }

    pub fn one(&mut self) -> Variable {
        Variable::from_target(self.api.one())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use plonky2::iop::witness::PartialWitness;

    use crate::builder::BuilderAPI;

    #[test]
    fn test_simple_circuit() {
        let mut api = BuilderAPI::new();
        let zero = api.zero();
        let one = api.one();
        let sum = api.add(zero, one);
        api.assert_is_equal(sum, one);

        let pw = PartialWitness::new();
        let data = api.build();
        let proof = data.prove(pw).unwrap();
        data.verify(proof).unwrap();
    }
}
