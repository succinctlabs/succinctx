use core::fmt::Debug;
use core::marker::PhantomData;

use itertools::Itertools;
use plonky2::hash::hash_types::{RichField, NUM_HASH_OUT_ELTS};
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::ProofWithPublicInputsTarget;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};
use plonky2x_derive::CircuitVariable;
use tokio::runtime::Runtime;

use super::hash::poseidon::poseidon256::PoseidonHashOutVariable;
use crate::backend::circuit::CircuitBuild;
use crate::backend::prover::{EnvProver, Prover};
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::CircuitVariable;
use crate::prelude::{GateRegistry, PlonkParameters, Variable, WitnessGeneratorRegistry};
use crate::utils::poseidon::compute_binary_merkle_tree_root;

trait ProofWithPublicInputsTargetUtils {
    fn read_start_from_pis<V: CircuitVariable>(&self) -> V;
    fn read_end_from_pis<V: CircuitVariable>(&self) -> V;
}

impl<const D: usize> ProofWithPublicInputsTargetUtils for ProofWithPublicInputsTarget<D> {
    fn read_start_from_pis<V: CircuitVariable>(&self) -> V {
        V::from_targets(&self.public_inputs[..V::nb_elements()])
    }

    fn read_end_from_pis<V: CircuitVariable>(&self) -> V {
        let public_inputs_len = self.public_inputs.len();
        V::from_targets(&self.public_inputs[public_inputs_len - V::nb_elements()..])
    }
}

#[derive(Debug, Clone, CircuitVariable)]
struct MapReduceInputVariable<C: CircuitVariable, I: CircuitVariable> {
    ctx: C,
    input: I,
}

#[derive(Debug, Clone, CircuitVariable)]
struct MapReduceOutputVariable<C: CircuitVariable, O: CircuitVariable> {
    ctx: C,
    output: O,
    acc: PoseidonHashOutVariable,
}

#[derive(Debug, Clone)]
pub struct MapReduceRecursiveProofGenerator<L, C, I, O, const D: usize>
where
    L: PlonkParameters<D>,
    <L as PlonkParameters<D>>::Config: GenericConfig<D, F = L::Field> + 'static,
    <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    C: CircuitVariable,
    I: CircuitVariable,
    O: CircuitVariable,
{
    /// The identifier for the map circuit.
    pub map_circuit_id: String,

    /// The identifiers for the reduce circuits.
    pub reduce_circuit_ids: Vec<String>,

    /// The global context for all circuits.
    pub ctx: C,

    /// The constant inputs to the map circuit.
    pub inputs: Vec<I::ValueType<L::Field>>,

    /// The proof target for the final circuit proof.
    pub proof: ProofWithPublicInputsTarget<D>,

    /// Phantom data.
    pub _phantom1: PhantomData<L>,
    pub _phantom2: PhantomData<O>,
}

impl<L, C, I, O, const D: usize> SimpleGenerator<L::Field, D>
    for MapReduceRecursiveProofGenerator<L, C, I, O, D>
