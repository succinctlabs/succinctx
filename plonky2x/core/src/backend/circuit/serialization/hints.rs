use core::fmt::Debug;
use core::marker::PhantomData;

use curta::chip::ec::edwards::scalar_mul::air::ScalarMulEd25519;
use curta::chip::ec::edwards::scalar_mul::generator::{
    SimpleScalarMulEd25519Generator, SimpleScalarMulEd25519HintGenerator,
};
use curta::chip::hash::blake::blake2b::generator::{
    BLAKE2BAirParameters, BLAKE2BGenerator, BLAKE2BHintGenerator,
};
use curta::machine::hash::sha::sha256::SHA256;
use curta::plonky2::cubic::arithmetic_gate::ArithmeticCubicGenerator;
use curta::plonky2::cubic::mul_gate::MulCubicGenerator;
use curta::plonky2::stark::generator::simple::SimpleStarkWitnessGenerator;
use plonky2::field::extension::Extendable;
use plonky2::gadgets::arithmetic::EqualityGenerator;
use plonky2::gadgets::arithmetic_extension::QuotientGeneratorExtension;
use plonky2::gadgets::range_check::LowHighGenerator;
use plonky2::gadgets::split_base::BaseSumGenerator;
use plonky2::gadgets::split_join::{SplitGenerator, WireSplitGenerator};
use plonky2::gates::arithmetic_base::ArithmeticBaseGenerator;
use plonky2::gates::arithmetic_extension::ArithmeticExtensionGenerator;
use plonky2::gates::base_sum::BaseSplitGenerator;
use plonky2::gates::coset_interpolation::InterpolationGenerator;
use plonky2::gates::exponentiation::ExponentiationGenerator;
use plonky2::gates::lookup::LookupGenerator;
use plonky2::gates::lookup_table::LookupTableGenerator;
use plonky2::gates::multiplication_extension::MulExtensionGenerator;
use plonky2::gates::poseidon::PoseidonGenerator;
use plonky2::gates::poseidon_mds::PoseidonMdsGenerator;
use plonky2::gates::random_access::RandomAccessGenerator;
use plonky2::gates::reducing::ReducingGenerator;
use plonky2::gates::reducing_extension::ReducingGenerator as ReducingExtensionGenerator;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{
    ConstantGenerator, CopyGenerator, NonzeroTestGenerator, RandomValueGenerator, SimpleGenerator,
    SimpleGeneratorAdapter, WitnessGenerator, WitnessGeneratorRef,
};
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::util::serialization::{Buffer, IoResult, Read, WitnessGeneratorSerializer, Write};

use super::registry::{SerializationRegistry, Serializer};
use super::PlonkParameters;
use crate::frontend::builder::watch::WatchGenerator;
use crate::frontend::ecc::ed25519::field::ed25519_base::Ed25519Base;
use crate::frontend::eth::beacon::generators::{
    BeaconAllWithdrawalsHint, BeaconBalanceBatchWitnessHint, BeaconBalanceGenerator,
    BeaconBalanceWitnessHint, BeaconBalancesGenerator, BeaconBlockRootsHint,
    BeaconExecutionPayloadHint, BeaconGraffitiHint, BeaconHeaderHint,
    BeaconHeadersFromOffsetRangeHint, BeaconHistoricalBlockGenerator, BeaconPartialBalancesHint,
    BeaconPartialValidatorsHint, BeaconValidatorBatchHint, BeaconValidatorGenerator,
    BeaconValidatorsGenerator, BeaconValidatorsHint, BeaconWithdrawalGenerator,
    BeaconWithdrawalsGenerator, CompressedBeaconValidatorBatchHint, Eth1BlockToSlotHint,
};
use crate::frontend::eth::beacon::vars::{
    BeaconBalancesVariable, BeaconHeaderVariable, BeaconValidatorVariable,
    BeaconValidatorsVariable, BeaconWithdrawalVariable, BeaconWithdrawalsVariable,
};
use crate::frontend::eth::mpt::generators::LteGenerator;
use crate::frontend::eth::storage::generators::{
    EthBlockGenerator, EthLogGenerator, EthStorageKeyGenerator, EthStorageProofHint,
};
use crate::frontend::hash::blake2::curta::MAX_NUM_CURTA_CHUNKS;
use crate::frontend::hash::deprecated::bit_operations::XOR3Generator;
use crate::frontend::hash::keccak::keccak256::Keccak256Generator;
use crate::frontend::hash::sha::curta::digest_hint::SHADigestHint;
use crate::frontend::hash::sha::curta::proof_hint::SHAProofHint;
use crate::frontend::hint::asynchronous::generator::{AsyncHintDataRef, AsyncHintRef};
use crate::frontend::hint::asynchronous::hint::AsyncHint;
use crate::frontend::hint::asynchronous::serializer::AsyncHintSerializer;
use crate::frontend::hint::simple::hint::Hint;
use crate::frontend::hint::simple::serializer::SimpleHintSerializer;
use crate::frontend::hint::synchronous::Async;
use crate::frontend::num::biguint::BigUintDivRemGenerator;
use crate::frontend::num::nonnative::nonnative::{
    NonNativeAdditionGenerator, NonNativeInverseGenerator, NonNativeMultipleAddsGenerator,
    NonNativeMultiplicationGenerator, NonNativeSubtractionGenerator,
};
use crate::frontend::num::u32::gates::add_many_u32::U32AddManyGenerator;
use crate::frontend::num::u32::gates::arithmetic_u32::U32ArithmeticGenerator;
use crate::frontend::num::u32::gates::comparison::ComparisonGenerator;
use crate::frontend::num::u32::gates::range_check_u32::U32RangeCheckGenerator;
use crate::frontend::num::u32::gates::subtraction_u32::U32SubtractionGenerator;
use crate::frontend::uint::uint64::U64Variable;
use crate::frontend::vars::{Bytes32Variable, SubArrayExtractorHint, U256Variable};
use crate::prelude::{ArrayVariable, BoolVariable, U32Variable, Variable};

