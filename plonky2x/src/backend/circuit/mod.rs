mod build;
pub mod config;
mod input;
mod mock;
mod output;
mod serialization;
mod witness;

use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};

pub use self::build::CircuitBuild;
pub use self::config::{DefaultParameters, Groth16VerifierParameters, PlonkParameters};
pub use self::input::PublicInput;
pub use self::mock::MockCircuitBuild;
pub use self::output::PublicOutput;
pub use self::serialization::{GateRegistry, Serializer, WitnessGeneratorRegistry};
pub use self::witness::generate_witness;
use crate::prelude::CircuitBuilder;

pub trait Circuit {
    /// Takes in an empty builder and defines the circuit.
    fn define<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>);

    /// Add generators to the generator_registry
    #[allow(unused_variables)]
    fn add_generators<L: PlonkParameters<D>, const D: usize>(
        generator_registry: &mut WitnessGeneratorRegistry<L, D>,
    ) where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
    }

    /// Add gates to the gate_registry
    #[allow(unused_variables)]
    fn add_gates<L: PlonkParameters<D>, const D: usize>(gate_registry: &mut GateRegistry<L, D>)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
    }
}
