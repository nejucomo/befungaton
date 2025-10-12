use crate::geometry::Position;

#[derive(Debug, thiserror::Error)]
#[error("error at {0}: {1}")]
pub struct SpaceFromStringError(pub Position, pub InvalidChar);

#[derive(Debug, thiserror::Error)]
#[error("invalid char {0:?}")]
pub struct InvalidChar(pub char);
