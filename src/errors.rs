//! Crate errors
use crate::geometry::Position;

/// A [Space](crate::Space) IO or parse error
#[derive(Debug, thiserror::Error)]
pub enum IOParseError {
    /// An IO error case
    #[error(transparent)]
    IO(#[from] std::io::Error),
    /// An Parse error case
    #[error(transparent)]
    Parse(#[from] ParseError),
}

/// A [Space](crate::Space) parse error
#[derive(Debug, thiserror::Error)]
#[error("error at {0}: {1}")]
pub struct ParseError(pub Position, pub InvalidChar);

/// An invalid character when representing a [Widget](crate::Widget)
#[derive(Debug, thiserror::Error)]
#[error("invalid char {0:?}")]
pub struct InvalidChar(pub char);
