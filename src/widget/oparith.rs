use crate::Glyph;
use crate::errors::InvalidChar;

use self::OpArith::*;

#[derive(Copy, Clone, Debug)]
pub enum OpArith {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

impl OpArith {
    pub fn calc(self, a: i32, b: i32) -> i32 {
        match self {
            Add => a + b,
            Sub => a - b,
            Mul => a * b,
            Div => a / b,
            Rem => a % b,
        }
    }
}

impl TryFrom<char> for OpArith {
    type Error = InvalidChar;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '+' => Ok(Add),
            '-' => Ok(Sub),
            '*' => Ok(Mul),
            '/' => Ok(Div),
            '%' => Ok(Rem),
            _ => Err(InvalidChar(c)),
        }
    }
}

impl Glyph for OpArith {
    fn glyph(&self) -> char {
        match self {
            Add => '+',
            Sub => '-',
            Mul => '*',
            Div => '/',
            Rem => '%',
        }
    }
}
