use std::time::Duration;

use aoc_core::SolutionCollection;
use aoc_solutions_2024::solutions::get_collection;
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmarks(c: &mut Criterion) {
    let collection = get_collection();
    for day in collection.get_days() {
        bench_day(c, day, &collection);
    }
}

fn bench_day(c: &mut Criterion, day: u32, collection: &SolutionCollection) {
    let mut group = c.benchmark_group(format!("day{}", day).as_str());
    // group.sample_size(20);
    group.measurement_time(Duration::from_secs(20));
    group.sampling_mode(criterion::SamplingMode::Flat);
    let (part1, part2) = collection.prepare_bench(&day);
    group.bench_function("part1", |b| {
        b.iter(|| {
            part1();
        })
    });
    group.bench_function("part2", |b| {
        b.iter(|| {
            part2();
        })
    });
    group.finish();
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
