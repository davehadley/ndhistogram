use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Display};

/// Returned if a Histogram binary operation fails (such as h1 + h2).
/// for example because the two histograms have incompatible binning.
#[derive(
    Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize,
)]
pub struct HistogramBinaryOperationError;

impl Display for HistogramBinaryOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HistogramBinaryOperationError")
    }
}

impl Error for HistogramBinaryOperationError {}
