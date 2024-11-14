use std::num::Wrapping;

use ndhistogram::{axis::Uniform, ndhistogram, Error, Histogram};

#[test]
fn test_wrapping_fill_wraps_on_overflow() -> Result<(), Error> {
    let mut hist = ndhistogram!(Uniform::new(1, 0.0, 1.0)?; Wrapping<u32>);
    hist.fill_with(&0.0, Wrapping(u32::MAX));
    hist.fill(&0.0);
    let actual = *hist.value(&0.0).unwrap();
    let expected = Wrapping(0);
    assert_eq!(expected, actual);
    Ok(())
}
