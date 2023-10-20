use serde::{Deserialize, Serialize};

use crate::frontend::ecc::ed25519::curve::curve_types::{AffinePoint, Curve};

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EDDSAPublicKey<C: Curve>(pub AffinePoint<C>);

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EDDSASignature<C: Curve> {
    pub r: AffinePoint<C>,
    pub s: C::ScalarField,
}
