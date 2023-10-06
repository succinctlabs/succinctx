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

// macro_rules! impl_circuit_variable_for_tuple {
//     ($($types:ident),+ $(,)? ) => {
//         impl<$($types: CircuitVariable),*> CircuitVariable for ($($types,)*) {
//             type ValueType<F: RichField> = ($($types::ValueType<F>,)*);

//             fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
//                 builder: &mut CircuitBuilder<L, D>,
//             ) -> Self {
//                 ($($types::init_unsafe(builder),)*)
//             }

//             fn variables(&self) -> Vec<Variable> {
//                 let mut vars = Vec::new();
//                 $(vars.extend(self.$types.variables());)*
//                 vars
//             }

//             fn from_variables_unsafe(variables: &[Variable]) -> Self {
//                 let mut start = 0;
//                 (
//                     $({
//                         let end = start + $types::nb_elements();
//                         let val = $types::from_variables_unsafe(&variables[start..end]);
//                         start = end;
//                         val
//                     },)*
//                 )
//             }

//             fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
//                 &self,
//                 builder: &mut CircuitBuilder<L, D>,
//             ) {
//                 $(self.$types.assert_is_valid(builder);)*
//             }

//             fn nb_elements() -> usize {
//                 0 $(+ $types::nb_elements())*
//             }

//             fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
//                 let mut vec = Vec::new();
//                 $(
//                     vec.extend($types::elements(value.$types).into_iter());
//                 )*
//                 vec
//             }

//             fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
//                 let mut start = 0;
//                 (
//                     $({
//                         let end = start + $types::nb_elements();
//                         let val = $types::from_elements(&elements[start..end]);
//                         start = end;
//                         val
//                     },)*
//                 )
//             }
//         }
//     };
// }

// impl_circuit_variable_for_tuple!(T1, T2);

impl<V1: CircuitVariable, V2: CircuitVariable> CircuitVariable for (V1, V2) {
    type ValueType<F: RichField> = (V1::ValueType<F>, V2::ValueType<F>);

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        (V1::init_unsafe(builder), V2::init_unsafe(builder))
    }

    fn variables(&self) -> Vec<Variable> {
        [&self.0.variables()[..], &self.1.variables()[..]].concat()
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        assert_eq!(variables.len(), V1::nb_elements() + V2::nb_elements());

        let v1 = V1::from_variables_unsafe(&variables[..V1::nb_elements()]);
        let v2 = V2::from_variables_unsafe(&variables[V1::nb_elements()..]);

        (v1, v2)
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        self.0.assert_is_valid(builder);
        self.1.assert_is_valid(builder);
    }

    fn nb_elements() -> usize {
        V1::nb_elements() + V2::nb_elements()
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        V1::elements(value.0)
            .into_iter()
            .chain(V2::elements(value.1))
            .collect()
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        assert_eq!(elements.len(), V1::nb_elements() + V2::nb_elements());
        (
            V1::from_elements(&elements[..V1::nb_elements()]),
            V2::from_elements(&elements[V1::nb_elements()..]),
        )
    }
}

