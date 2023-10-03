use array_macro::array;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

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

    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Self::ValueType<L::Field>,
    ) -> Self {
        value.map(|v| builder.constant(v))
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

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        self.clone().map(|v| v.get(witness))
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        assert_eq!(self.len(), value.len());
        for (v, value) in self.iter().zip(value) {
            v.set(witness, value);
        }
    }
}

impl<V1: CircuitVariable, V2: CircuitVariable> CircuitVariable for (V1, V2) {
    type ValueType<F: RichField> = (V1::ValueType<F>, V2::ValueType<F>);

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        (V1::init_unsafe(builder), V2::init_unsafe(builder))
    }

    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Self::ValueType<L::Field>,
    ) -> Self {
        (
            V1::constant(builder, value.0),
            V2::constant(builder, value.1),
        )
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

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        (self.0.get(witness), self.1.get(witness))
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.0.set(witness, value.0);
        self.1.set(witness, value.1);
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

    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Self::ValueType<L::Field>,
    ) -> Self {
        (
            V1::constant(builder, value.0),
            V2::constant(builder, value.1),
            V3::constant(builder, value.2),
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

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        (
            self.0.get(witness),
            self.1.get(witness),
            self.2.get(witness),
        )
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.0.set(witness, value.0);
        self.1.set(witness, value.1);
        self.2.set(witness, value.2);
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

    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Self::ValueType<L::Field>,
    ) -> Self {
        (
            V1::constant(builder, value.0),
            V2::constant(builder, value.1),
            V3::constant(builder, value.2),
            V4::constant(builder, value.3),
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

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        (
            self.0.get(witness),
            self.1.get(witness),
            self.2.get(witness),
            self.3.get(witness),
        )
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.0.set(witness, value.0);
        self.1.set(witness, value.1);
        self.2.set(witness, value.2);
        self.3.set(witness, value.3);
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

    fn constant<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
        value: Self::ValueType<L::Field>,
    ) -> Self {
        (
            V1::constant(builder, value.0),
            V2::constant(builder, value.1),
            V3::constant(builder, value.2),
            V4::constant(builder, value.3),
            V5::constant(builder, value.4),
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
            &variables[V1::nb_elements() + V2::nb_elements() + V3::nb_elements()
                ..V1::nb_elements() + V2::nb_elements() + V3::nb_elements() + V4::nb_elements()],
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

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        (
            self.0.get(witness),
            self.1.get(witness),
            self.2.get(witness),
            self.3.get(witness),
            self.4.get(witness),
        )
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.0.set(witness, value.0);
        self.1.set(witness, value.1);
        self.2.set(witness, value.2);
        self.3.set(witness, value.3);
        self.4.set(witness, value.4);
    }
}
