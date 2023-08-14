use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use crate::builder::{CircuitBuilder, D};
use crate::vars::BoolVariable;

impl<F: RichField + Extendable<D>> CircuitBuilder<F> {
    pub fn _false(&mut self) -> BoolVariable {
        let zero = self.zero();
        zero.into()
    }

    pub fn _true(&mut self) -> BoolVariable {
        let one = self.one();
        one.into()
    }

    /// Computes the or of two bits or i1 | i2.
    pub fn or(&mut self, i1: BoolVariable, i2: BoolVariable) -> BoolVariable {
        self.add(i1.0, i2.0).into()
    }

    /// Computes the and of two bits or i1 & i2.
    pub fn and(&mut self, i1: BoolVariable, i2: BoolVariable) -> BoolVariable {
        self.mul(i1.0, i2.0).into()
    }

    /// Computes the xor of two bits or i1 ^ i2.
    pub fn xor(&mut self, i1: BoolVariable, i2: BoolVariable) -> BoolVariable {
        let a_plus_b = self.add(i1.0, i2.0);
        let two_a_b = self.mul(i1.0, i2.0);
        self.sub(a_plus_b, two_a_b).into()
    }

    /// Computes the not of a bit or !i1.
    pub fn not(&mut self, i1: BoolVariable) -> BoolVariable {
        let one = self.one();
        self.sub(one, i1.0).into()
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
        let mut builder = CircuitBuilder::<GoldilocksField>::new();
        let b = builder.init::<BoolVariable>();

        let mut pw = PartialWitness::new();
        pw.set_target(b.0 .0, GoldilocksField::ONE);

        let value = pw.try_get_target(b.0 .0).unwrap();
        assert_eq!(GoldilocksField::ONE, value);
    }
}
