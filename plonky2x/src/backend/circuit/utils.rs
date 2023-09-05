use core::marker::PhantomData;

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
    ConstantGenerator, CopyGenerator, NonzeroTestGenerator, RandomValueGenerator,
};
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::recursion::dummy_circuit::DummyProofGenerator;
use plonky2::util::serialization::{Buffer, GateSerializer, IoResult, WitnessGeneratorSerializer};
use plonky2::{get_gate_tag_impl, impl_gate_serializer, read_gate_impl};

use crate::frontend::eth::beacon::generators::balance::BeaconBalanceGenerator;
use crate::frontend::eth::beacon::generators::balances::BeaconBalancesGenerator;
use crate::frontend::eth::beacon::generators::historical::BeaconHistoricalBlockGenerator;
use crate::frontend::eth::beacon::generators::validator::BeaconValidatorGenerator;
use crate::frontend::eth::beacon::generators::validators::BeaconValidatorsGenerator;
use crate::frontend::eth::beacon::generators::withdrawal::BeaconWithdrawalGenerator;
use crate::frontend::eth::beacon::generators::withdrawals::BeaconWithdrawalsGenerator;
use crate::frontend::eth::storage::generators::block::EthBlockGenerator;
use crate::frontend::eth::storage::generators::storage::{
    EthLogGenerator, EthStorageKeyGenerator, EthStorageProofGenerator,
};
use crate::frontend::hash::bit_operations::{XOR3Gate, XOR3Generator};
use crate::frontend::hash::keccak::keccak256::Keccak256Generator;
use crate::frontend::num::u32::gates::add_many_u32::{U32AddManyGate, U32AddManyGenerator};
use crate::frontend::num::u32::gates::arithmetic_u32::U32ArithmeticGate;
use crate::frontend::num::u32::gates::comparison::ComparisonGate;

