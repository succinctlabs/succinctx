use core::fmt::Debug;
use std::ops::{Index, Range};

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::{CircuitVariable, Variable};
use crate::frontend::builder::CircuitBuilder;

/// A variable in the circuit representing a fixed length array of variables.
/// We use this to avoid stack overflow arrays associated with fixed-length arrays.
#[derive(Debug, Clone)]
pub struct ArrayVariable<V: CircuitVariable, const N: usize> {
    elements: Vec<V>,
}

impl<V: CircuitVariable, const N: usize> ArrayVariable<V, N> {
    pub fn new(elements: Vec<V>) -> Self {
        assert_eq!(elements.len(), N);
        Self { elements }
    }

    pub fn as_slice(&self) -> &[V] {
        &self.elements
    }

    pub fn as_vec(&self) -> Vec<V> {
        self.elements.clone()
    }
}

impl<V: CircuitVariable, const N: usize> Index<usize> for ArrayVariable<V, N> {
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<V: CircuitVariable, const N: usize> Index<Range<usize>> for ArrayVariable<V, N> {
    type Output = [V];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.elements[range]
    }
}

impl<V: CircuitVariable, const N: usize> From<Vec<V>> for ArrayVariable<V, N> {
    fn from(elements: Vec<V>) -> Self {
        ArrayVariable::new(elements)
    }
}

impl<V: CircuitVariable, const N: usize> CircuitVariable for ArrayVariable<V, N> {
    type ValueType<F: RichField> = Vec<V::ValueType<F>>;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self {
            elements: (0..N).map(|_| V::init(builder)).collect(),
        }
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Vec<V::ValueType<F>>,
    ) -> Self {
        assert_eq!(value.len(), N);
        Self {
            elements: value.into_iter().map(|x| V::constant(builder, x)).collect(),
        }
    }

    fn variables(&self) -> Vec<Variable> {
        self.elements.iter().flat_map(|x| x.variables()).collect()
    }

    fn from_variables(variables: &[Variable]) -> Self {
        assert_eq!(variables.len(), N * V::nb_elements());
        let mut res = Vec::new();
        for i in 0..N {
            let start = i * V::nb_elements();
            let end = (i + 1) * V::nb_elements();
            let slice = &variables[start..end];
            res.push(V::from_variables(slice));
        }

        Self { elements: res }
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        self.elements.iter().map(|x| x.get(witness)).collect()
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        assert_eq!(value.len(), N);
        for (element, value) in self.elements.iter().zip(value) {
            element.set(witness, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_array_variable() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let x = builder.init::<BoolVariable>();
        let y = builder.init::<BoolVariable>();
        let array = ArrayVariable::<_, 2>::new(vec![x, y]);

        let mut pw = PartialWitness::new();

        x.set(&mut pw, true);
        y.set(&mut pw, false);
        array.set(&mut pw, vec![true, false]);

        let circuit = builder.build::<C>();
        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }
}
