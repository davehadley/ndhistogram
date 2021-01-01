use std::collections::HashMap;

use super::binrange::SingleValuedBinRange;
use super::Axis;

#[derive(Debug, Clone)]
pub struct Category<T = String> {
    map: HashMap<T, usize>,
}

// impl<T> Axis for Category<T> {
//     type Coordinate = T;

//     type BinRange = SingleValuedBinRange;

//     fn index(&self, coordinate: Self::Coordinate) -> usize {
//         todo!()
//     }

//     fn numbins(&self) -> usize {
//         todo!()
//     }

//     fn bin(&self, index: usize) -> Option<Self::BinRange> {
//         todo!()
//     }

//     fn indices(&self) -> Box<dyn Iterator<Item = usize>> {
//         Box::new(0..super.numbins())
//     }

//     fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (usize, Self::BinRange)> + 'a> {
//         Box::new(super.indices().map(move |it| (it, super.bin(it).unwrap())))
//     }

//     fn bins<'a>(&'a self) -> Box<dyn Iterator<Item = Self::BinRange> + 'a> {
//         Box::new(super.indices().map(move |it| super.bin(it).unwrap()))
//     }
// }
