use std::marker::PhantomData;

use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2::plonk::proof::{ProofWithPublicInputs, ProofWithPublicInputsTarget};
use plonky2::recursion::dummy_circuit::cyclic_base_proof;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};
use serde::{Deserialize, Serialize};

use super::FoldDefinition;
use crate::backend::circuit::{CircuitBuild, CircuitSerializer, DefaultSerializer, PublicOutput};
use crate::prelude::{CircuitVariable, PlonkParameters, U32Variable, WitnessWrite};
use crate::utils::serde::{deserialize_proof_with_pis_target, serialize_proof_with_pis_target};

#[derive(Debug, Serialize, Deserialize)]
pub struct FoldGenerator<Definition, Ctx, Element, Accumulator, Serializer, L, const D: usize>
where
    Definition:
        FoldDefinition<Ctx, Element, Accumulator, L, D> + std::fmt::Debug + Send + Sync + 'static,
    L: PlonkParameters<D>,
    Ctx: CircuitVariable,
    Element: CircuitVariable,
    Accumulator: CircuitVariable,
    Serializer: CircuitSerializer,
    <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
        AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
{
    pub circuit_id: String,
    pub ctx: Ctx,
    pub initial: Accumulator,
    #[serde(serialize_with = "serialize_proof_with_pis_target")]
    #[serde(deserialize_with = "deserialize_proof_with_pis_target")]
    pub proof: ProofWithPublicInputsTarget<D>,
    pub _phantom: PhantomData<(L, Element, Definition, Accumulator, Serializer)>,
}

impl<Definition, Ctx, Element, Accumulator, Serializer, L, const D: usize> Clone
    for FoldGenerator<Definition, Ctx, Element, Accumulator, Serializer, L, D>
where
    Definition:
        FoldDefinition<Ctx, Element, Accumulator, L, D> + std::fmt::Debug + Send + Sync + 'static,
    L: PlonkParameters<D>,
    Ctx: CircuitVariable,
    Element: CircuitVariable,
    Accumulator: CircuitVariable,
    Serializer: CircuitSerializer,
    <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
        AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
{
    fn clone(&self) -> Self {
        Self {
            circuit_id: self.circuit_id.clone(),
            ctx: self.ctx.clone(),
            initial: self.initial.clone(),
            proof: self.proof.clone(),
            _phantom: PhantomData::<(L, Element, Definition, Accumulator, Serializer)>,
        }
    }
}

impl<Definition, Ctx, Element, Accumulator, Serializer, L, const D: usize>
    FoldGenerator<Definition, Ctx, Element, Accumulator, Serializer, L, D>
where
    Definition:
        FoldDefinition<Ctx, Element, Accumulator, L, D> + std::fmt::Debug + Send + Sync + 'static,
    L: PlonkParameters<D>,
    Ctx: CircuitVariable,
    Element: CircuitVariable,
    Accumulator: CircuitVariable,
    Serializer: CircuitSerializer,
    <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
        AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
{
    pub fn id() -> String {
        "FoldGenerator".to_string()
    }
}

#[allow(clippy::type_complexity)]
fn prove_cycle<Ctx, Element, Accumulator, L, const D: usize>(
    circuit: &CircuitBuild<L, D>,
    ctx: Ctx::ValueType<L::Field>,
    acc: Accumulator::ValueType<L::Field>,
    initial: Accumulator::ValueType<L::Field>,
    element: Element::ValueType<L::Field>,
    index: u32,
    prev_result: &mut Option<(
        ProofWithPublicInputs<L::Field, L::Config, D>,
        PublicOutput<L, D>,
    )>,
) -> (
    ProofWithPublicInputs<L::Field, L::Config, D>,
    PublicOutput<L, D>,
)
where
    L: PlonkParameters<D>,
    Ctx: CircuitVariable,
    Element: CircuitVariable,
    Accumulator: CircuitVariable,
    <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
{
    let mut input = circuit.input();
    input.write::<Ctx>(ctx);
    input.write::<Element>(element);
    input.write::<Accumulator>(acc);
    input.write::<Accumulator>(initial);
    input.write::<U32Variable>(index);
    input.data_write(circuit.data.verifier_data());
    let proof = if let Some((proof, _)) = prev_result.take() {
        proof
    } else {
        cyclic_base_proof(
            &circuit.data.common,
            &circuit.data.verifier_only,
            vec![].into_iter().enumerate().collect(),
        )
    };
    input.proof_write(proof);
    circuit.prove(&input)
}

