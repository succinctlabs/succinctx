use crate::backend::config::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::{BoolVariable, Variable};

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn _false(&mut self) -> BoolVariable {
        let zero = self.zero::<Variable>();
        zero.into()
    }

    pub fn _true(&mut self) -> BoolVariable {
        let one = self.one::<Variable>();
        one.into()
    }
}

#[cfg(test)]
mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::field::types::Field;
    use plonky2::iop::witness::{PartialWitness, Witness, WitnessWrite};

    use super::*;
    use crate::backend::config::DefaultParameters;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_init_bool_and_set() {
        let mut builder = CircuitBuilder::<L, D>::new();
        let b = builder.init::<BoolVariable>();

        let mut pw = PartialWitness::new();
        pw.set_target(b.0 .0, GoldilocksField::ONE);

        let value = pw.try_get_target(b.0 .0).unwrap();
        assert_eq!(GoldilocksField::ONE, value);
    }
}
