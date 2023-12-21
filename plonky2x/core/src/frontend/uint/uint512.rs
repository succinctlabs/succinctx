use array_macro::array;
use ethers::types::U512;
use plonky2::hash::hash_types::RichField;

use super::Uint;
use crate::frontend::uint::num::biguint::{BigUintTarget, CircuitBuilderBiguint};
use crate::frontend::uint::num::u32::gadgets::arithmetic_u32::U32Target;
use crate::frontend::vars::{EvmVariable, SSZVariable, U32Variable};
use crate::prelude::{
    Add, BoolVariable, ByteVariable, Bytes32Variable, BytesVariable, CircuitBuilder,
    CircuitVariable, Div, LessThanOrEqual, Mul, One, PlonkParameters, Rem, Sub, Variable, Zero,
};
use crate::{make_uint32_n, make_uint32_n_tests};

impl Uint<16> for U512 {
    fn to_little_endian(&self, bytes: &mut [u8]) {
        self.to_little_endian(bytes);
    }

    fn from_little_endian(slice: &[u8]) -> Self {
        Self::from_little_endian(slice)
    }

    fn to_big_endian(&self, bytes: &mut [u8]) {
        self.to_big_endian(bytes);
    }

    fn from_big_endian(slice: &[u8]) -> Self {
        Self::from_big_endian(slice)
    }

    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }

    fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }

    fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        self.overflowing_mul(rhs)
    }
}

make_uint32_n!(U512Variable, U512, 16);
make_uint32_n_tests!(U512Variable, U512, 16);

mod tests2 {

    use ethers::types::U512;

    use super::*;
    use crate::backend::circuit::DefaultParameters;
    use crate::frontend::uint::Uint;
    use crate::prelude::*;

    type L = DefaultParameters;

    #[test]
    fn failing_test_mul() {
        const D: usize = 2;

        let _ = env_logger::builder().is_test(true).try_init();

        let a = <U512 as Uint<16>>::from_u32_limbs([
            1693531657, 1693531657, 1693531657, 1693531657, 1693531657, 1693531657, 1693531657,
            1693531657, 1693531657, 1693531657, 1693531657, 1693531657, 1693531657, 1693531657,
            1693531657, 1693531657,
        ]);
        let b = <U512 as Uint<16>>::from_u32_limbs([
            3656660001, 3656660001, 3656660001, 3656660001, 3656660001, 3656660001, 3656660001,
            3656660001, 3656660001, 3656660001, 3656660001, 3656660001, 3656660001, 3656660001,
            3656660001, 3656660001,
        ]);

        let mut builder = CircuitBuilder::<L, D>::new();

        let a = U512Variable::constant(&mut builder, a);
        let b = U512Variable::constant(&mut builder, b);
        let result = builder.mul(a, b);

        builder.watch(&result, "result");

        let circuit = builder.build();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }

    #[test]
    fn failing_test_mul2() {
        const D: usize = 2;

        let _ = env_logger::builder().is_test(true).try_init();

        let a_limbs = [
            1693531657, 1693531657, 1693531657, 1693531657, 1693531657, 1693531657, 1693531657,
            1693531657, 1693531657, 1693531657, 1693531657, 1693531657, 1693531657, 1693531657,
            1693531657, 1693531657,
        ];
        let b_limbs = [
            3656660001, 3656660001, 3656660001, 3656660001, 3656660001, 3656660001, 3656660001,
            3656660001, 3656660001, 3656660001, 3656660001, 3656660001, 3656660001, 3656660001,
            3656660001, 3656660001,
        ];

        let mut builder = CircuitBuilder::<L, D>::new();

        let a_targets = a_limbs
            .iter()
            .map(|x| {
                let t = builder
                    .api
                    .constant(GoldilocksField::from_canonical_u32(*x));
                U32Target::from_target_unsafe(t)
            })
            .collect::<Vec<_>>();
        let b_targets = b_limbs
            .iter()
            .map(|x| {
                let t = builder
                    .api
                    .constant(GoldilocksField::from_canonical_u32(*x));
                U32Target::from_target_unsafe(t)
            })
            .collect::<Vec<_>>();
        assert_eq!(a_targets.len(), b_targets.len());

        let a_biguint = BigUintTarget { limbs: a_targets };
        let b_biguint = BigUintTarget { limbs: b_targets };
        let product_biguint = builder.api.mul_biguint(&a_biguint, &b_biguint);

        //builder.watch(&result, "result");
        //builder.assert_is_equal(result, expected_result_var);

        let circuit = builder.build();
        let pw = PartialWitness::new();

        let proof = circuit.data.prove(pw).unwrap();
        circuit.data.verify(proof).unwrap();
    }
}
