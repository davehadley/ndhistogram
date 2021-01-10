#[macro_export]
macro_rules! ndhistogram {
    ($( $x:expr ),+ $(,)*; $type:ty $(;)*) => {
        {
            use crate::AxesTuple;
            let axes = crate::AxesTuple::from( (
            $(
                $x,
            )*
            ));
            $crate::ArrayHistogram::<_, $type>::new(axes)
        }
    };
    ($( $x:expr ),+ $(,)*) => {
        {
            use crate::AxesTuple;
            let axes = crate::AxesTuple::from((
            $(
                $x,
            )*
            ));
            $crate::ArrayHistogram::<_, f64>::new(axes)
        }
    };

}
