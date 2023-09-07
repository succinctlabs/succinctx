use core::any::{Any, TypeId};
use core::fmt::Debug;
use core::hash::Hash;
use core::marker::PhantomData;
use std::collections::HashMap;

use plonky2::field::extension::Extendable;
use plonky2::gadgets::arithmetic::EqualityGenerator;
use plonky2::gadgets::arithmetic_extension::QuotientGeneratorExtension;
use plonky2::gadgets::range_check::LowHighGenerator;
use plonky2::gadgets::split_base::BaseSumGenerator;
use plonky2::gadgets::split_join::{SplitGenerator, WireSplitGenerator};
use plonky2::gates::arithmetic_base::{ArithmeticBaseGenerator, ArithmeticGate};
use plonky2::gates::arithmetic_extension::{ArithmeticExtensionGate, ArithmeticExtensionGenerator};
use plonky2::gates::base_sum::{BaseSplitGenerator, BaseSumGate};
use plonky2::gates::constant::ConstantGate;
use plonky2::gates::coset_interpolation::{CosetInterpolationGate, InterpolationGenerator};
use plonky2::gates::exponentiation::{ExponentiationGate, ExponentiationGenerator};
use plonky2::gates::gate::{AnyGate, Gate, GateRef};
use plonky2::gates::lookup::{LookupGate, LookupGenerator};
use plonky2::gates::lookup_table::{LookupTableGate, LookupTableGenerator};
use plonky2::gates::multiplication_extension::{MulExtensionGate, MulExtensionGenerator};
use plonky2::gates::noop::NoopGate;
use plonky2::gates::poseidon::{PoseidonGate, PoseidonGenerator};
use plonky2::gates::poseidon_mds::{PoseidonMdsGate, PoseidonMdsGenerator};
use plonky2::gates::public_input::PublicInputGate;
use plonky2::gates::random_access::{RandomAccessGate, RandomAccessGenerator};
use plonky2::gates::reducing::{ReducingGate, ReducingGenerator};
use plonky2::gates::reducing_extension::{
    ReducingExtensionGate, ReducingGenerator as ReducingExtensionGenerator,
};
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{
    ConstantGenerator, CopyGenerator, NonzeroTestGenerator, RandomValueGenerator, SimpleGenerator,
    SimpleGeneratorAdapter, WitnessGenerator, WitnessGeneratorRef,
};
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::recursion::dummy_circuit::DummyProofGenerator;
use plonky2::util::serialization::{
    Buffer, GateSerializer, IoResult, Read, WitnessGeneratorSerializer, Write,
};

use crate::frontend::builder::watch::WatchGenerator;
use crate::frontend::eth::beacon::generators::balance::BeaconBalanceGenerator;
use crate::frontend::eth::beacon::generators::balances::BeaconBalancesGenerator;
use crate::frontend::eth::beacon::generators::historical::BeaconHistoricalBlockGenerator;
use crate::frontend::eth::beacon::generators::validator::BeaconValidatorGenerator;
use crate::frontend::eth::beacon::generators::validators::BeaconValidatorsGenerator;
use crate::frontend::eth::beacon::generators::withdrawal::BeaconWithdrawalGenerator;
use crate::frontend::eth::beacon::generators::withdrawals::BeaconWithdrawalsGenerator;
use crate::frontend::eth::beacon::vars::{
    BeaconBalancesVariable, BeaconValidatorVariable, BeaconValidatorsVariable,
    BeaconWithdrawalVariable, BeaconWithdrawalsVariable,
};
use crate::frontend::eth::storage::generators::block::EthBlockGenerator;
use crate::frontend::eth::storage::generators::storage::{
    EthLogGenerator, EthStorageKeyGenerator, EthStorageProofGenerator,
};
use crate::frontend::hash::bit_operations::{XOR3Gate, XOR3Generator};
use crate::frontend::hash::keccak::keccak256::Keccak256Generator;
use crate::frontend::num::biguint::BigUintDivRemGenerator;
use crate::frontend::num::u32::gates::add_many_u32::{U32AddManyGate, U32AddManyGenerator};
use crate::frontend::num::u32::gates::arithmetic_u32::{U32ArithmeticGate, U32ArithmeticGenerator};
use crate::frontend::num::u32::gates::comparison::{ComparisonGate, ComparisonGenerator};
use crate::frontend::uint::uint256::U256Variable;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::Bytes32Variable;

