use ndhistogram::{axis::Uniform, ndhistogram, HistND, Histogram};

use num_traits::Float;
use rand::{prelude::StdRng, SeedableRng};
use rand_distr::{Distribution, Normal, StandardNormal};

pub fn generate_nomal<T: Float>(mu: T, sigma: T, seed: u64) -> impl Iterator<Item = T>
where
    StandardNormal: Distribution<T>,
{
    let rng = StdRng::seed_from_u64(seed);
    Normal::new(mu, sigma).unwrap().sample_iter(rng)
}

type Hist1D = HistND<(Uniform<f64>,), f64>;
type Hist3D = HistND<(Uniform<f64>, Uniform<f64>, Uniform<f64>), f64>;

fn generate_normal_hist_1d(seed: u64) -> Hist1D {
    let mut hist = ndhistogram!(Uniform::new(10, -5.0, 5.0));
    generate_nomal(-1.0, 2.0, seed)
        .take(1000)
        .for_each(|it| hist.fill(&it));
    hist
}

fn generate_normal_hist_3d(seed: u64) -> Hist3D {
    let mut hist = ndhistogram!(
        Uniform::new(10, -5.0, 5.0),
        Uniform::new(10, -5.0, 5.0),
        Uniform::new(10, -5.0, 5.0),
    );
    let x = generate_nomal(-1.0, 2.0, seed + 1);
    let y = generate_nomal(0.0, 2.0, seed + 2);
    let z = generate_nomal(1.0, 2.0, seed + 3);
    let xyz = x.zip(y).zip(z).map(|((x, y), z)| (x, y, z));
    xyz.take(1000).for_each(|it| hist.fill(&it));
    hist
}

#[test]
fn test_ndhistogram_add_1d_elementwise() {
    let left = generate_normal_hist_1d(1);
    let right = generate_normal_hist_1d(2);
    let hadd = (&left + &right).unwrap();
    let actual: Vec<_> = hadd
        .iter()
        .map(|it| (it.index, it.bin, *it.value))
        .collect();
    let expected: Vec<_> = left
        .iter()
        .zip(right.into_iter())
        .map(|(l, r)| (l.index, l.bin, l.value + r.value))
        .collect();
    assert_eq!(actual, expected)
}

macro_rules! impl_binary_op {
    ($fnname1d:tt, $fnname3d:tt, $fnnamefails:tt, $mathsymbol:tt) => {
        #[test]
        fn $fnname1d() {
            let left = generate_normal_hist_1d(1);
            let right = generate_normal_hist_1d(2);
            let hadd = (&left $mathsymbol &right).unwrap();
            let actual: Vec<_> = hadd
                .iter()
                .map(|it| (it.index, it.bin, *it.value))
                .filter(|(_,_, v)| !v.is_nan())
                .collect();
            let expected: Vec<_> = left
                .iter()
                .zip(right.into_iter())
                .map(|(l, r)| (l.index, l.bin, l.value $mathsymbol r.value))
                .filter(|(_,_, v)| !v.is_nan())
                .collect();
            assert_eq!(actual, expected)
        }

        #[test]
        fn $fnnamefails() {
            let left = ndhistogram!(Uniform::new(10, -5.0, 5.0));
            let right = ndhistogram!(Uniform::new(10, -5.0, 6.0));
            let hadd = &left $mathsymbol &right;
            assert!(hadd.is_err())
        }


        #[test]
        fn $fnname3d() {
            let left = generate_normal_hist_3d(1);
            let right = generate_normal_hist_3d(2);
            let hadd = (&left $mathsymbol &right).unwrap();
            let actual: Vec<_> = hadd
                .iter()
                .map(|it| (it.index, it.bin, *it.value))
                .filter(|(_,_, v)| !v.is_nan())
                .collect();
            let expected: Vec<_> = left
                .iter()
                .zip(right.into_iter())
                .map(|(l, r)| (l.index, l.bin, l.value $mathsymbol r.value))
                .filter(|(_,_, v)| !v.is_nan())
                .collect();
            assert_eq!(actual, expected)
        }
    }
}

impl_binary_op! {test_ndhistogram_1d_elementwise_add, test_ndhistogram_3d_elementwise_add, test_ndhistogram_binning_mismatch_add, +}
impl_binary_op! {test_ndhistogram_1d_elementwise_mul, test_ndhistogram_3d_elementwise_mul, test_ndhistogram_binning_mismatch_mul, *}
impl_binary_op! {test_ndhistogram_1d_elementwise_sub, test_ndhistogram_3d_elementwise_sub, test_ndhistogram_binning_mismatch_sub, -}
impl_binary_op! {test_ndhistogram_1d_elementwise_div, test_ndhistogram_3d_elementwise_div, test_ndhistogram_binning_mismatch_div, /}

macro_rules! impl_binary_op_with_scalar {
    ($fnname1d:tt, $fnname3d:tt, $mathsymbol:tt) => {

        #[test]
        fn $fnname1d() {
            let left = generate_normal_hist_1d(1);
            let right = 2.0;
            let hadd = (&left $mathsymbol &right);
            let actual: Vec<_> = hadd
                .iter()
                .map(|it| (it.index, it.bin, *it.value))
                .filter(|(_,_, v)| !v.is_nan())
                .collect();
            let expected: Vec<_> = left
                .iter()
                .map(|l| (l.index, l.bin, l.value $mathsymbol right))
                .filter(|(_,_, v)| !v.is_nan())
                .collect();
            assert_eq!(actual, expected)
        }

    }
}

impl_binary_op_with_scalar! {test_ndhistogram_1d_scalar_add, test_ndhistogram_3d_scalar_add, +}
impl_binary_op_with_scalar! {test_ndhistogram_1d_scalar_mul, test_ndhistogram_3d_scalar_mul, *}
impl_binary_op_with_scalar! {test_ndhistogram_1d_scalar_sub, test_ndhistogram_3d_scalar_sub, -}
impl_binary_op_with_scalar! {test_ndhistogram_1d_scalar_div, test_ndhistogram_3d_scalar_div, /}

#[test]
fn test_ndhistogram_1d_equality() {
    let left = generate_normal_hist_1d(1);
    let right = left.clone();
    assert_eq!(left, right)
}

#[test]
fn test_ndhistogram_1d_inequality() {
    let left = generate_normal_hist_1d(1);
    let mut right = left.clone();
    right.fill(&1.0);
    assert_ne!(left, right)
}

#[test]
fn test_ndhistogram_3d_equality() {
    let left = generate_normal_hist_3d(1);
    let right = left.clone();
    assert_eq!(left, right)
}

#[test]
fn test_ndhistogram_3d_inequality() {
    let left = generate_normal_hist_3d(1);
    let mut right = left.clone();
    right.fill(&(1.0, 2.0, 3.0));
    assert_ne!(left, right)
}
