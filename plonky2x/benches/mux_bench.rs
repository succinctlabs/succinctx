extern crate plonky2x;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ethers::types::U256;
use plonky2::field::types::Field;
use plonky2x::{frontend::{builder::CircuitBuilder, vars::U256Variable}, prelude::{Variable, GoldilocksField, CircuitVariable}};

fn benchmark_select_index_with_select(c: &mut Criterion) {
    type F = GoldilocksField;
    // type C = PoseidonGoldilocksConfig;
    const D: usize = 2;

    let mut builder = CircuitBuilder::<F, D>::new();
    
    let selector = builder.constant::<Variable>(F::from_canonical_usize(42)); // Example selector value
    let inputs: Vec<u64> = (1..=786_432).collect();

    let input_variables = inputs
            .iter()
            .map(|x| U256Variable::constant(&mut builder, U256::from(*x)))
            .collect::<Vec<_>>();

    c.bench_function("select_index_with_select", |b| {
        b.iter(|| {
            let _result = builder.select_index_with_select(black_box(selector), black_box(&input_variables));
            // Black box the result to prevent optimization
        });
    });
}

fn benchmark_select_index(c: &mut Criterion) {
    type F = GoldilocksField;
    // type C = PoseidonGoldilocksConfig;
    const D: usize = 2;

    let mut builder = CircuitBuilder::<F, D>::new();

    let selector = builder.constant::<Variable>(F::from_canonical_usize(42)); // Example selector value
    let inputs: Vec<u64> = (1..=786_432).collect();

    let input_variables = inputs
            .iter()
            .map(|x| U256Variable::constant(&mut builder, U256::from(*x)))
            .collect::<Vec<_>>();

    c.bench_function("select_index", |b| {
        b.iter(|| {
            let _result = builder.select_index(black_box(selector), black_box(&input_variables));
            // Black box the result to prevent optimization
        });
    });
}

criterion_group!(
    benches,
    benchmark_select_index_with_select,
    benchmark_select_index
);
criterion_main!(benches);
