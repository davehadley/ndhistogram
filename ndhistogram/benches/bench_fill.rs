use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use ndhistogram::axis::{Uniform, Variable};
use ndhistogram::value::WeightedMean;
use ndhistogram::{ndhistogram, Histogram};
use rand::{prelude::StdRng, Rng, SeedableRng};

macro_rules! generate_fill_axis_benches {
    ($name:ident; $numbins:ident; $axis:expr;) => {
        paste::item! {

            fn [< bench_ $name _single_fill_1d >] (c: &mut Criterion) {
                let $numbins = 10000;
                let mut hist = ndhistogram!($axis);
                let mut rng = StdRng::seed_from_u64(12);
                c.bench_function(stringify!([< bench_ $name _single_fill_1d >]), |b| {
                    b.iter(|| hist.fill(&black_box(rng.gen_range(-0.1..1.1))))
                });
            }

            fn [< bench_ $name _single_fill_with_1d >] (c: &mut Criterion) {
                let $numbins = 10000;
                let mut hist = ndhistogram!($axis);
                let mut rng = StdRng::seed_from_u64(12);
                c.bench_function(stringify!([< bench_ $name _single_fill_with_1d >]), |b| {
                    b.iter(|| hist.fill_with(&black_box(rng.gen_range(-0.1..1.1)), black_box(rng.gen_range(0.0..2.0))))
                });
            }

            fn [< bench_ $name _single_fill_with_weighted_1d >] (c: &mut Criterion) {
                let $numbins = 10000;
                let mut hist = ndhistogram!($axis; WeightedMean);
                let mut rng = StdRng::seed_from_u64(12);
                c.bench_function(stringify!([< bench_ $name _single_fill_with_weighted_1d >]), |b| {
                    b.iter(|| hist.fill_with_weighted(&black_box(rng.gen_range(-0.1..1.1)), black_box(rng.gen_range(0.0..2.0)), black_box(rng.gen_range(0.0..2.0))))
                });
            }

            fn [< bench_ $name _iter_fill_2d_vs_num_fills >](c: &mut Criterion) {
                let $numbins = 1000;
                let mut hist = ndhistogram!($axis, $axis);
                let mut rng = StdRng::seed_from_u64(12);
                let mut group = c.benchmark_group(stringify!([< bench_ $name _iter_fill_2d_vs_num_fills >]));
                for size in vec![1000, 10000, 100000, 1000000] {
                    let input: Vec<_> = (0..size)
                        .map(|_| (rng.gen_range(-0.1..1.1), rng.gen_range(-0.1..1.1)))
                        .collect();
                    group.throughput(Throughput::Elements(size));
                    group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, data| {
                        b.iter(|| data.iter().for_each(|it| hist.fill(it)))
                    });
                }
            }

            fn [< bench_ $name _iter_fill_2d_vs_num_bins >](c: &mut Criterion) {
                let mut group = c.benchmark_group(stringify!([< bench_ $name _iter_fill_2d_vs_num_bins >]));
                for size in vec![10, 100, 1000, 10000] {
                    let $numbins = size;
                    let mut hist = ndhistogram!($axis, $axis);
                    let mut rng = StdRng::seed_from_u64(12);
                    let input: Vec<_> = (0..100000)
                        .map(|_| (rng.gen_range(-0.1..1.1), rng.gen_range(-0.1..1.1)))
                        .collect();
                    group.throughput(Throughput::Elements(size as u64));
                    group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, data| {
                        b.iter(|| data.iter().for_each(|it| hist.fill(it)))
                    });
                }
            }

            criterion_group!(
                [< bench_$name >],
                [< bench_ $name _single_fill_1d >],
                [< bench_ $name _single_fill_with_1d >],
                [< bench_ $name _single_fill_with_weighted_1d >],
                [< bench_ $name _iter_fill_2d_vs_num_fills >],
                [< bench_ $name _iter_fill_2d_vs_num_bins >]
            );
        }

    };
}

generate_fill_axis_benches! {uniform; numbins; Uniform::new(numbins, 0.0, 1.0);}

generate_fill_axis_benches! {variable; numbins; Variable::new((0..numbins+1).map(|it| (it as f64)/(numbins as f64)).collect::<Vec<f64>>());}

criterion_main!(bench_uniform, bench_variable);
