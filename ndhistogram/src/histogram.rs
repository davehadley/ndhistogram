use super::axes::Axes;
use super::binvalues::BinValues;
use std::fmt::Display;

#[macro_export]
macro_rules! ndhistogram {
    ( $( $x:expr ),+ ) => {
        {
            let axes = (
            $(
                $x,
            )*
        );
            Histogram::new(axes, VecBinValues::<f64>::new(10))
        }
    };
}

pub struct Histogram<T: Axes, B: BinValues> {
    axes: T,
    bins: B,
}

impl<T: Axes, B: BinValues> Histogram<T, B> {
    pub fn new(axes: T, bins: B) -> Histogram<T, B> {
        Histogram { axes, bins }
    }
}

impl<T: Axes, B: BinValues> Histogram<T, B> {
    pub fn fill(&mut self, coordinate: T::Coordinate, weight: B::Weight) {
        let index = self.axes.index(&coordinate);
        self.bins.fill(index, weight);
    }

    pub fn fill_from_iter<V, W>(&mut self, coordinate: V, weight: W)
    where
        V: IntoIterator<Item = T::Coordinate>,
        W: IntoIterator<Item = B::Weight>,
    {
        coordinate
            .into_iter()
            .zip(weight)
            .for_each(|(c, w)| self.fill(c, w));
    }
}

impl<T: Axes, B: BinValues> Display for Histogram<T, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Histogram")
    }
}
