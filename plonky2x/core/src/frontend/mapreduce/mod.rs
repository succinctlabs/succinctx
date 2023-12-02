//! Parallelizable computation within circuits with MapReduce.
//!
//! MapReduce is a programming model for parallelizable computation. In this module, we
//! borrow the MapReduce programming model to parallelize computation within circuits.
//!
//! In the context of circuits, we can use MapReduce to parallelize computation over a list of
//! *compile-time constants*. We can not have dynamic inputs because the outer most circuit
//! cannot store such a large list of inputs.
//!
//! We can use these compile-time constants in combination with a small amount of dynamic
//! data, also referred to as `ctx`, that is shared among all calls to map and reduce. For example,
//! we can pass in a block hash as the dynamic data to the map and reduce functions to say that
//! in each map call we want to grab the storage slot at slot i, which is a compile time constant.
//!
//! Under the hood, we compute each map in a seperate proof and perform the reductions by generating
//! a proof for each reduction between two proofs until we have a single proof.

pub mod generator;

use core::fmt::Debug;
use core::marker::PhantomData;

use itertools::Itertools;
use log::debug;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2x_derive::CircuitVariable;

use self::generator::MapReduceGenerator;
use super::hash::poseidon::poseidon256::PoseidonHashOutVariable;
use crate::backend::circuit::{CircuitBuild, CircuitSerializer};
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::CircuitVariable;
use crate::prelude::{ArrayVariable, PlonkParameters, Variable};
use crate::utils::poseidon::mapreduce_merkle_tree_root;
use crate::utils::proof::ProofWithPublicInputsTargetUtils;

/// The input to the map or reduce circuit.
#[derive(Debug, Clone, CircuitVariable)]
struct MapReduceInputVariable<Ctx: CircuitVariable, Input: CircuitVariable, const B: usize> {
    /// The context stores a variable passed in from the root circuit. Useful for passing in
    /// small amounts of dynamic data (i.e., a block hash) that is shared among all calls to map
    /// and reduce.
    ctx: Ctx,

    /// The input is a build-time constant passed in from the root circuit. To check the right
    /// input was passed, we hash the inputs and check it matches the expected hash in the root
    /// circuit.
    inputs: ArrayVariable<Input, B>,
}

/// The output of the map or reduce circuit.
#[derive(Debug, Clone, CircuitVariable)]
struct MapReduceOutputVariable<Ctx: CircuitVariable, Output: CircuitVariable> {
    /// The context stores a variable passed in from the root circuit. Useful for passing in
    /// small amounts of dynamic data (i.e., a block hash) that is shared among all calls to map
    /// and reduce.
    ctx: Ctx,

    // The output is the result of the map or reduce function.
    output: Output,

    // The accumulator is the hash of the inputs up to this point to the map or reduce function.
    acc: PoseidonHashOutVariable,
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    fn build_map<Ctx, Input, Output, MapFn, const B: usize>(
        &mut self,
        map_fn: &MapFn,
    ) -> CircuitBuild<L, D>
    where
        Ctx: CircuitVariable,
        Input: CircuitVariable,
        Output: CircuitVariable,
        MapFn: Fn(Ctx, ArrayVariable<Input, B>, &mut CircuitBuilder<L, D>) -> Output,
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        let mut builder = CircuitBuilder::<L, D>::new();
        builder.beacon_client = self.beacon_client.clone();
        builder.execution_client = self.execution_client.clone();

        // Read the inputs.
        let data = builder.read::<MapReduceInputVariable<Ctx, Input, B>>();

        // Apply the map function.
        let output = map_fn(data.clone().ctx, data.clone().inputs, &mut builder);

        // Compute the leaf hash for the input.
        let input_variables = data
            .clone()
            .inputs
            .as_vec()
            .iter()
            .flat_map(|i| i.variables())
            .collect_vec();
        let acc = builder.poseidon_hash(&input_variables);

