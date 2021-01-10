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
