#![allow(clippy::needless_range_loop)]

use ethers::types::U64;
use itertools::Itertools;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2x::backend::circuit::{Circuit, PlonkParameters};
use plonky2x::backend::function::VerifiableFunction;
use plonky2x::frontend::eth::beacon::vars::{BeaconBalancesVariable, BeaconValidatorsVariable};
use plonky2x::frontend::mapreduce::generator::MapReduceGenerator;
use plonky2x::frontend::uint::uint64::U64Variable;
use plonky2x::frontend::vars::SSZVariable;
use plonky2x::prelude::{Bytes32Variable, CircuitBuilder, HintRegistry};
use plonky2x::utils::bytes32;

// The withdrawal credentials of Lido validators.
const LIDO_WITHDRAWAL_CREDENTIALS: &str =
    "0x010000000000000000000000b9d7934878b5fb9610b3fe8a5e441e8fad7e293f";

/// An example source block root.
const BLOCK_ROOT: &str = "0x4f1dd351f11a8350212b534b3fca619a2a95ad8d9c16129201be4a6d73698adb";

/// The number of balances to fetch.
const NB_VALIDATORS: usize = 512;

/// The batch size for fetching balances and computing the local balance roots.
const BATCH_SIZE: usize = 256;

struct MapReduceValidatorCircuit;

impl Circuit for MapReduceValidatorCircuit {
    fn define<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        let block_root = builder.constant::<Bytes32Variable>(bytes32!(BLOCK_ROOT));
        let partial_validators = builder.beacon_get_partial_validators::<NB_VALIDATORS>(block_root);
        let partial_balances = builder.beacon_get_partial_balances::<NB_VALIDATORS>(block_root);
        let idxs = (0..NB_VALIDATORS).map(U64::from).collect_vec();

        let output = builder.mapreduce::<
            (BeaconValidatorsVariable, BeaconBalancesVariable),
            U64Variable,
            (Bytes32Variable, Bytes32Variable, U64Variable),
            _,
            _,
            BATCH_SIZE,
        >(
            (partial_validators, partial_balances),
            idxs,
            |(validators_root, balances_root), idxs, builder| {
                // Witness validators.
                let validators = builder.beacon_witness_validator_batch_::<BATCH_SIZE>(
                    validators_root,
                    idxs[0]
                );

                // Witness balances.
                let balances = builder.beacon_witness_balance_batch::<BATCH_SIZE>(
                    balances_root,
                    idxs[0]
                );

                // Convert validators to leafs.
                let lido_withdrawal_credentials = builder.constant::<Bytes32Variable>(
                    bytes32!(LIDO_WITHDRAWAL_CREDENTIALS)
                );
                let mut validator_leafs = Vec::new();
                let mut is_lido_validator = Vec::new();
                for i in 0..validators.len() {
                    validator_leafs.push(validators[i].hash_tree_root(builder));
                    is_lido_validator.push(builder.is_equal(
                        validators[i].withdrawal_credentials,
                        lido_withdrawal_credentials
                    ));
                }

                // Convert balances to leafs.
                let mut balance_leafs = Vec::new();
                let zero = builder.constant::<U64Variable>(U64::from(0));
                let mut sum = builder.constant::<U64Variable>(U64::from(0));
                for i in 0..idxs.len() / 4 {
                    let balances = [
                        balances[i*4],
                        balances[i*4+1],
                        balances[i*4+2],
                        balances[i*4+3],
                    ];
                    let masked_balances = [
                        builder.select(is_lido_validator[i*4], balances[0], zero),
                        builder.select(is_lido_validator[i*4+1], balances[1], zero),
                        builder.select(is_lido_validator[i*4+2], balances[2], zero),
                        builder.select(is_lido_validator[i*4+3], balances[3], zero),
                    ];
                    sum = builder.add_many(&masked_balances);
                    balance_leafs.push(builder.beacon_u64s_to_leaf(balances));
                }

                // Reduce validator leafs to a single root.
                let validators_acc = builder.ssz_hash_leafs(&validator_leafs);
                let balances_acc = builder.ssz_hash_leafs(&balance_leafs);

                // Return the respective accumulators and partial sum.
                (validators_acc, balances_acc, sum)
            },
            |_, left, right, builder| {
                (
                    builder.sha256_pair(left.0, right.0),
                    builder.sha256_pair(left.1, right.1),
                    builder.add(left.2, right.2)
                )
            }
        );

        builder.assert_is_equal(output.0, partial_validators.validators_root);
        builder.assert_is_equal(output.1, partial_balances.root);
        builder.watch(&output.2, "total balance");
    }

    fn register_generators<L: PlonkParameters<D>, const D: usize>(registry: &mut HintRegistry<L, D>)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        let id = MapReduceGenerator::<
            L,
            (BeaconValidatorsVariable, BeaconBalancesVariable),
            U64Variable,
            (Bytes32Variable, Bytes32Variable, U64Variable),
            BATCH_SIZE,
            D,
        >::id();
        registry.register_simple::<MapReduceGenerator<
            L,
            (BeaconValidatorsVariable, BeaconBalancesVariable),
            U64Variable,
            (Bytes32Variable, Bytes32Variable, U64Variable),
            BATCH_SIZE,
            D,
        >>(id);
    }
}

fn main() {
    VerifiableFunction::<MapReduceValidatorCircuit>::entrypoint();
}

#[cfg(test)]
mod tests {
    use plonky2x::prelude::DefaultParameters;
    use plonky2x::utils;

    use super::*;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_circuit() {
        env_logger::try_init().unwrap_or_default();

        let mut builder = CircuitBuilder::<L, D>::new();
        MapReduceValidatorCircuit::define(&mut builder);
        let circuit = builder.build();

        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);

        MapReduceValidatorCircuit::test_serialization::<L, D>();
    }
}