        // Write result.
        let result = MapReduceOutputVariable {
            ctx: data.ctx,
            acc,
            output,
        };
        builder.write(result);
        builder.build()
    }

    fn build_reduce<Ctx, Output, ReduceFn>(
        &mut self,
        child_circuit: &CircuitBuild<L, D>,
        reduce_fn: &ReduceFn,
    ) -> CircuitBuild<L, D>
    where
        Ctx: CircuitVariable,
        Output: CircuitVariable,
        ReduceFn: Fn(Ctx, Output, Output, &mut CircuitBuilder<L, D>) -> Output,
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        let mut builder = CircuitBuilder::<L, D>::new();

        // Read and verify the child proofs.
        let verifier_data = builder.constant_verifier_data::<L>(&child_circuit.data);
        let proof_left = builder.proof_read(&child_circuit.data.common);
        builder.verify_proof::<L>(&proof_left, &verifier_data, &child_circuit.data.common);
        let proof_right = builder.proof_read(&child_circuit.data.common);
        builder.verify_proof::<L>(&proof_right, &verifier_data, &child_circuit.data.common);

        // Assert that the contexts match.
        let input_left = proof_left.read_end_from_pis::<MapReduceOutputVariable<Ctx, Output>>();
        let input_right = proof_right.read_end_from_pis::<MapReduceOutputVariable<Ctx, Output>>();
        builder.assert_is_equal(input_left.clone().ctx, input_right.clone().ctx);

        // Apply the reduce function.
        let output = reduce_fn(
            input_left.clone().ctx,
            input_left.clone().output,
            input_right.clone().output,
            &mut builder,
        );

        // Compute the accumulator hash for the inputs.
        let acc = builder.poseidon_hash_pair(input_left.clone().acc, input_right.clone().acc);

        // Write result.
        let result = MapReduceOutputVariable {
            ctx: input_left.clone().ctx,
            acc,
            output,
        };
        builder.proof_write(result);

        builder.build()
    }

    pub fn mapreduce<Ctx, Input, Output, Serializer, const B: usize, MapFn, ReduceFn>(
        &mut self,
        ctx: Ctx,
        inputs: Vec<Input::ValueType<L::Field>>,
        map_fn: MapFn,
        reduce_fn: ReduceFn,
    ) -> Output
    where
        Ctx: CircuitVariable,
        Input: CircuitVariable,
        Output: CircuitVariable,
        Serializer: CircuitSerializer,
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
        <Input as CircuitVariable>::ValueType<<L as PlonkParameters<D>>::Field>: Sync + Send,
        MapFn: Fn(Ctx, ArrayVariable<Input, B>, &mut CircuitBuilder<L, D>) -> Output,
        ReduceFn: Fn(Ctx, Output, Output, &mut CircuitBuilder<L, D>) -> Output,
    {
        // Sanity checks.
        assert_eq!(inputs.len() % B, 0, "inputs length must be a multiple of B");
        assert!(
            (inputs.len() / B).is_power_of_two(),
            "inputs.len() / B must be a power of two"
        );

        // Compute the expected inputs accumulator.
        let expected_acc =
            self.constant::<PoseidonHashOutVariable>(mapreduce_merkle_tree_root::<L, Input, B, D>(
                &inputs,
            ));

        // The gate and witness generator serializers.
        let gate_serializer = Serializer::gate_registry::<L, D>();
        let generator_serializer = Serializer::generator_registry::<L, D>();

        // Build a map circuit which maps from I -> O using the closure `m`.
        let map_circuit = self.build_map(&map_fn);
        debug!("succesfully built map circuit: id={}", map_circuit.id());

        // Save map circuit and map circuit input target to build folder.
        let map_circuit_id = map_circuit.id();
        let map_circuit_path = format!("./build/{}.circuit", map_circuit_id);
        map_circuit.save(&map_circuit_path, &gate_serializer, &generator_serializer);

        // For each reduce layer, we build a reduce circuit which reduces two input proofs
        // to an output O.
        let nb_reduce_layers = ((inputs.len() / B) as f64).log2().ceil() as usize;
        let mut reduce_circuits = Vec::new();
        for i in 0..nb_reduce_layers {
            let child_circuit = if i == 0 {
                &map_circuit
            } else {
                &reduce_circuits[i - 1]
            };
            let reduce_circuit =
                self.build_reduce::<Ctx, Output, ReduceFn>(child_circuit, &reduce_fn);
            let reduce_circuit_id = reduce_circuit.id();
            let reduce_circuit_path = format!("./build/{}.circuit", reduce_circuit_id);
            reduce_circuit.save(
                &reduce_circuit_path,
                &gate_serializer,
                &generator_serializer,
            );
            reduce_circuits.push(reduce_circuit);
            debug!("succesfully built reduce circuit: id={}", reduce_circuit_id);
        }

        // Create generator to generate map and reduce proofs for each layer.
        let reduce_circuit_ids = reduce_circuits.iter().map(|c| c.id()).collect_vec();
        let final_circuit = &reduce_circuits[reduce_circuits.len() - 1];
        let final_proof = self.add_virtual_proof_with_pis(&final_circuit.data.common);
        let generator = MapReduceGenerator::<L, Ctx, Input, Output, Serializer, B, D> {
            map_circuit_id,
            reduce_circuit_ids,
            ctx: ctx.clone(),
            inputs: inputs.clone(),
            proof: final_proof.clone(),
            _phantom: PhantomData,
        };
        self.add_simple_generator(generator);

        // Verify the final proof.
        let final_verifier_data = self.constant_verifier_data::<L>(&final_circuit.data);
        self.verify_proof::<L>(
            &final_proof,
            &final_verifier_data,
            &final_circuit.data.common,
        );

        // Verify the inputs accumulator.
        let output = final_proof.read_end_from_pis::<MapReduceOutputVariable<Ctx, Output>>();
        self.assert_is_equal(output.acc, expected_acc);

        // Verify the context.
        self.assert_is_equal(output.ctx, ctx);

        // Return the output.
        output.output
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::field::types::Field;

    use crate::backend::circuit::DefaultSerializer;
    use crate::prelude::{CircuitBuilder, DefaultParameters, Variable};

    type F = GoldilocksField;
    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_simple_mapreduce_circuit() {
        env_logger::try_init().unwrap_or_default();

        let mut builder = CircuitBuilder::<L, D>::new();
        let ctx = builder.constant::<Variable>(F::from_canonical_u64(8));
        let inputs = vec![
            F::from_canonical_u64(0),
            F::from_canonical_u64(1),
            F::from_canonical_u64(2),
            F::from_canonical_u64(3),
        ];

        let output = builder.mapreduce::<Variable, Variable, Variable, DefaultSerializer, 2, _, _>(
            ctx,
            inputs,
            |ctx, inputs, builder| {
                builder.watch(&ctx, "ctx");
                let constant = builder.constant::<Variable>(F::ONE);
                let o1 = builder.add(inputs[0], constant);
                let o2 = builder.add(inputs[1], constant);
                builder.add(o1, o2)
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
