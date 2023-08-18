#[macro_use]
pub mod utils;

use core::fmt::Debug;
use core::marker::PhantomData;
use std::fs::{self, create_dir_all, File};
use std::io::Write;
use std::path::Path;

use plonky2::field::extension::Extendable;
use plonky2::gates::arithmetic_base::ArithmeticBaseGenerator;
use plonky2::gates::poseidon::PoseidonGenerator;
use plonky2::gates::poseidon_mds::PoseidonMdsGenerator;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{
    ConstantGenerator, GeneratedValues, RandomValueGenerator, SimpleGenerator,
};
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartialWitness, PartitionWitness};
use plonky2::plonk::circuit_data::{CircuitData, CommonCircuitData};
use plonky2::plonk::config::{
    AlgebraicHasher, GenericConfig, GenericHashOut, PoseidonGoldilocksConfig,
};
use plonky2::plonk::proof::{ProofWithPublicInputs, ProofWithPublicInputsTarget};
use plonky2::recursion::dummy_circuit::DummyProofGenerator;
use plonky2::util::serialization::{
    Buffer, DefaultGateSerializer, DefaultGeneratorSerializer, IoResult, WitnessGeneratorSerializer,
};

use crate::builder::CircuitBuilder;
use crate::impl_generator_serializer;
use crate::utils::hex;
use crate::vars::CircuitVariable;

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

#[derive(Debug, Clone)]
pub struct MapGenerator<
    F: RichField + Extendable<D>,
    C,
    I: CircuitVariable + Debug + Clone + Sync + Send + 'static,
    O: CircuitVariable + Debug + Clone + Sync + Send + 'static,
    const D: usize,
> where
    C: GenericConfig<D, F = F> + 'static,
    <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
{
    pub circuit_digest: String,
    pub input: I,
    pub input_inner: I,
    pub proof: ProofWithPublicInputsTarget<D>,
    pub output: O,
    pub _phantom1: PhantomData<F>,
    pub _phantom2: PhantomData<C>,
}

impl<
        F: RichField + Extendable<D>,
        C,
        I: CircuitVariable + Debug + Clone + Sync + Send + Default + 'static,
        O: CircuitVariable + Debug + Clone + Sync + Send + Default + 'static,
        const D: usize,
    > SimpleGenerator<F, D> for MapGenerator<F, C, I, O, D>
where
    C: GenericConfig<D, F = F> + 'static,
    <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
{
    fn id(&self) -> String {
        "MapGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        self.input.targets()
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        println!("{}", self.circuit_digest);
        let bytes = fs::read(format!("./build/{}.bin", self.circuit_digest)).unwrap();

        // Save the compiled circuit to disk.
        let gate_serializer = DefaultGateSerializer;
        let generator_serializer = CustomGeneratorSerializer::<C, D> {
            _phantom: PhantomData,
        };
        let data =
            CircuitData::<F, C, D>::from_bytes(&bytes, &gate_serializer, &generator_serializer)
                .unwrap();
        println!("{:?}", data);

        let mut pw = PartialWitness::new();
        self.input_inner.set(&mut pw, self.input.value(witness));
        let proof = data.prove(pw).unwrap();
        data.verify(proof).unwrap();
        println!("successfully generated proof within generator");
    }

    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        println!("serialize map generator");
        todo!()
    }

    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        println!("deserialize map generator");
        todo!()
    }
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn map<
        I: CircuitVariable + Debug + Clone + Sync + Send + Default + 'static,
        O: CircuitVariable + Debug + Clone + Sync + Send + Default + 'static,
        C,
        M,
    >(
        &mut self,
        inputs: Vec<I>,
        m: M,
    ) where
        C: GenericConfig<D, F = F> + 'static,
        <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
        M: Fn(I, &mut CircuitBuilder<F, D>) -> O,
    {
        // Build the inner circuit.
        let data = {
            let mut builder = CircuitBuilder::<F, D>::new();
            let input_inner = builder.init::<I>();
            m(input_inner.clone(), &mut builder);
            builder.build::<C>()
        };
        println!("{:?}", data);

        // Save the compiled circuit to disk.
        let gate_serializer = DefaultGateSerializer;
        let generator_serializer = CustomGeneratorSerializer::<C, D> {
            _phantom: PhantomData,
        };

        let bytes = data
            .to_bytes(&gate_serializer, &generator_serializer)
            .unwrap();

        let dir = Path::new("./build");
        create_dir_all(dir).unwrap();

        let elements = data.verifier_only.circuit_digest.elements;
        let digest = hex!(elements
            .iter()
            .map(|e| e.to_canonical_u64().to_be_bytes())
            .flatten()
            .collect::<Vec<u8>>());
        let path = dir.join(format!("{}.bin", digest));
        let mut file = File::create(path).unwrap();
        file.write_all(&bytes).unwrap();

        let data =
            CircuitData::<F, C, D>::from_bytes(&bytes, &gate_serializer, &generator_serializer)
                .unwrap();

        // // Set the verifier data target to be the verifier data, which is a constant.
        // let vd = self
        //     .api
        //     .add_virtual_verifier_data(data.common.config.fri_config.cap_height);

        // // Set the circuit digest.
        // for i in 0..vd.circuit_digest.elements.len() {
        //     let constant = self
        //         .api
        //         .constant(data.verifier_only.circuit_digest.elements[i]);
        //     self.api.connect(vd.circuit_digest.elements[i], constant);
        // }

        // // Set the constant sigmas cap.
        // for i in 0..vd.constants_sigmas_cap.0.len() {
        //     let cap = vd.constants_sigmas_cap.0[i].elements;
        //     for j in 0..cap.len() {
        //         let constant = self
        //             .api
        //             .constant(data.verifier_only.constants_sigmas_cap.0[i].elements[j]);
        //         self.api.connect(cap[j], constant);
        //     }
        // }

        // // Initialize proofs which will be witnessed from the generator.
        // let mut proofs = Vec::new();
        // for _ in 0..inputs.len() {
        //     let proof = self.api.add_virtual_proof_with_pis(&data.common);
        //     proofs.push(proof);
        // }

        // let generator = MapGenerator {
        //     circuit_digest: digest,
        //     input: inputs[0].to_owned(),
        //     input_inner,
        //     proof: proofs[0].to_owned(),
        //     output: self.init::<O>(),
        //     _phantom1: PhantomData::<F>,
        //     _phantom2: PhantomData::<C>,
        // };
        // self.api.add_simple_generator(generator);

        // Run the generator.

        // generator to generate the proofs
        // data = desrailzie(circuit_digest.bin)
        // pf1 = data.prove(input)

        // for i in 0..inputs.len() {
        //     self.api.verify_proof::<C>(&proofs[i], &vd, &data.common);
        // }

        // let mut outputs = Vec::new();
        // for i in 0..inputs.len() {
        //     let output = O::from_targets(proofs[i].public_inputs.as_slice());
        //     outputs.push(output)
        // }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;

    use crate::builder::CircuitBuilder;
    use crate::vars::Variable;

    #[test]
    fn test_simple_circuit() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let a = builder.constant::<Variable>(0);
        let b = builder.constant::<Variable>(1);
        let c = builder.constant::<Variable>(2);

        builder.map::<Variable, Variable, C, _>(vec![a, b, c], |input, builder| {
            let constant = builder.constant::<Variable>(1);
            let sum = builder.add(input, constant);
            sum
        });

        let pw = PartialWitness::new();
        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();
        data.verify(proof).unwrap();
    }
}