/// A registry to store serializers for witness generators.
///
/// New witness generators can be added to the registry by calling the `register` method,
/// specifying the type and the generator's id.
#[derive(Debug)]
pub struct WitnessGeneratorRegistry<F: RichField + Extendable<D>, const D: usize>(
    SerializationRegistry<String, F, WitnessGeneratorRef<F, D>, D>,
);

/// A registry to store serializers for gates.
///
/// New gates can be added to the registry by calling the `register` method.
#[derive(Debug)]
pub struct GateRegistry<F: RichField + Extendable<D>, const D: usize>(
    SerializationRegistry<TypeId, F, GateRef<F, D>, D>,
);

/// A trait for serializing and deserializing objects compatible with plonky2 traits.
pub trait Serializer<F: RichField + Extendable<D>, T, const D: usize> {
    fn read(&self, buf: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<T>;
    fn write(
        &self,
        buf: &mut Vec<u8>,
        object: &T,
        common_data: &CommonCircuitData<F, D>,
    ) -> IoResult<()>;
}

/// A registry for storing serializers for objects.
pub(crate) struct SerializationRegistry<K: Hash, F: RichField + Extendable<D>, T, const D: usize> {
    registry: HashMap<K, Box<dyn Serializer<F, T, D>>>,
    index: HashMap<K, usize>,
    type_ids: Vec<K>,
    current_index: usize,
}

impl<K: Hash + Debug, F: RichField + Extendable<D>, T: Debug, const D: usize> Debug
    for SerializationRegistry<K, F, T, D>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SerializationRegistry")
            .field("ids of registered objects", &self.registry.keys())
            .field("index", &self.index)
            .field("type_ids", &self.type_ids)
            .field("current_index", &self.current_index)
            .finish()
    }
}

