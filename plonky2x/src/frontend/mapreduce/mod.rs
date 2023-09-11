use core::fmt::Debug;
use core::marker::PhantomData;

use itertools::Itertools;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputsTarget;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};
use tokio::runtime::Runtime;

use crate::backend::circuit::Circuit;
use crate::backend::prover::{EnvProver, Prover};
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::CircuitVariable;
use crate::prelude::{GateRegistry, PlonkParameters, WitnessGeneratorRegistry};

#[derive(Debug, Clone)]
pub struct MapReduceRecursiveProofGenerator<L, I, O, const D: usize>
where
    L: PlonkParameters<D>,
    <L as PlonkParameters<D>>::Config: GenericConfig<D, F = L::Field> + 'static,
    <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    I: CircuitVariable,
    O: CircuitVariable,
{
    /// The identifier for the map circuit.
    pub map_circuit_id: String,

    /// The identifiers for the reduce circuits.
    pub reduce_circuit_ids: Vec<String>,

    /// The inputs to the map circuit.
    pub inputs: Vec<I>,

    /// The proof target for the final circuit proof.
    pub proof: ProofWithPublicInputsTarget<D>,

    /// Phantom data.
    pub _phantom1: PhantomData<L>,
    pub _phantom2: PhantomData<O>,
}

impl<L, I, O, const D: usize> SimpleGenerator<L::Field, D>
    for MapReduceRecursiveProofGenerator<L, I, O, D>
