#[macro_export]
macro_rules! ndhistogram {

    ($( $x:expr ),+ $(,)*; $type:ty $(;)*) => {
        {
            let axes = (
            $(
                $x,
            )*
        );
            $crate::histogram::ArrayHistogram::<_, $type>::new(axes)
        }
    };
    ($( $x:expr ),+ $(,)*) => {
        {
            let axes = (
            $(
                $x,
            )*
        );
            $crate::histogram::ArrayHistogram::<_, f64>::new(axes)
        }
    };

}
