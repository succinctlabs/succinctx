use plonky2::field::extension::FieldExtension;
use plonky2::iop::ext_target::ExtensionTarget;

use crate::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, CircuitVariable)]
pub struct ExtensionVariable<const DEG: usize> {
    pub coeff: [Variable; DEG],
}

impl<L: PlonkParameters<D>, const D: usize> ValueStream<L, D> {
    pub fn read_extension(&mut self) -> <L::Field as Extendable<D>>::Extension {
        let coeff: [_; D] = self.read_exact(D).to_vec().try_into().unwrap();
        <L::Field as Extendable<D>>::Extension::from_basefield_array(coeff)
    }

    pub fn read_extension_vec(
        &mut self,
        len: usize,
    ) -> Vec<<L::Field as Extendable<D>>::Extension> {
        (0..len).map(|_| self.read_extension()).collect()
    }

    pub fn write_extension(&mut self, value: <L::Field as Extendable<D>>::Extension) {
        let coeff = value.to_basefield_array().to_vec();
        self.write_slice(&coeff);
    }

    pub fn write_extension_vec(&mut self, values: Vec<<L::Field as Extendable<D>>::Extension>) {
        values.into_iter().for_each(|v| self.write_extension(v))
    }
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
