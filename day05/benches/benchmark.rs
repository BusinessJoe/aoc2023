use criterion::{criterion_group, criterion_main, Criterion};
use day5::{solution1, solution2};

pub fn criterion_benchmark(c: &mut Criterion) {
    let text = include_str!("../input.txt");
    c.bench_function("part 1", |b| b.iter(|| {
        let lines: Vec<&str> = text.lines().collect();
        solution1(&lines)
    }));
    c.bench_function("part 2", |b| b.iter(|| {
        let lines: Vec<&str> = text.lines().collect();
        solution2(&lines)
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
