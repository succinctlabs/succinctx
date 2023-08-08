use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use itertools::Itertools;
use num::bigint::BigUint;
use num::{Integer, One};
use plonky2::field::types::{Field, PrimeField, Sample};
use serde::{Deserialize, Serialize};

/// The base field of the curve25519 elliptic curve.
///
/// Its order is
/// ```ignore
/// P = 2**255 - 19
/// ```
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Ed25519Base(pub [u64; 4]);

fn biguint_from_array(arr: [u64; 4]) -> BigUint {
    BigUint::from_slice(&[
        arr[0] as u32,
        (arr[0] >> 32) as u32,
        arr[1] as u32,
        (arr[1] >> 32) as u32,
        arr[2] as u32,
        (arr[2] >> 32) as u32,
        arr[3] as u32,
        (arr[3] >> 32) as u32,
    ])
}

// impl Ed25519Base {
//     // untested for correctness
//     pub fn exponentiate(&self, exponent: BigUint) -> Self {
//         let s = self.to_canonical_biguint();
//         let p = BigUint::parse_bytes(b"57896044618658097711785492504343953926634992332820282019728792003956564819949", 10).unwrap();
//         Ed25519Base::from_noncanonical_biguint(s.modpow(&exponent, &p))
//     }

//     // untested for correctness
//     // inspired from curve25519-dalek implementation here https://github.com/dalek-cryptography/curve25519-dalek/blob/main/src/field.rs#L252
//     pub fn sqrt_ratio(u: Ed25519Base, v: Ed25519Base) -> (bool, Ed25519Base) {
//         let p = BigUint::parse_bytes(b"57896044618658097711785492504343953926634992332820282019728792003956564819949", 10).unwrap();
//         let five = BigUint::from_i32(5).unwrap();
//         let eight = BigUint::from_i32(8).unwrap();

//         let v3 = v.clone().square() * v;
//         let v7 = v3.clone().square() * v;

//         let r = (u * v3) * (u * v7).exponentiate((p-five)/eight);
//         let check = v * r.square();

//         let i = Ed25519Base::NEG_ONE;

//         let correct_sign = check == u;
//         let flipped_sign = check == -u;
//         let flipped_sign_i = check == -u * i;

//         let r_prime = r * i;
//         let r = if flipped_sign || flipped_sign_i { r_prime } else { r };

//         // is sign stored in idx 0 or 31?
//         let was_correct_sign = correct_sign || flipped_sign;
//         let r = if r.to_canonical_biguint().to_bytes_le()[31] == 1 & 1 { r * i } else { r };
//         (was_correct_sign, r)
//     }

//     // the following code does not work correctly :(
//     // untested for correctness
//     // pub fn sig_sqrt(u: Ed25519Base, v: Ed25519Base, x_0: bool) -> Option<Ed25519Base> {
//     //     let p = BigUint::parse_bytes(b"57896044618658097711785492504343953926634992332820282019728792003956564819949", 10).unwrap();
//     //     let one = BigUint::one();
//     //     let three = BigUint::from_i32(3).unwrap();
//     //     let four = BigUint::from_i32(4).unwrap();
//     //     let five = BigUint::from_i32(5).unwrap();
//     //     let eight = BigUint::from_i32(8).unwrap();
//     //     // let x_candidate= (u/v).exponentiate((p.clone()+three)/eight);
//     //     let x_candidate = (u * v.exponentiate(three)) * (u * v.exponentiate((p.clone()-five)/eight));
//     //     let neg_u = Ed25519Base::NEG_ONE * u;
//     //     let testor = v * x_candidate.square();
//     //     let x = if testor == u {
//     //         Some(x_candidate)
//     //     } else if testor == neg_u {
//     //         Some(x_candidate * Ed25519Base::TWO.exponentiate((p.clone()-one)/four))
//     //     } else {
//     //         None
//     //     };
//     //     match x {
//     //         Some(x) => {
//     //             let odd = x.to_canonical_biguint() % BigUint::from_i32(2).unwrap() == BigUint::one();
//     //             if x_0 && x == Ed25519Base::ZERO {
//     //                 None
//     //             } else if x_0 != odd {
//     //                 Some(Ed25519Base::from_noncanonical_biguint(p) - x)
//     //             } else {
//     //                 None
//     //             }
//     //         },
//     //         None => None
//     //     }
//     // }
//     // pub fn exponentiate(&self, exponent: BigUint) -> BigUint {
//     //     let s = self.to_canonical_biguint();
//     //     let p = BigUint::parse_bytes(b"57896044618658097711785492504343953926634992332820282019728792003956564819949", 10).unwrap();
//     //     s.modpow(&exponent, &p)
//     // }
//     // pub fn sig_sqrt(u: Ed25519Base, v: Ed25519Base, x_0: bool) -> Option<Ed25519Base> {
//     //     let one = BigUint::one();
//     //     let three = BigUint::from_i32(3).unwrap();
//     //     let four = BigUint::from_i32(4).unwrap();
//     //     let eight = BigUint::from_i32(8).unwrap();
//     //     let p = BigUint::parse_bytes(b"57896044618658097711785492504343953926634992332820282019728792003956564819949", 10).unwrap();
//     //     let u_biguint = u.to_canonical_biguint();
//     //     let v_biguint = v.to_canonical_biguint();
//     //     let x_candidate= (u/v).exponentiate((p.clone()+three)/eight);
//     //     let neg_u_biguint =
//     // }
// }