impl<V1: CircuitVariable, V2: CircuitVariable, V3: CircuitVariable> CircuitVariable
    for (V1, V2, V3)
{
    type ValueType<F: RichField> = (V1::ValueType<F>, V2::ValueType<F>, V3::ValueType<F>);

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        (
            V1::init_unsafe(builder),
            V2::init_unsafe(builder),
            V3::init_unsafe(builder),
        )
    }

    fn variables(&self) -> Vec<Variable> {
        [
            &self.0.variables()[..],
            &self.1.variables()[..],
            &self.2.variables()[..],
        ]
        .concat()
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        assert_eq!(
            variables.len(),
            V1::nb_elements() + V2::nb_elements() + V3::nb_elements()
        );

        let v1 = V1::from_variables_unsafe(&variables[..V1::nb_elements()]);
        let v2 = V2::from_variables_unsafe(
            &variables[V1::nb_elements()..V1::nb_elements() + V2::nb_elements()],
        );
        let v3 = V3::from_variables_unsafe(&variables[V1::nb_elements() + V2::nb_elements()..]);

        (v1, v2, v3)
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        self.0.assert_is_valid(builder);
        self.1.assert_is_valid(builder);
        self.2.assert_is_valid(builder);
    }

    fn nb_elements() -> usize {
        V1::nb_elements() + V2::nb_elements() + V3::nb_elements()
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        [
            &V1::elements(value.0)[..],
            &V2::elements(value.1)[..],
            &V3::elements(value.2)[..],
        ]
        .concat()
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        assert_eq!(
            elements.len(),
            V1::nb_elements() + V2::nb_elements() + V3::nb_elements()
        );
        (
            V1::from_elements(&elements[..V1::nb_elements()]),
            V2::from_elements(&elements[V1::nb_elements()..]),
            V3::from_elements(&elements[V1::nb_elements() + V2::nb_elements()..]),
        )
    }
}

impl<V1: CircuitVariable, V2: CircuitVariable, V3: CircuitVariable, V4: CircuitVariable>
    CircuitVariable for (V1, V2, V3, V4)
{
    type ValueType<F: RichField> = (
        V1::ValueType<F>,
        V2::ValueType<F>,
        V3::ValueType<F>,
        V4::ValueType<F>,
    );

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        (
            V1::init_unsafe(builder),
            V2::init_unsafe(builder),
            V3::init_unsafe(builder),
            V4::init_unsafe(builder),
        )
    }

    fn variables(&self) -> Vec<Variable> {
        [
            &self.0.variables()[..],
            &self.1.variables()[..],
            &self.2.variables()[..],
            &self.3.variables()[..],
        ]
        .concat()
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        assert_eq!(
            variables.len(),
            V1::nb_elements() + V2::nb_elements() + V3::nb_elements() + V4::nb_elements()
        );

        let v1 = V1::from_variables_unsafe(&variables[..V1::nb_elements()]);
        let v2 = V2::from_variables_unsafe(
            &variables[V1::nb_elements()..V1::nb_elements() + V2::nb_elements()],
        );
        let v3 = V3::from_variables_unsafe(
            &variables[V1::nb_elements() + V2::nb_elements()
                ..V1::nb_elements() + V2::nb_elements() + V3::nb_elements()],
        );
        let v4 = V4::from_variables_unsafe(
            &variables[V1::nb_elements() + V2::nb_elements() + V3::nb_elements()..],
        );

        (v1, v2, v3, v4)
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        self.0.assert_is_valid(builder);
        self.1.assert_is_valid(builder);
        self.2.assert_is_valid(builder);
        self.3.assert_is_valid(builder);
    }

    fn nb_elements() -> usize {
        V1::nb_elements() + V2::nb_elements() + V3::nb_elements() + V4::nb_elements()
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        [
            &V1::elements(value.0)[..],
            &V2::elements(value.1)[..],
            &V3::elements(value.2)[..],
            &V4::elements(value.3)[..],
        ]
        .concat()
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        assert_eq!(
            elements.len(),
            V1::nb_elements() + V2::nb_elements() + V3::nb_elements() + V4::nb_elements()
        );
        (
            V1::from_elements(&elements[..V1::nb_elements()]),
            V2::from_elements(&elements[V1::nb_elements()..]),
            V3::from_elements(&elements[V1::nb_elements() + V2::nb_elements()..]),
            V4::from_elements(
                &elements[V1::nb_elements() + V2::nb_elements() + V3::nb_elements()..],
            ),
        )
    }
}

