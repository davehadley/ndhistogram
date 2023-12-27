use ndhistogram::{
    axis::{Axis, Uniform},
    ndhistogram, Axes, AxesTuple, Fill, FillWith, FillWithWeighted, Histogram,
};

#[test]
fn test_trait_object_histogram() {
    let datum = Box::new(ndhistogram!(Uniform::new(10, -5.0, 5.0).unwrap(); i32));
    let _vec: Vec<Box<dyn Histogram<_, _>>> = vec![datum];
}

#[test]
fn test_trait_object_fill() {
    let datum = Box::new(0.0);
    let _vec: Vec<Box<dyn Fill>> = vec![datum];
}

#[test]
fn test_trait_object_fill_with() {
    let datum = Box::new(0.0);
    let _vec: Vec<Box<dyn FillWith<f64>>> = vec![datum];
}

#[test]
fn test_trait_object_fill_with_weighted() {
    let datum = Box::<ndhistogram::value::WeightedMean>::default();
    let _vec: Vec<Box<dyn FillWithWeighted<f64, f64>>> = vec![datum];
}

#[test]
fn test_trait_object_axis() {
    let datum = Box::new(Uniform::new(10, -5.0, 5.0).unwrap());
    let _vec: Vec<Box<dyn Axis<Coordinate = _, BinInterval = _>>> = vec![datum];
}

#[test]
fn test_trait_object_axes() {
    let datum: Box<AxesTuple<_>> = Box::new((Uniform::new(10, -5.0, 5.0).unwrap(),).into());
    let _vec: Vec<Box<dyn Axes<Coordinate = _, BinInterval = _>>> = vec![datum];
}