impl Default for Ed25519Base {
    fn default() -> Self {
        Self::ZERO
    }
}

impl PartialEq for Ed25519Base {
    fn eq(&self, other: &Self) -> bool {
        self.to_canonical_biguint() == other.to_canonical_biguint()
    }
}

impl Eq for Ed25519Base {}

impl Hash for Ed25519Base {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_canonical_biguint().hash(state)
    }
}

impl Display for Ed25519Base {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.to_canonical_biguint(), f)
    }
}

impl Debug for Ed25519Base {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.to_canonical_biguint(), f)
    }
}

impl Sample for Ed25519Base {
    #[inline]
    fn sample<R>(rng: &mut R) -> Self
    where
        R: rand::RngCore + ?Sized,
    {
        use num::bigint::RandBigInt;
        Self::from_noncanonical_biguint(rng.gen_biguint_below(&Self::order()))
    }
}

impl Field for Ed25519Base {
    const ZERO: Self = Self([0; 4]);
    const ONE: Self = Self([1, 0, 0, 0]);
    const TWO: Self = Self([2, 0, 0, 0]);
    const NEG_ONE: Self = Self([
        0xFFFFFFFFFFFFFFEC,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0x7FFFFFFFFFFFFFFF,
    ]);

    const TWO_ADICITY: usize = 1;
    const CHARACTERISTIC_TWO_ADICITY: usize = Self::TWO_ADICITY;

    // Sage: `g = GF(p).multiplicative_generator()`
    const MULTIPLICATIVE_GROUP_GENERATOR: Self = Self([2, 0, 0, 0]);

    // Sage: `g_2 = g^((p - 1) / 2)`
    const POWER_OF_TWO_GENERATOR: Self = Self::NEG_ONE;

    const BITS: usize = 256;

    fn order() -> BigUint {
        BigUint::from_slice(&[
            0xFFFFFFED, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
            0x7FFFFFFF,
        ])
    }
    fn characteristic() -> BigUint {
        Self::order()
    }

    fn try_inverse(&self) -> Option<Self> {
        if self.is_zero() {
            return None;
        }

        // Fermat's Little Theorem
        Some(self.exp_biguint(&(Self::order() - BigUint::one() - BigUint::one())))
    }

    fn from_noncanonical_biguint(val: BigUint) -> Self {
        Self(
            val.to_u64_digits()
                .into_iter()
                .pad_using(4, |_| 0)
                .collect::<Vec<_>>()[..]
                .try_into()
                .expect("error converting to u64 array"),
        )
    }

    #[inline]
    fn from_canonical_u64(n: u64) -> Self {
        Self([n, 0, 0, 0])
    }

    #[inline]
    fn from_noncanonical_u64(n: u64) -> Self {
        Self::from_canonical_u64(n)
    }

    #[inline]
    fn from_noncanonical_i64(n: i64) -> Self {
        if n >= 0 {
            Self::from_canonical_u64(n as u64)
        } else {
            Self::from_canonical_u64((-n) as u64).neg()
        }
    }

    #[inline]
    fn from_noncanonical_u128(n: u128) -> Self {
        Self([n as u64, (n >> 64) as u64, 0, 0])
    }

    #[inline]
    fn from_noncanonical_u96(n: (u64, u32)) -> Self {
        Self([n.0, n.1 as u64, 0, 0])
    }
}

impl PrimeField for Ed25519Base {
    fn to_canonical_biguint(&self) -> BigUint {
        let mut result = biguint_from_array(self.0);
        if result >= Self::order() {
            result -= Self::order();
        }
        result
    }
}

impl Neg for Ed25519Base {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        if self.is_zero() {
            Self::ZERO
        } else {
            Self::from_noncanonical_biguint(Self::order() - self.to_canonical_biguint())
        }
    }
}

impl Add for Ed25519Base {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        let mut result = self.to_canonical_biguint() + rhs.to_canonical_biguint();
        if result >= Self::order() {
            result -= Self::order();
        }
        Self::from_noncanonical_biguint(result)
    }
}

impl AddAssign for Ed25519Base {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sum for Ed25519Base {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, x| acc + x)
    }
}

impl Sub for Ed25519Base {
    type Output = Self;

    #[inline]
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

impl SubAssign for Ed25519Base {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Ed25519Base {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self::from_noncanonical_biguint(
            (self.to_canonical_biguint() * rhs.to_canonical_biguint()).mod_floor(&Self::order()),
        )
    }
}

impl MulAssign for Ed25519Base {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Product for Ed25519Base {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, x| acc * x).unwrap_or(Self::ONE)
    }
}

impl Div for Ed25519Base {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl DivAssign for Ed25519Base {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::test_field_arithmetic;

    test_field_arithmetic!(crate::ecc::ed25519::field::ed25519_base::Ed25519Base);
}
