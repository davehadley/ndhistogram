use thiserror::Error;

#[derive(Error, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Error {
    /// Returned if a Histogram binary operation fails (such as h1 + h2).
    /// for example because the two histograms have incompatible binning.
    #[error("histogram binary operation failed (check binning?)")]
    BinaryOperationError,
}
