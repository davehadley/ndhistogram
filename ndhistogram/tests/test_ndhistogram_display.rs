use ndhistogram::{axis::Uniform, ndhistogram, Histogram};

#[test]
fn test_histogram_display_blank() {
    let hist = ndhistogram!(Uniform::new(4, 0.0, 2.0));
    let actual = format!("{}", hist);
    let expected = "1D histogram
underflow  |
[0.0, 0.5) |
[0.5, 1.0) |
[1.0, 1.5) |
[1.5, 2.0) |
overflow   |
";
    assert_eq!(actual, expected);
}

#[test]
fn test_histogram_display_filled_positive() {
    let mut hist = ndhistogram!(Uniform::new(4, 0.0, 2.0));
    vec![
        (-1.0, 2.0),
        (0.1, 0.9),
        (0.6, 4.0),
        (1.1, 6.0),
        (1.6, 100.0),
        (2.1, 50.0),
    ]
    .iter()
    .for_each(|(x, w)| hist.fill_with(x, w));
    let actual = format!("{}", hist);
    let expected = "1D histogram
underflow  |#
[0.0, 0.5) |
[0.5, 1.0) |##
[1.0, 1.5) |###
[1.5, 2.0) |##################################################
overflow   |#########################
";
    assert_eq!(actual, expected);
}
