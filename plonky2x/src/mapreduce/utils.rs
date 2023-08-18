use core::marker::PhantomData;
use std::fs::{self, create_dir_all, File};
use std::io::Write;
use std::path::Path;

use plonky2::field::extension::Extendable;
use plonky2::gates::arithmetic_base::ArithmeticBaseGenerator;
use plonky2::gates::poseidon::PoseidonGenerator;
use plonky2::gates::poseidon_mds::PoseidonMdsGenerator;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{ConstantGenerator, RandomValueGenerator};
use plonky2::plonk::circuit_data::{CircuitData, CommonCircuitData};
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::recursion::dummy_circuit::DummyProofGenerator;
use plonky2::util::serialization::{
    Buffer, DefaultGateSerializer, IoResult, WitnessGeneratorSerializer,
};

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

pub fn read_circuit_from_build_dir<F: RichField + Extendable<D>, C, const D: usize>(
    name: &String,
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
    let bytes = fs::read(format!("./build/{}.bin", name)).unwrap();
    let data = CircuitData::<F, C, D>::from_bytes(&bytes, &gate_serializer, &generator_serializer)
        .unwrap();
    data
}

pub fn write_circuit_to_build_dir<F: RichField + Extendable<D>, C, const D: usize>(
    data: &CircuitData<F, C, D>,
    name: &String,
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

    // Serialize to bytes.
    let bytes = data
        .to_bytes(&gate_serializer, &generator_serializer)
        .unwrap();

    // Write bytes to "./build/{hex!(circuit_digest).bin}"
    let dir = Path::new("./build");
    create_dir_all(dir).unwrap();
    let elements = data.verifier_only.circuit_digest.elements;
    let path = dir.join(format!("{}.bin", name));
    let mut file = File::create(path).unwrap();
    file.write_all(&bytes).unwrap();

    // Assert that the circuit can be deserialized from bytes.
    CircuitData::<F, C, D>::from_bytes(&bytes, &gate_serializer, &generator_serializer).unwrap();
}

pub struct CustomGeneratorSerializer<C: GenericConfig<D>, const D: usize> {
    pub _phantom: PhantomData<C>,
}

impl<F: RichField + Extendable<D>, C, const D: usize> WitnessGeneratorSerializer<F, D>
    for CustomGeneratorSerializer<C, D>
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
        RandomValueGenerator, "RandomValueGenerator"
    }
}
