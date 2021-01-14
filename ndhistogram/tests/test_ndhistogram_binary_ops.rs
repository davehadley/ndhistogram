use ndhistogram::{axis::Uniform, ndhistogram, Histogram, Item, VecHistogram};

use num_traits::Float;
use rand::{prelude::StdRng, SeedableRng};
use rand_distr::{Distribution, Normal, StandardNormal};

pub fn generate_nomal<T: Float>(mu: T, sigma: T, seed: u64) -> impl Iterator<Item = T>
where
    StandardNormal: Distribution<T>,
{
    let mut rng = StdRng::seed_from_u64(seed);
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
