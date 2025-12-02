use rust02_25::{part1, part2};

use criterion::{Criterion, criterion_group, criterion_main};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../../input/part1.txt");

    {
        let mut group = c.benchmark_group("2025_02_part1");

        group.bench_function("default", |b| b.iter(|| part1::process(input)));
    }

    {
        let mut group = c.benchmark_group("2025_02_part2");

        group.bench_function("default", |b| b.iter(|| part2::process(input)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
