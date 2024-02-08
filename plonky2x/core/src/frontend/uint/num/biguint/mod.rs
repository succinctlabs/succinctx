use core::marker::PhantomData;

use itertools::Itertools;
use num::{BigUint, Integer, Zero};
use plonky2::field::extension::Extendable;
use plonky2::field::types::{PrimeField, PrimeField64};
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::iop::witness::{PartitionWitness, Witness};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};

use super::u32::gadgets::range_check::range_check_u32_circuit;
use super::u32::serialization::{ReadU32, WriteU32};
use crate::frontend::uint::num::u32::gadgets::arithmetic_u32::{CircuitBuilderU32, U32Target};
use crate::frontend::uint::num::u32::gadgets::multiple_comparison::list_le_u32_circuit;
use crate::frontend::uint::num::u32::witness::{GeneratedValuesU32, WitnessU32};
use crate::prelude::{BytesVariable, CircuitBuilder as Plonky2xCircuitBuilder, PlonkParameters};

#[derive(Clone, Debug, Default)]
pub struct BigUintTarget {
    pub limbs: Vec<U32Target>,
}

impl BigUintTarget {
    pub fn num_limbs(&self) -> usize {
        self.limbs.len()
    }

    pub fn get_limb(&self, i: usize) -> U32Target {
        self.limbs[i]
    }
}

// This function will convert a BytesVariable into a plonky2 BigUintTarget.
pub fn biguint_from_bytes_variable<L: PlonkParameters<D>, const D: usize, const N: usize>(
    builder: &mut Plonky2xCircuitBuilder<L, D>,
    bytes: BytesVariable<N>,
) -> BigUintTarget {
    assert!(bytes.len() % 32 == 0);

    // Convert to BigUintTarget.
    // Note that the limbs within the BigUintTarget are in little endian ordering, so
    // the least significant u32 should be processed first.
    let mut u32_targets = Vec::new();

    // Convert the bytes into bits.
    let mut le_bits: Vec<BoolTarget> = Vec::new();
    for i in 0..bytes.len() {
        le_bits.extend_from_slice(&bytes[i].as_le_bits().map(|x| x.into()));
    }

    for u32_chunk in le_bits.chunks(32) {
        u32_targets.push(U32Target::from_target_unsafe(
            builder.api.le_sum(u32_chunk.iter()),
        ));
    }

    BigUintTarget { limbs: u32_targets }
}

pub trait CircuitBuilderBiguint<F: RichField + Extendable<D>, const D: usize> {
    fn constant_biguint(&mut self, value: &BigUint) -> BigUintTarget;

    fn zero_biguint(&mut self) -> BigUintTarget;

    fn connect_biguint(&mut self, lhs: &BigUintTarget, rhs: &BigUintTarget);

    fn pad_biguints(
        &mut self,
        a: &BigUintTarget,
        b: &BigUintTarget,
    ) -> (BigUintTarget, BigUintTarget);

    fn cmp_biguint(&mut self, a: &BigUintTarget, b: &BigUintTarget) -> BoolTarget;

    fn add_virtual_biguint_target_unsafe(&mut self, num_limbs: usize) -> BigUintTarget;

    /// Add two `BigUintTarget`s.
    fn add_biguint(&mut self, a: &BigUintTarget, b: &BigUintTarget) -> BigUintTarget;

    /// Subtract two `BigUintTarget`s. We assume that the first is larger than the second.
    fn sub_biguint(&mut self, a: &BigUintTarget, b: &BigUintTarget) -> BigUintTarget;

    fn mul_biguint(&mut self, a: &BigUintTarget, b: &BigUintTarget) -> BigUintTarget;

    fn mul_biguint_by_bool(&mut self, a: &BigUintTarget, b: BoolTarget) -> BigUintTarget;

    /// Returns x * y + z. This is no more efficient than mul-then-add; it's purely for convenience (only need to call one CircuitBuilder function).
    fn mul_add_biguint(
        &mut self,
        x: &BigUintTarget,
        y: &BigUintTarget,
        z: &BigUintTarget,
    ) -> BigUintTarget;

