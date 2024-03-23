use core::panic;
use std::marker::PhantomData;

use log::debug;
use plonky2::iop::target::BoolTarget;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};

use crate::backend::circuit::{CircuitBuild, CircuitSerializer};
use crate::frontend::fold::generator::FoldGenerator;
use crate::frontend::fold::util::common_data_for_recursion;
use crate::prelude::{CircuitBuilder, CircuitVariable, PlonkParameters, U32Variable};

pub mod generator;
mod util;

pub trait FoldBuilderMethods<L: PlonkParameters<D>, const D: usize> {
    fn fold<Definition, Ctx, Element, Accumulator, Serializer>(
        &mut self,
        ctx: Ctx,
        initial: Accumulator,
    ) -> (U32Variable, Accumulator)
    where
        Definition: FoldDefinition<Ctx, Element, Accumulator, L, D>
            + std::fmt::Debug
            + Send
            + Sync
            + 'static,
        Ctx: CircuitVariable,
        Element: CircuitVariable,
        Accumulator: CircuitVariable,
        Serializer: CircuitSerializer,
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
        <Element as CircuitVariable>::ValueType<<L as PlonkParameters<D>>::Field>: Sync + Send,
        <Accumulator as CircuitVariable>::ValueType<<L as PlonkParameters<D>>::Field>: Sync + Send;
}

impl<L: PlonkParameters<D>, const D: usize> FoldBuilderMethods<L, D> for CircuitBuilder<L, D> {
    fn fold<Definition, Ctx, Element, Accumulator, Serializer>(
        &mut self,
        ctx: Ctx,
        initial: Accumulator,
    ) -> (U32Variable, Accumulator)
    where
        Definition: FoldDefinition<Ctx, Element, Accumulator, L, D>
            + std::fmt::Debug
            + Send
            + Sync
            + 'static,
        Ctx: CircuitVariable,
        Element: CircuitVariable,
        Accumulator: CircuitVariable,
        Serializer: CircuitSerializer,
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
        <Element as CircuitVariable>::ValueType<<L as PlonkParameters<D>>::Field>: Sync + Send,
        <Accumulator as CircuitVariable>::ValueType<<L as PlonkParameters<D>>::Field>: Sync + Send,
    {
        // Build and save inner circuit.
        let inner_circuit =
            build_inner::<Definition, Ctx, Element, Accumulator, Serializer, L, D>(None);
        debug!("successfully built circuit: id={}", inner_circuit.id());

        let gate_serializer = Serializer::gate_registry::<L, D>();
        let generator_serializer = Serializer::generator_registry::<L, D>();

        // Save cyclic inner circuit to build folder.
        let circuit_id = inner_circuit.id();
        let circuit_path = format!("./build/{}.circuit", circuit_id);
        inner_circuit.save(&circuit_path, &gate_serializer, &generator_serializer);

        // Generate cyclic proofs using generator.
        let final_proof = self.add_virtual_proof_with_pis(&inner_circuit.data.common);
        let generator = FoldGenerator {
            circuit_id,
            ctx: ctx.clone(),
            initial: initial.clone(),
            proof: final_proof.clone(),
            _phantom: PhantomData::<(L, Element, Definition, Accumulator, Serializer)>,
        };
        self.add_simple_generator(generator);

        // Read final proof from generator and verify.
        let final_verifier_data = self.constant_verifier_data::<L>(&inner_circuit.data);
        self.verify_proof::<L>(
            &final_proof,
            &final_verifier_data,
            &inner_circuit.data.common,
        );

        // Verify the inner proof ctx and initial acc are correct.
        let proof_pis = &final_proof.public_inputs;
        debug!("pis: {:?}", proof_pis.len());
        let mut ptr = 0;
        let ctx_elements = Ctx::nb_elements();
        let inner_ctx = Ctx::from_targets(&proof_pis[ptr..ptr + ctx_elements]);
        ptr += ctx_elements;
        let element_elements = Element::nb_elements();
        ptr += element_elements;
        let accumulator_elements = Accumulator::nb_elements();
        ptr += accumulator_elements;
        let inner_initial = Accumulator::from_targets(&proof_pis[ptr..ptr + accumulator_elements]);
        ptr += accumulator_elements;
        let u32_elements = U32Variable::nb_elements();
        let inner_index = U32Variable::from_targets(&proof_pis[ptr..ptr + u32_elements]);
        ptr += u32_elements;
        let inner_output = Accumulator::from_targets(&proof_pis[ptr..ptr + accumulator_elements]);

        self.assert_is_equal(inner_ctx, ctx);
        self.assert_is_equal(inner_initial, initial);

        // Return output acc which is after all inputs
        (inner_index, inner_output)
    }
}

