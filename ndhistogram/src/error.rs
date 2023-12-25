use thiserror::Error;

#[derive(Error, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Error {
    /// Returned if a Histogram binary operation fails (such as h1 + h2).
    /// for example because the two histograms have incompatible binning.
    #[error("histogram binary operation failed (check binning?)")]
    BinaryOperationError,
    #[error("number of bins should be positive and non-zero")]
    InvalidNumberOfBins,
    #[error("Invalid axis range. Low edge should not equal high edge.")]
    InvalidAxisRange,
    #[error("axis step size should be non-zero and positive")]
    InvalidStepSize,
    #[error("the number of bin edges must be at >= 2.")]
    InvalidNumberOfBinEdges,
    #[error("failed to sort bin_edges. The list of axis bin edges must be sortable.")]
    FailedToSortBinEdges,
}
