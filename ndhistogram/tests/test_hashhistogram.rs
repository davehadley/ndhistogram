use ndhistogram::{
    axis::{Category, Uniform, Variable},
    ndhistogram, sparsehistogram,
    value::WeightedMean,
    Histogram,
};
use rand::{prelude::StdRng, Rng, SeedableRng};

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

#[test]
fn test_hashhistogram_iter() {
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

    // check item iterator
    let mut sparseitems: Vec<_> = sparsehist.iter().collect();
    let mut vecitems: Vec<_> = vechist.iter().filter(|item| *item.value > 0).collect();
    sparseitems.sort_by_key(|item| item.index);
    vecitems.sort_by_key(|item| item.index);
    assert_eq!(sparseitems, vecitems);

    // check values iterator
    let mut sparseitems: Vec<_> = sparsehist.values().collect();
    let mut vecitems: Vec<_> = vechist.values().filter(|value| **value > 0).collect();
    sparseitems.sort();
    vecitems.sort();
    assert_eq!(sparseitems, vecitems);
}
