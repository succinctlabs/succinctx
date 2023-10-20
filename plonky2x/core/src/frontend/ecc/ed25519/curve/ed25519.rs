use plonky2::field::types::Field;
use serde::{Deserialize, Serialize};

use crate::frontend::ecc::ed25519::curve::curve_types::{AffinePoint, Curve};
use crate::frontend::ecc::ed25519::field::ed25519_base::Ed25519Base;
use crate::frontend::ecc::ed25519::field::ed25519_scalar::Ed25519Scalar;

#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Ed25519;

// Curve parameters can be found here:  https://medium.com/asecuritysite-when-bob-met-alice/whats-the-ed-in-ed25519-f980c822c263
impl Curve for Ed25519 {
    type BaseField = Ed25519Base;
    type ScalarField = Ed25519Scalar;

    const A: Ed25519Base = Ed25519Base::NEG_ONE;
    const D: Ed25519Base = Ed25519Base([
        0x75EB4DCA135978A3,
        0x00700A4D4141D8AB,
        0x8CC740797779E898,
        0x52036CEE2B6FFE73,
    ]);
    const GENERATOR_AFFINE: AffinePoint<Self> = AffinePoint {
        x: ED25519_GENERATOR_X,
        y: ED25519_GENERATOR_Y,
        zero: false,
    };
}

// Base point described here: https://crypto.stackexchange.com/questions/27392/base-point-in-ed25519
/// 15112221349535400772501151409588531511454012693041857206046113283949847762202
const ED25519_GENERATOR_X: Ed25519Base = Ed25519Base([
    0xC9562D608F25D51A,
    0x692CC7609525A7B2,
    0xC0A4E231FDD6DC5C,
    0x216936D3CD6E53FE,
]);

/// 46316835694926478169428394003475163141307993866256225615783033603165251855960
const ED25519_GENERATOR_Y: Ed25519Base = Ed25519Base([
    0x6666666666666658,
    0x6666666666666666,
    0x6666666666666666,
    0x6666666666666666,
]);
