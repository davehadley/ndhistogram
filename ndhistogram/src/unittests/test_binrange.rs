use std::{
    convert::TryInto,
    ops::{Range, RangeFrom, RangeTo},
};

use crate::axis::binrange::ContinuousBinRange;

#[test]
fn test_binrange_equality() {
    let underflow0a = ContinuousBinRange::underflow(0.0);
    let underflow0b = ContinuousBinRange::underflow(0.0);
    let underflow1a = ContinuousBinRange::underflow(1.0);

    let overflow0a = ContinuousBinRange::overflow(0.0);
    let overflow0b = ContinuousBinRange::overflow(0.0);
    let overflow1a = ContinuousBinRange::overflow(1.0);

    let bin0a = ContinuousBinRange::new(0.0, 1.0);
    let bin0b = ContinuousBinRange::new(0.0, 1.0);
    let bin1a = ContinuousBinRange::new(1.0, 2.0);

    assert_eq!(underflow0a, underflow0b);
    assert_eq!(overflow0a, overflow0b);
    assert_eq!(bin0a, bin0b);

    assert_ne!(underflow0a, underflow1a);
    assert_ne!(overflow0a, overflow1a);
    assert_ne!(bin0a, bin1a);

    assert_ne!(underflow0a, overflow0a);
    assert_ne!(underflow0a, bin0a);
    assert_ne!(bin0a, overflow0a);
}

#[test]
fn test_binrange_underflow_debug() {
    let underflow = ContinuousBinRange::underflow(0.0);
    println!("{:?}", underflow);
}

#[test]
fn test_binrange_overflow_debug() {
    let overflow = ContinuousBinRange::overflow(0.0);
    println!("{:?}", overflow);
}

#[test]
fn test_binrange_bin_debug() {
    let bin = ContinuousBinRange::new(0.0, 1.0);
    println!("{:?}", bin);
}

#[test]
fn test_binrange_underflow_display() {
    let underflow = ContinuousBinRange::underflow(0.0);
    println!("{}", underflow);
}

#[test]
fn test_binrange_overflow_display() {
    let overflow = ContinuousBinRange::overflow(0.0);
    println!("{}", overflow);
}

#[test]
fn test_binrange_bin_display() {
    let bin = ContinuousBinRange::new(0.0, 1.0);
    println!("{}", bin);
}

#[test]
fn test_binrange_bin_conversion() {
    let start = 0.0;
    let end = 1.0;
    let bin = ContinuousBinRange::new(start, end);
    let range: Range<_> = bin.try_into().unwrap();
    assert_eq!(range.start, start);
    assert_eq!(range.end, end);
}

#[test]
#[allow(clippy::float_cmp)]
fn test_binrange_underflow_conversion() {
    let end = 1.0;
    let bin = ContinuousBinRange::underflow(end);
    let range: RangeTo<_> = bin.try_into().unwrap();
    assert_eq!(range.end, end);
}

#[test]
#[allow(clippy::float_cmp)]
fn test_binrange_overflow_conversion() {
    let start = 1.0;
    let bin = ContinuousBinRange::overflow(start);
    let range: RangeFrom<_> = bin.try_into().unwrap();
    assert_eq!(range.start, start);
}
