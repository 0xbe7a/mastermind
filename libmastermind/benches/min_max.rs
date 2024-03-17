use criterion::{criterion_group, criterion_main, Criterion};
use libmastermind::positions::*;
use pprof::criterion::{Output, PProfProfiler};

fn bench_init(c: &mut Criterion) {
    let mut group = c.benchmark_group("Init-Time");
    group.bench_function("Variable Size", |b| {
        b.iter(|| {
            StandardCollection::generate_possibilities(5, 6);
        })
    });
    group.finish();
}

fn bench_evaluate(c: &mut Criterion) {
    let mut position_collection = StandardCollection::generate_possibilities(7, 7);

    position_collection.prune(&[0, 1, 2, 3, 4, 5, 6], 4, 3);

    let mut group = c.benchmark_group("Evaluate-Time");

    group.bench_function("Counter", |b| {
        let collection = StandardCollection::generate_possibilities(7, 7);
        b.iter(|| collection.find_best_guess(&position_collection))
    });
    group.finish();
}

fn bench_symmetry(c: &mut Criterion) {
    let mut group = c.benchmark_group("Symmetry-Prune");
    group.bench_function("Variable Size", |b| {
        let guesses = StandardCollection::generate_possibilities(5, 6);
        let collection = StandardCollection::generate_possibilities(5, 6);
        b.iter(|| collection.prune_symmetrys(&[], &guesses))
    });
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_evaluate//, bench_symmetry, bench_init
}
criterion_main!(benches);