impl<
        V1: CircuitVariable,
        V2: CircuitVariable,
        V3: CircuitVariable,
        V4: CircuitVariable,
        V5: CircuitVariable,
    > CircuitVariable for (V1, V2, V3, V4, V5)
{
    type ValueType<F: RichField> = (
        V1::ValueType<F>,
        V2::ValueType<F>,
        V3::ValueType<F>,
        V4::ValueType<F>,
        V5::ValueType<F>,
    );

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        (
            V1::init_unsafe(builder),
            V2::init_unsafe(builder),
            V3::init_unsafe(builder),
            V4::init_unsafe(builder),
            V5::init_unsafe(builder),
        )
    }

    fn variables(&self) -> Vec<Variable> {
        [
            &self.0.variables()[..],
            &self.1.variables()[..],
            &self.2.variables()[..],
            &self.3.variables()[..],
            &self.4.variables()[..],
        ]
        .concat()
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        assert_eq!(
            variables.len(),
            V1::nb_elements()
                + V2::nb_elements()
                + V3::nb_elements()
                + V4::nb_elements()
                + V5::nb_elements()
        );

        let v1 = V1::from_variables_unsafe(&variables[..V1::nb_elements()]);
        let v2 = V2::from_variables_unsafe(
            &variables[V1::nb_elements()..V1::nb_elements() + V2::nb_elements()],
        );
        let v3 = V3::from_variables_unsafe(
            &variables[V1::nb_elements() + V2::nb_elements()
                ..V1::nb_elements() + V2::nb_elements() + V3::nb_elements()],
        );
        let v4 = V4::from_variables_unsafe(
            &variables[V1::nb_elements() + V2::nb_elements() + V3::nb_elements()..],
        );
        let v5 = V5::from_variables_unsafe(
            &variables
                [V1::nb_elements() + V2::nb_elements() + V3::nb_elements() + V4::nb_elements()..],
        );

        (v1, v2, v3, v4, v5)
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        self.0.assert_is_valid(builder);
        self.1.assert_is_valid(builder);
        self.2.assert_is_valid(builder);
        self.3.assert_is_valid(builder);
        self.4.assert_is_valid(builder);
    }

    fn nb_elements() -> usize {
        V1::nb_elements()
            + V2::nb_elements()
            + V3::nb_elements()
            + V4::nb_elements()
            + V5::nb_elements()
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        [
            &V1::elements(value.0)[..],
            &V2::elements(value.1)[..],
            &V3::elements(value.2)[..],
            &V4::elements(value.3)[..],
            &V5::elements(value.4)[..],
        ]
        .concat()
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        assert_eq!(
            elements.len(),
            V1::nb_elements()
                + V2::nb_elements()
                + V3::nb_elements()
                + V4::nb_elements()
                + V5::nb_elements()
        );
        (
            V1::from_elements(&elements[..V1::nb_elements()]),
            V2::from_elements(&elements[V1::nb_elements()..]),
            V3::from_elements(&elements[V1::nb_elements() + V2::nb_elements()..]),
            V4::from_elements(
                &elements[V1::nb_elements() + V2::nb_elements() + V3::nb_elements()..],
            ),
            V5::from_elements(
                &elements[V1::nb_elements()
                    + V2::nb_elements()
                    + V3::nb_elements()
                    + V4::nb_elements()..],
            ),
        )
    }
}

