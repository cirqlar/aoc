use rust04_24::{part1, part2};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn part1(c: &mut Criterion) {
    let input = include_bytes!("../../input/part1.txt");

    c.bench_function("part1", |b| b.iter(|| part1::process_flatpar(input)));
}

criterion_group!(benches, part1);
criterion_main!(benches);

// fn main() {
//     divan::main();
// }

// #[divan::bench()]
// fn part1() -> i32 {
//     let input = include_bytes!("../../input/part1.txt");

//     part1::process(input)
// }

// #[divan::bench()]
// fn part1_flatpar() -> i32 {
//     let input = include_bytes!("../../input/part1.txt");

//     part1::process_flatpar(input)
// }

// #[divan::bench()]
// fn part2() -> i32 {
//     let input = include_bytes!("../../input/part1.txt");

//     part2::process(input)
// }

// #[divan::bench()]
// fn part2_flatpar() -> i32 {
//     let input = include_bytes!("../../input/part1.txt");

//     part2::process_flatpar(input)
// }

// #[divan::bench()]
// fn part1_bencher(bencher: divan::Bencher) {
//     let input = include_bytes!("../../input/part1.txt");

//     bencher.bench_local(|| part1::process(input));
// }

// #[divan::bench()]
// fn part1_flatpar_bencher(bencher: divan::Bencher) {
//     let input = include_bytes!("../../input/part1.txt");

//     bencher.bench_local(|| part1::process_flatpar(input));
// }

// #[divan::bench()]
// fn part2_bencher(bencher: divan::Bencher) {
//     let input = include_bytes!("../../input/part1.txt");

//     bencher.bench_local(|| part2::process(input));
// }

// #[divan::bench()]
// fn part2_flatpar_bencher(bencher: divan::Bencher) {
//     let input = include_bytes!("../../input/part1.txt");

//     bencher.bench_local(|| part2::process_flatpar(input));
// }