    fn _div_rem_biguint(
        &mut self,
        a: &BigUintTarget,
        b: &BigUintTarget,
        div_num_limbs: usize,
    ) -> (BigUintTarget, BigUintTarget);

    fn div_rem_biguint(
        &mut self,
        a: &BigUintTarget,
        b: &BigUintTarget,
    ) -> (BigUintTarget, BigUintTarget);

    fn div_rem_biguint_unsafe(
        &mut self,
        a: &BigUintTarget,
        b: &BigUintTarget,
    ) -> (BigUintTarget, BigUintTarget);

    fn div_biguint(&mut self, a: &BigUintTarget, b: &BigUintTarget) -> BigUintTarget;

    fn div_biguint_unsafe(&mut self, a: &BigUintTarget, b: &BigUintTarget) -> BigUintTarget;

    fn rem_biguint(&mut self, a: &BigUintTarget, b: &BigUintTarget) -> BigUintTarget;

    fn random_access_biguint(
        &mut self,
        access_index: Target,
        v: Vec<&BigUintTarget>,
    ) -> BigUintTarget;

    fn is_equal_biguint(&mut self, a: &BigUintTarget, b: &BigUintTarget) -> BoolTarget;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilderBiguint<F, D>
    for CircuitBuilder<F, D>
{
    fn constant_biguint(&mut self, value: &BigUint) -> BigUintTarget {
        let limb_values = value.to_u32_digits();
        let limbs = limb_values.iter().map(|&l| self.constant_u32(l)).collect();

        BigUintTarget { limbs }
    }

    fn zero_biguint(&mut self) -> BigUintTarget {
        self.constant_biguint(&BigUint::zero())
    }

    fn connect_biguint(&mut self, lhs: &BigUintTarget, rhs: &BigUintTarget) {
        let min_limbs = lhs.num_limbs().min(rhs.num_limbs());
        for i in 0..min_limbs {
            self.connect_u32(lhs.get_limb(i), rhs.get_limb(i));
        }

        for i in min_limbs..lhs.num_limbs() {
            self.assert_zero_u32(lhs.get_limb(i));
        }
        for i in min_limbs..rhs.num_limbs() {
            self.assert_zero_u32(rhs.get_limb(i));
        }
    }

    fn pad_biguints(
        &mut self,
        a: &BigUintTarget,
        b: &BigUintTarget,
    ) -> (BigUintTarget, BigUintTarget) {
        if a.num_limbs() > b.num_limbs() {
            let mut padded_b = b.clone();
            for _ in b.num_limbs()..a.num_limbs() {
                padded_b.limbs.push(self.zero_u32());
            }

            (a.clone(), padded_b)
        } else {
            let mut padded_a = a.clone();
            for _ in a.num_limbs()..b.num_limbs() {
                padded_a.limbs.push(self.zero_u32());
            }

            (padded_a, b.clone())
        }
    }

    fn cmp_biguint(&mut self, a: &BigUintTarget, b: &BigUintTarget) -> BoolTarget {
        let (a, b) = self.pad_biguints(a, b);

        list_le_u32_circuit(self, a.limbs, b.limbs)
    }

    fn add_virtual_biguint_target_unsafe(&mut self, num_limbs: usize) -> BigUintTarget {
        let limbs = self.add_virtual_u32_targets_unsafe(num_limbs);

        BigUintTarget { limbs }
    }

    fn add_biguint(&mut self, a: &BigUintTarget, b: &BigUintTarget) -> BigUintTarget {
        let num_limbs = a.num_limbs().max(b.num_limbs());

        let mut combined_limbs = vec![];
        let mut carry = self.zero_u32();
        for i in 0..num_limbs {
            let a_limb = (i < a.num_limbs())
                .then(|| a.limbs[i])
                .unwrap_or_else(|| self.zero_u32());
            let b_limb = (i < b.num_limbs())
                .then(|| b.limbs[i])
                .unwrap_or_else(|| self.zero_u32());

            let (new_limb, new_carry) = self.add_many_u32(&[carry, a_limb, b_limb]);
            carry = new_carry;
            combined_limbs.push(new_limb);
        }
        combined_limbs.push(carry);

        BigUintTarget {
            limbs: combined_limbs,
        }
    }

    fn sub_biguint(&mut self, a: &BigUintTarget, b: &BigUintTarget) -> BigUintTarget {
        let (a, b) = self.pad_biguints(a, b);
        let num_limbs = a.limbs.len();

        let mut result_limbs = vec![];

        let mut borrow = self.zero_u32();
        for i in 0..num_limbs {
            let (result, new_borrow) = self.sub_u32(a.limbs[i], b.limbs[i], borrow);
            result_limbs.push(result);
            borrow = new_borrow;
        }
        // Borrow should be zero here.

        BigUintTarget {
            limbs: result_limbs,
        }
    }

    fn mul_biguint(&mut self, a: &BigUintTarget, b: &BigUintTarget) -> BigUintTarget {
        let total_limbs = a.limbs.len() + b.limbs.len();

        let mut to_add = vec![vec![]; total_limbs];
        for i in 0..a.limbs.len() {
            for j in 0..b.limbs.len() {
                let (product, carry) = self.mul_u32(a.limbs[i], b.limbs[j]);
                to_add[i + j].push(product);
                to_add[i + j + 1].push(carry);
            }
        }

        let mut combined_limbs = vec![];
        let mut carry = self.zero_u32();
        for summands in &mut to_add {
            let (new_result, new_carry) = self.add_u32s_with_carry(summands, carry);
            combined_limbs.push(new_result);
            carry = new_carry;
        }
        combined_limbs.push(carry);

        BigUintTarget {
            limbs: combined_limbs,
        }
    }

    fn mul_biguint_by_bool(&mut self, a: &BigUintTarget, b: BoolTarget) -> BigUintTarget {
        let t = b.target;

        // Each limb will be multipled by 0 or 1, which will have a product that is within
        // U32Target's range.
        BigUintTarget {
            limbs: a
                .limbs
                .iter()
                .map(|&l| U32Target::from_target_unsafe(self.mul(l.target, t)))
                .collect(),
        }
    }

    fn mul_add_biguint(
        &mut self,
        x: &BigUintTarget,
        y: &BigUintTarget,
        z: &BigUintTarget,
    ) -> BigUintTarget {
        let prod = self.mul_biguint(x, y);
        self.add_biguint(&prod, z)
    }

    fn _div_rem_biguint(
        &mut self,
        a: &BigUintTarget,
        b: &BigUintTarget,
        div_num_limbs: usize,
    ) -> (BigUintTarget, BigUintTarget) {
        let b_len = b.limbs.len();
        let div = self.add_virtual_biguint_target_unsafe(div_num_limbs);
        let rem = self.add_virtual_biguint_target_unsafe(b_len);

        self.add_simple_generator(BigUintDivRemGenerator::<F, D> {
            a: a.clone(),
            b: b.clone(),
            div: div.clone(),
            rem: rem.clone(),
            _phantom: PhantomData,
        });

        range_check_u32_circuit(self, div.limbs.clone());
        range_check_u32_circuit(self, rem.limbs.clone());

        let div_b = self.mul_biguint(&div, b);
        let div_b_plus_rem = self.add_biguint(&div_b, &rem);
        self.connect_biguint(a, &div_b_plus_rem);

        // Assert that `r < b`. We do that by asserting that the result of `b \leq r` is `false`.
        let cmp_b_rem = self.cmp_biguint(b, &rem);
        self.assert_zero(cmp_b_rem.target);

        (div, rem)
    }

    fn div_rem_biguint(
        &mut self,
        a: &BigUintTarget,
        b: &BigUintTarget,
    ) -> (BigUintTarget, BigUintTarget) {
        let a_len = a.limbs.len();
        let (div, rem) = self._div_rem_biguint(a, b, a_len);
        (div, rem)
    }

    fn div_rem_biguint_unsafe(
        &mut self,
        a: &BigUintTarget,
        b: &BigUintTarget,
    ) -> (BigUintTarget, BigUintTarget) {
        let a_len = a.limbs.len();
        let b_len = b.limbs.len();
        let div_num_limbs = if b_len > a_len + 1 {
            0
        } else {
            a_len - b_len + 1
        };
        let (div, rem) = self._div_rem_biguint(a, b, div_num_limbs);
        (div, rem)
    }

    fn div_biguint(&mut self, a: &BigUintTarget, b: &BigUintTarget) -> BigUintTarget {
        let (div, _rem) = self.div_rem_biguint(a, b);
        div
    }

    fn div_biguint_unsafe(&mut self, a: &BigUintTarget, b: &BigUintTarget) -> BigUintTarget {
        let (div, _rem) = self.div_rem_biguint_unsafe(a, b);
        div
    }

    fn rem_biguint(&mut self, a: &BigUintTarget, b: &BigUintTarget) -> BigUintTarget {
        let (_div, rem) = self.div_rem_biguint(a, b);
        rem
    }

    fn random_access_biguint(
        &mut self,
        access_index: Target,
        v: Vec<&BigUintTarget>,
    ) -> BigUintTarget {
        // Check that all of the inputted biguint targets have the same number of limbs
        let num_limbs = v[0].num_limbs();
        for i in 1..v.len() {
            assert_eq!(v[i].num_limbs(), num_limbs);
        }

        // The random_access will be done on BigUintTarget limbs, which are U32Target types.
        let limbs = (0..num_limbs)
            .map(|i| {
                U32Target::from_target_unsafe(self.random_access(
                    access_index,
                    v.iter().map(|biguint| biguint.limbs[i].target).collect(),
                ))
            })
            .collect::<Vec<_>>();

        BigUintTarget { limbs }
    }

    fn is_equal_biguint(&mut self, a: &BigUintTarget, b: &BigUintTarget) -> BoolTarget {
        let mut ret = self._true();
        let false_t = self._false().target;

        let min_limbs = a.num_limbs().min(b.num_limbs());
        for i in 0..min_limbs {
            let limb_equal = self.is_equal_u32(a.get_limb(i), b.get_limb(i));
            ret = BoolTarget::new_unsafe(self.select(limb_equal, ret.target, false_t));
        }

        let zero_u32 = self.zero_u32();
        for i in min_limbs..a.num_limbs() {
            let is_zero = self.is_equal_u32(a.get_limb(i), zero_u32);
            ret = BoolTarget::new_unsafe(self.select(is_zero, ret.target, false_t));
        }
        for i in min_limbs..b.num_limbs() {
            let is_zero = self.is_equal_u32(b.get_limb(i), zero_u32);
            ret = BoolTarget::new_unsafe(self.select(is_zero, ret.target, false_t));
        }

        ret
    }
}

pub trait WitnessBigUint<F: PrimeField64>: Witness<F> {
    fn get_biguint_target(&self, target: BigUintTarget) -> BigUint;
    fn set_biguint_target(&mut self, target: &BigUintTarget, value: &BigUint);
}

impl<T: Witness<F>, F: PrimeField64> WitnessBigUint<F> for T {
    fn get_biguint_target(&self, target: BigUintTarget) -> BigUint {
        target
            .limbs
            .into_iter()
            .rev()
            .fold(BigUint::zero(), |acc, limb| {
                (acc << 32) + self.get_target(limb.target).to_canonical_biguint()
            })
    }

