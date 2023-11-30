//! Arithmetic operations.

use std::marker::PhantomData;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::mpt::generators::LteGenerator;
use crate::prelude::{BoolVariable, Variable};

/// The addition operation.
///
/// Types implementing this trait can be used within the `builder.add(lhs, rhs)` method.
pub trait Add<L: PlonkParameters<D>, const D: usize, Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn add<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as Add<L, D, Rhs>>::Output
    where
        Lhs: Add<L, D, Rhs>,
    {
        lhs.add(rhs, self)
    }

    pub fn add_many<T>(&mut self, values: &[T]) -> T
    where
        T: Add<L, D, T, Output = T> + Clone,
    {
        let mut sum = values[0].clone();
        for i in 1..values.len() {
            sum = self.add(sum, values[i].clone());
        }
        sum
    }
}

/// The subtraction operation.
///
/// Types implementing this trait can be used within the `builder.sub(lhs, rhs)` method.
pub trait Sub<L: PlonkParameters<D>, const D: usize, Rhs = Self> {
    type Output;

    fn sub(self, rhs: Rhs, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn sub<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as Sub<L, D, Rhs>>::Output
    where
        Lhs: Sub<L, D, Rhs>,
    {
        lhs.sub(rhs, self)
    }
}

/// The multiplication operation.
///
/// Types implementing this trait can be used within the `builder.mul(lhs, rhs)` method.
pub trait Mul<L: PlonkParameters<D>, const D: usize, Rhs = Self> {
    type Output;

    fn mul(self, rhs: Rhs, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn mul<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as Mul<L, D, Rhs>>::Output
    where
        Lhs: Mul<L, D, Rhs>,
    {
        lhs.mul(rhs, self)
    }
}

/// The negation operation.
///
/// Types implementing this trait can be used within the `builder.neg(value)` method.
pub trait Neg<L: PlonkParameters<D>, const D: usize> {
    type Output;

    fn neg(self, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn neg<T>(&mut self, value: T) -> <T as Neg<L, D>>::Output
    where
        T: Neg<L, D>,
    {
        value.neg(self)
    }
}

/// The division operation.
///
/// Types implementing this trait can be used within the `builder.div(lhs, rhs)` method.
pub trait Div<L: PlonkParameters<D>, const D: usize, Rhs = Self> {
    type Output;

    fn div(self, rhs: Rhs, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn div<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as Div<L, D, Rhs>>::Output
    where
        Lhs: Div<L, D, Rhs>,
    {
        lhs.div(rhs, self)
    }
}

/// The remainder operation.
///
/// Types implementing this trait can be used within the `builder.rem(lhs, rhs)` method.
pub trait Rem<L: PlonkParameters<D>, const D: usize, Rhs = Self> {
    type Output;

    fn rem(self, rhs: Rhs, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn rem<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as Rem<L, D, Rhs>>::Output
    where
        Lhs: Rem<L, D, Rhs>,
    {
        lhs.rem(rhs, self)
    }
}

/// The division + remainder operation.
///
/// Types implementing this trait can be used within the `builder.div_rem(lhs, rhs)` method.
pub trait DivRem<L: PlonkParameters<D>, const D: usize, Rhs = Self> {
    type Output;

    fn div_rem(self, rhs: Rhs, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn div_rem<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> <Lhs as DivRem<L, D, Rhs>>::Output
    where
        Lhs: DivRem<L, D, Rhs>,
    {
        lhs.div_rem(rhs, self)
    }
}

/// A zero element.
///
/// Types implementing this trait can be used via the `builder.zero()` method.
pub trait Zero<L: PlonkParameters<D>, const D: usize> {
    fn zero(builder: &mut CircuitBuilder<L, D>) -> Self;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn zero<T: Zero<L, D>>(&mut self) -> T {
        T::zero(self)
    }
}

/// A one element
///
/// Types implementing this trait can be used via the `builder.one()` method.
pub trait One<L: PlonkParameters<D>, const D: usize> {
    fn one(builder: &mut CircuitBuilder<L, D>) -> Self;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn one<T: One<L, D>>(&mut self) -> T {
        T::one(self)
    }
}

/// The less than or equal operation (<=).
///
/// Types implementing this trait can be used within the `builder.lte(lhs, rhs)` method.
pub trait LessThanOrEqual<L: PlonkParameters<D>, const D: usize, Rhs = Self> {
    fn lte(self, rhs: Rhs, builder: &mut CircuitBuilder<L, D>) -> BoolVariable;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// The less than or equal to operation (<=).
    pub fn lte<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> BoolVariable
    where
        Lhs: LessThanOrEqual<L, D, Rhs>,
    {
        lhs.lte(rhs, self)
    }

