use array_macro::array;
use plonky2::hash::hash_types::RichField;

use super::{CircuitVariable, Variable};
use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;

impl<const N: usize, V: CircuitVariable> CircuitVariable for [V; N] {
    type ValueType<F: RichField> = [V::ValueType<F>; N];

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        _builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        array![V::init_unsafe(_builder); N]
    }

    fn variables(&self) -> Vec<Variable> {
        self.iter().flat_map(|v| v.variables()).collect()
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        assert_eq!(variables.len(), N * V::nb_elements());

        core::array::from_fn(|i| {
            let start = i * V::nb_elements();
            let end = start + V::nb_elements();
            V::from_variables_unsafe(&variables[start..end])
        })
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        for variable in self.iter() {
            variable.assert_is_valid(builder);
        }
    }

    fn nb_elements() -> usize {
        V::nb_elements() * N
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        assert!(value.len() == N);
        value.into_iter().flat_map(|v| V::elements(v)).collect()
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        assert_eq!(elements.len(), N * V::nb_elements());
        elements
            .chunks_exact(V::nb_elements())
            .map(V::from_elements)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}

macro_rules! impl_circuit_variable_for_tuple {
    ($($var:ident),+) => {
        impl<$($var: CircuitVariable),+> CircuitVariable for ($($var),+) {
            type ValueType<F: RichField> = ($($var::ValueType<F>),+);

            fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
                builder: &mut CircuitBuilder<L, D>,
            ) -> Self {
                ($($var::init_unsafe(builder)),+)
            }

            fn variables(&self) -> Vec<Variable> {
                let ($($var),+) = self;
                [$(&$var.variables()[..]),+].concat()
            }

            fn from_variables_unsafe(variables: &[Variable]) -> Self {
                let mut start = 0;
                (
                    $({
                        let end = start + $types::nb_elements();
                        let val = $types::from_variables_unsafe(&variables[start..end]);
                        start = end;
                        val
                    },)*
                )
            }

            fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
                &self,
                builder: &mut CircuitBuilder<L, D>,
            ) {
                let ($($types),+) = self;
                $($types.assert_is_valid(builder);)+
            }

            fn nb_elements() -> usize {
                0 $(+ $types::nb_elements())+
            }

            fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
                let ($($types),+) = value;
                [$(&$types::elements($types)[..]),+].concat()
            }

            fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
                let mut start = 0;
                (
                    $({
                        let slice = {
                            let end = start + $types::nb_elements();
                            let slice = &elements[start..end];
                            start = end;
                            slice
                        };
                        $types::from_elements(slice)
                    }),+
                )
            }
        }
    };
}

// Implement CircuitVariable for tuples of lengths 2 to 8
impl_circuit_variable_for_tuple!(V1);
impl_circuit_variable_for_tuple!(V1, V2);
impl_circuit_variable_for_tuple!(V1, V2, V3);
impl_circuit_variable_for_tuple!(V1, V2, V3, V4);
impl_circuit_variable_for_tuple!(V1, V2, V3, V4, V5);
impl_circuit_variable_for_tuple!(V1, V2, V3, V4, V5, V6);
impl_circuit_variable_for_tuple!(V1, V2, V3, V4, V5, V6, V7);
