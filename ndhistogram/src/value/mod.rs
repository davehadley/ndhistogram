//! Bin Value types for ND-histograms
//!
//! This module contains implementations of types intended to be used as bin
//! value types in [Histogram](crate::Histogram)s.
//! See [crate front page](crate) for a summary of the provided types.
//!
mod weightedsum;
pub use weightedsum::WeightedSum;
mod sum;
pub use sum::Sum;
mod mean;
pub use mean::Mean;
mod weightedmean;
pub use weightedmean::WeightedMean;
