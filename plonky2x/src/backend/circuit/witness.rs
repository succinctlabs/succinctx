use alloc::collections::VecDeque;
use alloc::sync::Arc;
use alloc::task;
use std::collections::{HashMap, HashSet};

use anyhow::Result;
use futures::future::BoxFuture;
use futures::FutureExt;
use plonky2::field::extension::Extendable;
use plonky2::field::types::Field;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, WitnessGeneratorRef};
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartialWitness, PartitionWitness, Witness, WitnessWrite};
use plonky2::plonk::circuit_data::{CommonCircuitData, MockCircuitData, ProverOnlyCircuitData};
use plonky2::plonk::config::GenericConfig;

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
    inputs: PartialWitness<F>,
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

    for (t, v) in inputs.target_values.into_iter() {
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

/// Given a `PartialWitness` that has only inputs set, populates the rest of the witness using the
/// given set of generators.
pub async fn generate_witness_async<
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F> + 'static,
    const D: usize,
>(
    inputs: PartialWitness<F>,
    prover_data: Arc<MockCircuitData<F, C, D>>,
    common_data: &CommonCircuitData<F, D>,
) -> Result<Vec<Option<F>>, GenerateWitnessError> {
    dbg!(&inputs);
    let config = &common_data.config;
    let generator_indices_by_watches = &prover_data.prover_only.generator_indices_by_watches;
    dbg!(&generator_indices_by_watches);

    let state = vec![None; prover_data.prover_only.representative_map.len()];
    let state = Arc::new(std::sync::Mutex::new(state));

    let dependency_graph = prover_data
        .prover_only
        .generators
        .iter()
        .map(|g| {
            g.0.watch_list()
                .into_iter()
                .map(|t| t.index(config.num_wires, common_data.degree()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let sorted = topological_sort(&dependency_graph);

    let sorted = match sorted {
        Some(s) => s,
        None => todo!(),
    };

    let sorted = sorted
        .into_iter()
        .map(|i| GeneratorIndex {
            index: i,
            dependencies: prover_data.prover_only.generators[i]
                .0
                .watch_list()
                .into_iter()
                .map(|t| t.index(config.num_wires, common_data.degree()))
                .collect(),
        })
        .collect::<Vec<_>>();

    let num_wires = config.num_wires;
    let degree = common_data.degree();

    execute(sorted, {
        let state = state.clone();
        move |index| {
            // let provider_data = prover_data.clone();
            async move {
                let index = index.index;
                let (finished, buffer) = tokio::task::spawn_blocking({
                    let prover_data = prover_data.clone();
                    let state = state.clone();
                    move || {
                        let mut witness = PartitionWitness::new(
                            num_wires,
                            degree,
                            &prover_data.prover_only.representative_map,
                        );
                        {
                            witness.values = state.lock().unwrap().clone();
                        }
                        let mut buffer = GeneratedValues::empty();
                        let finished = prover_data.prover_only.generators[index].0.run(&witness, &mut buffer);
                        (finished, buffer)
                    }
                })
                .await
                .unwrap();
                {
                    let mut lock = state.lock().unwrap();
                    let mut witness =
                        PartitionWitness::new(num_wires, degree, &prover_data.prover_only.representative_map);
                    witness.values = lock.clone();
                    for (t, v) in buffer.target_values {
                        witness.set_target(t, v);
                    }
                    *lock = witness.values;
                }
                finished
            }
            .boxed()
        }
    })
    .await;

    let x = Ok(state.lock().unwrap().clone());
    x
}

fn dfs(
    node: usize,
    graph: &Vec<Vec<usize>>,
    visited: &mut HashSet<usize>,
    stack: &mut Vec<usize>,
    is_in_stack: &mut Vec<bool>,
) -> bool {
    if visited.contains(&node) {
        return true;
    }

    visited.insert(node);
    is_in_stack[node] = true;

    for &neighbor in &graph[node] {
        if is_in_stack[neighbor] || !dfs(neighbor, graph, visited, stack, is_in_stack) {
            return false;
        }
    }

    is_in_stack[node] = false;
    stack.push(node);

    true
}

fn topological_sort(graph: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    let mut is_in_stack = vec![false; graph.len()];
    for node in 0..graph.len() {
        if !visited.contains(&node) && !dfs(node, graph, &mut visited, &mut stack, &mut is_in_stack)
        {
            return None;
        }
    }

    Some(stack.into_iter().collect())
}

struct GeneratorIndex {
    index: usize,
    dependencies: HashSet<usize>,
}

enum Finished<G> {
    Complete(usize),
    Incomplete(G),
}

/// Generators must be ordered by topological sort.
async fn execute<F>(generators: Vec<GeneratorIndex>, f: F)
where
    F: FnOnce(&GeneratorIndex) -> BoxFuture<bool> + Clone + Send + Sync + 'static,
{
    let mut tasks = tokio::task::JoinSet::new();
    let mut deps_complete = HashSet::new();
    let mut iter = generators.into_iter().peekable();
    let mut queue = VecDeque::new();
    loop {
        // Check currently running tasks for completion.
        while let Some(Ok(finished)) = tasks.join_next().await {
            match finished {
                Finished::Complete(i) => {
                    deps_complete.insert(i);
                }
                Finished::Incomplete(generator) => {
                    queue.push_back(generator);
                }
            }
        }

        // Add as many generators that don't have any dependencies to the queue.
        while let Some(generator) = iter.peek() {
            let deps = &generator.dependencies;
            if deps.is_subset(&deps_complete) {
                queue.push_back(iter.next().unwrap());
            } else {
                break;
            }
        }

        // If the queue and tasks is empty, we're done.
        if queue.is_empty() && tasks.is_empty() {
            break;
        }

        // Add up to the maximum number of tasks from the queue to the task set.
        while !queue.is_empty() && tasks.len() < 10 {
            let generator = queue.pop_front().unwrap();
            let f = f.clone();
            tasks.spawn(async move {
                let finished = f(&generator).await;
                if finished {
                    Finished::Complete(generator.index)
                } else {
                    Finished::Incomplete(generator)
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::FutureExt;

    use super::*;

    #[test]
    fn test_topological_sort() {
        let deps = vec![vec![], vec![], vec![3], vec![1], vec![0, 1], vec![0, 2]];
        let expected = vec![0, 1, 3, 2, 4, 5];
        let sorted = topological_sort(&deps).unwrap();
        assert_eq!(sorted, expected);
    }

    #[tokio::test]
    async fn test_executor() {
        let input: [(usize, &[usize]); 6] = [
            (0, &[]),
            (1, &[]),
            (3, &[1]),
            (2, &[3]),
            (4, &[0, 1]),
            (5, &[0, 2]),
        ];
        let input: Vec<GeneratorIndex> = input
            .into_iter()
            .map(|(i, d)| GeneratorIndex {
                index: i,
                dependencies: d.iter().copied().collect(),
            })
            .collect();
        let expected = vec![0, 1, 3, 2, 4, 5];

        let (tx, mut rx) = tokio::sync::mpsc::channel(100);
        execute(input, |g| {
            async move {
                let index = g.index;
                let r = tx.send(index).await;
                r.unwrap();
                true
            }
            .boxed()
        })
        .await;

        let mut actual = Vec::new();
        while let Some(i) = rx.recv().await {
            actual.push(i);
        }
        assert_eq!(actual, expected);
    }
}
