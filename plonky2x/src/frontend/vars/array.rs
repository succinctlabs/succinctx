use core::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Index, Range};

use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::{CircuitVariable, Variable};
use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::mpt::generators::MuxGenerator;

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

    fn init<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>) -> Self {
        Self {
            elements: (0..N).map(|_| V::init(builder)).collect(),
        }
    }

    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Vec<V::ValueType<L::Field>>,
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

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn mux<V: CircuitVariable, const N: usize>(
        &mut self,
        array: ArrayVariable<V, N>,
        selector: Variable,
    ) -> V {
        let generator = MuxGenerator {
            input: array,
            select: selector,
            output: self.init::<V>(),
            _phantom: PhantomData::<L>,
        };
        self.add_simple_generator(generator.clone());
        generator.output
    }
}

#[cfg(test)]
mod tests {
    use curta::math::prelude::Field;

    use super::*;
    use crate::backend::circuit::DefaultParameters;
    use crate::prelude::*;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_array_variable() {
        let mut builder = CircuitBuilder::<L, D>::new();

        let x = builder.init::<BoolVariable>();
        let y = builder.init::<BoolVariable>();
        let array = ArrayVariable::<_, 2>::new(vec![x, y]);

        let mut pw = PartialWitness::new();

        x.set(&mut pw, true);
        y.set(&mut pw, false);
        array.set(&mut pw, vec![true, false]);

        let circuit = builder.build();
        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn test_mux() {
        type F = GoldilocksField;
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;

        let mut builder = CircuitBuilder::<F, D>::new();
        let b = builder.read::<ArrayVariable<BoolVariable, 3>>();
        let selector = builder.read::<Variable>();
        let result = builder.mux(b, selector);
        builder.write(result);

        let circuit = builder.build::<C>();
        let mut input = circuit.input();
        input.write::<ArrayVariable<BoolVariable, 3>>(vec![true, false, true]);
        input.write::<Variable>(F::from_canonical_u16(1));
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }
}
