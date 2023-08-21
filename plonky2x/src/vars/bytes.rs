use array_macro::array;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::CircuitVariable;
use crate::builder::CircuitBuilder;
use crate::ops::{BitAnd, BitOr, BitXor, Not, RotateLeft, RotateRight, Shl, Shr, Zero};
use crate::vars::ByteVariable;

/// A variable in the circuit representing a byte value.
#[derive(Debug, Clone, Copy)]
pub struct BytesVariable<const N: usize>(pub [ByteVariable; N]);

impl<const N: usize> CircuitVariable for BytesVariable<N> {
    type ValueType<F> = [u8; N];

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(array![_ => ByteVariable::init(builder); N])
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        assert!(
            value.len() == N,
            "vector of values has wrong length: expected {} got {}",
            N,
            value.len()
        );
        Self(array![i => ByteVariable::constant(builder, value[i]); N])
    }

    fn targets(&self) -> Vec<Target> {
        self.0.iter().flat_map(|b| b.targets()).collect()
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        self.0.map(|b| b.value(witness))
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        assert!(
            value.len() == N,
            "vector of values has wrong length: expected {} got {}",
            N,
            value.len()
        );
        for (b, v) in self.0.iter().zip(value) {
            b.set(witness, v);
        }
    }
}

impl<F: RichField + Extendable<D>, const D: usize, const N: usize> Not<F, D> for BytesVariable<N> {
    type Output = Self;

    fn not(self, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        BytesVariable(self.0.map(|x| builder.not(x)))
    }
}

impl<F: RichField + Extendable<D>, const D: usize, const N: usize> Zero<F, D> for BytesVariable<N> {
    fn zero(builder: &mut CircuitBuilder<F, D>) -> Self {
        builder.constant([0u8; N])
    }
}

impl<F: RichField + Extendable<D>, const D: usize, const N: usize> BitAnd<F, D>
    for BytesVariable<N>
{
    type Output = Self;

    fn bitand(self, rhs: Self, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let byte_fn = |i| builder.and(self.0[i], rhs.0[i]);
        BytesVariable(core::array::from_fn(byte_fn))
    }
}

impl<F: RichField + Extendable<D>, const D: usize, const N: usize> BitOr<F, D>
    for BytesVariable<N>
{
    type Output = Self;

    fn bitor(self, rhs: Self, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let byte_fn = |i| builder.or(self.0[i], rhs.0[i]);
        BytesVariable(core::array::from_fn(byte_fn))
    }
}

impl<F: RichField + Extendable<D>, const D: usize, const N: usize> BitXor<F, D>
    for BytesVariable<N>
{
    type Output = Self;

    fn bitxor(self, rhs: Self, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let byte_fn = |i| builder.xor(self.0[i], rhs.0[i]);
        BytesVariable(core::array::from_fn(byte_fn))
    }
}

