use ndhistogram::axis::binrange::SingleValuedBinRange;

#[test]
fn test_categorybinrange_equality() {
    let overflow0a = SingleValuedBinRange::overflow();
    let overflow0b = SingleValuedBinRange::overflow();

    let bin0a = SingleValuedBinRange::new("Zero");
    let bin0b = SingleValuedBinRange::new("Zero");
    let bin1a = SingleValuedBinRange::new("One");

    assert_eq!(overflow0a, overflow0b);

    assert_eq!(bin0a, bin0b);
    assert_ne!(bin0a, bin1a);
    assert_ne!(bin0a, overflow0a);
}

#[test]
fn test_categorybinrange_overflow_debug() {
    let overflow = SingleValuedBinRange::<String>::overflow();
    println!("{:?}", overflow);
}

#[test]
fn test_categorybinrange_bin_debug() {
    let bin = SingleValuedBinRange::new("One");
    println!("{:?}", bin);
}

#[test]
fn test_categorybinrange_overflow_display() {
    let overflow = SingleValuedBinRange::<String>::overflow();
    println!("{}", overflow);
}

#[test]
fn test_categorybinrange_bin_display() {
    let bin = SingleValuedBinRange::new("One");
    println!("{}", bin);
}

#[test]
fn test_categorybinrange_is_clone() {
    let bin = SingleValuedBinRange::new("One");
    assert_eq!(bin, bin.clone());
}

#[test]
fn test_categorybinrange_value_method() {
    let overflow = SingleValuedBinRange::<String>::overflow();
    let bin = SingleValuedBinRange::new("One");
    assert_eq!(bin.value(), Some(&"One"));
    assert_eq!(overflow.value(), None);
}
