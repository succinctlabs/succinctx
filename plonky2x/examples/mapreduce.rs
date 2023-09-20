use ethers::types::U64;
use itertools::Itertools;
use jemallocator::Jemalloc;
use log::debug;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2x::backend::circuit::{Circuit, PlonkParameters};
use plonky2x::backend::function::VerifiableFunction;
use plonky2x::frontend::eth::beacon::vars::BeaconBalancesVariable;
use plonky2x::frontend::mapreduce::generator::MapReduceGenerator;
use plonky2x::frontend::uint::uint64::U64Variable;
use plonky2x::prelude::{Bytes32Variable, CircuitBuilder};
use plonky2x::utils::bytes32;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

/// An example source block root.
const BLOCK_ROOT: &str = "0x4f1dd351f11a8350212b534b3fca619a2a95ad8d9c16129201be4a6d73698adb";

/// The number of balances to fetch.
const NB_BALANCES: usize = 1048576;

/// The batch size for fetching balances and computing the local balance roots.
const BATCH_SIZE: usize = 2048;

struct MapReduceCircuit;

impl Circuit for MapReduceCircuit {
    fn define<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        let block_root = builder.constant::<Bytes32Variable>(bytes32!(BLOCK_ROOT));
        let partial_balances = builder.beacon_get_partial_balances::<NB_BALANCES>(block_root);
        let idxs = (0..NB_BALANCES).map(U64::from).collect_vec();

        let output = builder
            .mapreduce::<BeaconBalancesVariable, U64Variable, (Bytes32Variable, U64Variable), _, _, BATCH_SIZE>(
                partial_balances,
                idxs,
                |balances_root, idxs, builder| {
                    // Witness balances.
                    let balances =
                        builder.beacon_get_balance_batch_witness::<BATCH_SIZE>(balances_root, idxs[0]);

                    // Convert balances to leafs.
                    let mut leafs = Vec::new();
                    let mut sum = builder.constant::<U64Variable>(U64::from(0));
                    for i in 0..idxs.len() / 4 {
                        let b = [
                            balances[i * 4],
                            balances[i * 4 + 1],
                            balances[i * 4 + 2],
                            balances[i * 4 + 3]
                        ];
                        sum = builder.add_many(&b);
                        leafs.push(builder.beacon_u64s_to_leaf(b));
                    }

                    // Reduce leafs to a single root.
                    while leafs.len() != 1 {
                        let mut tmp = Vec::new();
                        for i in 0..leafs.len() / 2 {
                            debug!("calling sha256 pair w/ curta");
                            tmp.push(builder.curta_sha256_pair(leafs[i*2], leafs[i*2+1]));
                        }
                        leafs = tmp;
                    }

                    (leafs[0], sum)
                },
                |_, left, right, builder| {
                    // Reduce two roots to a single root and compute the sum of the two balances.
                    (builder.sha256_pair(left.0, right.0), builder.add(left.1, right.1))
                }
            );

        builder.assert_is_equal(output.0, partial_balances.root);
        builder.watch(&output.1, "total balance");
        builder.write(output);
    }

    fn register_generators<L: PlonkParameters<D>, const D: usize>(
        registry: &mut plonky2x::prelude::WitnessGeneratorRegistry<L, D>,
    ) where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        let id = MapReduceGenerator::<
            L,
            BeaconBalancesVariable,
            U64Variable,
            (Bytes32Variable, U64Variable),
            BATCH_SIZE,
            D,
        >::id();
        registry.register_simple::<MapReduceGenerator<
            L,
            BeaconBalancesVariable,
            U64Variable,
            (Bytes32Variable, U64Variable),
            BATCH_SIZE,
            D,
        >>(id);
    }
}

fn main() {
    VerifiableFunction::<MapReduceCircuit>::entrypoint();
}

#[cfg(test)]
mod tests {
    use plonky2x::prelude::DefaultParameters;

    use super::*;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_circuit() {
        env_logger::try_init().unwrap_or_default();

        let mut builder = CircuitBuilder::<L, D>::new();
        MapReduceCircuit::define(&mut builder);
        let circuit = builder.build();

        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);

        MapReduceCircuit::test_serialization::<L, D>();
    }
}