where
    L: PlonkParameters<D>,
    <L as PlonkParameters<D>>::Config: GenericConfig<D, F = L::Field> + 'static,
    <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    I: CircuitVariable,
    O: CircuitVariable,
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

    fn run_once(
        &self,
        witness: &PartitionWitness<L::Field>,
        out_buffer: &mut GeneratedValues<L::Field>,
    ) {
        // The gate and witness generator serializers.
        let gate_serializer = GateRegistry::<L, D>::new();
        let generator_serializer = WitnessGeneratorRegistry::<L, D>::new();

        // Create the prover and the async runtime.
        let prover = EnvProver::new();
        let rt = Runtime::new().expect("failed to create tokio runtime");

        // Load the map circuit from disk & generate the proofs.
        let map_circuit_path = format!("./build/{}.circuit", self.map_circuit_id);
        let map_circuit =
            Circuit::<L, D>::load(&map_circuit_path, &gate_serializer, &generator_serializer)
                .unwrap();

        // Calculate the inputs to the map.
        let map_input_values = self.inputs.iter().map(|x| x.get(witness)).collect_vec();
        let map_inputs = Vec::new();
        for map_input_value in map_input_values {
            let map_input = map_circuit.input();
            map_input.write::<I>(map_input_value);
            map_inputs.push(map_input)
        }

        // Generate the proofs for the map layer.
        let mut proofs = rt.block_on(async { prover.batch_prove(&map_circuit, &map_inputs).await });

        // Each reduce layer takes N leave proofs and produces N / 2 proofs using a simple
        // linear and binary reduction strategy.
        let nb_reduce_layers = (self.inputs.len() as f64).log2().ceil() as usize;
        for i in 0..nb_reduce_layers {
            let reduce_circuit_path = format!("./build/{}.circuit", self.reduce_circuit_ids[i]);
            let reduce_circuit = Circuit::<L, D>::load(
                &reduce_circuit_path,
                &gate_serializer,
                &generator_serializer,
            )
            .unwrap();
            let nb_proofs = self.inputs.len() / (2usize.pow((i + 1) as u32));

            let targets = reduce_circuit_inputs.into();
            let mut values = Vec::new();
            for j in 0..nb_proofs {
                values.push(vec![proofs[j * 2].clone(), proofs[j * 2 + 1].clone()].into())
            }

            // Generate the proofs for the reduce layer and update the proofs buffer.
            proofs =
                rt.block_on(async { prover.prove_batch(&reduce_circuit, targets, values).await });
        }

        // Set the proof target with the final proof.
        out_buffer.set_proof_with_pis_target(&self.proof, &proofs[0]);
    }

    fn serialize(&self, dst: &mut Vec<u8>, _: &CommonCircuitData<F, D>) -> IoResult<()> {
        // Write map circuit.
        dst.write_usize(self.map_circuit_id.len())?;
        dst.write_all(self.map_circuit_id.as_bytes())?;

        // Write vector of reduce circuits.
        dst.write_usize(self.reduce_circuit_ids.len())?;
        for i in 0..self.reduce_circuit_ids.len() {
            dst.write_usize(self.reduce_circuit_ids[i].len())?;
            dst.write_all(self.reduce_circuit_ids[i].as_bytes())?;
        }

        // Write vector of input targets.
        dst.write_usize(self.inputs.len())?;
        for i in 0..self.inputs.len() {
            dst.write_target_vec(self.inputs[i].targets().as_slice())?;
        }

        // Write proof target.
        dst.write_target_proof_with_public_inputs(&self.proof)
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        // Read map circuit.
        let map_circuit_id_length = src.read_usize()?;
        let mut map_circuit_id = vec![0u8; map_circuit_id_length];
        src.read_exact(&mut map_circuit_id)?;

        // Read vector of reduce circuits.
        let mut reduce_circuit_ids = Vec::new();
        let reduce_circuit_ids_len = src.read_usize()?;
        for i in 0..reduce_circuit_ids_len {
            let reduce_circuit_id_length = src.read_usize()?;
            let mut reduce_circuit_id = vec![0u8; reduce_circuit_id_length];
            src.read_exact(&mut reduce_circuit_id)?;
            reduce_circuit_ids.push(String::from_utf8(reduce_circuit_id).unwrap());
        }

        // Read vector of input targest.
        let mut inputs = Vec::new();
        let inputs_len = src.read_usize()?;
        for i in 0..inputs_len {
            let input_targets = src.read_target_vec()?;
            inputs.push(I::from_targets(&input_targets));
        }

        // Read proof.
        let proof = src.read_target_proof_with_public_inputs()?;

        Ok(Self {
            map_circuit_id: String::from_utf8(map_circuit_id).unwrap(),
            reduce_circuit_ids,
            inputs,
            proof,
            _phantom1: PhantomData::<F>,
            _phantom2: PhantomData::<C>,
            _phantom3: PhantomData::<O>,
        })
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    fn build_map<I, O, M>(&mut self, map_fn: &M) -> Circuit<L, D>
    where
        I: CircuitVariable,
        O: CircuitVariable,
        M: Fn(I, &mut CircuitBuilder<L, D>) -> O,
    {
        let mut builder = CircuitBuilder::<L, D>::new();
        let input = builder.read::<I>();
        let output = map_fn(input.clone(), &mut builder);
        builder.write(output);
        builder.build()
    }

    fn build_reduce<I, O, R>(
        &mut self,
        child_circuit: &Circuit<L, D>,
        reduce_fn: &R,
    ) -> Circuit<L, D>
    where
        I: CircuitVariable,
        O: CircuitVariable,
        R: Fn(O, O, &mut CircuitBuilder<L, D>) -> O,
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        let mut builder = CircuitBuilder::<L, D>::new();
        let verifier_data = builder.constant_verifier_data(&child_circuit.data);

        let proof_left = builder.proof_read(child_circuit);
        let proof_right = builder.proof_read(child_circuit);

        builder.verify_proof(&proof_left, &verifier_data, &child_circuit.data.common);
        builder.verify_proof(&proof_right, &verifier_data, &child_circuit.data.common);

        let input_left = O::from_targets(&proof_left.public_inputs);
        let input_right = O::from_targets(&proof_right.public_inputs);
        let output = reduce_fn(input_left.clone(), input_right.clone(), &mut builder);

        builder.build()
    }

    pub fn mapreduce<I, O, M, R>(&mut self, inputs: Vec<I>, map_fn: M, reduce_fn: R) -> O
    where
        I: CircuitVariable,
        O: CircuitVariable,
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
        M: Fn(I, &mut CircuitBuilder<L, D>) -> O,
        R: Fn(O, O, &mut CircuitBuilder<L, D>) -> O,
    {
        // The gate and witness generator serializers.
        let gate_serializer = GateRegistry::<L, D>::new();
        let generator_serializer = WitnessGeneratorRegistry::<L, D>::new();

        // Build a map circuit which maps from I -> O using the closure `m`.
        let map_circuit = self.build_map(&map_fn);

        // Save map circuit and map circuit input target to build folder.
        let map_circuit_id = map_circuit.id();
        let map_circuit_path = format!("./build/{}.circuit", map_circuit_id);
        map_circuit.save(&map_circuit_path, &gate_serializer, &generator_serializer);

        // For each reduce layer, we build a reduce circuit which reduces two input proofs
        // to an output O.
        let nb_reduce_layers = (inputs.len() as f64).log2().ceil() as usize;
        let mut reduce_circuits = Vec::new();
        for i in 0..nb_reduce_layers {
            let child_circuit = if i == 0 {
                &map_circuit
            } else {
                &reduce_circuits[i - 1]
            };
            let reduce_circuit = self.build_reduce::<I, O, R>(child_circuit, &reduce_fn);
            let reduce_circuit_id = reduce_circuit.id();
            let reduce_circuit_path = format!("./build/{}.circuit", reduce_circuit_id);
            reduce_circuit.save(
                &reduce_circuit_path,
                &gate_serializer,
                &generator_serializer,
            );
            reduce_circuits.push(reduce_circuit);
        }

        // Create generator to generate map and reduce proofs for each layer.
        let reduce_circuit_ids = reduce_circuits.iter().map(|c| c.id()).collect_vec();
        let final_circuit = &reduce_circuits[reduce_circuits.len() - 1];
        let final_proof = self.add_virtual_proof_with_pis(&final_circuit.data.common);

        // let generator = MapReduceRecursiveProofGenerator::<F, C, I, O, D> {
        //     map_circuit_id,
        //     reduce_circuit_ids,
        //     inputs: inputs.clone(),
        //     proof: proof.clone(),
        //     _phantom1: PhantomData::<F>,
        //     _phantom2: PhantomData::<C>,
        //     _phantom3: PhantomData::<O>,
        // };
        // self.add_simple_generator(&generator);

        // Verify the final proof.
        let final_verifier_data = self.constant_verifier_data(&final_circuit.data);
        self.verify_proof(
            &final_proof,
            &final_verifier_data,
            &final_circuit.data.common,
        );

        // Deserialize the output from the final proof.
        O::from_targets(final_proof.public_inputs.as_slice())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::field::types::Field;

    use crate::prelude::{CircuitBuilder, DefaultParameters, Variable};

    type F = GoldilocksField;
    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_simple_mapreduce_circuit() {
        let mut builder = CircuitBuilder::<L, D>::new();
        let a = builder.constant::<Variable>(F::from_canonical_u64(0));
        let b = builder.constant::<Variable>(F::from_canonical_u64(1));
        let c = builder.constant::<Variable>(F::from_canonical_u64(3));
        let d = builder.constant::<Variable>(F::from_canonical_u64(4));

        let inputs = vec![a, b, c, d];
        let output = builder.mapreduce::<Variable, Variable, _, _>(
            inputs,
            |input, builder| {
                let constant = builder.constant::<Variable>(F::ONE);
                let sum = builder.add(input, constant);
                sum
            },
            |left, right, builder| {
                let sum = builder.add(left, right);
                sum
            },
        );

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
    }
}