    fn set_biguint_target(&mut self, target: &BigUintTarget, value: &BigUint) {
        let mut limbs = value.to_u32_digits();
        assert!(target.num_limbs() >= limbs.len());
        limbs.resize(target.num_limbs(), 0);
        for i in 0..target.num_limbs() {
            self.set_u32_target(target.limbs[i], limbs[i]);
        }
    }
}

pub trait GeneratedValuesBigUint<F: PrimeField> {
    fn set_biguint_target(&mut self, target: &BigUintTarget, value: &BigUint);
}

impl<F: PrimeField> GeneratedValuesBigUint<F> for GeneratedValues<F> {
    fn set_biguint_target(&mut self, target: &BigUintTarget, value: &BigUint) {
        let mut limbs = value.to_u32_digits();
        assert!(target.num_limbs() >= limbs.len());
        limbs.resize(target.num_limbs(), 0);
        for i in 0..target.num_limbs() {
            self.set_u32_target(target.get_limb(i), limbs[i]);
        }
    }
}

#[derive(Debug)]
pub struct BigUintDivRemGenerator<F: RichField + Extendable<D>, const D: usize> {
    a: BigUintTarget,
    b: BigUintTarget,
    div: BigUintTarget,
    rem: BigUintTarget,
    _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> BigUintDivRemGenerator<F, D> {
    pub fn id() -> String {
        "BigUintDivRemGenerator".to_string()
    }
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for BigUintDivRemGenerator<F, D>
{
    fn id(&self) -> String {
        Self::id()
    }

    fn dependencies(&self) -> Vec<Target> {
        self.a
            .limbs
            .iter()
            .chain(&self.b.limbs)
            .map(|&l| l.target)
            .collect()
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let a = witness.get_biguint_target(self.a.clone());
        let b = witness.get_biguint_target(self.b.clone());
        let (div, rem) = a.div_rem(&b);

        out_buffer.set_biguint_target(&self.div, &div);
        out_buffer.set_biguint_target(&self.rem, &rem);
    }

    fn serialize(&self, dst: &mut Vec<u8>, _: &CommonCircuitData<F, D>) -> IoResult<()> {
        dst.write_target_vec(&self.a.limbs.iter().map(|x| x.target).collect_vec())?;
        dst.write_target_vec(&self.b.limbs.iter().map(|x| x.target).collect_vec())?;
        dst.write_target_vec(&self.div.limbs.iter().map(|x| x.target).collect_vec())?;
        dst.write_target_vec(&self.rem.limbs.iter().map(|x| x.target).collect_vec())?;
        Ok(())
    }

    fn deserialize(
        src: &mut plonky2::util::serialization::Buffer,
        _: &CommonCircuitData<F, D>,
    ) -> IoResult<Self>
    where
        Self: Sized,
    {
        // The serialization function was saving U32Targets (for the BigUInt limbs), so
        // we assume that here.
        let a = BigUintTarget {
            limbs: src
                .read_target_vec()?
                .into_iter()
                .map(U32Target::from_target_unsafe)
                .collect_vec(),
        };
        let b = BigUintTarget {
            limbs: src
                .read_target_vec()?
                .into_iter()
                .map(U32Target::from_target_unsafe)
                .collect_vec(),
        };
        let div = BigUintTarget {
            limbs: src
                .read_target_vec()?
                .into_iter()
                .map(U32Target::from_target_unsafe)
                .collect_vec(),
        };
        let rem = BigUintTarget {
            limbs: src
                .read_target_vec()?
                .into_iter()
                .map(U32Target::from_target_unsafe)
                .collect_vec(),
        };
        Ok(Self {
            a,
            b,
            div,
            rem,
            _phantom: PhantomData,
        })
    }
}

pub trait WriteBigUint {
    fn write_target_biguint(&mut self, x: BigUintTarget) -> IoResult<()>;
}

impl WriteBigUint for Vec<u8> {
    #[inline]
    fn write_target_biguint(&mut self, x: BigUintTarget) -> IoResult<()> {
        self.write_usize(x.num_limbs())?;
        for limb in x.limbs.iter() {
            self.write_target_u32(*limb)?;
        }

        Ok(())
    }
}

pub trait ReadBigUint {
    fn read_target_biguint(&mut self) -> IoResult<BigUintTarget>;
}

impl ReadBigUint for Buffer<'_> {
    #[inline]
    fn read_target_biguint(&mut self) -> IoResult<BigUintTarget> {
        let length = self.read_usize()?;
        let limbs = (0..length)
            .map(|_| self.read_target_u32())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(BigUintTarget { limbs })
    }
}

#[cfg(test)]
mod tests {
    use num::{BigUint, FromPrimitive, Integer};
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::circuit_builder::CircuitBuilder;
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
    use rand::rngs::OsRng;
    use rand::Rng;