/// Builds the inner circuit for the fold. The circuit takes in context, element, previous acc,
/// initial acc, and index, and outputs the next accumulator.
fn build_inner<Definition, Ctx, Element, Accumulator, Serializer, L, const D: usize>(
    input_data: Option<CommonCircuitData<L::Field, D>>,
) -> CircuitBuild<L, D>
where
    Definition:
        FoldDefinition<Ctx, Element, Accumulator, L, D> + std::fmt::Debug + Send + Sync + 'static,
    Ctx: CircuitVariable,
    Element: CircuitVariable,
    Accumulator: CircuitVariable,
    Serializer: CircuitSerializer,
    L: PlonkParameters<D>,
    <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
        AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    <Element as CircuitVariable>::ValueType<<L as PlonkParameters<D>>::Field>: Sync + Send,
    <Accumulator as CircuitVariable>::ValueType<<L as PlonkParameters<D>>::Field>: Sync + Send,
{
    let mut builder = CircuitBuilder::<L, D>::new();
    // Explicitly enable cyclic IO.
    builder.use_cyclic_recursion();

    let ctx = builder.read::<Ctx>();
    let element = builder.read::<Element>();
    let prev = builder.read::<Accumulator>();
    let initial = builder.read::<Accumulator>();
    let index = builder.read::<U32Variable>();

    let zero_u32 = builder.constant(0u32);
    let true_bool = builder._true();

    // If index = 0, the inner proof will be dummy, and we should use the initial accumulator.
    let should_dummy = builder.is_equal(index, zero_u32);
    let prev_or_initial = builder.select(should_dummy, initial.clone(), prev.clone());

    // Call user defined fold step.
    let next_acc =
        Definition::fold_step(&ctx, element.clone(), prev_or_initial, index, &mut builder);
    builder.write(next_acc);

    // Close cyclic IO so we can use builder.proof_read and get # of public inputs.
    builder.close_cyclic_io();

    // Use dummy data for the first time, then once we know the expected real data, use that.
    let mut common_data = if input_data.is_none() {
        common_data_for_recursion::<L, D>()
    } else {
        input_data.clone().unwrap()
    };
    // Set the number of public inputs
    common_data.num_public_inputs = builder.api.num_public_inputs();
    // Read inner proof and decode its public inputs that we want to verify.
    let inner_cyclic_proof_with_pis = builder.proof_read(&common_data);
    let inner_cyclic_pis = &inner_cyclic_proof_with_pis.public_inputs;
    let mut ptr = 0;
    let ctx_elements = Ctx::nb_elements();
    let inner_ctx = Ctx::from_targets(&inner_cyclic_pis[ptr..ptr + ctx_elements]);
    ptr += ctx_elements;
    let element_elements = Element::nb_elements();
    ptr += element_elements;
    let accumulator_elements = Accumulator::nb_elements();
    ptr += accumulator_elements;
    let inner_initial =
        Accumulator::from_targets(&inner_cyclic_pis[ptr..ptr + accumulator_elements]);
    ptr += accumulator_elements;
    let u32_elements = U32Variable::nb_elements();
    let inner_index = U32Variable::from_targets(&inner_cyclic_pis[ptr..ptr + u32_elements]);
    ptr += u32_elements;
    let inner_output =
        Accumulator::from_targets(&inner_cyclic_pis[ptr..ptr + accumulator_elements]);

    // Ensure ctx is equal.
    let ctx_equal = builder.is_equal(inner_ctx, ctx);
    // Ensure initial acc is equal.
    let initial_equal = builder.is_equal(inner_initial, initial);
    // Ensure prev acc = inner proof output.
    let correct_prev = builder.is_equal(inner_output, prev);
    // Ensure index = inner_index + 1.
    let one_u32 = builder.constant(1_u32);
    let inner_index_plus_one = builder.add(inner_index, one_u32);
    let correct_index = builder.is_equal(inner_index_plus_one, index);

    // AND together above conditions.
    let mut inner_valid = builder.and(ctx_equal, initial_equal);
    inner_valid = builder.and(inner_valid, correct_prev);
    inner_valid = builder.and(inner_valid, correct_index);

    // Only verify inner validity if this is not the 0th proof.
    let inner_valid_or_dummy = builder.or(inner_valid, should_dummy);
    builder.assert_is_equal(inner_valid_or_dummy, true_bool);

    // Verify inner proof or dummy if index = 0.
    let not_dummy = builder.not(should_dummy);
    let should_verify_target = BoolTarget::new_unsafe(not_dummy.variable.0);
    let result = builder
        .api
        .conditionally_verify_cyclic_proof_or_dummy::<L::Config>(
            should_verify_target,
            &inner_cyclic_proof_with_pis,
            &common_data,
        );
    if result.is_err() {
        debug!("error: {:?}", result.err());
        panic!("failed to verify cyclic proof");
    }

    let (build, success) = builder.try_build();

    if !success && input_data.is_none() {
        build_inner::<Definition, _, _, _, Serializer, L, D>(Some(build.data.common))
    } else {
        build
    }
}

