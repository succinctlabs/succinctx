pub mod gates;
pub mod hints;
pub mod registry;

pub use gates::GateRegistry;
pub use hints::HintRegistry;
pub use registry::Serializer;

use super::PlonkParameters;

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
