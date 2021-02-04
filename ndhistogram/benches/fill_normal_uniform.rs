use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ndhistogram::axis::Uniform;
use ndhistogram::{ndhistogram, Histogram};

fn fill_normal_uniform_1d(n: u64) {
    let mut hist = ndhistogram!(Uniform::with_step_size(10, 0, 1));
    (0..n).for_each(|i| hist.fill(&i));
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fill_normal_uniform_1d", |b| {
        b.iter(|| fill_normal_uniform_1d(black_box(20)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
