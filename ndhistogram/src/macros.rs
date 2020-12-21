#[macro_export]
macro_rules! ndhistogram {
    ( $( $x:expr ),+;$t:ty ) => {
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