impl<F: RichField + Extendable<D>, K: Hash, T: Any, const D: usize>
    SerializationRegistry<K, F, T, D>
{
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            index: HashMap::new(),
            type_ids: Vec::new(),
            current_index: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WitnessGeneratorSerializerFn<W>(PhantomData<W>);

#[derive(Clone)]
pub struct GateSerializerFn<G>(PhantomData<G>);

impl<F: RichField + Extendable<D>, W: WitnessGenerator<F, D>, const D: usize>
    Serializer<F, WitnessGeneratorRef<F, D>, D> for WitnessGeneratorSerializerFn<W>
{
    fn read(
        &self,
        buf: &mut Buffer,
        common_data: &CommonCircuitData<F, D>,
    ) -> IoResult<WitnessGeneratorRef<F, D>> {
        let generator: IoResult<W> = WitnessGenerator::<F, D>::deserialize(buf, common_data);
        generator.map(|g| WitnessGeneratorRef::<F, D>::new(g))
    }

    fn write(
        &self,
        buf: &mut Vec<u8>,
        object: &WitnessGeneratorRef<F, D>,
        common_data: &CommonCircuitData<F, D>,
    ) -> IoResult<()> {
        object.0.serialize(buf, common_data)
    }
}

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

impl<F: RichField + Extendable<D>, const D: usize> WitnessGeneratorRegistry<F, D> {
    /// Registers a new witness generator with the given id.
    pub fn register<W: WitnessGenerator<F, D>>(&mut self, id: String) {
        let exists = self.0.registry.insert(
            id.clone(),
            Box::new(WitnessGeneratorSerializerFn::<W>(PhantomData)),
        );

        if exists.is_some() {
            panic!("Generator type {} already registered", id);
        }

        self.0.type_ids.push(id.clone());
        self.0.index.insert(id, self.0.current_index);
        self.0.current_index += 1;
    }

    /// Registers a new simple witness generator with the given id.
    pub fn register_simple<SG: SimpleGenerator<F, D>>(&mut self, id: String) {
        self.register::<SimpleGeneratorAdapter<F, SG, D>>(id)
    }
}

impl<F: RichField + Extendable<D>, const D: usize> GateRegistry<F, D> {
    /// Registers a new gate.
    pub fn register<G: AnyGate<F, D>>(&mut self) {
        let type_id = TypeId::of::<G>();
        let exists = self
            .0
            .registry
            .insert(type_id, Box::new(GateSerializerFn::<G>(PhantomData)));

        if exists.is_some() {
            panic!("Gate type already registered");
        }

        self.0.type_ids.push(type_id);
        self.0.index.insert(type_id, self.0.current_index);
        self.0.current_index += 1;
    }
}

impl<F, const D: usize> WitnessGeneratorSerializer<F, D> for WitnessGeneratorRegistry<F, D>
where
    F: RichField + Extendable<D>,
{
    fn read_generator(
        &self,
        buf: &mut Buffer,
        common_data: &CommonCircuitData<F, D>,
    ) -> IoResult<WitnessGeneratorRef<F, D>> {
        let idx = buf.read_usize()?;
        let type_id = &self.0.type_ids[idx];

        self.0
            .registry
            .get(type_id)
            .unwrap_or_else(|| panic!("Generator type not registered {}", type_id))
            .read(buf, common_data)
    }

    fn write_generator(
        &self,
        buf: &mut Vec<u8>,
        generator: &WitnessGeneratorRef<F, D>,
        common_data: &CommonCircuitData<F, D>,
    ) -> IoResult<()> {
        let type_id = generator.0.id();
        let idx = self
            .0
            .index
            .get(&type_id)
            .unwrap_or_else(|| panic!("Generator type not registered {}", type_id));
        buf.write_usize(*idx)?;

        self.0
            .registry
            .get(&type_id)
            .unwrap_or_else(|| panic!("Generator type not registered {}", type_id))
            .write(buf, generator, common_data)?;
        Ok(())
    }
}

impl<F, const D: usize> GateSerializer<F, D> for GateRegistry<F, D>
where
    F: RichField + Extendable<D>,
{
    fn read_gate(
        &self,
        buf: &mut Buffer,
        common_data: &CommonCircuitData<F, D>,
    ) -> IoResult<GateRef<F, D>> {
        let idx = buf.read_usize()?;
        let type_id = self.0.type_ids[idx];

        self.0
            .registry
            .get(&type_id)
            .unwrap_or_else(|| panic!("Gate type not registered {:?}", type_id))
            .read(buf, common_data)
    }

    fn write_gate(
        &self,
        buf: &mut Vec<u8>,
        gate: &GateRef<F, D>,
        common_data: &CommonCircuitData<F, D>,
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

macro_rules! register_watch_generator {
    ($registry:ident, $($type:ty),*) => {
        $(
            let generator_id = WatchGenerator::<$type>::id();
            $registry.register_simple::<WatchGenerator<$type>>(generator_id);
        )*
    };
}

impl<F: RichField + Extendable<D>, const D: usize> WitnessGeneratorRegistry<F, D> {
    /// Creates a new registry with all the default generators that are used in a Plonky2x circuit.
    pub fn new<C: GenericConfig<D, F = F> + 'static>() -> Self
    where
        C::Hasher: AlgebraicHasher<F>,
    {
        let mut r = Self(SerializationRegistry::new());

        let dummy_proof_id = DummyProofGenerator::<F, C, D>::default().id();
        r.register_simple::<DummyProofGenerator<F, C, D>>(dummy_proof_id);

        let arithmetic_generator_id = ArithmeticBaseGenerator::<F, D>::default().id();
        r.register_simple::<ArithmeticBaseGenerator<F, D>>(arithmetic_generator_id);

        let constant_generator_id = ConstantGenerator::<F>::default().id();
        r.register_simple::<ConstantGenerator<F>>(constant_generator_id);

        let poseidon_generator_id = PoseidonGenerator::<F, D>::default().id();
        r.register_simple::<PoseidonGenerator<F, D>>(poseidon_generator_id);

        let poseidon_mds_generator_id =
            SimpleGenerator::<F, D>::id(&PoseidonMdsGenerator::<D>::default());
        r.register_simple::<PoseidonMdsGenerator<D>>(poseidon_mds_generator_id);

        let random_value_generator_id =
            SimpleGenerator::<F, D>::id(&RandomValueGenerator::default());
        r.register_simple::<RandomValueGenerator>(random_value_generator_id);

        let arithmetic_extension_generator_id =
            SimpleGenerator::<F, D>::id(&ArithmeticExtensionGenerator::<F, D>::default());
        r.register_simple::<ArithmeticExtensionGenerator<F, D>>(arithmetic_extension_generator_id);

        let base_split_generator_id =
            SimpleGenerator::<F, D>::id(&BaseSplitGenerator::<2>::default());
        r.register_simple::<BaseSplitGenerator<2>>(base_split_generator_id);

        let base_sum_generator_id = SimpleGenerator::<F, D>::id(&BaseSumGenerator::<2>::default());
        r.register_simple::<BaseSumGenerator<2>>(base_sum_generator_id);

        let copy_generator_id = SimpleGenerator::<F, D>::id(&CopyGenerator::default());
        r.register_simple::<CopyGenerator>(copy_generator_id);

        let equality_generator_id = SimpleGenerator::<F, D>::id(&EqualityGenerator::default());
        r.register_simple::<EqualityGenerator>(equality_generator_id);

        let exponentiation_generator_id =
            SimpleGenerator::<F, D>::id(&ExponentiationGenerator::<F, D>::default());
        r.register_simple::<ExponentiationGenerator<F, D>>(exponentiation_generator_id);

        let interpolation_generator_id =
            SimpleGenerator::<F, D>::id(&InterpolationGenerator::<F, D>::default());
        r.register_simple::<InterpolationGenerator<F, D>>(interpolation_generator_id);

        let lookup_generator_id = SimpleGenerator::<F, D>::id(&LookupGenerator::default());
        r.register_simple::<LookupGenerator>(lookup_generator_id);

        let lookup_table_generator_id =
            SimpleGenerator::<F, D>::id(&LookupTableGenerator::default());
        r.register_simple::<LookupTableGenerator>(lookup_table_generator_id);

        let low_high_generator_id = SimpleGenerator::<F, D>::id(&LowHighGenerator::default());
        r.register_simple::<LowHighGenerator>(low_high_generator_id);

        let mul_extension_generator_id =
            SimpleGenerator::<F, D>::id(&MulExtensionGenerator::<F, D>::default());
        r.register_simple::<MulExtensionGenerator<F, D>>(mul_extension_generator_id);

        let nonzero_test_generator_id =
            SimpleGenerator::<F, D>::id(&NonzeroTestGenerator::default());
        r.register_simple::<NonzeroTestGenerator>(nonzero_test_generator_id);

        let quotient_generator_extension_id =
            SimpleGenerator::<F, D>::id(&QuotientGeneratorExtension::<D>::default());
        r.register_simple::<QuotientGeneratorExtension<D>>(quotient_generator_extension_id);

        let random_access_generator_id =
            SimpleGenerator::<F, D>::id(&RandomAccessGenerator::<F, D>::default());
        r.register_simple::<RandomAccessGenerator<F, D>>(random_access_generator_id);

        let reducing_generator_id = SimpleGenerator::<F, D>::id(&ReducingGenerator::<D>::default());
        r.register_simple::<ReducingGenerator<D>>(reducing_generator_id);

        let reducing_extension_generator_id =
            SimpleGenerator::<F, D>::id(&ReducingExtensionGenerator::<D>::default());
        r.register_simple::<ReducingExtensionGenerator<D>>(reducing_extension_generator_id);

        let split_generator_id = SimpleGenerator::<F, D>::id(&SplitGenerator::default());
        r.register_simple::<SplitGenerator>(split_generator_id);

        let wire_split_generator_id = SimpleGenerator::<F, D>::id(&WireSplitGenerator::default());
        r.register_simple::<WireSplitGenerator>(wire_split_generator_id);

        let eth_storage_proof_generator_id = EthStorageProofGenerator::<F, D>::id();
        r.register_simple::<EthStorageProofGenerator<F, D>>(eth_storage_proof_generator_id);

        let eth_log_generator_id = EthLogGenerator::<F, D>::id();
        r.register_simple::<EthLogGenerator<F, D>>(eth_log_generator_id);

        let eth_block_generator_id = EthBlockGenerator::<F, D>::id();
        r.register_simple::<EthBlockGenerator<F, D>>(eth_block_generator_id);

        let eth_storage_key_generator_id = EthStorageKeyGenerator::<F, D>::id();
        r.register_simple::<EthStorageKeyGenerator<F, D>>(eth_storage_key_generator_id);

        let keccak256_generator_id = Keccak256Generator::<F, D>::id();
        r.register_simple::<Keccak256Generator<F, D>>(keccak256_generator_id);

        let beacon_balance_generator_id = BeaconBalanceGenerator::<F, D>::id();
        r.register_simple::<BeaconBalanceGenerator<F, D>>(beacon_balance_generator_id);

        let beacon_balances_generator_id = BeaconBalancesGenerator::<F, D>::id();
        r.register_simple::<BeaconBalancesGenerator<F, D>>(beacon_balances_generator_id);

        let beacon_validator_generator_id = BeaconValidatorGenerator::<F, D>::id();
        r.register_simple::<BeaconValidatorGenerator<F, D>>(beacon_validator_generator_id);

        let beacon_validators_generator_id = BeaconValidatorsGenerator::<F, D>::id();
        r.register_simple::<BeaconValidatorsGenerator<F, D>>(beacon_validators_generator_id);

        let beacon_withdrawal_generator_id = BeaconWithdrawalGenerator::<F, D>::id();
        r.register_simple::<BeaconWithdrawalGenerator<F, D>>(beacon_withdrawal_generator_id);

        let beacon_withdrawals_generator_id = BeaconWithdrawalsGenerator::<F, D>::id();
        r.register_simple::<BeaconWithdrawalsGenerator<F, D>>(beacon_withdrawals_generator_id);

        let beacon_historical_block_generator_id = BeaconHistoricalBlockGenerator::<F, D>::id();
        r.register_simple::<BeaconHistoricalBlockGenerator<F, D>>(
            beacon_historical_block_generator_id,
        );

        let big_uint_div_rem_generator_id = BigUintDivRemGenerator::<F, D>::id();
        r.register_simple::<BigUintDivRemGenerator<F, D>>(big_uint_div_rem_generator_id);

        let u32_arithmetic_generator_id = U32ArithmeticGenerator::<F, D>::id();
        r.register_simple::<U32ArithmeticGenerator<F, D>>(u32_arithmetic_generator_id);

        let u32_add_many_generator_id = U32AddManyGenerator::<F, D>::id();
        r.register_simple::<U32AddManyGenerator<F, D>>(u32_add_many_generator_id);

        let comparison_generator_id = ComparisonGenerator::<F, D>::id();
        r.register_simple::<ComparisonGenerator<F, D>>(comparison_generator_id);

        let xor3_generator_id = XOR3Generator::<F, D>::id();
        r.register_simple::<XOR3Generator<F, D>>(xor3_generator_id);

        register_watch_generator!(
            r,
            U64Variable,
            U256Variable,
            Bytes32Variable,
            BeaconValidatorsVariable,
            BeaconBalancesVariable,
            BeaconWithdrawalsVariable,
            BeaconWithdrawalVariable,
            BeaconValidatorVariable
        );

        r
    }
}

impl<F: RichField + Extendable<D>, const D: usize> GateRegistry<F, D> {
    #[allow(clippy::new_without_default)]
    /// Creates a new registry with all the default gates that are used in a Plonky2x circuit.
    pub fn new() -> Self {
        let mut r = Self(SerializationRegistry::new());

        r.register::<ArithmeticGate>();
        r.register::<ArithmeticExtensionGate<D>>();
        r.register::<BaseSumGate<2>>();
        r.register::<ConstantGate>();
        r.register::<CosetInterpolationGate<F, D>>();
        r.register::<ExponentiationGate<F, D>>();
        r.register::<LookupGate>();
        r.register::<LookupTableGate>();
        r.register::<MulExtensionGate<D>>();
        r.register::<NoopGate>();
        r.register::<PoseidonMdsGate<F, D>>();
        r.register::<PoseidonGate<F, D>>();
        r.register::<PublicInputGate>();
        r.register::<RandomAccessGate<F, D>>();
        r.register::<ReducingExtensionGate<D>>();
        r.register::<ReducingGate<D>>();
        r.register::<XOR3Gate>();
        r.register::<ComparisonGate<F, D>>();
        r.register::<U32AddManyGate<F, D>>();
        r.register::<U32ArithmeticGate<F, D>>();

        r
    }
}

#[cfg(test)]
mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;

    use super::*;
    use crate::prelude::CircuitBuilder;

    #[test]
    fn test_witness_serialization() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let builder = CircuitBuilder::<F, D>::new();
        let common_data = builder.build::<C>().data.common;

        let registry = WitnessGeneratorRegistry::<F, D>::new::<C>();
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
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let builder = CircuitBuilder::<F, D>::new();
        let common_data = builder.build::<C>().data.common;

        let registry = GateRegistry::<F, D>::new();

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