    use super::*;

    #[test]
    fn test_biguint_add() {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut rng = OsRng;

        let x_value = BigUint::from_u128(rng.gen()).unwrap();
        let y_value = BigUint::from_u128(rng.gen()).unwrap();
        let expected_z_value = &x_value + &y_value;

        let config = CircuitConfig::standard_recursion_config();
        let mut pw = PartialWitness::new();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x = builder.add_virtual_biguint_target_unsafe(x_value.to_u32_digits().len());
        let y = builder.add_virtual_biguint_target_unsafe(y_value.to_u32_digits().len());
        let z = builder.add_biguint(&x, &y);
        let expected_z =
            builder.add_virtual_biguint_target_unsafe(expected_z_value.to_u32_digits().len());
        builder.connect_biguint(&z, &expected_z);

        pw.set_biguint_target(&x, &x_value);
        pw.set_biguint_target(&y, &y_value);
        pw.set_biguint_target(&expected_z, &expected_z_value);

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();
        data.verify(proof).unwrap();
    }

    #[test]
    fn test_biguint_sub() {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut rng = OsRng;

        let mut x_value = BigUint::from_u128(rng.gen()).unwrap();
        let mut y_value = BigUint::from_u128(rng.gen()).unwrap();
        if y_value > x_value {
            (x_value, y_value) = (y_value, x_value);
        }
        let expected_z_value = &x_value - &y_value;

        let config = CircuitConfig::standard_recursion_config();
        let pw = PartialWitness::new();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x = builder.constant_biguint(&x_value);
        let y = builder.constant_biguint(&y_value);
        let z = builder.sub_biguint(&x, &y);
        let expected_z = builder.constant_biguint(&expected_z_value);

        builder.connect_biguint(&z, &expected_z);

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();
        data.verify(proof).unwrap();
    }

