/// Creates a [Histogram](crate::Histogram).
///
/// This macro is the recommended method for constructing new Histograms.
/// The arguments are a command separated list of [Axis](crate::axis::Axis).
/// Optionally, the type of the [Histogram](crate::Histogram) bin values may specified after a semi-colon after
/// the list of [Axis](crate::axis::Axis). If the bin value type is not specified, the default is f64.
///
/// # Example
///
/// ## Creating a 1D Histogram
/// ```rust
/// use ndhistogram::axis::Uniform;
/// use std::fmt::Display;
/// use ndhistogram::{ndhistogram, Histogram};
///
/// let hist1D = ndhistogram!(Uniform::new(10, -5.0, 5.0));
/// println!("{}", hist1D);
/// ```
///
/// ## Creating a 2D Histogram
/// ```rust
/// use ndhistogram::axis::Uniform;
/// use ndhistogram::{ndhistogram, Histogram};
///
/// let hist2D = ndhistogram!(Uniform::new(10, -5.0, 5.0), Uniform::new(10, -5.0, 5.0));
/// println!("{}", hist2D);
/// ```
#[macro_export]
macro_rules! ndhistogram {

    ($( $x:expr ),+ $(,)*; $type:ty $(;)*) => {
        {
            let axes = (
            $(
                $x,
            )*
        );
            $crate::VecHistogram::<_, $type>::new(axes)
        }
    };
    ($( $x:expr ),+ $(,)*) => {
        {
            let axes = (
            $(
                $x,
            )*
        );
            $crate::VecHistogram::<_, f64>::new(axes)
        }
    };

}
