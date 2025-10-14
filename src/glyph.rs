/// A value which has a [char] representation
pub trait Glyph {
    /// The [char] representation of `self`
    fn glyph(&self) -> char;
}
