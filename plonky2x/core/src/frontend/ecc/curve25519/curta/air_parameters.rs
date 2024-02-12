use core::marker::PhantomData;

use serde::{Deserialize, Serialize};
use starkyx::chip::ec::edwards::ed25519::instruction::Ed25519FpInstruction;
use starkyx::chip::AirParameters;

use crate::prelude::PlonkParameters;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ed25519AirParameters<L, const D: usize>(PhantomData<L>);

impl<L: PlonkParameters<D>, const D: usize> AirParameters for Ed25519AirParameters<L, D> {
    type Field = L::Field;
    type CubicParams = L::CubicParams;

    type Instruction = Ed25519FpInstruction;

    const NUM_ARITHMETIC_COLUMNS: usize = 1632;
    const NUM_FREE_COLUMNS: usize = 20;
    const EXTENDED_COLUMNS: usize = 2502;
}
