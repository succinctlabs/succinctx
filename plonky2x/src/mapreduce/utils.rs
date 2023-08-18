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
