//! Crate errors
use crate::geometry::Position;

/// A [Space](crate::Space) parse error
#[derive(Debug, thiserror::Error)]
#[error("error at {0}: {1}")]
pub struct SpaceFromStringError(pub Position, pub InvalidChar);

/// An invalid character when representing a [Widget](crate::Widget)
#[derive(Debug, thiserror::Error)]
#[error("invalid char {0:?}")]
pub struct InvalidChar(pub char);
