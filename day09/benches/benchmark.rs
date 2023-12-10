use criterion::{criterion_group, criterion_main, Criterion};
use day9::{solution1, solution1_fd};

pub fn criterion_benchmark(c: &mut Criterion) {
    let text = include_str!("../input.txt");
    let mut group = c.benchmark_group("Part 1");
    group.bench_function("Pascal's Triangle", |b| b.iter(|| {
        solution1(text.lines())
    }));
    group.bench_function("First differences", |b| b.iter(|| {
        solution1_fd(text.lines())
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
