use thiserror::Error;

/// All errors that can occur in this crate can be converted into this type to
/// make handling errors from this crate easier if the user does not care about
/// the specific details of the cause of the error.
#[derive(Error, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Error {
    /// Returned if a Histogram binary operation fails (such as h1 + h2).
    #[error(transparent)]
    BinaryOperationError(#[from] BinaryOperationError),
    /// Returned if an Axis cannot be created due to invalid input parameters.
    #[error(transparent)]
    AxisError(#[from] AxisError),
}

#[derive(Error, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Returned if a Histogram binary operation fails (such as h1 + h2).
/// for example because the two histograms have incompatible binning.
#[error("histogram binary operation failed (check binning?)")]
pub struct BinaryOperationError;

/// Errors that can occur when creating an Axis, usually due to invalid input parameters.
#[derive(Error, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AxisError {
    /// Failed to create an axis as the number of bins is invalid for this axis type.
    #[error("number of bins should be positive and non-zero and must be convertible to the coordinate type")]
    InvalidNumberOfBins,
    /// Failed to create an axis due to an invalid range for this axis type (for example the low edge equaling the high edge).
    #[error("Invalid axis range. Low edge should not equal high edge.")]
    InvalidAxisRange,
    /// Failed to create an axis due to an invalid step size for this axis type (for example a negative step size).
    #[error("axis step size should be non-zero and positive")]
    InvalidStepSize,
    /// Failed to create an axis due to an invalid number of bin edges for this axis type.
    #[error("the number of bin edges must be at >= 2.")]
    InvalidNumberOfBinEdges,
    /// Failed to create an axis as the input bin edges are not sortable. This can happen if one of the bin edges is NaN.
    #[error("failed to sort bin_edges. The list of axis bin edges must be sortable.")]
    FailedToSortBinEdges,
}
