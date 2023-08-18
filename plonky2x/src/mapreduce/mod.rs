#[macro_use]
pub mod utils;

use core::fmt::Debug;
use core::marker::PhantomData;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartialWitness, PartitionWitness, WitnessWrite};
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputsTarget;
use plonky2::util::serialization::{Buffer, IoResult};

use crate::builder::CircuitBuilder;
use crate::mapreduce::utils::{read_circuit_from_build_dir, write_circuit_to_build_dir};
use crate::utils::hex;
use crate::vars::CircuitVariable;

#[derive(Debug, Clone)]
pub struct RecursiveProofGenerator<
    F: RichField + Extendable<D>,
    C,
    I: CircuitVariable + Debug + Clone + Sync + Send + 'static,
    O: CircuitVariable + Debug + Clone + Sync + Send + 'static,
    const D: usize,
> where
    C: GenericConfig<D, F = F> + 'static,
    <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
{
    /// The circuit digest is used as an "id" to know which inner circuit to run from the build
    /// folder.
    pub circuit_id: String,

    /// The input target within the inner circuit. It should encapsulate all public inputs.
    pub input_inner: I,

    /// The input target from the outer circuit used to set the inner input target.
    pub input_outer: I,

    /// The output target within the outer circuit. It is used to store the output of the inner
    /// circuit.
    pub output_outer: O,

    /// The proof that verifies that f_inner(input) = output within the outer circuit.
    pub proof_outer: ProofWithPublicInputsTarget<D>,

    pub _phantom1: PhantomData<F>,

    pub _phantom2: PhantomData<C>,
}

impl<
        F: RichField + Extendable<D>,
        C,
        I: CircuitVariable + Debug + Clone + Sync + Send + Default + 'static,
        O: CircuitVariable + Debug + Clone + Sync + Send + Default + 'static,
        const D: usize,
    > SimpleGenerator<F, D> for RecursiveProofGenerator<F, C, I, O, D>
where
    C: GenericConfig<D, F = F> + 'static,
    <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
{
    fn id(&self) -> String {
        "MapGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        self.input_outer.targets()
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        // Read the inner circuit from the build folder.
        let data = read_circuit_from_build_dir::<F, C, D>(&self.circuit_id);

        // Set the inputs to the inner circuit.
        let mut pw = PartialWitness::new();
        let input_value = self.input_outer.value(witness);
        self.input_inner.set(&mut pw, input_value);

        // Generate the inner proof.
        let proof = data.prove(pw).unwrap();
        data.verify(proof.clone()).unwrap();

        // Set the proof target in the outer circuit with the generated proof.
        out_buffer.set_proof_with_pis_target(&self.proof_outer, &proof);

        // Set the output target in the outer circuit with the output of the inner circuit.
        let output_targets = self.output_outer.targets();
        for i in 0..output_targets.len() {
            out_buffer.set_target(output_targets[i], proof.public_inputs[i]);
        }

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
        let (data, input_inner) = {
            let mut builder = CircuitBuilder::<F, D>::new();
            let input_inner = builder.init::<I>();
            let output_inner = m(input_inner.clone(), &mut builder);
            builder
                .api
                .register_public_inputs(output_inner.targets().as_slice());
            (builder.build::<C>(), input_inner)
        };
        println!("built inner circuit");

        // Calculate the circuit digest.
        let digest = hex!(data
            .verifier_only
            .circuit_digest
            .elements
            .iter()
            .map(|e| e.to_canonical_u64().to_be_bytes())
            .flatten()
            .collect::<Vec<u8>>());

        // Save the compiled circuit to disk.
        write_circuit_to_build_dir(&data, &digest);
        println!("saved circuit to disk at {}", digest);

        // Set the verifier data target to be the verifier data, which is a constant.
        let vd = self
            .api
            .add_virtual_verifier_data(data.common.config.fri_config.cap_height);

        // Set the circuit digest.
        for i in 0..vd.circuit_digest.elements.len() {
            let constant = self
                .api
                .constant(data.verifier_only.circuit_digest.elements[i]);
            self.api.connect(vd.circuit_digest.elements[i], constant);
        }

        // Set the constant sigmas cap.
        for i in 0..vd.constants_sigmas_cap.0.len() {
            let cap = vd.constants_sigmas_cap.0[i].elements;
            for j in 0..cap.len() {
                let constant = self
                    .api
                    .constant(data.verifier_only.constants_sigmas_cap.0[i].elements[j]);
                self.api.connect(cap[j], constant);
            }
        }

        // Initialize proofs which will be witnessed from the generator.
        let mut proofs = Vec::new();
        for _ in 0..inputs.len() {
            let proof = self.api.add_virtual_proof_with_pis(&data.common);
            proofs.push(proof);
        }

        for i in 0..proofs.len() {
            let generator = RecursiveProofGenerator {
                circuit_id: digest.clone(),
                input_inner: input_inner.clone(),
                input_outer: inputs[i].to_owned(),
                output_outer: self.init::<O>(),
                proof_outer: proofs[i].to_owned(),
                _phantom1: PhantomData::<F>,
                _phantom2: PhantomData::<C>,
            };
            self.api
                .register_public_inputs(generator.output_outer.targets().as_slice());
            self.api.verify_proof::<C>(&proofs[0], &vd, &data.common);
            self.api.add_simple_generator(generator);
        }
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

        println!("compiling outer circuit");
        let data = builder.build::<C>();

        println!("proving outer circuit");
        let pw = PartialWitness::new();
        let proof = data.prove(pw).unwrap();
        data.verify(proof.clone()).unwrap();

        println!("result: {:?}", proof.public_inputs);
    }
}
