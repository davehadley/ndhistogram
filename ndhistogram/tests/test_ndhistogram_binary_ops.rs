use ndhistogram::{
    axis::{Uniform, Variable},
    ndhistogram, Axes, Histogram, VecHistogram,
};

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

#[test]
fn test_ndhistogram_add_1d_elementwise() {
    let mut left = ndhistogram!(Uniform::new(10, -5.0, 5.0));
    let mut right = left.clone();
    generate_nomal(-1.0, 2.0, 1)
        .take(1000)
        .for_each(|it| left.fill(&it));
    generate_nomal(1.0, 2.0, 2)
        .take(1000)
        .for_each(|it| right.fill(&it));
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

#[test]
fn test_ndhistogram_add_1d_elementwise_fails_with_different_binning() {
    let left = ndhistogram!(Uniform::new(10, -5.0, 5.0));
    let right = ndhistogram!(Uniform::new(10, -5.0, 6.0));
    let hadd = &left + &right;
    assert!(hadd.is_err())
}

#[test]
fn test_ndhistogram_add_nd_elementwise() {
    let mut left = ndhistogram!(
        Uniform::new(10, -5.0, 5.0),
        Uniform::new(10, -5.0, 5.0),
        Uniform::new(10, -5.0, 5.0),
    );
    let mut right = left.clone();
    fn fill_histo(
        histo: &mut VecHistogram<(Uniform<f64>, Uniform<f64>, Uniform<f64>), f64>,
        seed: u64,
    ) {
        let x = generate_nomal(-1.0, 2.0, seed + 1);
        let y = generate_nomal(0.0, 2.0, seed + 2);
        let z = generate_nomal(1.0, 2.0, seed + 3);
        let xyz = x.zip(y).zip(z).map(|((x, y), z)| (x, y, z));
        xyz.take(1000).for_each(|it| histo.fill(&it));
    }
    fill_histo(&mut left, 10);
    fill_histo(&mut right, 20);
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
