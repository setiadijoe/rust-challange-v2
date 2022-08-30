use criterion::{black_box, criterion_group, criterion_main, Criterion};
use testing::sploosh;

pub fn sploosh_benchmark(c: &mut Criterion) {
    c.bench_function("sploosh test", |b| b.iter(|| sploosh(black_box(5), black_box(5), black_box(5))));
}

criterion_group!(benches, sploosh_benchmark);
criterion_main!(benches);