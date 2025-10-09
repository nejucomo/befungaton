use crate::{Glyph, GlyphStyle};

use self::Direction::*;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' | '▲' => Ok(North),
            'v' | '▼' => Ok(South),
            '>' | '▶' => Ok(East),
            '<' | '◀' => Ok(West),
            _ => Err(()),
        }
    }
}

impl Glyph for Direction {
    fn glyph(&self, style: GlyphStyle) -> char {
        use GlyphStyle::*;

        match style {
            Ascii => match *self {
                North => '^',
                South => 'v',
                East => '>',
                West => '<',
            },
            Cute => match *self {
                North => '▲',
                South => '▼',
                East => '▶',
                West => '◀',
            },
        }
    }
}
