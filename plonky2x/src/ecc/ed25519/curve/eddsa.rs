use num::BigUint;
use plonky2::field::types::Field;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};

use crate::ecc::ed25519::curve::curve_types::{AffinePoint, Curve, CurveScalar};
use crate::ecc::ed25519::curve::ed25519::Ed25519;
use crate::ecc::ed25519::field::ed25519_scalar::Ed25519Scalar;

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EDDSAPublicKey<C: Curve>(pub AffinePoint<C>);

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EDDSASignature<C: Curve> {
    pub r: AffinePoint<C>,
    pub s: C::ScalarField,
}

pub fn verify_message(
    msg: &Vec<bool>,
    sig: &EDDSASignature<Ed25519>,
    pk: &EDDSAPublicKey<Ed25519>,
) -> bool {
    let mut hash_bytes = Vec::new();

    let mut r_bytes = sig.r.compress_point().to_bytes_le();
    // Need to pad it to 32 bytes
    for _i in r_bytes.len()..32 {
        r_bytes.push(0);
    }
    hash_bytes.extend_from_slice(r_bytes.as_slice());

    let mut pk_bytes = pk.0.compress_point().to_bytes_le();
    // Need to pad it to 32 bytes
    for _i in pk_bytes.len()..32 {
        pk_bytes.push(0);
    }
    hash_bytes.extend_from_slice(pk_bytes.as_slice());

    assert!(msg.len() % 8 == 0);
    for chunk in msg.chunks(8) {
        let mut byte_val = 0_u8;
        for i in 0..8 {
            if chunk[i] {
                byte_val += 1 << (7 - i);
            }
        }
        hash_bytes.push(byte_val);
    }

    let mut hasher = Sha512::new();
    hasher.update(hash_bytes.as_slice());
    let hash = hasher.finalize();
    let h_big_int = BigUint::from_bytes_le(hash.as_slice());

    let h_mod_25519 = h_big_int % Ed25519Scalar::order();
    let h = Ed25519Scalar::from_noncanonical_biguint(h_mod_25519);

    assert!(pk.0.is_valid());
    let s_g = (CurveScalar(sig.s) * Ed25519::GENERATOR_PROJECTIVE).to_affine();
    let h_pk = (CurveScalar(h) * pk.0.to_projective()).to_affine();
    let rhs = sig.r + h_pk;

    s_g == rhs
}

#[cfg(test)]
mod tests {
    use num::BigUint;
    use plonky2::field::types::Field;

    use crate::ecc::ed25519::curve::curve_types::AffinePoint;
    use crate::ecc::ed25519::curve::eddsa::{verify_message, EDDSAPublicKey, EDDSASignature};
    use crate::ecc::ed25519::field::ed25519_scalar::Ed25519Scalar;

    fn to_bits(msg: Vec<u8>) -> Vec<bool> {
        let mut res = Vec::new();
        for i in 0..msg.len() {
            let char = msg[i];
            for j in 0..8 {
                if (char & (1 << (7 - j))) != 0 {
                    res.push(true);
                } else {
                    res.push(false);
                }
            }
        }
        res
    }

    #[test]
    fn test_eddsa() {
        let message = [1, 164, 81, 146, 119, 87, 120, 84, 45, 84, 206, 199, 171, 245, 50, 223, 18, 145, 16, 20, 30, 74, 39, 118, 236, 132, 187, 1, 187, 203, 3, 182, 59, 16, 197, 8, 0, 235, 7, 0, 0, 0, 0, 0, 0, 25, 2, 0, 0, 0, 0, 0, 0];
        let pubkey_bytes = [43, 167, 192, 11, 252, 193, 43, 86, 163, 6, 196, 30, 196, 76, 65, 16, 66, 208, 184, 55, 164, 13, 128, 252, 101, 47, 165, 140, 207, 183, 134, 0];
        let signature = [181, 147, 15, 125, 55, 28, 34, 104, 182, 165, 82, 204, 204, 73, 16, 207, 185, 157, 77, 145, 128, 9, 51, 132, 54, 115, 29, 172, 162, 95, 181, 176, 47, 25, 165, 27, 174, 193, 83, 51, 85, 17, 162, 57, 133, 169, 77, 68, 160, 216, 58, 230, 14, 128, 149, 202, 53, 8, 232, 253, 28, 251, 207, 6];

        let msg_bits = to_bits(message.to_vec());

        let sig_r = AffinePoint::new_from_compressed_point(&signature[0..32]);
        assert!(sig_r.is_valid());

        let sig_s_biguint = BigUint::from_bytes_le(&signature[32..64]);
        let sig_s = Ed25519Scalar::from_noncanonical_biguint(sig_s_biguint);
        let sig = EDDSASignature { r: sig_r, s: sig_s };

        let pub_key = AffinePoint::new_from_compressed_point(&pubkey_bytes[..]);
        assert!(pub_key.is_valid());

        let result = verify_message(&msg_bits, &sig, &EDDSAPublicKey(pub_key));

        assert!(result);
    }

}
