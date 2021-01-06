use super::Axis;

#[derive(Clone, Debug)]
pub struct NoFlow<T> {
    axis: T,
}

impl<T: Axis> NoFlow<T> {
    fn new(axis: T) -> Self {
        NoFlow { axis }
    }
}

// impl<T: Axis> Axis for NoFlow<T> {
//     type Coordinate = <T as Axis>::Coordinate;
//     type BinRange = <T as Axis>::BinRange;

//     fn index(&self, coordinate: &Self::Coordinate) -> Option<usize> {
//         self.axis.index(coordinate);
//     }

//     fn numbins(&self) -> usize {
//         todo!()
//     }

//     fn bin(&self, index: usize) -> Option<Self::BinRange> {
//         todo!()
//     }
// }
