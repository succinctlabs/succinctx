use core::fmt::Debug;

use plonky2::field::extension::Extendable;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use serde::{Deserialize, Serialize};
use starkyx::math::goldilocks::cubic::GoldilocksCubicParameters;
use starkyx::math::prelude::CubicParameters;
use starkyx::plonky2::stark::config::{CurtaConfig, CurtaPoseidonGoldilocksConfig};

use crate::backend::wrapper::plonky2_config::PoseidonBN128GoldilocksConfig;

/// Parameters such as the field, hash function, etc. used for the circuit.
pub trait PlonkParameters<const D: usize>:
    Debug + Clone + PartialEq + Sync + Send + 'static
{
    type Field: RichField + Extendable<D>;

    type Config: GenericConfig<D, F = Self::Field, FE = <Self::Field as Extendable<D>>::Extension>
        + 'static;

    type CurtaConfig: CurtaConfig<
        D,
        F = Self::Field,
        FE = <Self::Field as Extendable<D>>::Extension,
    >;

    type CubicParams: CubicParameters<Self::Field>;
}

/// Default parameters for the circuit. Uses the `PoseidonGoldilocksConfig` in Plonky2.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefaultParameters;

impl PlonkParameters<2> for DefaultParameters {
    type Field = GoldilocksField;

    type CubicParams = GoldilocksCubicParameters;

    type Config = PoseidonGoldilocksConfig;

    type CurtaConfig = CurtaPoseidonGoldilocksConfig;
}

/// Wrapper parameters for the circuit. Uses the `PoseidonBN128GoldilocksConfig` in Plonky2.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Groth16WrapperParameters;

impl PlonkParameters<2> for Groth16WrapperParameters {
    type Field = GoldilocksField;

    type CubicParams = GoldilocksCubicParameters;

    type Config = PoseidonBN128GoldilocksConfig;

    type CurtaConfig = CurtaPoseidonGoldilocksConfig;
}
