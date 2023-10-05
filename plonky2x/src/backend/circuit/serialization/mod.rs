pub mod gates;
pub mod hints;
pub mod registry;

pub use gates::GateRegistry;
pub use hints::HintRegistry;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
pub use registry::Serializer;

use super::{Circuit, PlonkParameters};

/// A trait that allows a type to define a custom generator and gate registry.
///
/// It is used when builder methods need access to custom serializers for recursive proofs.
pub trait CircuitSerializer {
    fn generator_registry<L: PlonkParameters<D>, const D: usize>() -> HintRegistry<L, D>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>;

    fn gate_registry<L: PlonkParameters<D>, const D: usize>() -> GateRegistry<L, D>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>;
}

/// A serializer that has the default gate registry and generator registry.
pub struct DefaultSerializer;

impl CircuitSerializer for DefaultSerializer {
    fn generator_registry<L: PlonkParameters<D>, const D: usize>() -> HintRegistry<L, D>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        let mut generator_registry = HintRegistry::new();
        generator_registry
    }

    fn gate_registry<L: PlonkParameters<D>, const D: usize>() -> GateRegistry<L, D>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        let mut gate_registry = GateRegistry::new();
        gate_registry
    }
}

/// A default implementation of `CircuitSerializer` for any `Circuit`.
impl<C: Circuit> CircuitSerializer for C {
    fn generator_registry<L: PlonkParameters<D>, const D: usize>() -> HintRegistry<L, D>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        let mut generator_registry = HintRegistry::new();
        C::register_generators::<L, D>(&mut generator_registry);
        generator_registry
    }

    fn gate_registry<L: PlonkParameters<D>, const D: usize>() -> GateRegistry<L, D>
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        let mut gate_registry = GateRegistry::new();
        C::register_gates::<L, D>(&mut gate_registry);
        gate_registry
    }
}

#[cfg(test)]
mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::gates::arithmetic_base::ArithmeticGate;
    use plonky2::gates::gate::GateRef;
    use plonky2::iop::generator::{ConstantGenerator, SimpleGenerator, WitnessGeneratorRef};
    use plonky2::util::serialization::{Buffer, GateSerializer, WitnessGeneratorSerializer};

    use crate::backend::circuit::serialization::gates::GateRegistry;
    use crate::backend::circuit::serialization::hints::HintRegistry;
    use crate::backend::circuit::DefaultParameters;
    use crate::prelude::CircuitBuilder;

    type L = DefaultParameters;
    type F = GoldilocksField;
    const D: usize = 2;

    #[test]
    fn test_witness_serialization() {
        let builder = CircuitBuilder::<L, D>::new();
        let common_data = builder.build().data.common;

        let registry = HintRegistry::<L, D>::new();
        let raw_generator = WitnessGeneratorRef::new(ConstantGenerator::<F>::default().adapter());

        let mut bytes = Vec::<u8>::new();
        registry
            .write_generator(&mut bytes, &raw_generator, &common_data)
            .unwrap();

        let mut buffer = Buffer::new(&bytes);

        let read_generator = registry.read_generator(&mut buffer, &common_data).unwrap();
        assert_eq!(raw_generator, read_generator);
    }

    #[test]
    fn test_gate_serialization() {
        let builder = CircuitBuilder::<L, D>::new();
        let common_data = builder.build().data.common;

        let registry = GateRegistry::<L, D>::new();

        let raw_gate: GateRef<F, D> =
            GateRef::new(ArithmeticGate::new_from_config(&common_data.config));

        let mut bytes = Vec::<u8>::new();
        registry
            .write_gate(&mut bytes, &raw_gate, &common_data)
            .unwrap();

        let mut buffer = Buffer::new(&bytes);
        let read_gate = registry.read_gate(&mut buffer, &common_data).unwrap();

        assert_eq!(raw_gate, read_gate);
    }
}
