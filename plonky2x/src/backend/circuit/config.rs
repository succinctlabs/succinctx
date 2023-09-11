use core::fmt::Debug;

use curta::math::goldilocks::cubic::GoldilocksCubicParameters;
use curta::math::prelude::CubicParameters;
use plonky2::field::extension::Extendable;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use serde::{Deserialize, Serialize};

use crate::backend::wrapper::plonky2_config::PoseidonBN128GoldilocksConfig;

/// Parameters such as the field, hash function, etc. used for the circuit.
pub trait PlonkParameters<const D: usize>:
    Debug + Clone + PartialEq + Sync + Send + 'static
{
    type Field: RichField + Extendable<D>;

    type Config: GenericConfig<D, F = Self::Field, FE = <Self::Field as Extendable<D>>::Extension>
        + 'static;

    type CubicParams: CubicParameters<Self::Field>;
}

/// Default parameters for the circuit. Uses the `PoseidonGoldilocksConfig` in Plonky2.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefaultParameters;

impl PlonkParameters<2> for DefaultParameters {
    type Field = GoldilocksField;

    type CubicParams = GoldilocksCubicParameters;

    type Config = PoseidonGoldilocksConfig;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Groth16VerifierParameters;

impl PlonkParameters<2> for Groth16VerifierParameters {
    type Field = GoldilocksField;

    type CubicParams = GoldilocksCubicParameters;

    type Config = PoseidonBN128GoldilocksConfig;
}
