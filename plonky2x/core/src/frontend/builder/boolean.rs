use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::{BoolVariable, Variable};
use crate::prelude::CircuitVariable;

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn _false(&mut self) -> BoolVariable {
        let zero = self.zero::<Variable>();
        BoolVariable::from_variables_unsafe(&[zero])
    }

    pub fn _true(&mut self) -> BoolVariable {
        let one = self.one::<Variable>();
        BoolVariable::from_variables_unsafe(&[one])
    }
}

#[cfg(test)]
mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::field::types::Field;
    use plonky2::iop::witness::{PartialWitness, Witness, WitnessWrite};

    use super::*;
    use crate::backend::circuit::DefaultParameters;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_init_bool_and_set() {
        let mut builder = CircuitBuilder::<L, D>::new();
        let b = builder.init::<BoolVariable>();

        let mut pw = PartialWitness::new();
        pw.set_target(b.variable.0, GoldilocksField::ONE);

        let value = pw.try_get_target(b.variable.0).unwrap();
        assert_eq!(GoldilocksField::ONE, value);
    }
}