impl<
        V1: CircuitVariable,
        V2: CircuitVariable,
        V3: CircuitVariable,
        V4: CircuitVariable,
        V5: CircuitVariable,
        V6: CircuitVariable,
    > CircuitVariable for (V1, V2, V3, V4, V5, V6)
{
    type ValueType<F: RichField> = (
        V1::ValueType<F>,
        V2::ValueType<F>,
        V3::ValueType<F>,
        V4::ValueType<F>,
        V5::ValueType<F>,
        V6::ValueType<F>,
    );

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        (
            V1::init_unsafe(builder),
            V2::init_unsafe(builder),
            V3::init_unsafe(builder),
            V4::init_unsafe(builder),
            V5::init_unsafe(builder),
            V6::init_unsafe(builder),
        )
    }

    fn variables(&self) -> Vec<Variable> {
        [
            &self.0.variables()[..],
            &self.1.variables()[..],
            &self.2.variables()[..],
            &self.3.variables()[..],
            &self.4.variables()[..],
            &self.5.variables()[..],
        ]
        .concat()
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        assert_eq!(
            variables.len(),
            V1::nb_elements()
                + V2::nb_elements()
                + V3::nb_elements()
                + V4::nb_elements()
                + V5::nb_elements()
                + V6::nb_elements()
        );

        let v1 = V1::from_variables_unsafe(&variables[..V1::nb_elements()]);
        let v2 = V2::from_variables_unsafe(
            &variables[V1::nb_elements()..V1::nb_elements() + V2::nb_elements()],
        );
        let v3 = V3::from_variables_unsafe(
            &variables[V1::nb_elements() + V2::nb_elements()
                ..V1::nb_elements() + V2::nb_elements() + V3::nb_elements()],
        );
        let v4 = V4::from_variables_unsafe(
            &variables[V1::nb_elements() + V2::nb_elements() + V3::nb_elements()
                ..V1::nb_elements() + V2::nb_elements() + V3::nb_elements() + V4::nb_elements()],
        );
        let v5 = V5::from_variables_unsafe(
            &variables[V1::nb_elements() + V2::nb_elements() + V3::nb_elements() + V4::nb_elements()
                ..V1::nb_elements()
                    + V2::nb_elements()
                    + V3::nb_elements()
                    + V4::nb_elements()
                    + V5::nb_elements()],
        );
        let v6 = V6::from_variables_unsafe(
            &variables[V1::nb_elements()
                + V2::nb_elements()
                + V3::nb_elements()
                + V4::nb_elements()
                + V5::nb_elements()
                ..V1::nb_elements()
                    + V2::nb_elements()
                    + V3::nb_elements()
                    + V4::nb_elements()
                    + V5::nb_elements()
                    + V6::nb_elements()],
        );

        (v1, v2, v3, v4, v5, v6)
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        self.0.assert_is_valid(builder);
        self.1.assert_is_valid(builder);
        self.2.assert_is_valid(builder);
        self.3.assert_is_valid(builder);
        self.4.assert_is_valid(builder);
        self.5.assert_is_valid(builder);
    }

    fn nb_elements() -> usize {
        V1::nb_elements()
            + V2::nb_elements()
            + V3::nb_elements()
            + V4::nb_elements()
            + V5::nb_elements()
            + V6::nb_elements()
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        [
            &V1::elements(value.0)[..],
            &V2::elements(value.1)[..],
            &V3::elements(value.2)[..],
            &V4::elements(value.3)[..],
            &V5::elements(value.4)[..],
            &V6::elements(value.5)[..],
        ]
        .concat()
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        assert_eq!(
            elements.len(),
            V1::nb_elements() + V2::nb_elements() + V3::nb_elements() + V4::nb_elements()
        );
        (
            V1::from_elements(&elements[..V1::nb_elements()]),
            V2::from_elements(&elements[V1::nb_elements()..]),
            V3::from_elements(&elements[V1::nb_elements() + V2::nb_elements()..]),
            V4::from_elements(
                &elements[V1::nb_elements() + V2::nb_elements() + V3::nb_elements()..],
            ),
            V5::from_elements(
                &elements[V1::nb_elements()
                    + V2::nb_elements()
                    + V3::nb_elements()
                    + V4::nb_elements()..],
            ),
            V6::from_elements(
                &elements[V1::nb_elements()
                    + V2::nb_elements()
                    + V3::nb_elements()
                    + V4::nb_elements()
                    + V5::nb_elements()..],
            ),
        )
    }
}