pub trait HintSerializer<L: PlonkParameters<D>, const D: usize>:
    WitnessGeneratorSerializer<L::Field, D>
{
    fn read_async_hint(
        &self,
        buf: &mut Buffer,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<AsyncHintDataRef<L, D>>;

    fn write_async_hint(
        &self,
        buf: &mut Vec<u8>,
        hint: &AsyncHintDataRef<L, D>,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<()>;
}

/// A registry to store serializers for witness generators.
///
/// New witness generators can be added to the registry by calling the `register` method,
/// specifying the type and the generator's id.
#[derive(Debug)]
pub struct HintRegistry<L: PlonkParameters<D>, const D: usize> {
    generators: SerializationRegistry<String, L::Field, WitnessGeneratorRef<L::Field, D>, D>,
    async_hints: SerializationRegistry<String, L::Field, AsyncHintDataRef<L, D>, D>,
}

/// A serializer for a plonky2 witness generator.
///
/// This function keeps track of the generator type and the `serialize` and `deserialize` methods.
#[derive(Debug, Clone)]
pub struct WitnessGeneratorSerializerFn<W>(PhantomData<W>);

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

impl<L: PlonkParameters<D>, const D: usize> HintRegistry<L, D> {
    /// Registers a new witness generator with the given id.
    pub fn register_generator<W: WitnessGenerator<L::Field, D>>(&mut self, id: String) {
        let serializer = WitnessGeneratorSerializerFn::<W>(PhantomData);
        self.generators.register(id, serializer).unwrap()
    }

    /// Registers a new simple witness generator with the given id.
    pub fn register_simple<SG: SimpleGenerator<L::Field, D>>(&mut self, id: String) {
        self.register_generator::<SimpleGeneratorAdapter<L::Field, SG, D>>(id)
    }

    /// Registers a hint into the registry.
    pub fn register_hint<H: Hint<L, D>>(&mut self) {
        let serializer = SimpleHintSerializer::<L, H>::new();
        let id = H::id();
        self.generators.register(id, serializer).unwrap();
    }

    /// Registers an asynchronous hint into the registry.
    pub fn register_async_hint<H: AsyncHint<L, D>>(&mut self) {
        let serializer = AsyncHintSerializer::<L, H>::new();
        let id = AsyncHintRef::<L, D>::id(H::id());
        self.generators
            .register(id.clone(), serializer.clone())
            .unwrap();
        self.async_hints.register(id, serializer).unwrap();
    }
}

macro_rules! register_watch_generator {
    ($registry:ident, $l:ty, $d:ty, $($type:ty),*) => {
        $(
            let generator_id = WatchGenerator::<$l, $d, $type>::id();
            $registry.register_simple::<WatchGenerator<$l, $d, $type>>(generator_id);
        )*
    };
}

macro_rules! register_powers_of_two {
    ($r:ident, $hint:ident) => {
        $r.register_hint::<$hint<2>>();
        $r.register_hint::<$hint<4>>();
        $r.register_hint::<$hint<8>>();
        $r.register_hint::<$hint<16>>();
        $r.register_hint::<$hint<32>>();
        $r.register_hint::<$hint<64>>();
        $r.register_hint::<$hint<128>>();
        $r.register_hint::<$hint<256>>();
        $r.register_hint::<$hint<512>>();
        $r.register_hint::<$hint<1024>>();
        $r.register_hint::<$hint<2048>>();
        $r.register_hint::<$hint<4096>>();
        $r.register_hint::<$hint<8192>>();
        $r.register_hint::<$hint<16384>>();
        $r.register_hint::<$hint<32768>>();
        $r.register_hint::<$hint<65536>>();
        $r.register_hint::<$hint<131072>>();
        $r.register_hint::<$hint<262144>>();
        $r.register_hint::<$hint<524288>>();
        $r.register_hint::<$hint<1048576>>();
    };
}

impl<L: PlonkParameters<D>, const D: usize> HintRegistry<L, D>
where
    <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
{
    /// Creates a new registry with all the default generators that are used in a Plonky2x circuit.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut r = Self {
            generators: SerializationRegistry::new(),
            async_hints: SerializationRegistry::new(),
        };

        let arithmetic_generator_id = ArithmeticBaseGenerator::<L::Field, D>::default().id();
        r.register_simple::<ArithmeticBaseGenerator<L::Field, D>>(arithmetic_generator_id);

        let constant_generator_id = ConstantGenerator::<L::Field>::default().id();
        r.register_simple::<ConstantGenerator<L::Field>>(constant_generator_id);

        let poseidon_generator_id = PoseidonGenerator::<L::Field, D>::default().id();
        r.register_simple::<PoseidonGenerator<L::Field, D>>(poseidon_generator_id);

        let poseidon_mds_generator_id =
            SimpleGenerator::<L::Field, D>::id(&PoseidonMdsGenerator::<D>::default());
        r.register_simple::<PoseidonMdsGenerator<D>>(poseidon_mds_generator_id);

        let random_value_generator_id =
            SimpleGenerator::<L::Field, D>::id(&RandomValueGenerator::default());
        r.register_simple::<RandomValueGenerator>(random_value_generator_id);

        let arithmetic_extension_generator_id = SimpleGenerator::<L::Field, D>::id(
            &ArithmeticExtensionGenerator::<L::Field, D>::default(),
        );
        r.register_simple::<ArithmeticExtensionGenerator<L::Field, D>>(
            arithmetic_extension_generator_id,
        );

        let base_split_generator_id =
            SimpleGenerator::<L::Field, D>::id(&BaseSplitGenerator::<2>::default());
        r.register_simple::<BaseSplitGenerator<2>>(base_split_generator_id);

        let base_sum_generator_id =
            SimpleGenerator::<L::Field, D>::id(&BaseSumGenerator::<2>::default());
        r.register_simple::<BaseSumGenerator<2>>(base_sum_generator_id);

        let copy_generator_id = SimpleGenerator::<L::Field, D>::id(&CopyGenerator::default());
        r.register_simple::<CopyGenerator>(copy_generator_id);

        let equality_generator_id =
            SimpleGenerator::<L::Field, D>::id(&EqualityGenerator::default());
        r.register_simple::<EqualityGenerator>(equality_generator_id);

        let exponentiation_generator_id =
            SimpleGenerator::<L::Field, D>::id(&ExponentiationGenerator::<L::Field, D>::default());
        r.register_simple::<ExponentiationGenerator<L::Field, D>>(exponentiation_generator_id);

        let interpolation_generator_id =
            SimpleGenerator::<L::Field, D>::id(&InterpolationGenerator::<L::Field, D>::default());
        r.register_simple::<InterpolationGenerator<L::Field, D>>(interpolation_generator_id);

        let lookup_generator_id = SimpleGenerator::<L::Field, D>::id(&LookupGenerator::default());
        r.register_simple::<LookupGenerator>(lookup_generator_id);

        let lookup_table_generator_id =
            SimpleGenerator::<L::Field, D>::id(&LookupTableGenerator::default());
        r.register_simple::<LookupTableGenerator>(lookup_table_generator_id);

        let low_high_generator_id =
            SimpleGenerator::<L::Field, D>::id(&LowHighGenerator::default());
        r.register_simple::<LowHighGenerator>(low_high_generator_id);

        let mul_extension_generator_id =
            SimpleGenerator::<L::Field, D>::id(&MulExtensionGenerator::<L::Field, D>::default());
        r.register_simple::<MulExtensionGenerator<L::Field, D>>(mul_extension_generator_id);

        let nonzero_test_generator_id =
            SimpleGenerator::<L::Field, D>::id(&NonzeroTestGenerator::default());
        r.register_simple::<NonzeroTestGenerator>(nonzero_test_generator_id);

        let quotient_generator_extension_id =
            SimpleGenerator::<L::Field, D>::id(&QuotientGeneratorExtension::<D>::default());
        r.register_simple::<QuotientGeneratorExtension<D>>(quotient_generator_extension_id);

        let random_access_generator_id =
            SimpleGenerator::<L::Field, D>::id(&RandomAccessGenerator::<L::Field, D>::default());
        r.register_simple::<RandomAccessGenerator<L::Field, D>>(random_access_generator_id);

        let reducing_generator_id =
            SimpleGenerator::<L::Field, D>::id(&ReducingGenerator::<D>::default());
        r.register_simple::<ReducingGenerator<D>>(reducing_generator_id);

        let reducing_extension_generator_id =
            SimpleGenerator::<L::Field, D>::id(&ReducingExtensionGenerator::<D>::default());
        r.register_simple::<ReducingExtensionGenerator<D>>(reducing_extension_generator_id);

        let split_generator_id = SimpleGenerator::<L::Field, D>::id(&SplitGenerator::default());
        r.register_simple::<SplitGenerator>(split_generator_id);

        let wire_split_generator_id =
            SimpleGenerator::<L::Field, D>::id(&WireSplitGenerator::default());
        r.register_simple::<WireSplitGenerator>(wire_split_generator_id);

        let eth_log_generator_id = EthLogGenerator::<L, D>::id();
        r.register_simple::<EthLogGenerator<L, D>>(eth_log_generator_id);

        let eth_block_generator_id = EthBlockGenerator::<L, D>::id();
        r.register_simple::<EthBlockGenerator<L, D>>(eth_block_generator_id);

        let eth_storage_key_generator_id = EthStorageKeyGenerator::<L, D>::id();
        r.register_simple::<EthStorageKeyGenerator<L, D>>(eth_storage_key_generator_id);

        let keccak256_generator_id = Keccak256Generator::<L, D>::id();
        r.register_simple::<Keccak256Generator<L, D>>(keccak256_generator_id);

        let beacon_balance_generator_id = BeaconBalanceGenerator::<L, D>::id();
        r.register_simple::<BeaconBalanceGenerator<L, D>>(beacon_balance_generator_id);

        let beacon_balances_generator_id = BeaconBalancesGenerator::<L, D>::id();
        r.register_simple::<BeaconBalancesGenerator<L, D>>(beacon_balances_generator_id);

        let beacon_validator_generator_id = BeaconValidatorGenerator::<L, D>::id();
        r.register_simple::<BeaconValidatorGenerator<L, D>>(beacon_validator_generator_id);

        let beacon_validators_generator_id = BeaconValidatorsGenerator::<L, D>::id();
        r.register_simple::<BeaconValidatorsGenerator<L, D>>(beacon_validators_generator_id);

        let beacon_withdrawal_generator_id = BeaconWithdrawalGenerator::<L, D>::id();
        r.register_simple::<BeaconWithdrawalGenerator<L, D>>(beacon_withdrawal_generator_id);

        let beacon_withdrawals_generator_id = BeaconWithdrawalsGenerator::<L, D>::id();
        r.register_simple::<BeaconWithdrawalsGenerator<L, D>>(beacon_withdrawals_generator_id);

        let beacon_historical_block_generator_id =
            BeaconHistoricalBlockGenerator::<L::Field, D>::id();
        r.register_simple::<BeaconHistoricalBlockGenerator<L::Field, D>>(
            beacon_historical_block_generator_id,
        );

        let big_uint_div_rem_generator_id = BigUintDivRemGenerator::<L::Field, D>::id();
        r.register_simple::<BigUintDivRemGenerator<L::Field, D>>(big_uint_div_rem_generator_id);

        let u32_arithmetic_generator_id = U32ArithmeticGenerator::<L::Field, D>::id();
        r.register_simple::<U32ArithmeticGenerator<L::Field, D>>(u32_arithmetic_generator_id);

        let u32_add_many_generator_id = U32AddManyGenerator::<L::Field, D>::id();
        r.register_simple::<U32AddManyGenerator<L::Field, D>>(u32_add_many_generator_id);

        let u32_subtraction_generator_id = U32SubtractionGenerator::<L::Field, D>::id();
        r.register_simple::<U32SubtractionGenerator<L::Field, D>>(u32_subtraction_generator_id);

        let comparison_generator_id = ComparisonGenerator::<L::Field, D>::id();
        r.register_simple::<ComparisonGenerator<L::Field, D>>(comparison_generator_id);

        let xor3_generator_id = XOR3Generator::<L::Field, D>::id();
        r.register_simple::<XOR3Generator<L::Field, D>>(xor3_generator_id);

        let le_generator_id = LteGenerator::<L, D>::id();
        r.register_simple::<LteGenerator<L, D>>(le_generator_id);

        let simple_stark_witness_generator_id = SimpleStarkWitnessGenerator::<
            ScalarMulEd25519<L::Field, L::CubicParams>,
            L::CurtaConfig,
            D,
        >::id();
        r.register_simple::<SimpleStarkWitnessGenerator<
            ScalarMulEd25519<L::Field, L::CubicParams>,
            L::CurtaConfig,
            D,
        >>(simple_stark_witness_generator_id);

        r.register_hint::<BeaconBalanceWitnessHint>();
        r.register_hint::<Eth1BlockToSlotHint>();
        r.register_hint::<BeaconExecutionPayloadHint>();
        r.register_hint::<BeaconHeaderHint>();
        r.register_hint::<BeaconAllWithdrawalsHint>();

        r.register_async_hint::<EthStorageProofHint<L, D>>();
        r.register_async_hint::<BeaconValidatorsHint>();

        register_powers_of_two!(r, BeaconBalanceBatchWitnessHint);
        register_powers_of_two!(r, BeaconPartialBalancesHint);
        register_powers_of_two!(r, BeaconValidatorBatchHint);
        register_powers_of_two!(r, BeaconPartialValidatorsHint);
        register_powers_of_two!(r, CompressedBeaconValidatorBatchHint);
        let id = NonNativeAdditionGenerator::<L::Field, D, Ed25519Base>::default().id();
        r.register_simple::<NonNativeAdditionGenerator<L::Field, D, Ed25519Base>>(id);

        let id = NonNativeInverseGenerator::<L::Field, D, Ed25519Base>::default().id();
        r.register_simple::<NonNativeInverseGenerator<L::Field, D, Ed25519Base>>(id);

        let id = NonNativeMultipleAddsGenerator::<L::Field, D, Ed25519Base>::default().id();
        r.register_simple::<NonNativeMultipleAddsGenerator<L::Field, D, Ed25519Base>>(id);

        let id = NonNativeMultiplicationGenerator::<L::Field, D, Ed25519Base>::default().id();
        r.register_simple::<NonNativeMultiplicationGenerator<L::Field, D, Ed25519Base>>(id);

        let id = NonNativeSubtractionGenerator::<L::Field, D, Ed25519Base>::default().id();
        r.register_simple::<NonNativeSubtractionGenerator<L::Field, D, Ed25519Base>>(id);

        let id =
            SimpleScalarMulEd25519Generator::<L::Field, L::CubicParams, L::CurtaConfig, D>::id();
        r.register_simple::<SimpleScalarMulEd25519Generator<L::Field, L::CubicParams, L::CurtaConfig, D>>(id);

        let id = "SimpleScalarMulEd25519HintGenerator";
        r.register_simple::<SimpleScalarMulEd25519HintGenerator<L::Field, D>>(id.to_string());

        let id = U32RangeCheckGenerator::<L::Field, D>::id();
        r.register_simple::<U32RangeCheckGenerator<L::Field, D>>(id);

        let id = ArithmeticCubicGenerator::<L::Field, D>::id();
        r.register_simple::<ArithmeticCubicGenerator<L::Field, D>>(id);

        let id = MulCubicGenerator::<L::Field, D>::id();
        r.register_simple::<MulCubicGenerator<L::Field, D>>(id);

        let blake2b_hint_generator_id = BLAKE2BHintGenerator::id();
        r.register_simple::<BLAKE2BHintGenerator>(blake2b_hint_generator_id);

        let blake2b_generator = BLAKE2BGenerator::<
            L::Field,
            L::CubicParams,
            L::CurtaConfig,
            D,
            BLAKE2BAirParameters<L::Field, L::CubicParams>,
            MAX_NUM_CURTA_CHUNKS,
        >::id();
        r.register_simple::<BLAKE2BGenerator<
            L::Field,
            L::CubicParams,
            L::CurtaConfig,
            D,
            BLAKE2BAirParameters<L::Field, L::CubicParams>,
            MAX_NUM_CURTA_CHUNKS,
        >>(blake2b_generator);

        r.register_hint::<SubArrayExtractorHint>();

        r.register_hint::<BeaconBlockRootsHint>();

        r.register_hint::<BeaconGraffitiHint>();

        r.register_hint::<SHADigestHint<SHA256, 64>>();
        r.register_async_hint::<Async<SHADigestHint<SHA256, 64>>>();

        r.register_hint::<SHAProofHint<SHA256, 64>>();
        r.register_async_hint::<Async<SHAProofHint<SHA256, 64>>>();

        register_powers_of_two!(r, BeaconHeadersFromOffsetRangeHint);

        register_watch_generator!(
            r,
            L,
            D,
            Variable,
            BoolVariable,
            U32Variable,
            U64Variable,
            U256Variable,
            Bytes32Variable,
            BeaconValidatorsVariable,
            BeaconBalancesVariable,
            BeaconWithdrawalsVariable,
            BeaconWithdrawalVariable,
            BeaconValidatorVariable,
            BeaconHeaderVariable,
            ArrayVariable<Bytes32Variable, 8192>
        );

        r
    }
}

