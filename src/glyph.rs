pub trait Glyph {
    fn glyph(&self, style: GlyphStyle) -> char;
}

#[derive(Copy, Clone, Debug)]
pub enum GlyphStyle {
    Ascii,
    Cute,
}
