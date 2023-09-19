use std::env;

use ethers::types::U64;
use itertools::Itertools;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2x::backend::circuit::{Circuit, PlonkParameters};
use plonky2x::backend::function::VerifiableFunction;
use plonky2x::frontend::eth::beacon::vars::BeaconBalancesVariable;
use plonky2x::frontend::mapreduce::generator::MapReduceGenerator;
use plonky2x::frontend::uint::uint64::U64Variable;
use plonky2x::prelude::{Bytes32Variable, CircuitBuilder};
use plonky2x::utils::bytes32;
use plonky2x::utils::eth::beacon::BeaconClient;

struct MapReduceCircuit {}

impl Circuit for MapReduceCircuit {
    fn define<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        let rpc_url = env::var("CONSENSUS_RPC_1").unwrap();
        let client = BeaconClient::new(rpc_url);
        builder.set_beacon_client(client);

        let block_root = builder.constant::<Bytes32Variable>(bytes32!(
            "0x4f1dd351f11a8350212b534b3fca619a2a95ad8d9c16129201be4a6d73698adb"
        ));
        let balances_root = builder.beacon_get_balances(block_root);
        let idxs = (0..8).map(U64::from).collect_vec();

        let output = builder
            .mapreduce::<BeaconBalancesVariable, U64Variable, Bytes32Variable, _, _, 4>(
                balances_root,
                idxs,
                |balances_root, idxs, builder| {
                    let b0 = builder.beacon_get_balance_witness(balances_root, idxs[0]);
                    let b1 = builder.beacon_get_balance_witness(balances_root, idxs[1]);
                    let b2 = builder.beacon_get_balance_witness(balances_root, idxs[2]);
                    let b3 = builder.beacon_get_balance_witness(balances_root, idxs[3]);
                    let leaf = builder.beacon_u64s_to_leaf([b0, b1, b2, b3]);
                    builder.curta_sha256(&leaf.as_bytes());
                    leaf
                },
                |_, left, right, builder| builder.sha256_pair(left, right),
            );

        builder.watch(&output, "output");
        builder.write(output);
    }

    fn register_generators<L: PlonkParameters<D>, const D: usize>(
        registry: &mut plonky2x::prelude::WitnessGeneratorRegistry<L, D>,
    ) where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        let id =
            MapReduceGenerator::<L, BeaconBalancesVariable, U64Variable, Bytes32Variable, 4, D>::id(
            );
        registry.register_simple::<MapReduceGenerator<L, BeaconBalancesVariable, U64Variable, Bytes32Variable, 4, D>>(id);
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