#[macro_export]
macro_rules! impl_generator_serializer {
    ($serializer:ty, $( $generator:ty, $name:expr ),* $(,)* ) => {
        fn read_generator(
            &self,
            buf: &mut Buffer,
            common_data: &CommonCircuitData<F, D>,
        ) -> IoResult<plonky2::iop::generator::WitnessGeneratorRef<F, D>> {
            let tag = plonky2::util::serialization::Read::read_u32(buf)?;
            read_generator_impl! {
                buf,
                tag,
                common_data,
                $( $generator ),*
            }
        }

        fn write_generator(
            &self,
            buf: &mut Vec<u8>,
            generator: &plonky2::iop::generator::WitnessGeneratorRef<F, D>,
            common_data: &CommonCircuitData<F, D>,
        ) -> IoResult<()> {
            let tag = get_generator_tag_impl! {
                generator,
                $( $generator, $name ),*
            }?;
            plonky2::util::serialization::Write::write_u32(buf, tag)?;
            generator.0.serialize(buf, common_data)?;
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! get_generator_tag_impl {
    ($generator:expr, $( $generator_type:ty, $name:expr),+ ) => {{
        let mut i = 0..;
        $(if let (tag, true) = (i.next().unwrap(), $generator.0.id() == $name) {
            Ok(tag)
        } else)*
        {
            panic!("attempted to serialize generator with id {} which is unsupported by this generator serializer", $generator.0.id());
        }
    }};
}

#[macro_export]
macro_rules! read_generator_impl {
    ($buf:expr, $tag:expr, $common:expr, $($generator_types:ty),+) => {{
        let tag = $tag;
        let buf = $buf;
        let mut i = 0..;

        $(if tag == i.next().unwrap() {
        let generator =
            <$generator_types as plonky2::iop::generator::SimpleGenerator<F, D>>::deserialize(buf, $common)?;
        Ok(plonky2::iop::generator::WitnessGeneratorRef::<F, D>::new(
            plonky2::iop::generator::SimpleGenerator::<F, D>::adapter(generator),
        ))
        } else)*
        {
            Err(plonky2::util::serialization::IoError)
        }
    }};
}

pub struct CustomGeneratorSerializer<C: GenericConfig<D>, const D: usize> {
    pub _phantom: PhantomData<C>,
}

impl<F, C, const D: usize> WitnessGeneratorSerializer<F, D> for CustomGeneratorSerializer<C, D>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F> + 'static,
    C::Hasher: AlgebraicHasher<F>,
{
    impl_generator_serializer! {
        CustomGeneratorSerializer,
        DummyProofGenerator<F, C, D>, "DummyProofGenerator",
        ArithmeticBaseGenerator<F, D>, "ArithmeticBaseGenerator",
        ConstantGenerator<F>, "ConstantGenerator",
        PoseidonGenerator<F, D>, "PoseidonGenerator",
        PoseidonMdsGenerator<D>, "PoseidonMdsGenerator",
        RandomValueGenerator, "RandomValueGenerator",
        ArithmeticBaseGenerator<F, D>, "ArithmeticBaseGenerator",
        ArithmeticExtensionGenerator<F, D>, "ArithmeticExtensionGenerator",
        BaseSplitGenerator<2>, "BaseSplitGenerator",
        BaseSumGenerator<2>, "BaseSumGenerator",
        CopyGenerator, "CopyGenerator",
        DummyProofGenerator<F, C, D>, "DummyProofGenerator",
        EqualityGenerator, "EqualityGenerator",
        ExponentiationGenerator<F, D>, "ExponentiationGenerator",
        InterpolationGenerator<F, D>, "InterpolationGenerator",
        LookupGenerator, "LookupGenerator",
        LookupTableGenerator, "LookupTableGenerator",
        LowHighGenerator, "LowHighGenerator",
        MulExtensionGenerator<F, D>, "MulExtensionGenerator",
        NonzeroTestGenerator, "NonzeroTestGenerator",
        PoseidonGenerator<F, D>, "PoseidonGenerator",
        PoseidonMdsGenerator<D>, "PoseidonMdsGenerator",
        QuotientGeneratorExtension<D>, "QuotientGeneratorExtension",
        RandomAccessGenerator<F, D>, "RandomAccessGenerator",
        RandomValueGenerator, "RandomValueGenerator",
        ReducingGenerator<D>, "ReducingGenerator",
        ReducingExtensionGenerator<D>, "ReducingExtensionGenerator",
        SplitGenerator, "SplitGenerator",
        WireSplitGenerator, "WireSplitGenerator",
        EthStorageProofGenerator<F, D>, "EthStorageProofGenerator",
        EthLogGenerator<F, D>, "EthLogGenerator",
        EthBlockGenerator<F, D>, "EthBlockGenerator",
        EthStorageKeyGenerator<F, D>, "EthStorageKeyGenerator",
        Keccak256Generator<F, D>, "Keccak256Generator",
        BeaconBalanceGenerator<F, D>, "BeaconValidatorBalanceGenerator",
        BeaconValidatorGenerator<F, D>, "BeaconValidatorGenerator",
        BeaconValidatorsGenerator<F, D>, "BeaconValidatorsGenerator",
        U32AddManyGenerator<F, D>, "U32AddManyGenerator",
        XOR3Generator<F, D>, "XOR3Generator",
        BeaconBalanceGenerator<F, D>, "BeaconBalanceGenerator",
        BeaconBalancesGenerator<F, D>, "BeaconBalancesGenerator",
        BeaconValidatorGenerator<F, D>, "BeaconValidatorGenerator",
        BeaconValidatorsGenerator<F, D>, "BeaconValidatorsGenerator",
        BeaconWithdrawalGenerator<F, D>, "BeaconWithdrawalGenerator",
        BeaconWithdrawalsGenerator<F, D>, "BeaconWithdrawalsGenerator",
        BeaconHistoricalBlockGenerator<F, D>, "BeaconHistoricalBlockGenerator",
    }
}

pub struct CustomGateSerializer;

impl<F: RichField + Extendable<D>, const D: usize> GateSerializer<F, D> for CustomGateSerializer {
    impl_gate_serializer! {
        CustomGateSerializer,
        ArithmeticGate,
        ArithmeticExtensionGate<D>,
        BaseSumGate<2>,
        ConstantGate,
        CosetInterpolationGate<F, D>,
        ExponentiationGate<F, D>,
        LookupGate,
        LookupTableGate,
        MulExtensionGate<D>,
        NoopGate,
        PoseidonMdsGate<F, D>,
        PoseidonGate<F, D>,
        PublicInputGate,
        RandomAccessGate<F, D>,
        ReducingExtensionGate<D>,
        ReducingGate<D>,
        U32AddManyGate<F, D>,
        XOR3Gate,
        ComparisonGate<F, D>,
        U32ArithmeticGate<F, D>
    }
}
