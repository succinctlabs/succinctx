#![feature(generic_const_exprs)]
extern crate plonky2x;

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use ethers::types::U256;
use plonky2::field::types::Field;
use plonky2x::{frontend::{builder::CircuitBuilder, vars::U256Variable}, prelude::{Variable, GoldilocksField, CircuitVariable}};

fn benchmark_select_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("mux");
    group.sample_size(10);

    type F = GoldilocksField;
    const D: usize = 2;
    let mut builder = CircuitBuilder::<F, D>::new();
    let selector = builder.constant::<Variable>(F::from_canonical_usize(42)); // Example selector value
    let inputs: Vec<u64> = (1..=74851).collect();

    let input_variables = inputs
            .iter()
            .map(|x| U256Variable::constant(&mut builder, U256::from(*x)))
            .collect::<Vec<_>>();

    for i in [20u64, 21u64].iter() {
        group.bench_with_input(BenchmarkId::new("select_index_with_random_access", i), i, 
            |b, _| b.iter(|| builder.select_index(selector, &input_variables)));
        group.bench_with_input(BenchmarkId::new("select_index_with_select", i), i, 
            |b, _| b.iter(|| builder.select_index_with_select(selector, &input_variables)));
    }
    group.finish();
}

criterion_group!(
    benches,
    benchmark_select_index
);
criterion_main!(benches);
