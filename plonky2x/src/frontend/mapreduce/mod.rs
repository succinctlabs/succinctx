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
use plonky2::iop::witness::{Witness, WitnessWrite};
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2x_derive::CircuitVariable;

use self::generator::MapReduceGenerator;
use super::hash::poseidon::poseidon256::PoseidonHashOutVariable;
use crate::backend::circuit::CircuitBuild;
use crate::frontend::builder::CircuitBuilder;
use crate::frontend::vars::CircuitVariable;
use crate::prelude::{GateRegistry, PlonkParameters, Variable, WitnessGeneratorRegistry};
use crate::utils::poseidon::compute_binary_merkle_tree_root;
use crate::utils::proof::ProofWithPublicInputsTargetUtils;

/// The input to the map or reduce circuit.
#[derive(Debug, Clone, CircuitVariable)]
struct MapReduceInputVariable<C: CircuitVariable, I: CircuitVariable> {
    /// The context stores a variable passed in from the root circuit. Useful for passing in
    /// small amounts of dynamic data (i.e., a block hash) that is shared among all calls to map
    /// and reduce.
    ctx: C,

    /// The input is a build-time constant passed in from the root circuit. To check the right
    /// input was passed, we hash the inputs and check it matches the expected hash in the root
    /// circuit.
    input: I,
}

/// The output of the map or reduce circuit.
#[derive(Debug, Clone, CircuitVariable)]
struct MapReduceOutputVariable<C: CircuitVariable, O: CircuitVariable> {
    /// The context stores a variable passed in from the root circuit. Useful for passing in
    /// small amounts of dynamic data (i.e., a block hash) that is shared among all calls to map
    /// and reduce.
    ctx: C,

    // The output is the result of the map or reduce function.
    output: O,

    // The accumulator is the hash of the inputs up to this point to the map or reduce function.
    acc: PoseidonHashOutVariable,
}

impl<L: PlonkParameters<D>, const D: usize> CircuitBuilder<L, D> {
    /// Builds a map circuit which maps from I -> O using the closure `m`.
    fn build_map<C, I, O, M>(&mut self, map_fn: &M) -> CircuitBuild<L, D>
    where
        C: CircuitVariable,
        I: CircuitVariable,
        O: CircuitVariable,
        M: Fn(C, I, &mut CircuitBuilder<L, D>) -> O,
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        let mut builder = CircuitBuilder::<L, D>::new();

        // Read the input.
        let data = builder.read::<MapReduceInputVariable<C, I>>();

        // Apply the map function.
        let output = map_fn(data.clone().ctx, data.clone().input, &mut builder);

        // Compute the leaf hash for the input.
        let acc = builder.poseidon_hash_n_to_hash_no_pad(&data.clone().input.variables());

        // Write result.
        let result = MapReduceOutputVariable {
            ctx: data.clone().ctx,
            acc,
            output,
        };
        builder.write(result);

        builder.build()
    }

    /// Builds a reduce circuit which reduces two input proofs to an output O using the closure `r`.
    fn build_reduce<C, O, R>(
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

        // Read and verify the child proofs.
        let verifier_data = builder.constant_verifier_data(&child_circuit.data);
        let proof_left = builder.proof_read(child_circuit);
        builder.verify_proof(&proof_left, &verifier_data, &child_circuit.data.common);
        let proof_right = builder.proof_read(child_circuit);
        builder.verify_proof(&proof_right, &verifier_data, &child_circuit.data.common);

        // Assert that the contexts match.
        let input_left = proof_left.read_end_from_pis::<MapReduceOutputVariable<C, O>>();
        let input_right = proof_right.read_end_from_pis::<MapReduceOutputVariable<C, O>>();
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
        // Compute the expected inputs accumulator.
        let expected_acc = self.constant::<PoseidonHashOutVariable>(
            compute_binary_merkle_tree_root::<L, I, D>(&inputs),
        );

        // The gate and witness generator serializers.
        let gate_serializer = GateRegistry::<L, D>::new();
        let generator_serializer = WitnessGeneratorRegistry::<L, D>::new();

        // Build a map circuit which maps from I -> O using the closure `m`.
        let map_circuit = self.build_map(&map_fn);
        debug!("succesfully built map circuit: id={}", map_circuit.id());

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
            debug!("succesfully built reduce circuit: id={}", reduce_circuit_id);
        }

        // Create generator to generate map and reduce proofs for each layer.
        let reduce_circuit_ids = reduce_circuits.iter().map(|c| c.id()).collect_vec();
        let final_circuit = &reduce_circuits[reduce_circuits.len() - 1];
        let final_proof = self.add_virtual_proof_with_pis(&final_circuit.data.common);
        let generator = MapReduceGenerator::<L, C, I, O, D> {
            map_circuit_id,
            reduce_circuit_ids,
            ctx: ctx.clone(),
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

        // Verify the inputs accumulator.
        let output = final_proof.read_end_from_pis::<MapReduceOutputVariable<C, O>>();
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
