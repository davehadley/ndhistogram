use std::any::Any;

use crate::{axis::Uniform, ArrayHistogram};

#[test]
fn test_ndhistogram_1d_uniform_constructor() {
    let hist = ndhistogram!(Uniform::new(5, 0.0, 1.0));
    assert_eq!(
        hist.type_id(),
        ArrayHistogram::<_, f64>::new((Uniform::new(5, 0.0, 1.0),)).type_id()
    )
}

#[test]
fn test_ndhistogram_1d_uniform_constructor_with_explicittype() {
    let hist = ndhistogram!(Uniform::new(5, 0.0, 1.0); f64);
    assert_eq!(
        hist.type_id(),
        ArrayHistogram::<_, f64>::new((Uniform::new(5, 0.0, 1.0),)).type_id()
    )
}

#[test]
fn test_ndhistogram_1d_uniform_constructor_with_ident() {
    let ax = Uniform::new(5, 0.0, 1.0);
    let hist = ndhistogram!(ax);
    assert_eq!(
        hist.type_id(),
        ArrayHistogram::<_, f64>::new((Uniform::new(5, 0.0, 1.0),)).type_id()
    )
}

#[test]
fn test_ndhistogram_2d_uniform_constructor() {
    let hist = ndhistogram!(Uniform::new(5, 0.0, 1.0), Uniform::new(5, 0.0, 1.0));
    assert_eq!(
        hist.type_id(),
        ArrayHistogram::<_, f64>::new((Uniform::new(5, 0.0, 1.0), Uniform::new(5, 0.0, 1.0)))
            .type_id()
    )
}

#[test]
fn test_ndhistogram_2d_uniform_constructor_with_explicit_type() {
    let hist = ndhistogram!(Uniform::new(5, 0.0, 1.0), Uniform::new(5, 0.0, 1.0); f64);
    assert_eq!(
        hist.type_id(),
        ArrayHistogram::<_, f64>::new((Uniform::new(5, 0.0, 1.0), Uniform::new(5, 0.0, 1.0)))
            .type_id()
    )
}

#[test]
fn test_ndhistogram_2d_uniform_constructor_with_ident() {
    let x = Uniform::new(5, 0.0, 1.0);
    let y = Uniform::new(5, 0.0, 1.0);
    let hist = ndhistogram!(x, y);
    assert_eq!(
        hist.type_id(),
        ArrayHistogram::<_, f64>::new((Uniform::new(5, 0.0, 1.0), Uniform::new(5, 0.0, 1.0)))
            .type_id()
    )
}
