extern crate ndhistogram;
use ndhistogram::axis::Uniform;
use ndhistogram::{ndhistogram, Error, Histogram};

fn main() -> Result<(), Error> {
    let mut hist = ndhistogram!(Uniform::new(10, 0.0, 1.0)?);

    // fill a single number
    hist.fill(&0.0);
    Ok(())
}
