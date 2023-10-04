//! Witness generation for Plonky circuits.
//!
//! These methods are based on the [`generate_partial_witness`][1] method in `plonky2`, with the
//! added functionality to have non-blocking witness generation for asynchronous hints and
//! computationally expensive generators.
//!
//! [1] : https://github.com/mir-protocol/plonky2/blob/main/plonky2/src/iop/generator.rs#L19

use alloc::collections::BTreeMap;

use anyhow::{anyhow, Error, Result};
use curta::maybe_rayon::rayon;
use log::trace;
use plonky2::iop::generator::{GeneratedValues, WitnessGeneratorRef};
use plonky2::iop::witness::{PartialWitness, PartitionWitness, WitnessWrite};
use plonky2::plonk::circuit_data::{CommonCircuitData, ProverOnlyCircuitData};
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::oneshot;

use super::PlonkParameters;
use crate::frontend::hint::asynchronous::generator::{AsyncHintDataRef, AsyncHintRef};
use crate::frontend::hint::asynchronous::handler::HintHandler;

/// Given a `PartialWitness` that has only inputs set, populates the rest of the witness using the
/// given set of generators.
pub fn generate_witness<'a, L: PlonkParameters<D>, const D: usize>(
    inputs: PartialWitness<L::Field>,
    prover_data: &'a ProverOnlyCircuitData<L::Field, L::Config, D>,
    common_data: &'a CommonCircuitData<L::Field, D>,
    async_generator_refs: &'a BTreeMap<usize, AsyncHintDataRef<L, D>>,
) -> Result<PartitionWitness<'a, L::Field>> {
    // If async hints are present, set up the a handler and initialize the generators with the
    // handler's communication channel.
    let (tx_handler_error, rx_handler_error) = oneshot::channel();
    let async_generators = match async_generator_refs.is_empty() {
        true => BTreeMap::new(),
        false => {
            let (tx, rx) = unbounded_channel();
            // Initialize the hint handler.
            let mut hint_handler = HintHandler::<L, D>::new(rx);

            // Spawn a runtime and run the hint handler.
            let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
            rayon::spawn(move || {
                let result = rt.block_on(hint_handler.run());
                if let Err(e) = result {
                    tx_handler_error.send(e).unwrap();
                }
            });

            BTreeMap::from_iter(
                async_generator_refs
                    .iter()
                    .map(|(i, g)| (*i, g.0.generator(tx.clone()))),
            )
        }
    };

    fill_witness_values::<L, D>(
        inputs,
        prover_data,
        common_data,
        async_generators,
        rx_handler_error,
    )
}

pub async fn generate_witness_async<'a, L: PlonkParameters<D>, const D: usize>(
    inputs: PartialWitness<L::Field>,
    prover_data: &'a ProverOnlyCircuitData<L::Field, L::Config, D>,
    common_data: &'a CommonCircuitData<L::Field, D>,
    async_generator_refs: &'a BTreeMap<usize, AsyncHintDataRef<L, D>>,
) -> Result<PartitionWitness<'a, L::Field>> {
    // If async hints are present, set up the a handler and initialize the generators with the
    // handler's communication channel.
    let (tx_handler_error, rx_handler_error) = oneshot::channel();
    let async_generators = match async_generator_refs.is_empty() {
        true => BTreeMap::new(),
        false => {
            let (tx, rx) = unbounded_channel();
            // Initialize the hint handler.
            let mut hint_handler = HintHandler::<L, D>::new(rx);

            // Spawn a runtime and run the hint handler.
            tokio::spawn(async move {
                let result = hint_handler.run().await;
                if let Err(e) = result {
                    tx_handler_error.send(e).unwrap();
                }
            });

            BTreeMap::from_iter(
                async_generator_refs
                    .iter()
                    .map(|(i, g)| (*i, g.0.generator(tx.clone()))),
            )
        }
    };

    tokio::task::block_in_place(move || {
        fill_witness_values::<L, D>(
            inputs,
            prover_data,
            common_data,
            async_generators,
            rx_handler_error,
        )
    })
}

/// Fill in the witness after intiializing async generators.
fn fill_witness_values<'a, L: PlonkParameters<D>, const D: usize>(
    inputs: PartialWitness<L::Field>,
    prover_data: &'a ProverOnlyCircuitData<L::Field, L::Config, D>,
    common_data: &'a CommonCircuitData<L::Field, D>,
    async_generators: BTreeMap<usize, AsyncHintRef<L, D>>,
    mut rx_handler_error: oneshot::Receiver<Error>,
) -> Result<PartitionWitness<'a, L::Field>> {
    let config = &common_data.config;
    let generators = &prover_data.generators;
    let generator_indices_by_watches = &prover_data.generator_indices_by_watches;

    // Build a list of "pending" generators which are queued to be run. Initially, all generators
    // are queued.
    let mut pending_generator_indices: Vec<_> = (0..generators.len()).collect();

    // We also track a list of "expired" generators which have already returned false.
    let mut generator_is_expired = vec![false; generators.len()];
    let mut remaining_generators = generators.len();

    let mut buffer = GeneratedValues::empty();
    let mut witness = PartitionWitness::new(
        config.num_wires,
        common_data.degree(),
        &prover_data.representative_map,
    );

    for (t, v) in inputs.target_values.into_iter() {
        witness.set_target(t, v);
    }

    // Keep running generators until we fail to make progress.
    let mut iter: usize = 0;
    while !pending_generator_indices.is_empty() {
        trace!("iter: {}", iter);
        let mut next_pending_generator_indices = Vec::new();
        // let mut next_pending_async_generator_indices = Vec::new();

        for &generator_idx in &pending_generator_indices {
            if generator_is_expired[generator_idx] {
                continue;
            }

            if let Some(async_gen) = async_generators.get(&generator_idx) {
                if let Ok(e) = rx_handler_error.try_recv() {
                    return Err(e);
                }
                let finished = async_gen.0.run(&witness, &mut buffer)?;
                if finished {
                    generator_is_expired[generator_idx] = true;
                    remaining_generators -= 1;
                } else {
                    next_pending_generator_indices.push(generator_idx);
                }
            } else {
                let finished = generators[generator_idx].0.run(&witness, &mut buffer);
                if finished {
                    generator_is_expired[generator_idx] = true;
                    remaining_generators -= 1;
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
        iter += 1;
        if iter % 100 == 0 {
            trace!("pending_generator_indices: {:?}", pending_generator_indices);
        }
    }

    if remaining_generators > 0 {
        return Err(get_generator_error::<L, D>(
            generators,
            generator_is_expired,
        ));
    }

    trace!(
        "finished filling in witness: nb_public_inputs={}",
        prover_data.public_inputs.len()
    );
    Ok(witness)
}

#[inline]
fn get_generator_error<L: PlonkParameters<D>, const D: usize>(
    generators: &[WitnessGeneratorRef<L::Field, D>],
    generator_is_expired: Vec<bool>,
) -> Error {
    let mut generators_not_run = Vec::new();
    for i in 0..generator_is_expired.len() {
        if !generator_is_expired[i] {
            let generator = &generators[i];
            generators_not_run.push(generator.0.id());
        }
    }
    anyhow!(
        "Witness generation failed, generators not run: {:?}",
        generators_not_run
    )
}
