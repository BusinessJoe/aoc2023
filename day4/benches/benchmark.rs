use criterion::{criterion_group, criterion_main, Criterion};
use day4::solutions;

pub fn criterion_benchmark(c: &mut Criterion) {
    let text = include_str!("../input.txt");
    c.bench_function("solutions", |b| b.iter(|| {
        let lines: Vec<&str> = text.lines().collect();
        solutions(&lines)
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
