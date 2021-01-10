use rand::{
    prelude::{StdRng, ThreadRng},
    thread_rng, Rng, SeedableRng,
};

use crate::axis::{Axis, Category, Uniform};
use crate::AxesTuple;

#[test]
fn test_axes_2d() {
    let xaxis = Uniform::new(3, 0.0, 3.0);
    let yaxis = Category::new(vec!["A", "B"]);
    let axes3d = AxesTuple::from((xaxis.clone(), yaxis.clone()));
    let mut rng = StdRng::seed_from_u64(12);
    let ntests = 10000;
    (0..ntests)
        .map(|_| {
            (
                rng.gen_range(-1.0..4.0),
                if rng.gen() { "A" } else { "B" },
            )
        })
        .map(|coord| {
            (
                coord,
                axes3d.bin(axes3d.index(&coord).unwrap()).unwrap(),
                (
                    xaxis.bin(xaxis.index(&coord.0).unwrap()).unwrap(),
                    yaxis.bin(yaxis.index(&coord.1).unwrap()).unwrap(),
                ),
            )
        })
        .enumerate().for_each(|(nthtest, (coord, actual, expected))| assert_eq!(actual, expected,
            "\nFailed on test:{}/{}.\n3D histogram failed to give expected result for:\n(x,y,z)={:?}", 
            nthtest, ntests, coord));
}

// #[test]
// fn test_axes_3d() {
//     let X = Uniform::new(2, 0.0, 2.0);
//     let Y = Uniform::new(3, 0.0, 3.0);
//     let Z = Category::new(vec!["A", "B"]);
//     let axes3d = (X.clone(), Y.clone(), Z.clone());
//     let mut rng = StdRng::seed_from_u64(123);
//     let ntests = 10000;
//     (0..ntests)
//         .map(|_| {
//             (
//                 rng.gen_range(-1.0..3.0),
//                 rng.gen_range(-1.0..4.0),
//                 if rng.gen() { "A" } else { "B" },
//             )
//         })
//         .map(|coord| {
//             (
//                 coord,
//                 axes3d.bin(axes3d.index(&coord).unwrap()).unwrap(),
//                 (
//                     X.bin(X.index(&coord.0).unwrap()).unwrap(),
//                     Y.bin(Y.index(&coord.1).unwrap()).unwrap(),
//                     Z.bin(Z.index(&coord.2).unwrap()).unwrap(),
//                 ),
//             )
//         })
//         .enumerate().for_each(|(nthtest, (coord, actual, expected))| assert_eq!(actual, expected,
//             "\nFailed on test:{}/{}.\n3D histogram failed to give expected result for:\n(x,y,z)={:?}",
//             nthtest, ntests, coord));
// }

macro_rules! make_nd_axes {
    () => {(,)};
    ($($d:ident)+) => {
        AxesTuple::from((
        $(
            {
                let $d = Uniform::new(2, 0.0, 2.0);
                $d
            },
        )*
    ))
    };
}

macro_rules! make_test_nd_axes {
    ($fnname:ident($($d:ident:$dimnum:tt),+)) => {
        #[test]
        fn $fnname() {
            let axes = make_nd_axes!($($d)+);
            $(let $d = Uniform::new(2, 0.0, 2.0);)+

            let mut rng = StdRng::seed_from_u64(12);
            let ntests = 10000;
            (0..ntests)
                .map(|_| {
                    (
                        $(
                            {
                                let $d = rng.gen_range(-0.1..2.1);
                                $d
                            },
                        )*
                    )
                })
                .map(|coord| {
                    (
                        coord,
                        axes.bin(axes.index(&coord).unwrap()).unwrap(),
                        (
                            $(
                                {
                                    let $d = $d.bin($d.index(&coord.$dimnum).unwrap()).unwrap();
                                    $d
                                },
                            )*
                        ),
                    )
                })
                .enumerate().for_each(|(nthtest, (coord, actual, expected))| assert_eq!(actual, expected,
                    "\nFailed on test:{}/{}.\nND histogram failed to give expected result for:\n(x,y,z)={:?}",
                    nthtest, ntests, coord));
        }
    }
}

make_test_nd_axes!(macro_test_axes_2d(x: 0, y: 1));
make_test_nd_axes!(macro_test_axes_3d(x: 0, y: 1, z: 2));
make_test_nd_axes!(macro_test_axes_4d(x: 0, y: 1, z: 2, t: 3));
make_test_nd_axes!(macro_test_axes_5d(x0: 0, x1: 1, x2: 2, x3: 3, x4: 4));
make_test_nd_axes!(macro_test_axes_6d(x0: 0, x1: 1, x2: 2, x3: 3, x4: 4, x5: 5));
make_test_nd_axes!(macro_test_axes_7d(x0: 0, x1: 1, x2: 2, x3: 3, x4: 4, x5: 5, x6: 6));
make_test_nd_axes!(macro_test_axes_8d(x0: 0, x1: 1, x2: 2, x3: 3, x4: 4, x5: 5, x6: 6, x7: 7));
make_test_nd_axes!(macro_test_axes_9d(x0: 0, x1: 1, x2: 2, x3: 3, x4: 4, x5: 5, x6: 6, x7: 7, x8: 8));
make_test_nd_axes!(macro_test_axes_10d(x0: 0, x1: 1, x2: 2, x3: 3, x4: 4, x5: 5, x6: 6, x7: 7, x8: 8, x9: 9));
make_test_nd_axes!(macro_test_axes_11d(x0: 0, x1: 1, x2: 2, x3: 3, x4: 4, x5: 5, x6: 6, x7: 7, x8: 8, x9: 9, x10: 10));
make_test_nd_axes!(macro_test_axes_12d(x0: 0, x1: 1, x2: 2, x3: 3, x4: 4, x5: 5, x6: 6, x7: 7, x8: 8, x9: 9, x10: 10, x11: 11));
