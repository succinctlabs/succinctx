use alloc::collections::BTreeMap;

use anyhow::Result;
use curta::maybe_rayon::rayon;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::GeneratedValues;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartialWitness, PartitionWitness, Witness, WitnessWrite};
use plonky2::plonk::circuit_data::{CommonCircuitData, ProverOnlyCircuitData};
use plonky2::plonk::config::GenericConfig;
use tokio::sync::mpsc::unbounded_channel;

use super::PlonkParameters;
use crate::frontend::generator::asynchronous::generator::AsyncGeneratorRef;
use crate::frontend::generator::asynchronous::handler::HintHandler;

#[derive(Debug, Clone)]
pub enum GenerateWitnessError {
    GeneratorsNotRun(Vec<Target>),
}

/// Given a `PartialWitness` that has only inputs set, populates the rest of the witness using the
/// given set of generators.
pub fn generate_witness<
    'a,
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F>,
    const D: usize,
>(
    pw: PartialWitness<F>,
    prover_data: &'a ProverOnlyCircuitData<F, C, D>,
    common_data: &'a CommonCircuitData<F, D>,
) -> Result<PartitionWitness<'a, F>, GenerateWitnessError> {
    let config = &common_data.config;
    let generators = &prover_data.generators;
    let generator_indices_by_watches = &prover_data.generator_indices_by_watches;

    let mut witness = PartitionWitness::new(
        config.num_wires,
        common_data.degree(),
        &prover_data.representative_map,
    );

    for (t, v) in pw.target_values.into_iter() {
        witness.set_target(t, v);
    }

    // Build a list of "pending" generators which are queued to be run. Initially, all generators
    // are queued.
    let mut pending_generator_indices: Vec<_> = (0..generators.len()).collect();

    // We also track a list of "expired" generators which have already returned false.
    let mut generator_is_expired = vec![false; generators.len()];
    let mut remaining_generators = generators.len();

    let mut buffer = GeneratedValues::empty();

    // Keep running generators until we fail to make progress.
    while !pending_generator_indices.is_empty() {
        let mut next_pending_generator_indices = Vec::new();

        for &generator_idx in &pending_generator_indices {
            if generator_is_expired[generator_idx] {
                continue;
            }

            let finished = generators[generator_idx].0.run(&witness, &mut buffer);
            if finished {
                generator_is_expired[generator_idx] = true;
                remaining_generators -= 1;
            }

            // Merge any generated values into our witness, and get a list of newly-populated
            // targets' representatives.
            let new_target_reps = buffer
                .target_values
                .drain(..)
                .flat_map(|(t, v)| witness.set_target_returning_rep(t, v));

            // Enqueue unfinished generators that were watching one of the newly populated targets.
            for watch in new_target_reps {
                let opt_watchers = generator_indices_by_watches.get(&watch);
                if let Some(watchers) = opt_watchers {
                    for &watching_generator_idx in watchers {
                        if !generator_is_expired[watching_generator_idx] {
                            next_pending_generator_indices.push(watching_generator_idx);
                        }
                    }
                }
            }
        }

        pending_generator_indices = next_pending_generator_indices;
    }

    if remaining_generators > 0 {
        let mut unpopulated_targets = Vec::new();
        for i in 0..generator_is_expired.len() {
            if !generator_is_expired[i] {
                let generator = &generators[i];
                let watch_list = generator.0.watch_list();
                for t in watch_list {
                    if witness.try_get_target(t).is_none() {
                        unpopulated_targets.push(t);
                    }
                }
            }
        }
        return Err(GenerateWitnessError::GeneratorsNotRun(unpopulated_targets));
    }

    assert_eq!(
        remaining_generators, 0,
        "{} generators weren't run",
        remaining_generators,
    );

    Ok(witness)
}

pub fn generate_partial_witness_with_hints<'a, L: PlonkParameters<D>, const D: usize>(
    inputs: PartialWitness<L::Field>,
    prover_data: &'a ProverOnlyCircuitData<L::Field, L::Config, D>,
    common_data: &'a CommonCircuitData<L::Field, D>,
    async_generator_refs: &'a BTreeMap<usize, AsyncGeneratorRef<L, D>>,
) -> PartitionWitness<'a, L::Field> {
    let config = &common_data.config;
    let generators = &prover_data.generators;
    let generator_indices_by_watches = &prover_data.generator_indices_by_watches;

    let mut witness = PartitionWitness::new(
        config.num_wires,
        common_data.degree(),
        &prover_data.representative_map,
    );

    for (t, v) in inputs.target_values.into_iter() {
        witness.set_target(t, v);
    }

    // Build a list of "pending" generators which are queued to be run. Initially, all generators
    // are queued.
    let mut pending_generator_indices: Vec<_> = (0..generators.len()).collect();

    // We also track a list of "expired" generators which have already returned false.
    let mut generator_is_expired = vec![false; generators.len()];
    let mut remaining_generators = generators.len();
    let mut remaining_async_generators = async_generator_refs.len();

    let mut buffer = GeneratedValues::empty();

    let (tx, rx) = unbounded_channel();
    let mut hint_handler = HintHandler::<L, D>::new(rx);

    let async_generators = BTreeMap::from_iter(
        async_generator_refs
            .iter()
            .map(|(i, g)| (*i, g.0.generator(tx.clone()))),
    );

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");

    rayon::spawn(move || {
        rt.block_on(hint_handler.run()).unwrap();
    });

    // Keep running generators until we fail to make progress.
    while !pending_generator_indices.is_empty() {
        let mut next_pending_generator_indices = Vec::new();
        // let mut next_pending_async_generator_indices = Vec::new();

        for &generator_idx in &pending_generator_indices {
            if generator_is_expired[generator_idx] {
                continue;
            }

            let finished = generators[generator_idx].0.run(&witness, &mut buffer);
            if finished {
                generator_is_expired[generator_idx] = true;
                remaining_generators -= 1;
            }

            if let Some(async_gen) = async_generators.get(&generator_idx) {
                let finished = async_gen.0.run(&witness, &mut buffer);
                if finished {
                    remaining_async_generators -= 1;
                } else {
                    next_pending_generator_indices.push(generator_idx);
                    generator_is_expired[generator_idx] = false;
                    remaining_generators += 1;
                }
            }

            // Merge any generated values into our witness, and get a list of newly-populated
            // targets' representatives.
            let new_target_reps = buffer
                .target_values
                .drain(..)
                .flat_map(|(t, v)| witness.set_target_returning_rep(t, v));

            // Enqueue unfinished generators that were watching one of the newly populated targets.
            for watch in new_target_reps {
                let opt_watchers = generator_indices_by_watches.get(&watch);
                if let Some(watchers) = opt_watchers {
                    for &watching_generator_idx in watchers {
                        if !generator_is_expired[watching_generator_idx] {
                            next_pending_generator_indices.push(watching_generator_idx);
                        }
                    }
                }
            }
        }

        pending_generator_indices = next_pending_generator_indices;
    }

    assert_eq!(
        remaining_generators, 0,
        "{} generators weren't run",
        remaining_generators,
    );

    assert_eq!(
        remaining_async_generators, 0,
        "{} async generators weren't run",
        remaining_async_generators,
    );

    witness
}
