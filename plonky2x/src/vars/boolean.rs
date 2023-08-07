use plonky2::iop::target::{BoolTarget, Target};

use super::Variable;

#[derive(Debug, Clone, Copy)]
pub struct BoolVariable(pub Variable);

impl BoolVariable {
    pub fn from_variable(value: Variable) -> Self {
        Self(value)
    }

    pub fn from_target(value: Target) -> Self {
        Self(Variable::from_target(value))
    }

    pub fn from_bool_target(value: BoolTarget) -> Self {
        Self(Variable::from_target(value.target))
    }
}

impl From<Variable> for BoolVariable {
    fn from(item: Variable) -> Self {
        Self(item)
    }
}

impl From<Target> for BoolVariable {
    fn from(item: Target) -> Self {
        Self(Variable::from_target(item))
    }
}

impl From<BoolTarget> for BoolVariable {
    fn from(item: BoolTarget) -> Self {
        Self(Variable::from_target(item.target))
    }
}
