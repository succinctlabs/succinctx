use core::fmt::Debug;

use curta::math::goldilocks::cubic::GoldilocksCubicParameters;
use curta::math::prelude::CubicParameters;
use plonky2::field::extension::Extendable;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

pub trait PlonkParameters<const D: usize>:
    Debug + Clone + PartialEq + Sync + Send + 'static
{
    type Field: RichField + Extendable<D>;

    type Config: GenericConfig<D, F = Self::Field, FE = <Self::Field as Extendable<D>>::Extension>
        + 'static;

    type CubicParams: CubicParameters<Self::Field>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefaultParameters;

impl PlonkParameters<2> for DefaultParameters {
    type Field = GoldilocksField;

    type CubicParams = GoldilocksCubicParameters;

    type Config = PoseidonGoldilocksConfig;
}
