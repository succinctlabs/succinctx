mod build;
pub mod config;
mod input;
mod mock;
mod output;
mod serialization;
mod witness;

use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};

pub use self::build::CircuitBuild;
pub use self::config::{DefaultParameters, PlonkParameters};
pub use self::input::PublicInput;
pub use self::mock::MockCircuitBuild;
pub use self::output::PublicOutput;
pub use self::serialization::{GateRegistry, Serializer, WitnessGeneratorRegistry};
use crate::prelude::CircuitBuilder;

pub trait Circuit {
    /// Takes in an empty builder and defines the circuit.
    fn define<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>);

    /// Generates the witness registry.
    fn generators<L: PlonkParameters<D>, const D: usize>() -> WitnessGeneratorRegistry<L, D>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        WitnessGeneratorRegistry::<L, D>::new()
    }

    /// Geneates the gate registry.
    fn gates<L: PlonkParameters<D>, const D: usize>() -> GateRegistry<L, D>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        GateRegistry::<L, D>::new()
    }
}
