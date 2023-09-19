use core::fmt::Debug;
use core::marker::PhantomData;

use itertools::Itertools;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};
use plonky2::plonk::circuit_data::{CircuitData, CommonCircuitData};
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputsTarget;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};
use tokio::runtime::Runtime;

use crate::backend::prover::enviroment::EnviromentProver;
use crate::backend::prover::Prover;
use crate::frontend::builder::CircuitBuilder;
use crate::vars::CircuitVariable;

#[derive(Debug, Clone)]
pub struct MapReduceRecursiveProofGenerator<F, C, I, O, const D: usize>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F> + 'static,
    <C as GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
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

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        // Create the prover and the async runtime.
        let prover = EnviromentProver::new();
        let rt = Runtime::new().expect("failed to create tokio runtime");

        // Load the map circuit from disk & generate the proofs.
        let map_circuit_path = format!("./build/{}.circuit", self.map_circuit_id);
        let (map_circuit, map_circuit_input) = CircuitData::<F, C, D>::load::<I>(map_circuit_path);
        let targets = map_circuit_input.targets().into();
        let values = self
            .inputs
            .iter()
            .map(|x| witness.get_targets(x.targets().as_slice()).into())
            .collect_vec();
        let mut proofs =
            rt.block_on(async { prover.prove_batch(&map_circuit, targets, values).await });

        // Each reduce layer takes N leave proofs and produces N / 2 proofs using a simple
        // linear and binary reduction strategy.
        let nb_reduce_layers = (self.inputs.len() as f64).log2().ceil() as usize;
        for i in 0..nb_reduce_layers {
            let reduce_circuit_path = format!("./build/{}.circuit", self.reduce_circuit_ids[i]);
            let (reduce_circuit, _, reduce_circuit_inputs) =
                CircuitData::<F, C, D>::load_with_proof_targets(reduce_circuit_path);
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

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    fn build_map_circuit<I, O, C, M>(&mut self, m: &M) -> (CircuitData<F, C, D>, I)
    where
        I: CircuitVariable,
        O: CircuitVariable,
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

    fn build_reduce_circuit<I, O, C, R>(
        &mut self,
        cd: &CircuitData<F, C, D>,
        r: &R,
    ) -> (CircuitData<F, C, D>, Vec<ProofWithPublicInputsTarget<D>>)
    where
        I: CircuitVariable,
        O: CircuitVariable,
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
        I: CircuitVariable,
        O: CircuitVariable,
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
            reduce_circuit.save_with_proof_targets(
                child_circuit_data,
                &reduce_circuit_inputs,
                reduce_circuit_path,
            );
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
    use log::debug;
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::field::types::Field;
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;

    use crate::builder::CircuitBuilder;
    use crate::utils;
    use crate::vars::{CircuitVariable, Variable};

    #[test]
    fn test_simple_mapreduce_circuit() {
        utils::setup_logger();
        type F = GoldilocksField;
        type C = PoseidonGoldilocksConfig;
        const D: usize = 2;

        let mut builder = CircuitBuilder::<F, D>::new();

        let a = builder.constant::<Variable>(F::from_canonical_u64(0));
        let b = builder.constant::<Variable>(F::from_canonical_u64(1));
        let c = builder.constant::<Variable>(F::from_canonical_u64(3));
        let d = builder.constant::<Variable>(F::from_canonical_u64(4));

        let inputs = vec![a, b, c, d];
        let output = builder.mapreduce::<Variable, Variable, C, _, _>(
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
        builder.register_public_inputs(output.targets().as_slice());

        debug!("compiling outer circuit");
        let data = builder.build::<C>();

        debug!("proving outer circuit");
        let pw = PartialWitness::new();
        let proof = data.prove(pw).unwrap();
        data.verify(proof.clone()).unwrap();

        debug!("result: {:?}", proof.public_inputs);
    }
}
