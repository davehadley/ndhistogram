use ndhistogram::axis::{
    Axis, Uniform, UniformCyclic, UniformNoFlow, Variable, VariableCyclic, VariableNoFlow,
};

macro_rules! test_get_index_of_nan {
    ($fnname:ident, $constructor:expr) => {
        #[test]
        fn $fnname() {
            let ax = $constructor;
            let actual = ax.index(&std::f64::NAN);
            let expected = None;
            assert_eq!(expected, actual)
        }
    };
}

macro_rules! test_get_index_of_inf {
    ($fnname:ident, $constructor:expr, $expected:expr) => {
        #[test]
        fn $fnname() {
            let ax = $constructor;
            let actual = ax.index(&std::f64::INFINITY);
            let expected = $expected;
            assert_eq!(expected, actual)
        }
    };
}

macro_rules! test_get_index_of_neg_inf {
    ($fnname:ident, $constructor:expr, $expected:expr) => {
        #[test]
        fn $fnname() {
            let ax = $constructor;
            let actual = ax.index(&std::f64::NEG_INFINITY);
            let expected = $expected;
            assert_eq!(expected, actual)
        }
    };
}

test_get_index_of_nan!(test_uniform_get_index_of_nan, Uniform::new(5, 0.0, 1.0));
test_get_index_of_nan!(
    test_uniformnoflow_get_index_of_nan,
    UniformNoFlow::new(5, 0.0, 1.0)
);

test_get_index_of_nan!(
    test_uniformcyclic_get_index_of_nan,
    UniformCyclic::new(5, 0.0, 1.0)
);
test_get_index_of_nan!(
    test_variable_get_index_of_nan,
    Variable::new(vec![0.0, 1.0])
);
test_get_index_of_nan!(
    test_variablenoflow_get_index_of_nan,
    VariableNoFlow::new(vec![0.0, 1.0])
);

test_get_index_of_nan!(
    test_variablecyclic_get_index_of_nan,
    VariableCyclic::new(vec![0.0, 1.0])
);

test_get_index_of_inf!(
    test_uniform_get_index_of_inf,
    Uniform::new(5, 0.0, 1.0),
    Some(6)
);
test_get_index_of_inf!(
    test_uniformnoflow_get_index_of_inf,
    UniformNoFlow::new(5, 0.0, 1.0),
    None
);

test_get_index_of_inf!(
    test_uniformcyclic_get_index_of_inf,
    UniformCyclic::new(5, 0.0, 1.0),
    None
);
test_get_index_of_inf!(
    test_variable_get_index_of_inf,
    Variable::new(vec![0.0, 1.0]),
    Some(2)
);
test_get_index_of_inf!(
    test_variablenoflow_get_index_of_inf,
    VariableNoFlow::new(vec![0.0, 1.0]),
    None
);

test_get_index_of_inf!(
    test_variablecyclic_get_index_of_inf,
    VariableCyclic::new(vec![0.0, 1.0]),
    None
);

test_get_index_of_neg_inf!(
    test_uniform_get_index_of_neg_inf,
    Uniform::new(5, 0.0, 1.0),
    Some(0)
);
test_get_index_of_neg_inf!(
    test_uniformnoflow_get_index_of_neg_inf,
    UniformNoFlow::new(5, 0.0, 1.0),
    None
);

test_get_index_of_neg_inf!(
    test_uniformcyclic_get_index_of_neg_inf,
    UniformCyclic::new(5, 0.0, 1.0),
    None
);
test_get_index_of_neg_inf!(
    test_variable_get_index_of_neg_inf,
    Variable::new(vec![0.0, 1.0]),
    Some(0)
);
test_get_index_of_neg_inf!(
    test_variablenoflow_get_index_of_neg_inf,
    VariableNoFlow::new(vec![0.0, 1.0]),
    None
);

test_get_index_of_neg_inf!(
    test_variablecyclic_get_index_of_neg_inf,
    VariableCyclic::new(vec![0.0, 1.0]),
    None
);
