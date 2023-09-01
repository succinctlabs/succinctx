use std::env;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::AlgebraicHasher;
use plonky2x::backend::circuit::Circuit;
use plonky2x::backend::function::CircuitFunction;
use plonky2x::prelude::{CircuitBuilder, Variable};

struct Function {}

impl CircuitFunction for Function {
    fn build<F, C, const D: usize>() -> Circuit<F, C, D>
    where
        F: RichField + Extendable<D>,
        C: plonky2::plonk::config::GenericConfig<D, F = F> + 'static,
        <C as plonky2::plonk::config::GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
    {
        let mut builder = CircuitBuilder::<F, D>::new();
        let a = builder.constant::<Variable>(F::from_canonical_u64(0));
        let b = builder.constant::<Variable>(F::from_canonical_u64(1));
        let c = builder.constant::<Variable>(F::from_canonical_u64(3));
        let d = builder.constant::<Variable>(F::from_canonical_u64(4));
        let inputs = vec![a, b, c, d];
        let output = builder.mapreduce::<Variable, Variable, C, _, _>(
            inputs,
            |input, builder| {
                let constant = builder.constant::<Variable>(F::ONE);
                builder.add(input, constant)
            },
            |left, right, builder| builder.add(left, right),
        );
        builder.write(output);
        builder.build::<C>()
    }
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    Function::cli();
}

#[cfg(test)]
mod tests {
    use plonky2::field::types::Field;
    use plonky2x::prelude::{GoldilocksField, PoseidonGoldilocksConfig};

    use super::*;

    type F = GoldilocksField;
    type C = PoseidonGoldilocksConfig;
    const D: usize = 2;

    #[test]
    fn test_circuit_function_field() {
        env_logger::init();
        let circuit = Function::build::<F, C, D>();
        let input = circuit.input();
        let (proof, output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        let sum = output.read::<Variable>();
        assert_eq!(sum, F::from_canonical_u64(12));
    }
}
