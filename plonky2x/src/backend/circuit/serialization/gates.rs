use core::any::TypeId;
use core::fmt::Debug;
use core::marker::PhantomData;

use curta::plonky2::cubic::arithmetic_gate::ArithmeticCubicGate;
use curta::plonky2::cubic::mul_gate::MulCubicGate;
use plonky2::field::extension::Extendable;
use plonky2::gates::arithmetic_base::ArithmeticGate;
use plonky2::gates::arithmetic_extension::ArithmeticExtensionGate;
use plonky2::gates::base_sum::BaseSumGate;
use plonky2::gates::constant::ConstantGate;
use plonky2::gates::coset_interpolation::CosetInterpolationGate;
use plonky2::gates::exponentiation::ExponentiationGate;
use plonky2::gates::gate::{AnyGate, Gate, GateRef};
use plonky2::gates::lookup::LookupGate;
use plonky2::gates::lookup_table::LookupTableGate;
use plonky2::gates::multiplication_extension::MulExtensionGate;
use plonky2::gates::noop::NoopGate;
use plonky2::gates::poseidon::PoseidonGate;
use plonky2::gates::poseidon_mds::PoseidonMdsGate;
use plonky2::gates::public_input::PublicInputGate;
use plonky2::gates::random_access::RandomAccessGate;
use plonky2::gates::reducing::ReducingGate;
use plonky2::gates::reducing_extension::ReducingExtensionGate;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, GateSerializer, IoResult, Read, Write};

use super::registry::{SerializationRegistry, Serializer};
use super::PlonkParameters;
use crate::frontend::hash::bit_operations::XOR3Gate;
use crate::frontend::num::u32::gates::add_many_u32::U32AddManyGate;
use crate::frontend::num::u32::gates::arithmetic_u32::U32ArithmeticGate;
use crate::frontend::num::u32::gates::comparison::ComparisonGate;
use crate::frontend::num::u32::gates::range_check_u32::U32RangeCheckGate;
use crate::frontend::num::u32::gates::subtraction_u32::U32SubtractionGate;

/// A registry to store serializers for gates.
///
/// New gates can be added to the registry by calling the `register` method.
#[derive(Debug)]
pub struct GateRegistry<L: PlonkParameters<D>, const D: usize>(
    SerializationRegistry<TypeId, L::Field, GateRef<L::Field, D>, D>,
);

/// A serializer for a specific gate type.
#[derive(Clone)]
pub struct GateSerializerFn<G>(PhantomData<G>);

impl<F: RichField + Extendable<D>, G: AnyGate<F, D>, const D: usize> Serializer<F, GateRef<F, D>, D>
    for GateSerializerFn<G>
{
    fn read(
        &self,
        buf: &mut Buffer,
        common_data: &CommonCircuitData<F, D>,
    ) -> IoResult<GateRef<F, D>> {
        let gate: IoResult<G> = Gate::<F, D>::deserialize(buf, common_data);
        gate.map(|g| GateRef::<F, D>::new(g))
    }

    fn write(
        &self,
        buf: &mut Vec<u8>,
        object: &GateRef<F, D>,
        common_data: &CommonCircuitData<F, D>,
    ) -> IoResult<()> {
        object.0.serialize(buf, common_data)
    }
}

impl<L: PlonkParameters<D>, const D: usize> GateRegistry<L, D> {
    /// Registers a new gate.
    pub fn register<G: AnyGate<L::Field, D>>(&mut self) {
        let type_id = TypeId::of::<G>();
        self.0
            .register(type_id, GateSerializerFn::<G>(PhantomData))
            .unwrap();
    }

    /// Creates a new registry with all the default gates that are used in a Plonky2x circuit.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut r = Self(SerializationRegistry::new());

        r.register::<ArithmeticGate>();
        r.register::<ArithmeticExtensionGate<D>>();
        r.register::<BaseSumGate<2>>();
        r.register::<ConstantGate>();
        r.register::<CosetInterpolationGate<L::Field, D>>();
        r.register::<ExponentiationGate<L::Field, D>>();
        r.register::<LookupGate>();
        r.register::<LookupTableGate>();
        r.register::<MulExtensionGate<D>>();
        r.register::<NoopGate>();
        r.register::<PoseidonMdsGate<L::Field, D>>();
        r.register::<PoseidonGate<L::Field, D>>();
        r.register::<PublicInputGate>();
        r.register::<RandomAccessGate<L::Field, D>>();
        r.register::<ReducingExtensionGate<D>>();
        r.register::<ReducingGate<D>>();
        r.register::<XOR3Gate>();
        r.register::<ComparisonGate<L::Field, D>>();
        r.register::<U32AddManyGate<L::Field, D>>();
        r.register::<U32ArithmeticGate<L::Field, D>>();
        r.register::<U32SubtractionGate<L::Field, D>>();
        r.register::<U32RangeCheckGate<L::Field, D>>();
        r.register::<ArithmeticCubicGate>();
        r.register::<MulCubicGate>();

        r
    }
}

impl<L: PlonkParameters<D>, const D: usize> GateSerializer<L::Field, D> for GateRegistry<L, D> {
    fn read_gate(
        &self,
        buf: &mut Buffer,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<GateRef<L::Field, D>> {
        let idx = buf.read_usize()?;
        let type_id = self.0.identifiers[idx];

        self.0
            .registry
            .get(&type_id)
            .unwrap_or_else(|| panic!("Gate type not registered {:?}", type_id))
            .read(buf, common_data)
    }

    fn write_gate(
        &self,
        buf: &mut Vec<u8>,
        gate: &GateRef<L::Field, D>,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<()> {
        // let type_id = Any::type_id(&(*gate.0));
        let type_id = gate.0.as_any().type_id();
        let idx = self
            .0
            .index
            .get(&type_id)
            .unwrap_or_else(|| panic!("Gate type not registered {:?}", gate));
        buf.write_usize(*idx)?;

        self.0
            .registry
            .get(&type_id)
            .unwrap_or_else(|| panic!("Gate type not registered {:?}", gate))
            .write(buf, gate, common_data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::gates::arithmetic_base::ArithmeticGate;
    use plonky2::gates::gate::GateRef;
    use plonky2::util::serialization::{Buffer, GateSerializer};

    use crate::backend::circuit::serialization::gates::GateRegistry;
    use crate::backend::circuit::DefaultParameters;
    use crate::prelude::CircuitBuilder;

    type L = DefaultParameters;
    type F = GoldilocksField;
    const D: usize = 2;

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
