use crate::Glyph;
use crate::errors::InvalidChar;

use self::Digit::*;

/// A single digit widget
#[derive(Copy, Clone, Debug)]
pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl TryFrom<char> for Digit {
    type Error = InvalidChar;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Zero),
            '1' => Ok(One),
            '2' => Ok(Two),
            '3' => Ok(Three),
            '4' => Ok(Four),
            '5' => Ok(Five),
            '6' => Ok(Six),
            '7' => Ok(Seven),
            '8' => Ok(Eight),
            '9' => Ok(Nine),
            _ => Err(InvalidChar(c)),
        }
    }
}

impl Glyph for Digit {
    fn glyph(&self) -> char {
        match self {
            Zero => '0',
            One => '1',
            Two => '2',
            Three => '3',
            Four => '4',
            Five => '5',
            Six => '6',
            Seven => '7',
            Eight => '8',
            Nine => '9',
        }
    }
}

impl From<Digit> for i32 {
    fn from(d: Digit) -> Self {
        match d {
            Zero => 0,
            One => 1,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
        }
    }
}
