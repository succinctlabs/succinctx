use plonky2::plonk::proof::ProofWithPublicInputsTarget;

use crate::prelude::CircuitVariable;

pub trait ProofWithPublicInputsTargetUtils {
    fn read_start_from_pis<V: CircuitVariable>(&self) -> V;
    fn read_end_from_pis<V: CircuitVariable>(&self) -> V;
}

impl<const D: usize> ProofWithPublicInputsTargetUtils for ProofWithPublicInputsTarget<D> {
    fn read_start_from_pis<V: CircuitVariable>(&self) -> V {
        V::from_targets(&self.public_inputs[..V::nb_elements()])
    }

    fn read_end_from_pis<V: CircuitVariable>(&self) -> V {
        let public_inputs_len = self.public_inputs.len();
        V::from_targets(&self.public_inputs[public_inputs_len - V::nb_elements()..])
    }
}
