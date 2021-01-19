use ndhistogram::{
    axis::{Category, Uniform, Variable},
    ndhistogram, sparsehistogram,
    value::WeightedMean,
    Histogram,
};

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
