use core::marker::PhantomData;
use std::fs::{self, File};
use std::io::Write;

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
    ConstantGenerator, CopyGenerator, NonzeroTestGenerator, RandomValueGenerator,
};
use plonky2::plonk::circuit_data::{CircuitData, CommonCircuitData};
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputsTarget;
use plonky2::recursion::dummy_circuit::DummyProofGenerator;
use plonky2::util::serialization::{
    Buffer, DefaultGateSerializer, IoResult, Read, WitnessGeneratorSerializer,
    Write as Plonky2Write,
};

// use crate::mapreduce::MapReduceRecursiveProofGenerator;
use crate::utils::hex;
use crate::vars::{CircuitVariable, Variable};

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
            log::log!(log::Level::Error, "attempted to serialize generator with id {} which is unsupported by this generator serializer", $generator.0.id());
            Err(plonky2::util::serialization::IoError)
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

pub fn load_circuit<F: RichField + Extendable<D>, C, const D: usize>(
    path: &String,
) -> CircuitData<F, C, D>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F> + 'static,
    C::Hasher: AlgebraicHasher<F>,
{
    let gate_serializer = DefaultGateSerializer;
    let generator_serializer = CustomGeneratorSerializer::<C, D> {
        _phantom: PhantomData,
    };
    let bytes = fs::read(path).unwrap();
    let data = CircuitData::<F, C, D>::from_bytes(&bytes, &gate_serializer, &generator_serializer)
        .unwrap();
    data
}

// pub fn load_circuit_variable<V: CircuitVariable>(path: &String) -> V {
//     let bytes = fs::read(path).unwrap();
//     let mut buffer = Buffer::new(&bytes);
//     let targets = buffer.read_target_vec().unwrap();
//     V::from_targets(&targets)
// }

pub fn load_proof_with_pis_target<const D: usize>(path: &String) -> ProofWithPublicInputsTarget<D> {
    let bytes = fs::read(path).unwrap();
    let mut buffer = Buffer::new(&bytes);
    let proof_with_pis = buffer.read_target_proof_with_public_inputs().unwrap();
    proof_with_pis
}

// pub fn load_circuit_input_from_build_dir<
//     F: RichField + Extendable<D>,
//     C,
//     I: CircuitVariable,
//     const D: usize,
// >(
//     name: &String,
// ) -> I
// where
//     F: RichField + Extendable<D>,
//     C: GenericConfig<D, F = F> + 'static,
//     C::Hasher: AlgebraicHasher<F>,
// {
//     let bytes = fs::read(format!("./build/{}.input", name)).unwrap();
//     let mut buffer = Buffer::new(&bytes);
//     let targets = buffer.read_target_vec().unwrap();
//     I::from_targets(&targets)
// }

pub fn save_circuit<F: RichField + Extendable<D>, C, const D: usize>(
    data: &CircuitData<F, C, D>,
    path: String,
) where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F> + 'static,
    C::Hasher: AlgebraicHasher<F>,
{
    // Setup serializers.
    let gate_serializer = DefaultGateSerializer;
    let generator_serializer = CustomGeneratorSerializer::<C, D> {
        _phantom: PhantomData,
    };

    // Serialize circuit data to bytes.
    let circuit_bytes = data
        .to_bytes(&gate_serializer, &generator_serializer)
        .unwrap();

    // Write bytes to path.
    let mut file = File::create(path).unwrap();
    file.write_all(&circuit_bytes).unwrap();

    // Assert that the circuit can be deserialized from bytes.
    CircuitData::<F, C, D>::from_bytes(&circuit_bytes, &gate_serializer, &generator_serializer)
        .unwrap();
}

pub fn save_circuit_variable<V: CircuitVariable>(variable: V, path: String) {
    // Serialize variable to bytes.
    let mut input_bytes: Vec<u8> = Vec::new();
    input_bytes.write_target_vec(&variable.targets()).unwrap();

    // Write bytes to path.
    let mut file = File::create(path).unwrap();
    file.write_all(&input_bytes).unwrap();
}

pub fn save_proof_with_pis_target<const D: usize>(
    proof_with_pis: ProofWithPublicInputsTarget<D>,
    path: String,
) {
    // Serialize input variable to bytes.
    let mut input_bytes: Vec<u8> = Vec::new();
    input_bytes
        .write_target_proof_with_public_inputs(&proof_with_pis)
        .unwrap();

    // Write bytes to "./build/{hex!(circuit_digest).input}"
    let mut file = File::create(path).unwrap();
    file.write_all(&input_bytes).unwrap();
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
        // MapReduceRecursiveProofGenerator<F, C, Variable, Variable, D>, "MapReduceRecursiveProofGenerator",
    }
}

pub trait CircuitDataIdentifiable<F: RichField + Extendable<D>, C, const D: usize>
where
    C: GenericConfig<D, F = F> + 'static,
    <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
{
    fn id(&self) -> String;
}

impl<F: RichField + Extendable<D>, C, const D: usize> CircuitDataIdentifiable<F, C, D>
    for CircuitData<F, C, D>
where
    C: GenericConfig<D, F = F> + 'static,
    <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
{
    fn id(&self) -> String {
        let circuit_digest = hex!(self
            .verifier_only
            .circuit_digest
            .elements
            .iter()
            .map(|e| e.to_canonical_u64().to_be_bytes())
            .flatten()
            .collect::<Vec<u8>>());
        circuit_digest[0..22].to_string()
    }
}