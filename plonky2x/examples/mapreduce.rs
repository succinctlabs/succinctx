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
            "0xce041ceab7feb54821794e170bace390a41f34743f25b224e95d193ecb4d8052"
        ));
        let balances_root = builder.beacon_get_balances(block_root);
        let idxs = (0..8).map(U64::from).collect_vec();

        let output = builder.mapreduce::<BeaconBalancesVariable, U64Variable, U64Variable, _, _>(
            balances_root,
            idxs,
            |balances_root, idx, builder| {
                let rpc_url = env::var("CONSENSUS_RPC_1").unwrap();
                let client = BeaconClient::new(rpc_url);
                builder.set_beacon_client(client);
                builder.beacon_get_balance(balances_root, idx)
            },
            |_, left, right, builder| builder.add(left, right),
        );

        builder.watch(&output, "output");
        builder.write(output);
    }

    fn register_generators<L: PlonkParameters<D>, const D: usize>(
        registry: &mut plonky2x::prelude::WitnessGeneratorRegistry<L, D>,
    ) where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        let id = MapReduceGenerator::<L, BeaconBalancesVariable, U64Variable, U64Variable, D>::id();
        registry.register_simple::<MapReduceGenerator<L, BeaconBalancesVariable, U64Variable, U64Variable, D>>(id);
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
        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);

        MapReduceCircuit::test_serialization::<L, D>();
    }
}
