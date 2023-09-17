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
    fn define<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>;

    /// Add generators to the generator registry.
    #[allow(unused_variables)]
    fn register_generators<L: PlonkParameters<D>, const D: usize>(
        registry: &mut WitnessGeneratorRegistry<L, D>,
    ) where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
    }

    /// Add gates to the gate registry.
    #[allow(unused_variables)]
    fn register_gates<L: PlonkParameters<D>, const D: usize>(registry: &mut GateRegistry<L, D>)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
    }

    // Tests that the circuit can be serialized and deserialized.
    fn test_serialization<L: PlonkParameters<D>, const D: usize>()
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        let mut builder = CircuitBuilder::<L, D>::new();
        Self::define(&mut builder);
        let circuit = builder.build();

        let mut generator_registry = WitnessGeneratorRegistry::<L, D>::new();
        Self::register_generators(&mut generator_registry);

        let mut gate_registry = GateRegistry::<L, D>::new();
        Self::register_gates(&mut gate_registry);

        circuit.test_serializers(&gate_registry, &generator_registry);
    }
}
