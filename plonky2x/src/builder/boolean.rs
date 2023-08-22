use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use crate::builder::CircuitBuilder;
use crate::vars::{BoolVariable, Variable};

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
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

    #[test]
    fn test_init_bool_and_set() {
        type F = GoldilocksField;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();
        let b = builder.init::<BoolVariable>();

        let mut pw = PartialWitness::new();
        pw.set_target(b.0 .0, GoldilocksField::ONE);

        let value = pw.try_get_target(b.0 .0).unwrap();
        assert_eq!(GoldilocksField::ONE, value);
    }
}
