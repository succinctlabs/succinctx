use plonky2::iop::ext_target::ExtensionTarget;

use crate::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, CircuitVariable)]
pub struct ExtensionVariable<const DEG: usize> {
    pub coeff: [Variable; DEG],
}

impl<const D: usize> From<ExtensionTarget<D>> for ExtensionVariable<D> {
    fn from(target: ExtensionTarget<D>) -> Self {
        Self {
            coeff: target.0.map(Variable),
        }
    }
}

impl<const D: usize> From<ExtensionVariable<D>> for ExtensionTarget<D> {
    fn from(target: ExtensionVariable<D>) -> Self {
        Self(target.coeff.map(|v| v.0))
    }
}