impl<F: RichField + Extendable<D>, const D: usize, const N: usize> Shr<F, D, usize>
    for BytesVariable<N>
{
    type Output = Self;

    fn shr(self, rhs: usize, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        assert!(
            rhs < 8 * N,
            "shift amount is too large, must be less than {}",
            8 * N
        );
        let self_bits = self
            .0
            .iter()
            .flat_map(|x| x.to_be_bits())
            .collect::<Vec<_>>();
        let shr_bit = |i| {
            if i < rhs {
                builder.constant(false)
            } else {
                self_bits[i - rhs]
            }
        };
        let shr_bits = (0..8 * N).map(shr_bit).collect::<Vec<_>>();

        BytesVariable(
            shr_bits
                .chunks_exact(8)
                .map(|chunk| ByteVariable(chunk.try_into().unwrap()))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl<F: RichField + Extendable<D>, const D: usize, const N: usize> Shl<F, D, usize>
    for BytesVariable<N>
{
    type Output = Self;

    fn shl(self, rhs: usize, builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        assert!(
            rhs < 8 * N,
            "shift amount is too large, must be less than {}",
            8 * N
        );
        let self_bits = self
            .0
            .iter()
            .flat_map(|x| x.to_be_bits())
            .collect::<Vec<_>>();
        let shl_bit = |i| {
            if i + rhs > 8 * N - 1 {
                builder.constant(false)
            } else {
                self_bits[i + rhs]
            }
        };
        let shl_bits = (0..8 * N).map(shl_bit).collect::<Vec<_>>();

        BytesVariable(
            shl_bits
                .chunks_exact(8)
                .map(|chunk| ByteVariable(chunk.try_into().unwrap()))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl<F: RichField + Extendable<D>, const D: usize, const N: usize> RotateLeft<F, D, usize>
    for BytesVariable<N>
{
    type Output = Self;

    fn rotate_left(self, rhs: usize, _builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let self_bits = self
            .0
            .iter()
            .flat_map(|x| x.to_be_bits())
            .collect::<Vec<_>>();
        let rot_bit = |i| self_bits[(i + rhs) % (8 * N)];
        let rot_bits = (0..8 * N).map(rot_bit).collect::<Vec<_>>();

        BytesVariable(
            rot_bits
                .chunks_exact(8)
                .map(|chunk| ByteVariable(chunk.try_into().unwrap()))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl<F: RichField + Extendable<D>, const D: usize, const N: usize> RotateRight<F, D, usize>
    for BytesVariable<N>
{
    type Output = Self;

    fn rotate_right(self, rhs: usize, _builder: &mut CircuitBuilder<F, D>) -> Self::Output {
        let self_bits = self
            .0
            .iter()
            .flat_map(|x| x.to_be_bits())
            .collect::<Vec<_>>();
        let rot_bit = |i| self_bits[(i + 8 * N - rhs) % (8 * N)];
        let rot_bits = (0..8 * N).map(rot_bit).collect::<Vec<_>>();

        BytesVariable(
            rot_bits
                .chunks_exact(8)
                .map(|chunk| ByteVariable(chunk.try_into().unwrap()))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use crate::prelude::*;

    #[test]
    fn test_bytes_operations() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let num_tests = 32;

        let mut x_vec = Vec::new();
        let mut y_vec = Vec::new();

        let mut x_and_y_vec = Vec::new();
        let mut x_or_y_vec = Vec::new();
        let mut x_xor_y_vec = Vec::new();
        let mut x_shr_vec = Vec::new();
        let mut x_shl_vec = Vec::new();
        let mut x_rotl_vec = Vec::new();
        let mut x_rotr_vec = Vec::new();

        for i in 0..num_tests {
            let x = builder.init::<BytesVariable<4>>();
            let y = builder.init::<BytesVariable<4>>();
            x_vec.push(x);
            y_vec.push(y);

            let x_and_y = builder.and(x, y);
            x_and_y_vec.push(x_and_y);

            let x_or_y = builder.or(x, y);
            x_or_y_vec.push(x_or_y);

            let x_xor_y = builder.xor(x, y);
            x_xor_y_vec.push(x_xor_y);

            let x_shr = builder.shr(x, i);
            x_shr_vec.push(x_shr);

            let x_shl = builder.shl(x, i);
            x_shl_vec.push(x_shl);

            let x_rotl = builder.rotate_left(x, i);
            x_rotl_vec.push(x_rotl);

            let x_rotr = builder.rotate_right(x, i);
            x_rotr_vec.push(x_rotr);
        }

        let data = builder.build::<C>();
        let mut pw = PartialWitness::new();

        let mut rng = thread_rng();
        for (i, ((((((((x, y), x_and_y), x_or_y), x_xor_y), x_shr), x_shl), x_rotl), x_rotr)) in
            x_vec
                .iter()
                .zip(y_vec.iter())
                .zip(x_and_y_vec.iter())
                .zip(x_or_y_vec)
                .zip(x_xor_y_vec)
                .zip(x_shr_vec)
                .zip(x_shl_vec)
                .zip(x_rotl_vec)
                .zip(x_rotr_vec)
                .enumerate()
        {
            let x_val = rng.gen::<u32>();
            let y_val = rng.gen::<u32>();

            x.set(&mut pw, x_val.to_be_bytes());
            y.set(&mut pw, y_val.to_be_bytes());

            x_and_y.set(&mut pw, (x_val & y_val).to_be_bytes());
            x_or_y.set(&mut pw, (x_val | y_val).to_be_bytes());
            x_xor_y.set(&mut pw, (x_val ^ y_val).to_be_bytes());
            x_shr.set(&mut pw, (x_val >> i).to_be_bytes());
            x_shl.set(&mut pw, (x_val << i).to_be_bytes());
            x_rotl.set(&mut pw, (x_val.rotate_left(i as u32)).to_be_bytes());
            x_rotr.set(&mut pw, (x_val.rotate_right(i as u32)).to_be_bytes());
        }

        let proof = data.prove(pw).unwrap();
        data.verify(proof).unwrap();
    }
}
