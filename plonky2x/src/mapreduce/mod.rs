#[macro_use]
pub mod utils;
pub mod api;
pub mod serialize;

use core::fmt::Debug;
use core::marker::PhantomData;

use itertools::Itertools;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartialWitness, PartitionWitness, WitnessWrite};
use plonky2::plonk::circuit_data::{CircuitData, CommonCircuitData};
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputsTarget;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};

use crate::builder::CircuitBuilder;
use crate::mapreduce::serialize::CircuitDataSerializable;
use crate::vars::{proof_with_pis_to_targets, CircuitVariable};

#[derive(Debug, Clone)]
pub struct MapReduceRecursiveProofGenerator<F, C, I, O, const D: usize>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F> + 'static,
    <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
    I: CircuitVariable + Debug + Clone + Sync + Send + 'static,
    O: CircuitVariable + Debug + Clone + Sync + Send + 'static,
{
    pub map_circuit_id: String,

    pub reduce_circuit_ids: Vec<String>,

    pub inputs: Vec<I>,

    pub proof: ProofWithPublicInputsTarget<D>,

    pub _phantom1: PhantomData<F>,

    pub _phantom2: PhantomData<C>,

    pub _phantom3: PhantomData<O>,
}

impl<F, C, I, O, const D: usize> SimpleGenerator<F, D>
    for MapReduceRecursiveProofGenerator<F, C, I, O, D>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F> + 'static,
    <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
    I: CircuitVariable + Debug + Clone + Sync + Send + Default + 'static,
    O: CircuitVariable + Debug + Clone + Sync + Send + Default + 'static,
{
    fn id(&self) -> String {
        "MapReduceRecursiveProofGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets = Vec::new();
        for i in 0..self.inputs.len() {
            targets.extend(self.inputs[i].targets());
        }
        targets
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        // Load map circuit and map circuit input of type I.
        let map_circuit_path = format!("./build/{}.circuit", self.map_circuit_id);
        let (map_circuit, map_circuit_input) = CircuitData::<F, C, D>::load::<I>(map_circuit_path);
        println!("loaded map circuit from disk");

        // Based on the inputs passed in the outer most circuit compute proofs which map
        // the inputs of type I to the outputs of type O.
        let mut proofs = Vec::new();
        for i in 0..self.inputs.len() {
            let mut pw = PartialWitness::new();
            map_circuit_input.set(&mut pw, self.inputs[i].value(witness));
            let proof = map_circuit.prove(pw).unwrap();
            map_circuit.verify(proof.clone()).unwrap();
            proofs.push(proof);
            println!("generated map proof {}/{}", i + 1, self.inputs.len());
        }

        // Now, we need to reduce the proofs to a single proof using the reduce circuits for
        // each layer. To do so, we load in the appropriate reduce circuit and the left/right
        // proof targets. We then set the left/right proof targets to the proofs we have
        // computed so far and generate a new proof. We repeat this process until we have
        // a single proof.
        let nb_reduce_layers = (self.inputs.len() as f64).log2().ceil() as usize;
        for i in 0..nb_reduce_layers {
            let mut next_proofs = Vec::new();

            let reduce_circuit_path = format!("./build/{}.circuit", self.reduce_circuit_ids[i]);
            let (reduce_circuit, reduce_circuit_inputs) =
                CircuitData::<F, C, D>::load_with_proof_targets(reduce_circuit_path);
            let left = reduce_circuit_inputs[0].to_owned();
            let right = reduce_circuit_inputs[1].to_owned();

            let nb_proofs = self.inputs.len() / (2usize.pow((i + 1) as u32));
            for j in 0..nb_proofs {
                let mut pw = PartialWitness::new();
                pw.set_proof_with_pis_target(&left, &proofs[j * 2]);
                pw.set_proof_with_pis_target(&right, &proofs[j * 2 + 1]);

                let proof = reduce_circuit.prove(pw).unwrap();
                reduce_circuit.verify(proof.clone()).unwrap();
                next_proofs.push(proof);
                println!(
                    "generated reduce proof {}/{} for layer {}/{}",
                    j + 1,
                    nb_proofs,
                    i + 1,
                    nb_reduce_layers
                );
            }

            proofs = next_proofs.clone();
        }

        // We now have a single proof which we can set as the proof target to be verified in the
        // outer most circuit.
        out_buffer.set_proof_with_pis_target(&self.proof, &proofs[0]);
    }

    #[allow(unused_variables)]
    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        dst.write_usize(self.map_circuit_id.len()).unwrap();
        plonky2::util::serialization::Write::write_all(dst, self.map_circuit_id.as_bytes())
            .unwrap();

        dst.write_usize(self.reduce_circuit_ids.len()).unwrap();
        for i in 0..self.reduce_circuit_ids.len() {
            dst.write_usize(self.reduce_circuit_ids[i].len()).unwrap();
            plonky2::util::serialization::Write::write_all(
                dst,
                self.reduce_circuit_ids[i].as_bytes(),
            )
            .unwrap();
        }

        dst.write_usize(self.inputs.len()).unwrap();
        for i in 0..self.inputs.len() {
            dst.write_target_vec(self.inputs[i].targets().as_slice())
                .unwrap();
        }
        dst.write_target_proof_with_public_inputs(&self.proof)
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        let map_circuit_id_length = src.read_usize().unwrap();
        let mut map_circuit_id = vec![0u8; map_circuit_id_length];
        src.read_exact(&mut map_circuit_id).unwrap();

        let mut reduce_circuit_ids = Vec::new();
        let reduce_circuit_ids_len = src.read_usize().unwrap();
        for i in 0..reduce_circuit_ids_len {
            let reduce_circuit_id_length = src.read_usize().unwrap();
            let mut reduce_circuit_id = vec![0u8; reduce_circuit_id_length];
            src.read_exact(&mut reduce_circuit_id).unwrap();
            reduce_circuit_ids.push(String::from_utf8(reduce_circuit_id).unwrap());
        }

        let mut inputs = Vec::new();
        let inputs_len = src.read_usize().unwrap();
        for i in 0..inputs_len {
            let input_targets = src.read_target_vec().unwrap();
            inputs.push(I::from_targets(&input_targets));
        }

        Ok(Self {
            map_circuit_id: String::from_utf8(map_circuit_id).unwrap(),
            reduce_circuit_ids,
            inputs,
            proof: src.read_target_proof_with_public_inputs().unwrap(),
            _phantom1: PhantomData::<F>,
            _phantom2: PhantomData::<C>,
            _phantom3: PhantomData::<O>,
        })
    }
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    pub fn build_map_circuit<I, O, C, M>(&mut self, m: &M) -> (CircuitData<F, C, D>, I)
    where
        I: CircuitVariable + Debug + Clone + Sync + Send + Default + 'static,
        O: CircuitVariable + Debug + Clone + Sync + Send + Default + 'static,
        C: GenericConfig<D, F = F> + 'static,
        <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
        M: Fn(I, &mut CircuitBuilder<F, D>) -> O,
    {
        let mut builder = CircuitBuilder::<F, D>::new();
        let input = builder.init::<I>();
        let output = m(input.clone(), &mut builder);
        builder.register_public_inputs(output.targets().as_slice());
        (builder.build::<C>(), input)
    }

    pub fn build_reduce_circuit<I, O, C, R>(
        &mut self,
        cd: &CircuitData<F, C, D>,
        r: &R,
    ) -> (CircuitData<F, C, D>, Vec<ProofWithPublicInputsTarget<D>>)
    where
        I: CircuitVariable + Debug + Clone + Sync + Send + Default + 'static,
        O: CircuitVariable + Debug + Clone + Sync + Send + Default + 'static,
        C: GenericConfig<D, F = F> + 'static,
        <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
        R: Fn(O, O, &mut CircuitBuilder<F, D>) -> O,
    {
        let mut builder = CircuitBuilder::<F, D>::new();
        let vd = builder.constant_verifier_data(&cd);

        let proof_left = builder.add_virtual_proof_with_pis(&cd.common);
        let proof_right = builder.add_virtual_proof_with_pis(&cd.common);

        builder.verify_proof::<C>(&proof_left, &vd, &cd.common);
        builder.verify_proof::<C>(&proof_right, &vd, &cd.common);

        let input_left = O::from_targets(&proof_left.public_inputs);
        let input_right = O::from_targets(&proof_right.public_inputs);
        let output = r(input_left.clone(), input_right.clone(), &mut builder);

        builder.register_public_inputs(output.targets().as_slice());
        (builder.build::<C>(), vec![proof_left, proof_right])
    }

    pub fn mapreduce<I, O, C, M, R>(&mut self, inputs: Vec<I>, m: M, r: R) -> O
    where
        I: CircuitVariable + Debug + Clone + Sync + Send + Default + 'static,
        O: CircuitVariable + Debug + Clone + Sync + Send + Default + 'static,
        C: GenericConfig<D, F = F> + 'static,
        <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
        M: Fn(I, &mut CircuitBuilder<F, D>) -> O,
        R: Fn(O, O, &mut CircuitBuilder<F, D>) -> O,
    {
        // Build a map circuit which maps from I -> O using the closure `m`.
        let (map_circuit, map_circuit_input) = self.build_map_circuit(&m);

        // Save map circuit and map circuit input target to build folder.
        let map_circuit_id = map_circuit.id();
        let map_circuit_path = format!("./build/{}.circuit", map_circuit_id);
        map_circuit.save(map_circuit_input, map_circuit_path);

        // For each reduce layer, we need to build a reduce circuit which reduces two input proofs
        // to an output O.
        let nb_reduce_layers = (inputs.len() as f64).log2().ceil() as usize;
        let mut reduce_circuits = Vec::new();
        for i in 0..nb_reduce_layers {
            // Build a reduce circuit which maps f(Proof[O], Proof[O]) -> O using the closure `r`.
            let child_circuit_data = if i == 0 {
                &map_circuit
            } else {
                &reduce_circuits[i - 1]
            };
            let (reduce_circuit, reduce_circuit_inputs) =
                self.build_reduce_circuit::<I, O, C, R>(child_circuit_data, &r);

            // Save reduce circuit and reduce circuit input proofs to build folder.
            let reduce_circuit_id = reduce_circuit.id();
            let reduce_circuit_path = format!("./build/{}.circuit", reduce_circuit_id);
            reduce_circuit.save_with_proof_targets(&reduce_circuit_inputs, reduce_circuit_path);
            reduce_circuits.push(reduce_circuit);
        }

        // Create generator to generate map and reduce proofs for each layer.
        let reduce_circuit_ids = reduce_circuits.iter().map(|c| c.id()).collect_vec();
        let last_reduce_circuit = &reduce_circuits[reduce_circuits.len() - 1];
        let proof = self.add_virtual_proof_with_pis(&last_reduce_circuit.common);
        let generator = MapReduceRecursiveProofGenerator::<F, C, I, O, D> {
            map_circuit_id,
            reduce_circuit_ids,
            inputs: inputs.clone(),
            proof: proof.clone(),
            _phantom1: PhantomData::<F>,
            _phantom2: PhantomData::<C>,
            _phantom3: PhantomData::<O>,
        };
        self.add_simple_generator(&generator);

        // Verify the final proof.
        let vd = self.constant_verifier_data(last_reduce_circuit);
        self.verify_proof::<C>(&proof, &vd, &last_reduce_circuit.common);

        // Deserialize the output from the final proof.
        O::from_targets(generator.proof.public_inputs.as_slice())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;

    use crate::builder::CircuitBuilder;
    use crate::vars::{CircuitVariable, Variable};

    #[test]
    fn test_simple_mapreduce_circuit() {
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let a = builder.constant::<Variable>(0);
        let b = builder.constant::<Variable>(1);
        let c = builder.constant::<Variable>(2);
        let d = builder.constant::<Variable>(3);

        let inputs = vec![a, b, c, d];
        let output = builder.mapreduce::<Variable, Variable, C, _, _>(
            inputs,
            |input, builder| {
                let constant = builder.constant::<Variable>(1);
                let sum = builder.add(input, constant);
                sum
            },
            |left, right, builder| {
                let sum = builder.add(left, right);
                sum
            },
        );
        builder.register_public_inputs(output.targets().as_slice());

        println!("compiling outer circuit");
        let data = builder.build::<C>();

        println!("proving outer circuit");
        let pw = PartialWitness::new();
        let proof = data.prove(pw).unwrap();
        data.verify(proof.clone()).unwrap();

        println!("result: {:?}", proof.public_inputs);
    }
}