impl<Definition, Ctx, Element, Accumulator, Serializer, L, const D: usize>
    SimpleGenerator<L::Field, D>
    for FoldGenerator<Definition, Ctx, Element, Accumulator, Serializer, L, D>
where
    Definition:
        FoldDefinition<Ctx, Element, Accumulator, L, D> + std::fmt::Debug + Send + Sync + 'static,
    L: PlonkParameters<D>,
    Ctx: CircuitVariable,
    Element: CircuitVariable,
    Accumulator: CircuitVariable,
    Serializer: CircuitSerializer,
    <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
        AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
{
    fn id(&self) -> String {
        Self::id()
    }

    fn dependencies(&self) -> Vec<Target> {
        let mut targets = Vec::new();
        targets.extend(self.ctx.targets());
        targets.extend(self.initial.targets());
        targets
    }

    #[allow(clippy::type_complexity)]
    fn run_once(
        &self,
        witness: &PartitionWitness<L::Field>,
        out_buffer: &mut GeneratedValues<L::Field>,
    ) {
        let gate_serializer = DefaultSerializer::gate_registry::<L, D>();
        let generator_serializer = DefaultSerializer::generator_registry::<L, D>();
        let circuit_path = format!("./build/{}.circuit", self.circuit_id);
        let circuit =
            CircuitBuild::<L, D>::load(&circuit_path, &gate_serializer, &generator_serializer)
                .unwrap();

        let ctx_value = self.ctx.get(witness);
        let initial_value = self.initial.get(witness);

        let iterator = Definition::init(ctx_value.clone());

        let mut last_result: Option<(
            ProofWithPublicInputs<
                <L as PlonkParameters<D>>::Field,
                <L as PlonkParameters<D>>::Config,
                D,
            >,
            PublicOutput<L, D>,
        )> = None;
        let mut i = 0_u32;
        loop {
            // If first iteration, use initial value, otherwise use previous accumulator.
            let prev_acc = if let Some((_, ref mut output)) = last_result {
                output.read::<Accumulator>()
            } else {
                initial_value.clone()
            };

            // Call user definition to get next element.
            let element = iterator.next(i, prev_acc.clone());
            if element.is_none() {
                break;
            }

            // Generate proof.
            last_result = Some(prove_cycle::<Ctx, Element, Accumulator, L, D>(
                &circuit,
                ctx_value.clone(),
                prev_acc,
                initial_value.clone(),
                element.unwrap(),
                i,
                &mut last_result,
            ));
            i += 1;
        }

        out_buffer.set_proof_with_pis_target(&self.proof, &last_result.unwrap().0);
    }

    fn serialize(&self, dst: &mut Vec<u8>, _: &CommonCircuitData<L::Field, D>) -> IoResult<()> {
        // Write map circuit.
        dst.write_usize(self.circuit_id.len())?;
        dst.write_all(self.circuit_id.as_bytes())?;

        // Write context.
        dst.write_target_vec(&self.ctx.targets())?;

        dst.write_target_vec(&self.initial.targets())?;

        // Write proof target.
        dst.write_target_proof_with_public_inputs(&self.proof)
    }

    fn deserialize(src: &mut Buffer, _: &CommonCircuitData<L::Field, D>) -> IoResult<Self> {
        // Read map circuit.
        let circuit_id_length = src.read_usize()?;
        let mut circuit_id = vec![0u8; circuit_id_length];
        src.read_exact(&mut circuit_id)?;

        // Read context.
        let ctx = Ctx::from_targets(&src.read_target_vec()?);

        let initial = Accumulator::from_targets(&src.read_target_vec()?);

        // Read proof.
        let proof: ProofWithPublicInputsTarget<D> = src.read_target_proof_with_public_inputs()?;

        // todo!()
        Ok(Self {
            circuit_id: String::from_utf8(circuit_id).unwrap(),
            ctx,
            initial,
            proof,
            _phantom: PhantomData::<(L, Element, Definition, Accumulator, Serializer)>,
        })
    }
}
