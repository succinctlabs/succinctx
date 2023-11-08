use std::fmt::Debug;

use curta::chip::register::element;
use itertools::Itertools;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::BoolTarget;
use serde::{Deserialize, Serialize};

use super::{ArrayVariable, ByteVariable, CircuitVariable, Variable};
use crate::backend::circuit::PlonkParameters;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::eth::rlp::utils::{MPTNodeFixedSize, MAX_MPT_NODE_SIZE, MAX_RLP_ITEM_SIZE};
use crate::frontend::ops::{BitAnd, BitOr, BitXor, Not};

/// A variable in the circuit representing an MPT node.
///
/// The MPT node is represented as a tuple of `(data, lens, len)`, where:
/// - `data` is an array of bytes representing the data of the node.
/// - `lens` is an array of `Variable`s representing the true length of each element in `data`.
/// - `len` is a `Variable` representing the true length of `data`.
///
/// The vast majority of time, a node is a branch node.
#[derive(Debug, Clone)]
#[allow(clippy::manual_non_exhaustive)]
pub struct MPTVariable {
    pub data: ArrayVariable<ArrayVariable<ByteVariable, MAX_RLP_ITEM_SIZE>, MAX_MPT_NODE_SIZE>,
    pub lens: ArrayVariable<Variable, MAX_MPT_NODE_SIZE>,
    pub len: Variable,

    /// This private field is here to force all instantiations to go the methods below.
    _private: (),
}

impl CircuitVariable for MPTVariable {
    type ValueType<F: RichField> = MPTNodeFixedSize;

    fn nb_elements() -> usize {
        MAX_MPT_NODE_SIZE * MAX_RLP_ITEM_SIZE + MAX_MPT_NODE_SIZE + 1
    }

    fn from_elements<F: RichField>(elements: &[F]) -> Self::ValueType<F> {
        todo!()
    }
    fn targets(&self) -> Vec<plonky2::iop::target::Target> {
        self.variables().into_iter().map(|v| v.0).collect()
    }

    fn from_targets(targets: &[plonky2::iop::target::Target]) -> Self {
        Self::from_variables_unsafe(&targets.iter().map(|t| Variable(*t)).collect_vec())
    }

    fn init_unsafe<L: PlonkParameters<D>, const D: usize>(
        builder: &mut CircuitBuilder<L, D>,
    ) -> Self {
        Self {
            data: ArrayVariable::init_unsafe(builder),
            lens: ArrayVariable::init_unsafe(builder),
            len: Variable::init_unsafe(builder),
            _private: (),
        }
    }

    fn variables(&self) -> Vec<Variable> {
        todo!()
    }

    fn from_variables_unsafe(variables: &[Variable]) -> Self {
        todo!()
    }

    fn assert_is_valid<L: PlonkParameters<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<L, D>,
    ) {
        self.data.assert_is_valid(builder);
        self.lens.assert_is_valid(builder);
        self.len.assert_is_valid(builder);
    }

    fn elements<F: RichField>(value: Self::ValueType<F>) -> Vec<F> {
        value
            .data
            .into_iter()
            .take(value.len)
            .flat_map(|v| {
                v.data
                    .into_iter()
                    .map(F::from_canonical_u8)
                    .take(v.len)
                    .collect_vec()
            })
            .collect()
    }
}

impl From<MPTNodeFixedSize> for MPTVariable {
    fn from(v: MPTNodeFixedSize) -> Self {
        todo!()
    }
}

#[allow(clippy::from_over_into)]
impl Into<MPTNodeFixedSize> for MPTVariable {
    fn into(self) -> MPTNodeFixedSize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::MPTVariable;
    use crate::backend::circuit::DefaultParameters;
    use crate::frontend::eth::rlp::utils::MPTNodeFixedSize;
    use crate::prelude::*;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_elements() {
        let mut mpt = MPTNodeFixedSize::default();
        mpt.len = 2;
        let mut exp: Vec<GoldilocksField> = vec![];
        for i in 0..2 {
            mpt.data[i].len = 3;
            for j in 0..3 {
                let v = 10 * i + j;
                mpt.data[i].data[j] = v as u8;
                exp.push(GoldilocksField::from_canonical_usize(v));
            }
        }
        assert_eq!(MPTVariable::elements::<GoldilocksField>(mpt), exp);
    }

    #[test]
    fn test_mpt() {
        // For now, this test just initializes the MPT variable without any checks whatsoever.
        let mut builder = CircuitBuilder::<L, D>::new();

        let mpt = builder.init::<MPTVariable>();

        // TODO: This test is obviously incomplete.
        // let y = builder.init::<BoolVariable>();

        // let not_x = builder.not(x);
        // let not_y = builder.not(y);
        // let x_and_y = builder.and(x, y);
        // let x_and_x = builder.and(x, x);
        // let x_or_y = builder.or(x, y);
        // let x_or_x = builder.or(x, x);
        // let y_or_y = builder.or(y, y);
        // let x_xor_y = builder.xor(x, y);
        // let x_xor_x = builder.xor(x, x);
        // let y_xor_y = builder.xor(y, y);

        // let mut pw: PartialWitness<GoldilocksField> = PartialWitness::new();

        // x.set(&mut pw, true);
        // y.set(&mut pw, false);

        // not_x.set(&mut pw, false);
        // not_y.set(&mut pw, true);
        // x_and_y.set(&mut pw, false);
        // x_and_x.set(&mut pw, true);
        // x_or_y.set(&mut pw, true);
        // x_or_x.set(&mut pw, true);
        // y_or_y.set(&mut pw, false);
        // x_xor_y.set(&mut pw, true);
        // x_xor_x.set(&mut pw, false);
        // y_xor_y.set(&mut pw, false);

        // let circuit = builder.build();
        // let proof = circuit.data.prove(pw).unwrap();
        // circuit.data.verify(proof).unwrap();
    }
}