impl<
        V1: CircuitVariable,
        V2: CircuitVariable,
        V3: CircuitVariable,
        V4: CircuitVariable,
        V5: CircuitVariable,
        V6: CircuitVariable,
        V7: CircuitVariable,
    > CircuitVariable for (V1, V2, V3, V4, V5, V6, V7)
{
    type ValueType<F: RichField> = (
        V1::ValueType<F>,
        V2::ValueType<F>,
        V3::ValueType<F>,
        V4::ValueType<F>,
        V5::ValueType<F>,
        V6::ValueType<F>,
        V7::ValueType<F>,
    );

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        (
            V1::init_unsafe(builder),
            V2::init_unsafe(builder),
            V3::init_unsafe(builder),
            V4::init_unsafe(builder),
            V5::init_unsafe(builder),
            V6::init_unsafe(builder),
            V7::init_unsafe(builder),
        )
    }

    fn variables(&self) -> Vec<Variable> {
        [
            &self.0.variables()[..],
            &self.1.variables()[..],
            &self.2.variables()[..],
            &self.3.variables()[..],
            &self.4.variables()[..],
            &self.5.variables()[..],
            &self.6.variables()[..],
        ]
        .concat()
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        assert_eq!(
            variables.len(),
            V1::nb_elements()
                + V2::nb_elements()
                + V3::nb_elements()
                + V4::nb_elements()
                + V5::nb_elements()
                + V6::nb_elements()
                + V7::nb_elements()
        );

        let v1 = V1::from_variables_unsafe(&variables[..V1::nb_elements()]);
        let v2 = V2::from_variables_unsafe(
            &variables[V1::nb_elements()..V1::nb_elements() + V2::nb_elements()],
        );
        let v3 = V3::from_variables_unsafe(
            &variables[V1::nb_elements() + V2::nb_elements()
                ..V1::nb_elements() + V2::nb_elements() + V3::nb_elements()],
        );
        let v4 = V4::from_variables_unsafe(
            &variables[V1::nb_elements() + V2::nb_elements() + V3::nb_elements()
                ..V1::nb_elements() + V2::nb_elements() + V3::nb_elements() + V4::nb_elements()],
        );
        let v5 = V5::from_variables_unsafe(
            &variables[V1::nb_elements() + V2::nb_elements() + V3::nb_elements() + V4::nb_elements()
                ..V1::nb_elements()
                    + V2::nb_elements()
                    + V3::nb_elements()
                    + V4::nb_elements()
                    + V5::nb_elements()],
        );
        let v6 = V6::from_variables_unsafe(
            &variables[V1::nb_elements()
                + V2::nb_elements()
                + V3::nb_elements()
                + V4::nb_elements()
                + V5::nb_elements()
                ..V1::nb_elements()
                    + V2::nb_elements()
                    + V3::nb_elements()
                    + V4::nb_elements()
                    + V5::nb_elements()
                    + V6::nb_elements()],
        );
        let v7 = V7::from_variables_unsafe(
            &variables[V1::nb_elements()
                + V2::nb_elements()
                + V3::nb_elements()
                + V4::nb_elements()
                + V5::nb_elements()
                + V6::nb_elements()
                ..V1::nb_elements()
                    + V2::nb_elements()
                    + V3::nb_elements()
                    + V4::nb_elements()
                    + V5::nb_elements()
                    + V6::nb_elements()
                    + V7::nb_elements()],
        );

        (v1, v2, v3, v4, v5, v6, v7)
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        self.0.assert_is_valid(builder);
        self.1.assert_is_valid(builder);
        self.2.assert_is_valid(builder);
        self.3.assert_is_valid(builder);
        self.4.assert_is_valid(builder);
        self.5.assert_is_valid(builder);
        self.6.assert_is_valid(builder);
    }

    fn nb_elements() -> usize {
        V1::nb_elements()
            + V2::nb_elements()
            + V3::nb_elements()
            + V4::nb_elements()
            + V5::nb_elements()
            + V6::nb_elements()
            + V7::nb_elements()
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        [
            &V1::elements(value.0)[..],
            &V2::elements(value.1)[..],
            &V3::elements(value.2)[..],
            &V4::elements(value.3)[..],
            &V5::elements(value.4)[..],
            &V6::elements(value.5)[..],
            &V7::elements(value.6)[..],
        ]
        .concat()
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        assert_eq!(
            elements.len(),
            V1::nb_elements() + V2::nb_elements() + V3::nb_elements() + V4::nb_elements()
        );
        (
            V1::from_elements(&elements[..V1::nb_elements()]),
            V2::from_elements(&elements[V1::nb_elements()..]),
            V3::from_elements(&elements[V1::nb_elements() + V2::nb_elements()..]),
            V4::from_elements(
                &elements[V1::nb_elements() + V2::nb_elements() + V3::nb_elements()..],
            ),
            V5::from_elements(
                &elements[V1::nb_elements()
                    + V2::nb_elements()
                    + V3::nb_elements()
                    + V4::nb_elements()..],
            ),
            V6::from_elements(
                &elements[V1::nb_elements()
                    + V2::nb_elements()
                    + V3::nb_elements()
                    + V4::nb_elements()
                    + V5::nb_elements()..],
            ),
            V7::from_elements(
                &elements[V1::nb_elements()
                    + V2::nb_elements()
                    + V3::nb_elements()
                    + V4::nb_elements()
                    + V5::nb_elements()
                    + V6::nb_elements()..],
            ),
        )
    }
}

