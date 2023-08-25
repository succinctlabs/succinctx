use alloc::vec::Vec;
use core::marker::PhantomData;

use itertools::Itertools;
use plonky2::field::extension::Extendable;
use plonky2::field::types::Field;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::plonk::circuit_builder::CircuitBuilder;

use crate::frontend::num::biguint::BigUintTarget;
use crate::frontend::num::nonnative::nonnative::NonNativeTarget;
use crate::frontend::num::u32::gadgets::arithmetic_u32::{CircuitBuilderU32, U32Target};

pub trait CircuitBuilderSplit<F: RichField + Extendable<D>, const D: usize> {
    fn split_u32_to_4_bit_limbs(&mut self, val: U32Target) -> Vec<Target>;

    fn split_u32_to_16_bit_limbs(&mut self, val: U32Target) -> Vec<Target>;

    fn split_nonnative_to_16_bit_limbs<FF: Field>(
        &mut self,
        val: &NonNativeTarget<FF>,
    ) -> Vec<Target>;

    fn split_nonnative_to_4_bit_limbs<FF: Field>(
        &mut self,
        val: &NonNativeTarget<FF>,
    ) -> Vec<Target>;

    fn split_nonnative_to_2_bit_limbs<FF: Field>(
        &mut self,
        val: &NonNativeTarget<FF>,
    ) -> Vec<Target>;

    // Note: assumes its inputs are 4-bit limbs, and does not range-check.
    fn recombine_nonnative_4_bit_limbs<FF: Field>(
        &mut self,
        limbs: Vec<Target>,
    ) -> NonNativeTarget<FF>;

    // Note: assumes its inputs are 16-bit limbs, and does not range-check.
    fn recombine_nonnative_16_bit_limbs<FF: Field>(
        &mut self,
        limbs: Vec<Target>,
    ) -> NonNativeTarget<FF>;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilderSplit<F, D>
    for CircuitBuilder<F, D>
{
    fn split_u32_to_4_bit_limbs(&mut self, val: U32Target) -> Vec<Target> {
        let two_bit_limbs = self.split_le_base::<4>(val.0, 16);
        let four = self.constant(F::from_canonical_usize(4));
        let combined_limbs = two_bit_limbs
            .iter()
            .tuples()
            .map(|(&a, &b)| self.mul_add(b, four, a))
            .collect();

        combined_limbs
    }

    fn split_u32_to_16_bit_limbs(&mut self, val: U32Target) -> Vec<Target> {
        let mut combined_limbs = Vec::new();
        let bits = self.u32_to_bits_le(val);
        for bit_16_chunk in bits.chunks(16) {
            let limb = self.le_sum(bit_16_chunk.iter());
            combined_limbs.push(limb);
        }

        combined_limbs
    }

    fn split_nonnative_to_16_bit_limbs<FF: Field>(
        &mut self,
        val: &NonNativeTarget<FF>,
    ) -> Vec<Target> {
        val.value
            .limbs
            .iter()
            .flat_map(|&l| self.split_u32_to_16_bit_limbs(l))
            .collect()
    }

    fn split_nonnative_to_4_bit_limbs<FF: Field>(
        &mut self,
        val: &NonNativeTarget<FF>,
    ) -> Vec<Target> {
        val.value
            .limbs
            .iter()
            .flat_map(|&l| self.split_u32_to_4_bit_limbs(l))
            .collect()
    }

    fn split_nonnative_to_2_bit_limbs<FF: Field>(
        &mut self,
        val: &NonNativeTarget<FF>,
    ) -> Vec<Target> {
        val.value
            .limbs
            .iter()
            .flat_map(|&l| self.split_le_base::<4>(l.0, 16))
            .collect()
    }

    // Note: assumes its inputs are 4-bit limbs, and does not range-check.
    fn recombine_nonnative_4_bit_limbs<FF: Field>(
        &mut self,
        limbs: Vec<Target>,
    ) -> NonNativeTarget<FF> {
        let base = self.constant_u32(1 << 4);
        let u32_limbs = limbs
            .chunks(8)
            .map(|chunk| {
                let mut combined_chunk = self.zero_u32();
                for i in (0..8).rev() {
                    let (low, _high) = self.mul_add_u32(combined_chunk, base, U32Target(chunk[i]));
                    combined_chunk = low;
                }
                combined_chunk
            })
            .collect();

        NonNativeTarget {
            value: BigUintTarget { limbs: u32_limbs },
            _phantom: PhantomData,
        }
    }

    // Note: assumes its inputs are 16-bit limbs, and does not range-check.
    fn recombine_nonnative_16_bit_limbs<FF: Field>(
        &mut self,
        limbs: Vec<Target>,
    ) -> NonNativeTarget<FF> {
        let base = self.constant_u32(1 << 16);
        let u32_limbs = limbs
            .chunks(2)
            .map(|chunk| {
                let mut combined_chunk = self.zero_u32();
                for i in (0..2).rev() {
                    let (low, _high) = self.mul_add_u32(combined_chunk, base, U32Target(chunk[i]));
                    combined_chunk = low;
                }
                combined_chunk
            })
            .collect();

        NonNativeTarget {
            value: BigUintTarget { limbs: u32_limbs },
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use curta::chip::utils::biguint_to_16_digits_field;
    use num::bigint::RandBigInt;
    use plonky2::field::secp256k1_scalar::Secp256K1Scalar;
    use plonky2::field::types::Sample;
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
    use rand::thread_rng;

    use super::*;
    use crate::frontend::ecc::ed25519::field::ed25519_base::Ed25519Base;
    use crate::frontend::num::biguint::CircuitBuilderBiguint;
    use crate::frontend::num::nonnative::nonnative::{CircuitBuilderNonNative, NonNativeTarget};

    #[test]
    fn test_split_nonnative() -> Result<()> {
        type FF = Secp256K1Scalar;
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_ecc_config();
        let pw = PartialWitness::new();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x = FF::rand();
        let x_target = builder.constant_nonnative(x);
        let split = builder.split_nonnative_to_4_bit_limbs(&x_target);
        let combined: NonNativeTarget<Secp256K1Scalar> =
            builder.recombine_nonnative_4_bit_limbs(split);
        builder.connect_nonnative(&x_target, &combined);

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();
        data.verify(proof)
    }

    #[test]
    fn test_split_nonnative_16_bit_limbs() -> Result<()> {
        type FF = Ed25519Base;
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_ecc_config();
        let pw = PartialWitness::new();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let mut rng = thread_rng();
        let x = rng.gen_biguint(256);
        let x_limbs = biguint_to_16_digits_field(&x, 16);
        let mut x_limbs_expected = Vec::new();

        for i in 0..x_limbs.len() {
            let expected_limb = builder.constant(x_limbs[i]);
            x_limbs_expected.push(expected_limb);
        }

        let x_target = builder.constant_biguint(&x);
        let x_target_nonnative = builder.biguint_to_nonnative::<FF>(&x_target);
        let x_limbs_target = builder.split_nonnative_to_16_bit_limbs(&x_target_nonnative);

        for i in 0..x_limbs_target.len() {
            builder.connect(x_limbs_target[i], x_limbs_expected[i]);
        }

        let x_combined = builder.recombine_nonnative_16_bit_limbs(x_limbs_target);
        builder.connect_nonnative(&x_target_nonnative, &x_combined);

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();
        data.verify(proof)
    }
}
