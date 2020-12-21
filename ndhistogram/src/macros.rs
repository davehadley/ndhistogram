#[macro_export]
macro_rules! ndhistogram {
    (($t:ty)? $(,)? $( $x:expr ),+ ) => {
        {
            let axes = (
            $(
                $x,
            )*
        );
            $crate::histogram::ArrayHistogram::<_, $t>::new(axes)
        }
    };
    ($( $x:expr ),+) => {
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