pub trait FoldDefinition<Ctx, Element, Accumulator, L, const D: usize>
where
    Ctx: CircuitVariable,
    Element: CircuitVariable,
    Accumulator: CircuitVariable,
    L: PlonkParameters<D>,
    <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
        AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
{
    fn init(ctx: Ctx::ValueType<L::Field>) -> Self;

    fn next(
        &self,
        index: u32,
        current_acc: Accumulator::ValueType<L::Field>,
    ) -> Option<Element::ValueType<L::Field>>;

    fn fold_step(
        ctx: &Ctx,
        element: Element,
        acc: Accumulator,
        index: U32Variable,
        builder: &mut CircuitBuilder<L, D>,
    ) -> Accumulator;
}

#[cfg(test)]
mod tests {

    use log::info;
    use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};

    use super::FoldDefinition;
    use crate::backend::circuit::DefaultSerializer;
    use crate::frontend::fold::FoldBuilderMethods;
    use crate::prelude::{CircuitBuilder, DefaultParameters, PlonkParameters, U32Variable};

    type L = DefaultParameters;
    const D: usize = 2;

    #[derive(Debug, Clone)]
    struct TestFoldDefinition(u32);

    impl<L, const D: usize>
        FoldDefinition<U32Variable, U32Variable, (U32Variable, U32Variable), L, D>
        for TestFoldDefinition
    where
        L: PlonkParameters<D>,
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        fn init(ctx: u32) -> Self {
            Self(ctx)
        }

        fn next(&self, index: u32, _: (u32, u32)) -> Option<u32> {
            if index < self.0 {
                Some((index + 1).pow(2))
            } else {
                None
            }
        }

        fn fold_step(
            _: &U32Variable,
            element: U32Variable,
            acc: (U32Variable, U32Variable),
            index: U32Variable,
            builder: &mut CircuitBuilder<L, D>,
        ) -> (U32Variable, U32Variable) {
            let (_, prev_sum) = acc;
            let new_sum = builder.add(prev_sum, element);
            let one_u32 = builder.constant(1u32);
            let index_plus_one = builder.add(index, one_u32);
            let expected_element = builder.mul(index_plus_one, index_plus_one);
            let is_expected = builder.is_equal(expected_element, element);
            let true_bool = builder._true();
            builder.assert_is_equal(is_expected, true_bool);
            (index_plus_one, new_sum)
        }
    }

    #[test]
    fn test_recursion() {
        let mut builder = CircuitBuilder::<L, D>::new();
        let num = builder.evm_read::<U32Variable>();
        let zero_u32 = builder.constant::<U32Variable>(0u32);
        let (last_index, result) = builder
            .fold::<TestFoldDefinition, _, _, _, DefaultSerializer>(num, (zero_u32, zero_u32));
        builder.evm_write(result.0);
        builder.evm_write(result.1);
        builder.evm_write(last_index);
        let circuit = builder.build();

        let mut input = circuit.input();
        input.evm_write::<U32Variable>(4);
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);

        let mut result = output.clone();
        let last_element = result.evm_read::<U32Variable>();
        let sum = result.evm_read::<U32Variable>();
        let last_index = result.evm_read::<U32Variable>();
        info!("last_element: {}", last_element);
        info!("sum: {}", sum);
        info!("last_index: {}", last_index);

        assert_eq!(last_element, 4);
        assert_eq!(sum, 30);
        assert_eq!(last_index, 3);
    }
}