    /// The less than operation (<).
    pub fn lt<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> BoolVariable
    where
        Lhs: LessThanOrEqual<L, D, Lhs>,
        Rhs: Sub<L, D, Rhs, Output = Lhs> + One<L, D>,
    {
        let one = self.one::<Rhs>();
        let upper_bound = rhs.sub(one, self);
        self.lte(lhs, upper_bound)
    }

    /// The greater than operation (>).
    pub fn gt<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> BoolVariable
    where
        Lhs: Sub<L, D, Lhs, Output = Rhs> + One<L, D>,
        Rhs: LessThanOrEqual<L, D, Rhs>,
    {
        self.lt(rhs, lhs)
    }

    /// The greater than or equal to operation (>=).
    pub fn gte<Lhs, Rhs>(&mut self, lhs: Lhs, rhs: Rhs) -> BoolVariable
    where
        Lhs: Sub<L, D, Lhs, Output = Rhs> + One<L, D>,
        Rhs: LessThanOrEqual<L, D, Lhs>,
    {
        self.lte(rhs, lhs)
    }

    /// The within range operation (lhs <= variable < rhs).
    pub fn within_range<V>(&mut self, variable: V, lhs: V, rhs: V) -> BoolVariable
    where
        V: LessThanOrEqual<L, D, V> + Sub<L, D, V, Output = V> + One<L, D> + Clone,
    {
        let lower_bound_satisfied = self.lte(lhs, variable.clone());
        let upper_bound_satisfied = self.lt(variable, rhs);
        self.and(lower_bound_satisfied, upper_bound_satisfied)
    }
}

impl<L: PlonkParameters<D>, const D: usize> LessThanOrEqual<L, D> for Variable {
    fn lte(self, rhs: Variable, builder: &mut CircuitBuilder<L, D>) -> BoolVariable {
        // TODO: FIX
        let generator: LteGenerator<L, D> = LteGenerator {
            lhs: self,
            rhs,
            output: builder.init::<BoolVariable>(),
            _phantom: PhantomData,
        };
        builder.add_simple_generator(generator.clone());
        generator.output
    }
}

mod tests {
    #[allow(unused_imports)]
    use crate::prelude::{BoolVariable, DefaultBuilder, U32Variable};

    #[test]
    fn test_math_gt() {
        let mut builder = DefaultBuilder::new();

        let v0 = builder.read::<U32Variable>();
        let v1 = builder.read::<U32Variable>();
        let result = builder.read::<BoolVariable>();
        let computed_result = builder.gt(v0, v1);
        builder.assert_is_equal(result, computed_result);

        let circuit = builder.build();

        let test_cases = [
            (10u32, 20u32, false),
            (10u32, 10u32, false),
            (10u32, 5u32, true),
        ];

        for test_case in test_cases.iter() {
            let mut input = circuit.input();
            input.write::<U32Variable>(test_case.0);
            input.write::<U32Variable>(test_case.1);
            input.write::<BoolVariable>(test_case.2);

            let (proof, output) = circuit.prove(&input);
            circuit.verify(&proof, &input, &output);
        }
    }

    #[test]
    fn test_math_gte() {
        let mut builder = DefaultBuilder::new();

        let v0 = builder.read::<U32Variable>();
        let v1 = builder.read::<U32Variable>();
        let result = builder.read::<BoolVariable>();
        let computed_result = builder.gte(v0, v1);
        builder.assert_is_equal(result, computed_result);

        let circuit = builder.build();

        let test_cases = [
            (10u32, 20u32, false),
            (10u32, 10u32, true),
            (10u32, 5u32, true),
        ];

        for test_case in test_cases.iter() {
            let mut input = circuit.input();
            input.write::<U32Variable>(test_case.0);
            input.write::<U32Variable>(test_case.1);
            input.write::<BoolVariable>(test_case.2);

            let (proof, output) = circuit.prove(&input);
            circuit.verify(&proof, &input, &output);
        }
    }
}
