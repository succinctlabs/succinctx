use plonky2::gadgets::polynomial::PolynomialCoeffsExtTarget;
use plonky2::iop::ext_target::ExtensionTarget;

use super::extension::ExtensionVariable;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolynomialCoeffsExtVariable<const D: usize>(pub Vec<ExtensionVariable<D>>);

impl<const D: usize> From<PolynomialCoeffsExtTarget<D>> for PolynomialCoeffsExtVariable<D> {
    fn from(target: PolynomialCoeffsExtTarget<D>) -> Self {
        Self(target.0.into_iter().map(ExtensionVariable::from).collect())
    }
}

impl<const D: usize> From<PolynomialCoeffsExtVariable<D>> for PolynomialCoeffsExtTarget<D> {
    fn from(target: PolynomialCoeffsExtVariable<D>) -> Self {
        Self(target.0.into_iter().map(ExtensionTarget::from).collect())
    }
}
