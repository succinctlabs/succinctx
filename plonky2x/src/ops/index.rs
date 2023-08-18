use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;

use crate::builder::CircuitBuilder;

/// Intereface for random access
///
/// This operation is invoked by builder.random_access(variable, index)
pub trait RandomAccess<F: RichField + Extendable<D>, const D: usize, Idx>
where
    Idx: ?Sized,
{
    type Output;

    fn random_access(self, index: Idx, builder: &mut CircuitBuilder<F, D>) -> Self::Output;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn random_access<T, Idx>(
        &mut self,
        variable: T,
        index: Idx,
    ) -> <T as RandomAccess<F, D, Idx>>::Output
    where
        T: RandomAccess<F, D, Idx>,
    {
        variable.random_access(index, self)
    }
}