    #[test]
    fn test_biguint_mul() {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut rng = OsRng;

        let x_value = BigUint::from_u128(rng.gen()).unwrap();
        let y_value = BigUint::from_u128(rng.gen()).unwrap();
        let expected_z_value = &x_value * &y_value;

        let config = CircuitConfig::standard_recursion_config();
        let mut pw = PartialWitness::new();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x = builder.add_virtual_biguint_target_unsafe(x_value.to_u32_digits().len());
        let y = builder.add_virtual_biguint_target_unsafe(y_value.to_u32_digits().len());
        let z = builder.mul_biguint(&x, &y);
        let expected_z =
            builder.add_virtual_biguint_target_unsafe(expected_z_value.to_u32_digits().len());
        builder.connect_biguint(&z, &expected_z);

        pw.set_biguint_target(&x, &x_value);
        pw.set_biguint_target(&y, &y_value);
        pw.set_biguint_target(&expected_z, &expected_z_value);

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();
        data.verify(proof).unwrap();
    }

    #[test]
    fn test_biguint_cmp() {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut rng = OsRng;

        let x_value = BigUint::from_u128(rng.gen()).unwrap();
        let y_value = BigUint::from_u128(rng.gen()).unwrap();

        let config = CircuitConfig::standard_recursion_config();
        let pw = PartialWitness::new();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x = builder.constant_biguint(&x_value);
        let y = builder.constant_biguint(&y_value);
        let cmp = builder.cmp_biguint(&x, &y);
        let expected_cmp = builder.constant_bool(x_value <= y_value);

        builder.connect(cmp.target, expected_cmp.target);

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();
        data.verify(proof).unwrap();
    }

    #[test]
    fn test_biguint_div_rem() {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut rng = OsRng;

        let mut x_value = BigUint::from_u128(rng.gen()).unwrap();
        let mut y_value = BigUint::from_u128(rng.gen()).unwrap();
        if y_value > x_value {
            (x_value, y_value) = (y_value, x_value);
        }
        let (expected_div_value, expected_rem_value) = x_value.div_rem(&y_value);

        let config = CircuitConfig::standard_recursion_config();
        let pw = PartialWitness::new();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x = builder.constant_biguint(&x_value);
        let y = builder.constant_biguint(&y_value);
        let (div, rem) = builder.div_rem_biguint(&x, &y);

        let expected_div = builder.constant_biguint(&expected_div_value);
        let expected_rem = builder.constant_biguint(&expected_rem_value);

        builder.connect_biguint(&div, &expected_div);
        builder.connect_biguint(&rem, &expected_rem);

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();
        data.verify(proof).unwrap()
    }
}
