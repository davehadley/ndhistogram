use std::any::Any;

use ndhistogram::{axis::Uniform, VecHistogram};
use ndhistogram::{ndhistogram, AxesTuple};

#[test]
fn test_ndhistogram_1d_uniform_constructor() {
    let hist = ndhistogram!(Uniform::new(5, 0.0, 1.0).unwrap());
    assert_eq!(
        hist.type_id(),
        VecHistogram::<AxesTuple<_>, f64>::new((Uniform::new(5, 0.0, 1.0).unwrap(),).into())
            .type_id()
    )
}

#[test]
fn test_ndhistogram_1d_uniform_constructor_with_explicittype() {
    let hist = ndhistogram!(Uniform::new(5, 0.0, 1.0).unwrap(); f64);
    assert_eq!(
        hist.type_id(),
        VecHistogram::<AxesTuple<_>, f64>::new((Uniform::new(5, 0.0, 1.0).unwrap(),).into())
            .type_id()
    )
}

#[test]
fn test_ndhistogram_1d_uniform_constructor_with_ident() {
    let ax = Uniform::new(5, 0.0, 1.0).unwrap();
    let hist = ndhistogram!(ax);
    assert_eq!(
        hist.type_id(),
        VecHistogram::<AxesTuple<_>, f64>::new((Uniform::new(5, 0.0, 1.0).unwrap(),).into())
            .type_id()
    )
}

#[test]
fn test_ndhistogram_2d_uniform_constructor() {
    let hist = ndhistogram!(
        Uniform::new(5, 0.0, 1.0).unwrap(),
        Uniform::new(5, 0.0, 1.0).unwrap()
    );
    assert_eq!(
        hist.type_id(),
        VecHistogram::<AxesTuple<_>, f64>::new(
            (
                Uniform::new(5, 0.0, 1.0).unwrap(),
                Uniform::new(5, 0.0, 1.0).unwrap()
            )
                .into()
        )
        .type_id()
    )
}

#[test]
fn test_ndhistogram_2d_uniform_constructor_with_explicit_type() {
    let hist =
        ndhistogram!(Uniform::new(5, 0.0, 1.0).unwrap(), Uniform::new(5, 0.0, 1.0).unwrap(); f64);
    assert_eq!(
        hist.type_id(),
        VecHistogram::<AxesTuple<_>, f64>::new(
            (
                Uniform::new(5, 0.0, 1.0).unwrap(),
                Uniform::new(5, 0.0, 1.0).unwrap()
            )
                .into()
        )
        .type_id()
    )
}

#[test]
fn test_ndhistogram_2d_uniform_constructor_with_ident() {
    let x = Uniform::new(5, 0.0, 1.0).unwrap();
    let y = Uniform::new(5, 0.0, 1.0).unwrap();
    let hist = ndhistogram!(x, y);
    assert_eq!(
        hist.type_id(),
        VecHistogram::<AxesTuple<_>, f64>::new(
            (
                Uniform::new(5, 0.0, 1.0).unwrap(),
                Uniform::new(5, 0.0, 1.0).unwrap()
            )
                .into()
        )
        .type_id()
    )
}
