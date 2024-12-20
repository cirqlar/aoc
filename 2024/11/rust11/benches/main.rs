use rust11_24::part2;

use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../../input/part1.txt");

    {
        let mut group = c.benchmark_group("day11_part1");

        group.bench_function("default", |b| b.iter(|| part2::process(input, 25)));
    }

    {
        let mut group = c.benchmark_group("day11_part2");

        group.bench_function("default", |b| b.iter(|| part2::process(input, 75)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
