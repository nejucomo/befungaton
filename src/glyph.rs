use crate::errors::InvalidChar;

pub trait Glyph: TryFrom<char, Error = InvalidChar> {
    fn glyph(&self) -> char;
}