impl<L: PlonkParameters<D>, const D: usize> WitnessGeneratorSerializer<L::Field, D>
    for HintRegistry<L, D>
{
    fn read_generator(
        &self,
        buf: &mut Buffer,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<WitnessGeneratorRef<L::Field, D>> {
        let idx = buf.read_usize()?;
        let id = &self.generators.identifiers[idx];

        self.generators
            .registry
            .get(id)
            .unwrap_or_else(|| panic!("Generator type not registered {}", id))
            .read(buf, common_data)
    }

    fn write_generator(
        &self,
        buf: &mut Vec<u8>,
        generator: &WitnessGeneratorRef<L::Field, D>,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<()> {
        let id = generator.0.id();
        let idx = self
            .generators
            .index
            .get(&id)
            .unwrap_or_else(|| panic!("Generator type not registered {}", id));
        buf.write_usize(*idx)?;

        self.generators
            .registry
            .get(&id)
            .unwrap_or_else(|| panic!("Generator type not registered {}", id))
            .write(buf, generator, common_data)?;
        Ok(())
    }
}

impl<L: PlonkParameters<D>, const D: usize> HintSerializer<L, D> for HintRegistry<L, D> {
    fn read_async_hint(
        &self,
        buf: &mut Buffer,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<AsyncHintDataRef<L, D>> {
        let idx = buf.read_usize()?;
        let id = &self.async_hints.identifiers[idx];

        self.async_hints
            .registry
            .get(id)
            .unwrap_or_else(|| panic!("Hint type not registered {}", id))
            .read(buf, common_data)
    }

    fn write_async_hint(
        &self,
        buf: &mut Vec<u8>,
        hint: &AsyncHintDataRef<L, D>,
        common_data: &CommonCircuitData<L::Field, D>,
    ) -> IoResult<()> {
        let id = hint.0.id();
        let idx = self
            .async_hints
            .index
            .get(&id)
            .unwrap_or_else(|| panic!("Generator type not registered {}", id));
        buf.write_usize(*idx)?;

        self.async_hints
            .registry
            .get(&id)
            .unwrap_or_else(|| panic!("Generator type not registered {}", id))
            .write(buf, hint, common_data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::iop::generator::{ConstantGenerator, SimpleGenerator, WitnessGeneratorRef};
    use plonky2::util::serialization::{Buffer, WitnessGeneratorSerializer};

    use crate::backend::circuit::serialization::hints::HintRegistry;
    use crate::backend::circuit::DefaultParameters;
    use crate::prelude::CircuitBuilder;

    type L = DefaultParameters;
    type F = GoldilocksField;
    const D: usize = 2;

    #[test]
    fn test_witness_generator_serialization() {
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
}