impl<
        V1: CircuitVariable,
        V2: CircuitVariable,
        V3: CircuitVariable,
        V4: CircuitVariable,
        V5: CircuitVariable,
        V6: CircuitVariable,
        V7: CircuitVariable,
        V8: CircuitVariable,
    > CircuitVariable for (V1, V2, V3, V4, V5, V6, V7, V8)
{
    type ValueType<F: RichField> = (
        V1::ValueType<F>,
        V2::ValueType<F>,
        V3::ValueType<F>,
        V4::ValueType<F>,
        V5::ValueType<F>,
        V6::ValueType<F>,
        V7::ValueType<F>,
        V8::ValueType<F>,
    );

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        (
            V1::init_unsafe(builder),
            V2::init_unsafe(builder),
            V3::init_unsafe(builder),
            V4::init_unsafe(builder),
            V5::init_unsafe(builder),
            V6::init_unsafe(builder),
            V7::init_unsafe(builder),
            V8::init_unsafe(builder),
        )
    }

    fn variables(&self) -> Vec<Variable> {
        [
            &self.0.variables()[..],
            &self.1.variables()[..],
            &self.2.variables()[..],
            &self.3.variables()[..],
            &self.4.variables()[..],
            &self.5.variables()[..],
            &self.6.variables()[..],
            &self.7.variables()[..],
        ]
        .concat()
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        assert_eq!(
            variables.len(),
            V1::nb_elements()
                + V2::nb_elements()
                + V3::nb_elements()
                + V4::nb_elements()
                + V5::nb_elements()
                + V6::nb_elements()
                + V7::nb_elements()
                + V8::nb_elements()
        );

        let v1 = V1::from_variables_unsafe(&variables[..V1::nb_elements()]);
        let v2 = V2::from_variables_unsafe(
            &variables[V1::nb_elements()..V1::nb_elements() + V2::nb_elements()],
        );
        let v3 = V3::from_variables_unsafe(
            &variables[V1::nb_elements() + V2::nb_elements()
                ..V1::nb_elements() + V2::nb_elements() + V3::nb_elements()],
        );
        let v4 = V4::from_variables_unsafe(
            &variables[V1::nb_elements() + V2::nb_elements() + V3::nb_elements()
                ..V1::nb_elements() + V2::nb_elements() + V3::nb_elements() + V4::nb_elements()],
        );
        let v5 = V5::from_variables_unsafe(
            &variables[V1::nb_elements() + V2::nb_elements() + V3::nb_elements() + V4::nb_elements()
                ..V1::nb_elements()
                    + V2::nb_elements()
                    + V3::nb_elements()
                    + V4::nb_elements()
                    + V5::nb_elements()],
        );
        let v6 = V6::from_variables_unsafe(
            &variables[V1::nb_elements()
                + V2::nb_elements()
                + V3::nb_elements()
                + V4::nb_elements()
                + V5::nb_elements()
                ..V1::nb_elements()
                    + V2::nb_elements()
                    + V3::nb_elements()
                    + V4::nb_elements()
                    + V5::nb_elements()
                    + V6::nb_elements()],
        );
        let v7 = V7::from_variables_unsafe(
            &variables[V1::nb_elements()
                + V2::nb_elements()
                + V3::nb_elements()
                + V4::nb_elements()
                + V5::nb_elements()
                + V6::nb_elements()
                ..V1::nb_elements()
                    + V2::nb_elements()
                    + V3::nb_elements()
                    + V4::nb_elements()
                    + V5::nb_elements()
                    + V6::nb_elements()
                    + V7::nb_elements()],
        );
        let v8 = V8::from_variables_unsafe(
            &variables[V1::nb_elements()
                + V2::nb_elements()
                + V3::nb_elements()
                + V4::nb_elements()
                + V5::nb_elements()
                + V6::nb_elements()
                + V7::nb_elements()..],
        );

        (v1, v2, v3, v4, v5, v6, v7, v8)
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        self.0.assert_is_valid(builder);
        self.1.assert_is_valid(builder);
        self.2.assert_is_valid(builder);
        self.3.assert_is_valid(builder);
        self.4.assert_is_valid(builder);
        self.5.assert_is_valid(builder);
        self.6.assert_is_valid(builder);
        self.7.assert_is_valid(builder);
    }

    fn nb_elements() -> usize {
        V1::nb_elements()
            + V2::nb_elements()
            + V3::nb_elements()
            + V4::nb_elements()
            + V5::nb_elements()
            + V6::nb_elements()
            + V7::nb_elements()
            + V8::nb_elements()
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        [
            &V1::elements(value.0)[..],
            &V2::elements(value.1)[..],
            &V3::elements(value.2)[..],
            &V4::elements(value.3)[..],
            &V5::elements(value.4)[..],
            &V6::elements(value.5)[..],
            &V7::elements(value.6)[..],
            &V8::elements(value.7)[..],
        ]
        .concat()
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        assert_eq!(
            elements.len(),
            V1::nb_elements() + V2::nb_elements() + V3::nb_elements() + V4::nb_elements()
        );
        (
            V1::from_elements(&elements[..V1::nb_elements()]),
            V2::from_elements(&elements[V1::nb_elements()..]),
            V3::from_elements(&elements[V1::nb_elements() + V2::nb_elements()..]),
            V4::from_elements(
                &elements[V1::nb_elements() + V2::nb_elements() + V3::nb_elements()..],
            ),
            V5::from_elements(
                &elements[V1::nb_elements()
                    + V2::nb_elements()
                    + V3::nb_elements()
                    + V4::nb_elements()..],
            ),
            V6::from_elements(
                &elements[V1::nb_elements()
                    + V2::nb_elements()
                    + V3::nb_elements()
                    + V4::nb_elements()
                    + V5::nb_elements()..],
            ),
            V7::from_elements(
                &elements[V1::nb_elements()
                    + V2::nb_elements()
                    + V3::nb_elements()
                    + V4::nb_elements()
                    + V5::nb_elements()
                    + V6::nb_elements()..],
            ),
            V8::from_elements(
                &elements[V1::nb_elements()
                    + V2::nb_elements()
                    + V3::nb_elements()
                    + V4::nb_elements()
                    + V5::nb_elements()
                    + V6::nb_elements()
                    + V7::nb_elements()..],
            ),
        )
    }
}
