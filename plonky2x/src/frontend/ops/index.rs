use crate::backend::config::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;

/// Interface for random access over a generic index type.
///
/// Types implementing this trait can be used with the `builder.random_access(variable, index)`
/// method. This trait can be used for implementing random access for indices that are
/// not known during circuit construction.
pub trait RandomAccess<L: PlonkParameters<D>, const D: usize, Idx>
where
    Idx: ?Sized,
{
    type Output;

    fn random_access(self, index: Idx, builder: &mut CircuitBuilder<L, D>) -> Self::Output;
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    pub fn random_access<T, Idx>(
        &mut self,
        variable: T,
        index: Idx,
    ) -> <T as RandomAccess<L, D, Idx>>::Output
    where
        T: RandomAccess<L, D, Idx>,
    {
        variable.random_access(index, self)
    }
}
