use crate::Cell;

use self::Direction::*;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
