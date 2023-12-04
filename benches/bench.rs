use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[path = "../src/bin/day_01.rs"]
mod day_01;
#[path = "../src/bin/day_02.rs"]
mod day_02;
#[path = "../src/bin/day_03.rs"]
mod day_03;
#[path = "../src/bin/day_04.rs"]
mod day_04;

fn criterion_benchmark(c: &mut Criterion) {
    {
        static INPUT: &str = include_str!("../inputs/day_01");
        let mut g = c.benchmark_group("day_01");
        g.bench_function("part1", |b| b.iter(|| day_01::part1(black_box(INPUT))));
        g.bench_function("part2", |b| b.iter(|| day_01::part2(black_box(INPUT))));
        g.finish();
    }

    {
        static INPUT: &str = include_str!("../inputs/day_02");
        let mut g = c.benchmark_group("day_02");
        g.bench_function("part1", |b| b.iter(|| day_02::part1(black_box(INPUT))));
        g.bench_function("part2", |b| b.iter(|| day_02::part2(black_box(INPUT))));
        g.finish();
    }

    {
        static INPUT: &str = include_str!("../inputs/day_03");
        let mut g = c.benchmark_group("day_03");
        g.bench_function("part1", |b| b.iter(|| day_03::part1(black_box(INPUT))));
        g.bench_function("part2", |b| b.iter(|| day_03::part2(black_box(INPUT))));
        g.finish();
    }

    {
        static INPUT: &str = include_str!("../inputs/day_04");
        let mut g = c.benchmark_group("day_04");
        g.bench_function("part1", |b| b.iter(|| day_04::part1(black_box(INPUT))));
        g.bench_function("part2", |b| b.iter(|| day_04::part2(black_box(INPUT))));
        g.finish();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
