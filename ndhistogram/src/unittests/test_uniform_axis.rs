use crate::axis::{Axis, Uniform};

#[test]
fn test_uniform_size() {
    let ax = Uniform::new(5, 0.0, 1.0);
    assert_eq!(ax.size(), 5 + 2)
}

#[test]
fn test_uniform_get_index() {
    let ax = Uniform::new(5, 0.0, 1.0);
    let actual: Vec<usize> = (-2..7).map(|x| ax.index(&(x as f64 / 5.0))).collect();
    let expected = vec![0, 0, 1, 2, 3, 4, 5, 6, 6];
    assert_eq!(expected, actual)
}
