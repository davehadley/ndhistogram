use ndhistogram::{
    axis::{Category, Uniform, UniformNoFlow, Variable},
    ndhistogram, sparsehistogram,
    value::WeightedMean,
    HashHistogram, Hist2D, HistND, Histogram, SparseHist1D, SparseHist2D, SparseHist3D,
    SparseHistND, VecHistogram,
};
use num_traits::Float;
use rand::{prelude::StdRng, Rng, SeedableRng};
use rand_distr::{uniform::UniformChar, Distribution, Normal, StandardNormal};

#[test]
fn test_hashhistogram_fill() {
    let mut hist = sparsehistogram!(Uniform::new(10, 0.0, 10.0); i64);
    hist.fill(&0.5);
    hist.fill_with(&0.5, 2);
    assert_eq!(hist.value(&0.5), Some(&3))
}

fn assert_float_eq(left: f64, right: f64) {
    assert!(
        (left - right).abs() < (1e-6 * (left + right)),
        format!("left={} does not equal right={}", left, right)
    )
}

#[test]
fn test_hashhistogram_fill_with_weighted() {
    let mut hist = sparsehistogram!(Uniform::new(10, 0.0, 10.0); WeightedMean<i32, i32>);
    hist.fill_with_weighted(&0.5, 1, 1);
    hist.fill_with_weighted(&0.5, 2, 2);
    hist.fill_with_weighted(&0.5, 3, 3);
    assert_float_eq(hist.value(&0.5).unwrap().mean(), 14.0 / 6.0)
}

#[test]
fn test_hashhistogram_axes() {
    let x = Uniform::new(10, 0.0, 10.0);
    let y = Variable::new(vec![1.0, 2.0, 10.0]);
    let z = Category::new(vec!["A", "B", "C"]);
    let sparsehist = sparsehistogram!(x.clone(), y.clone(), z.clone());
    let vechist = ndhistogram!(x, y, z);
    assert_eq!(sparsehist.axes(), vechist.axes())
}

fn random_2d_vec_and_sparse_hist() -> (
    Hist2D<Uniform, Variable, i32>,
    SparseHist2D<Uniform, Variable, i32>,
) {
    let x = Uniform::new(10, 0.0, 10.0);
    let y = Variable::new(vec![0.0, 2.0, 5.0, 10.0]);
    let mut sparsehist = sparsehistogram!(x.clone(), y.clone(); i32);
    let mut vechist = ndhistogram!(x, y; i32);

    // fill both histograms with the same data
    let mut rng = StdRng::seed_from_u64(123);
    (0..1000)
        .map(|_| {
            let rx: f64 = rng.gen_range(-1.0..11.0);
            let ry: f64 = rng.gen_range(-1.0..11.0);
            let rz: i32 = rng.gen_range(1..10);
            (rx, ry, rz)
        })
        .for_each(|(x, y, z)| {
            sparsehist.fill_with(&(x, y), z);
            vechist.fill_with(&(x, y), z);
        });
    (vechist, sparsehist)
}

#[test]
fn test_hashhistogram_iter() {
    let (vechist, sparsehist) = random_2d_vec_and_sparse_hist();
    // check item iterator
    let mut sparseitems: Vec<_> = sparsehist.iter().collect();
    let mut vecitems: Vec<_> = vechist.iter().filter(|item| *item.value > 0).collect();
    sparseitems.sort_by_key(|item| item.index);
    vecitems.sort_by_key(|item| item.index);
    assert_eq!(sparseitems, vecitems);
}

#[test]
fn test_hashhistogram_values() {
    let (vechist, sparsehist) = random_2d_vec_and_sparse_hist();
    // check values iterator
    let mut sparseitems: Vec<_> = sparsehist.values().collect();
    let mut vecitems: Vec<_> = vechist.values().filter(|value| **value > 0).collect();
    sparseitems.sort();
    vecitems.sort();
    assert_eq!(sparseitems, vecitems);
}

#[test]
fn test_hashhistogram_iter_mut() {
    let (mut vechist, mut sparsehist) = random_2d_vec_and_sparse_hist();
    // check mutable item iterator
    let mut sparseitems: Vec<_> = sparsehist.iter_mut().collect();
    let mut vecitems: Vec<_> = vechist.iter_mut().filter(|item| *item.value > 0).collect();
    sparseitems.sort_by_key(|item| item.index);
    vecitems.sort_by_key(|item| item.index);
    assert_eq!(sparseitems, vecitems);
}

#[test]
fn test_hashhistogram_values_mut() {
    let (mut vechist, mut sparsehist) = random_2d_vec_and_sparse_hist();
    // check mutable values iterator
    let mut sparseitems: Vec<_> = sparsehist.values_mut().collect();
    let mut vecitems: Vec<_> = vechist.values_mut().filter(|value| **value > 0).collect();
    sparseitems.sort();
    vecitems.sort();
    assert_eq!(sparseitems, vecitems);
}

#[test]
fn test_hashhistogram_display() {
    let hist = sparsehistogram!(Uniform::new(4, 0.0, 2.0));
    format!("{}", hist);
}

#[test]
fn test_hashhistogram_clone() {
    let hist = sparsehistogram!(Uniform::new(4, 0.0, 2.0));
    let clone = hist.clone();
    assert_eq!(hist, clone);
}

fn generate_normal_hist_1d(seed: u64) -> SparseHist1D<Uniform> {
    let mut hist = sparsehistogram!(Uniform::new(10, -5.0, 5.0));
    generate_nomal(-1.0, 2.0, seed)
        .take(1000)
        .for_each(|it| hist.fill(&it));
    hist
}

fn generate_normal_hist_3d(seed: u64) -> SparseHist3D<Uniform, Uniform, Uniform> {
    let mut hist = sparsehistogram!(
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

impl_binary_op! {test_sparsehistogram_1d_elementwise_add, test_sparsehistogram_3d_elementwise_add, test_sparsehistogram_binning_mismatch_add, +}
impl_binary_op! {test_sparsehistogram_1d_elementwise_mul, test_sparsehistogram_3d_elementwise_mul, test_sparsehistogram_binning_mismatch_mul, *}
impl_binary_op! {test_sparsehistogram_1d_elementwise_sub, test_sparsehistogram_3d_elementwise_sub, test_sparsehistogram_binning_mismatch_sub, -}
impl_binary_op! {test_sparsehistogram_1d_elementwise_div, test_sparsehistogram_3d_elementwise_div, test_sparsehistogram_binning_mismatch_div, /}

pub fn generate_nomal<T: Float>(mu: T, sigma: T, seed: u64) -> impl Iterator<Item = T>
where
    StandardNormal: Distribution<T>,
{
    let rng = StdRng::seed_from_u64(seed);
    Normal::new(mu, sigma).unwrap().sample_iter(rng)
}
