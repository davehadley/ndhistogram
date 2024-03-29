use ndhistogram::axis::{Axis, CategoryNoFlow, SingleValueBinInterval};

#[test]
fn test_categorynoflow_num_bins() {
    let ax = CategoryNoFlow::new(vec!["A", "B", "C"]);
    assert_eq!(ax.num_bins(), 3)
}

#[test]
fn test_categorynoflow_get_index() {
    let cats = vec!["A", "B"];
    let ax = CategoryNoFlow::new(cats.clone());
    let actual: Vec<usize> = cats.iter().map(|c| ax.index(c).unwrap()).collect();
    let expected = vec![0, 1];
    assert_eq!(expected, actual)
}

#[test]
fn test_categorynoflow_get_index_overflow() {
    let cats = vec!["A", "B"];
    let ax = CategoryNoFlow::new(cats);
    assert_eq!(ax.index(&"D"), None)
}

#[test]
fn test_categorynoflow_get_bin() {
    let cats = vec!["A", "B", "C"];
    let ax = CategoryNoFlow::new(cats);
    let actual: Vec<_> = (0..4).map(|it| ax.bin(it)).collect();
    let expected: Vec<_> = vec![
        Some(SingleValueBinInterval::new("A")),
        Some(SingleValueBinInterval::new("B")),
        Some(SingleValueBinInterval::new("C")),
        None,
    ];
    assert_eq!(expected, actual);
}

#[test]
fn test_categorynoflow_clone() {
    let ax = CategoryNoFlow::new(vec!["A", "B", "C"]);
    assert_eq!(ax, ax.clone());
}

#[test]
fn test_categorynoflow_debug_display() {
    let ax = CategoryNoFlow::new(vec!["A", "B", "C"]);
    println!("{:?}", ax);
}

#[test]
fn test_categorynoflow_display() {
    let ax = CategoryNoFlow::new(vec!["A", "B", "C"]);
    println!("{}", ax);
    assert_eq!(format!("{}", ax), "{{A}, {B}, {C}, {overflow}}")
}

#[test]
fn test_categorynoflow_iterate_indices() {
    let ax = CategoryNoFlow::new(vec!["A", "B", "C"]);
    let actual: Vec<usize> = ax.indices().collect();
    let expected = vec![0, 1, 2];
    assert_eq!(expected, actual);
}

#[test]
fn test_categorynoflow_iterate_items() {
    let ax = CategoryNoFlow::new(vec!["A", "B"]);
    let actual: Vec<_> = ax.into_iter().collect();
    let expected: Vec<(usize, _)> = vec![
        (0, SingleValueBinInterval::new("A")),
        (1, SingleValueBinInterval::new("B")),
    ];
    assert_eq!(expected, actual);
}

#[test]
fn test_categorynoflow_iterate_bin() {
    let ax = CategoryNoFlow::new(vec!["A", "B"]);
    let actual: Vec<_> = ax.bins().collect();
    let expected: Vec<_> = vec![
        SingleValueBinInterval::new("A"),
        SingleValueBinInterval::new("B"),
    ];
    assert_eq!(expected, actual);
}

#[test]
fn test_iter_axis() {
    let ax = CategoryNoFlow::new(vec!["A", "B"]);
    let expected: Vec<_> = ax.iter().collect();
    let actual: Vec<_> = ax.into_iter().collect();
    assert_eq!(expected, actual);
}