where
    L: PlonkParameters<D>,
    <L as PlonkParameters<D>>::Config: GenericConfig<D, F = L::Field> + 'static,
    <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    C: CircuitVariable,
    I: CircuitVariable,
    O: CircuitVariable,
    <I as CircuitVariable>::ValueType<<L as PlonkParameters<D>>::Field>: Sync + Send,
{
    fn id(&self) -> String {
        "MapReduceRecursiveProofGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets = Vec::new();
        targets.extend(self.ctx.targets());
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
            CircuitBuild::<L, D>::load(&map_circuit_path, &gate_serializer, &generator_serializer)
                .unwrap();

        // Calculate the inputs to the map.
        let ctx_value = self.ctx.get(witness);
        let map_input_values = &self.inputs;
        let mut map_inputs = Vec::new();
        for map_input_value in map_input_values {
            let mut map_input = map_circuit.input();
            map_input.write::<MapReduceInputVariable<C, I>>(MapReduceInputVariableValue {
                ctx: ctx_value.clone(),
                input: map_input_value.to_owned(),
            });
            map_inputs.push(map_input)
        }

        // Generate the proofs for the map layer.
        let (mut proofs, _) =
            rt.block_on(async { prover.batch_prove(&map_circuit, &map_inputs).await.unwrap() });

        // Process each reduce layer.
        let nb_reduce_layers = (self.inputs.len() as f64).log2().ceil() as usize;
        for i in 0..nb_reduce_layers {
            // Load the reduce circuit from disk.
            let reduce_circuit_path = format!("./build/{}.circuit", self.reduce_circuit_ids[i]);
            let reduce_circuit = CircuitBuild::<L, D>::load(
                &reduce_circuit_path,
                &gate_serializer,
                &generator_serializer,
            )
            .unwrap();

            // Calculate the inputs to the reduce layer.
            let nb_proofs = self.inputs.len() / (2usize.pow((i + 1) as u32));
            let mut reduce_inputs = Vec::new();
            for j in 0..nb_proofs {
                let mut reduce_input = reduce_circuit.input();
                reduce_input.proof_write(proofs[j * 2].clone());
                reduce_input.proof_write(proofs[j * 2 + 1].clone());
                reduce_inputs.push(reduce_input);
            }

            // Generate the proofs for the reduce layer and update the proofs buffer.
            (proofs, _) = rt.block_on(async {
                prover
                    .batch_prove(&reduce_circuit, &reduce_inputs)
                    .await
                    .unwrap()
            });
        }

        // Set the proof target with the final proof.
        out_buffer.set_proof_with_pis_target(&self.proof, &proofs[0]);
    }

    fn serialize(&self, dst: &mut Vec<u8>, _: &CommonCircuitData<L::Field, D>) -> IoResult<()> {
        // Write map circuit.
        dst.write_usize(self.map_circuit_id.len())?;
        dst.write_all(self.map_circuit_id.as_bytes())?;

        // Write vector of reduce circuits.
        dst.write_usize(self.reduce_circuit_ids.len())?;
        for i in 0..self.reduce_circuit_ids.len() {
            dst.write_usize(self.reduce_circuit_ids[i].len())?;
            dst.write_all(self.reduce_circuit_ids[i].as_bytes())?;
        }

        // Write context.
        dst.write_target_vec(&self.ctx.targets())?;

        // Write vector of input values.
        dst.write_usize(self.inputs.len())?;
        for i in 0..self.inputs.len() {
            dst.write_field_vec::<L::Field>(&I::elements::<L, D>(self.inputs[i].clone()))?;
        }

        // Write proof target.
        dst.write_target_proof_with_public_inputs(&self.proof)
    }

    fn deserialize(src: &mut Buffer, _: &CommonCircuitData<L::Field, D>) -> IoResult<Self> {
        // Read map circuit.
        let map_circuit_id_length = src.read_usize()?;
        let mut map_circuit_id = vec![0u8; map_circuit_id_length];
        src.read_exact(&mut map_circuit_id)?;

        // Read vector of reduce circuits.
        let mut reduce_circuit_ids = Vec::new();
        let reduce_circuit_ids_len = src.read_usize()?;
        for _ in 0..reduce_circuit_ids_len {
            let reduce_circuit_id_length = src.read_usize()?;
            let mut reduce_circuit_id = vec![0u8; reduce_circuit_id_length];
            src.read_exact(&mut reduce_circuit_id)?;
            reduce_circuit_ids.push(String::from_utf8(reduce_circuit_id).unwrap());
        }

        // Read context.
        let ctx = C::from_targets(&src.read_target_vec()?);

        // Read vector of input targest.
        let mut inputs = Vec::new();
        let inputs_len = src.read_usize()?;
        for _ in 0..inputs_len {
            let input_elements: Vec<L::Field> = src.read_field_vec(I::nb_elements())?;
            inputs.push(I::from_elements::<L, D>(&input_elements));
        }

        // Read proof.
        let proof = src.read_target_proof_with_public_inputs()?;

        Ok(Self {
            map_circuit_id: String::from_utf8(map_circuit_id).unwrap(),
            reduce_circuit_ids,
            ctx,
            inputs,
            proof,
            _phantom1: PhantomData::<L>,
            _phantom2: PhantomData::<O>,
        })
    }
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Builds a map circuit which maps from I -> O using the closure `m`.
    pub fn build_map<C, I, O, M>(&mut self, map_fn: &M) -> CircuitBuild<L, D>
    where
        C: CircuitVariable,
        I: CircuitVariable,
        O: CircuitVariable,
        M: Fn(C, I, &mut CircuitBuilder<L, D>) -> O,
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        let mut builder = CircuitBuilder::<L, D>::new();
        let data = builder.read::<MapReduceInputVariable<C, I>>();
        let output = map_fn(data.clone().ctx, data.clone().input, &mut builder);
        let acc = builder.poseidon_hash_n_to_hash_no_pad(&data.clone().input.variables());
        let result = MapReduceOutputVariable {
            ctx: data.clone().ctx,
            acc,
            output,
        };
        builder.write(result);
        builder.build()
    }

    /// Builds a reduce circuit which reduces two input proofs to an output O using the closure `r`.
    pub fn build_reduce<C, O, R>(
        &mut self,
        child_circuit: &CircuitBuild<L, D>,
        reduce_fn: &R,
    ) -> CircuitBuild<L, D>
    where
        C: CircuitVariable,
        O: CircuitVariable,
        R: Fn(C, O, O, &mut CircuitBuilder<L, D>) -> O,
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        let mut builder = CircuitBuilder::<L, D>::new();
        let verifier_data = builder.constant_verifier_data(&child_circuit.data);

        let proof_left = builder.proof_read(child_circuit);
        builder.verify_proof(&proof_left, &verifier_data, &child_circuit.data.common);

        let proof_right = builder.proof_read(child_circuit);
        builder.verify_proof(&proof_right, &verifier_data, &child_circuit.data.common);

        let input_left = proof_left.read_end_from_pis::<MapReduceOutputVariable<C, O>>();
        let input_right = proof_right.read_end_from_pis::<MapReduceOutputVariable<C, O>>();
        builder.assert_is_equal(input_left.clone().ctx, input_right.clone().ctx);

        let output = reduce_fn(
            input_left.clone().ctx,
            input_left.clone().output,
            input_right.clone().output,
            &mut builder,
        );
        let acc = builder.poseidon_hash_pair(input_left.clone().acc, input_right.clone().acc);
        let result = MapReduceOutputVariable {
            ctx: input_left.clone().ctx,
            acc,
            output,
        };

        builder.proof_write(result);
        builder.build()
    }

    pub fn mapreduce<C, I, O, M, R>(
        &mut self,
        ctx: C,
        inputs: Vec<I::ValueType<L::Field>>,
        map_fn: M,
        reduce_fn: R,
    ) -> O
    where
        C: CircuitVariable,
        I: CircuitVariable,
        O: CircuitVariable,
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
        <I as CircuitVariable>::ValueType<<L as PlonkParameters<D>>::Field>: Sync + Send,
        M: Fn(C, I, &mut CircuitBuilder<L, D>) -> O,
        R: Fn(C, O, O, &mut CircuitBuilder<L, D>) -> O,
    {
        // Compute the expected inputs hash.
        let expected_inputs_hash = self.constant::<PoseidonHashOutVariable>(
            compute_binary_merkle_tree_root::<L, I, D>(&inputs),
        );

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
            let reduce_circuit = self.build_reduce::<C, O, R>(child_circuit, &reduce_fn);
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

        let generator = MapReduceRecursiveProofGenerator::<L, C, I, O, D> {
            map_circuit_id,
            reduce_circuit_ids,
            ctx,
            inputs: inputs.clone(),
            proof: final_proof.clone(),
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        };
        self.add_simple_generator(generator);

        // Verify the final proof.
        let final_verifier_data = self.constant_verifier_data(&final_circuit.data);
        self.verify_proof(
            &final_proof,
            &final_verifier_data,
            &final_circuit.data.common,
        );

        // Verify the inputs hash.
        let output = final_proof.read_end_from_pis::<MapReduceOutputVariable<C, O>>();
        self.assert_is_equal(output.acc, expected_inputs_hash);

        // Return the output.
        output.output
    }

    pub fn compute_binary_merkle_tree_root<V: CircuitVariable>(
        &mut self,
        variables: &[V],
    ) -> PoseidonHashOutVariable
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        let variables = variables.to_vec();

        // Calculate leafs.
        let mut leafs = Vec::new();
        for i in 0..variables.len() {
            let input = &variables[i];
            let h = self.poseidon_hash_n_to_hash_no_pad(&input.variables());
            leafs.push(h);
        }

        // Pad leafs to a power of two with the zero leaf.
        let zero = self.zero();
        let h_zero = PoseidonHashOutVariable::from_variables(&[zero; NUM_HASH_OUT_ELTS]);
        while leafs.len() < leafs.len().next_power_of_two() {
            leafs.push(h_zero.clone());
        }

        // Calculate the root.
        while leafs.len() != 1 {
            let mut tmp = Vec::new();
            for i in 0..leafs.len() / 2 {
                let left = leafs[i * 2].clone();
                let right = leafs[i * 2 + 1].clone();
                let h = self.poseidon_hash_pair(left, right);
                self.watch(&h, "h");
                tmp.push(h);
            }
            leafs = tmp;
        }

        leafs[0].to_owned()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::field::types::Field;

    use crate::frontend::hash::poseidon::poseidon256::PoseidonHashOutVariable;
    use crate::prelude::{CircuitBuilder, DefaultParameters, Variable};
    use crate::utils::poseidon::compute_binary_merkle_tree_root;

    type F = GoldilocksField;
    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_binary_merkle_root_match() {
        env_logger::try_init().unwrap_or_default();

        let values = [
            F::from_canonical_u64(0),
            F::from_canonical_u64(1),
            F::from_canonical_u64(2),
            F::from_canonical_u64(0),
        ];
        let hash = compute_binary_merkle_tree_root::<L, Variable, D>(&values);

        let mut builder = CircuitBuilder::<L, D>::new();
        let a = builder.constant::<Variable>(values[0]);
        let b = builder.constant::<Variable>(values[1]);
        let c = builder.constant::<Variable>(values[2]);
        let d = builder.constant::<Variable>(values[3]);
        let h = builder.compute_binary_merkle_tree_root(&[a, b, c, d]);
        builder.write(h);

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        let result = output.read::<PoseidonHashOutVariable>();

        result
            .elements
            .iter()
            .zip(hash.elements.iter())
            .for_each(|(a, b)| {
                assert_eq!(a, b);
            });
    }

    #[test]
    fn test_simple_mapreduce_circuit() {
        env_logger::try_init().unwrap_or_default();

        let mut builder = CircuitBuilder::<L, D>::new();
        let ctx = builder.constant::<Variable>(F::from_canonical_u64(8));
        let inputs = vec![
            F::from_canonical_u64(0),
            F::from_canonical_u64(1),
            F::from_canonical_u64(2),
            F::from_canonical_u64(0),
        ];

        let output = builder.mapreduce::<Variable, Variable, Variable, _, _>(
            ctx,
            inputs,
            |ctx, input, builder| {
                builder.watch(&ctx, "ctx");
                let constant = builder.constant::<Variable>(F::ONE);
                builder.add(input, constant)
            },
            |ctx, left, right, builder| {
                builder.watch(&ctx, "ctx");
                builder.add(left, right)
            },
        );
        builder.watch(&output, "output");
        builder.write(output);

        let circuit = builder.build();
        let input = circuit.input();
        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        let result = output.read::<Variable>();
        println!("{}", result);
    }
}
