use ndarray::{Array, ArrayBase, IxDyn};

use crate::{axis::Axis, Histogram};

// impl <A, V, T, D> From<T> for ArrayBase<V, D> where T:Histogram<A, V>, A: Axis {
//     fn from(value: impl Histogram<A, V>) -> Self {
//         todo!()
//     }
// }

fn convert_to_ndarray<A: Axis, V, D>(histogram: impl Histogram<A, V>) -> Array<V, IxDyn> {
    let shape = vec![2, 3];
    let array = Array::from_shape_fn(shape, |it| {});
    array
}

#[cfg(test)]
mod tests {
    use crate::{axis::Uniform, histogram::convert_to_ndarray, ndhistogram};

    #[test]
    fn test_convert_from_ndarray() {
        let hist = ndhistogram!(Uniform::new(5, 0, 5));
        let array = convert_to_ndarray(hist);
        println!("{}", array);
    }
}
