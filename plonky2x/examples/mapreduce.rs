use log::debug;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig};
use plonky2x::backend::circuit::{Circuit, PlonkParameters};
use plonky2x::backend::function::VerifiableFunction;
use plonky2x::frontend::mapreduce::MapReduceRecursiveProofGenerator;
use plonky2x::prelude::{CircuitBuilder, Field, Variable};

struct MapReduceCircuit {}

impl Circuit for MapReduceCircuit {
    fn define<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>)
    where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher:
            AlgebraicHasher<<L as PlonkParameters<D>>::Field>,
    {
        let ctx = builder.constant::<Variable>(L::Field::from_canonical_u64(8));
        let inputs = vec![
            L::Field::from_canonical_u64(0),
            L::Field::from_canonical_u64(1),
            L::Field::from_canonical_u64(2),
            L::Field::from_canonical_u64(0),
        ];

        let output = builder.mapreduce::<Variable, Variable, Variable, _, _>(
            ctx,
            inputs,
            |ctx, input, builder| {
                debug!("map");
                builder.watch(&ctx, "ctx");
                let constant = builder.constant::<Variable>(L::Field::ONE);
                builder.add(input, constant)
            },
            |ctx, left, right, builder| {
                debug!("reduce");
                builder.watch(&ctx, "ctx");
                builder.add(left, right)
            },
        );

        builder.watch(&output, "output");
        builder.write(output);
    }

    fn add_generators<L: PlonkParameters<D>, const D: usize>(
        registry: &mut plonky2x::prelude::WitnessGeneratorRegistry<L, D>,
    ) where
        <<L as PlonkParameters<D>>::Config as GenericConfig<D>>::Hasher: AlgebraicHasher<L::Field>,
    {
        let id = MapReduceRecursiveProofGenerator::<L, Variable, Variable, Variable, D>::id();
        registry.register_simple::<MapReduceRecursiveProofGenerator::<L, Variable, Variable, Variable, D>>(id);
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
        let mut builder = CircuitBuilder::<L, D>::new();
        MapReduceCircuit::define(&mut builder);
        let circuit = builder.build();
        let input = circuit.input();
        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        let value = output.read::<Variable>();
        println!("{}", value);
    }
}
