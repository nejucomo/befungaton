use crate::Direction;

/// A Cell is isomorphic to a unicode scalar value
#[derive(Copy, Clone)]
pub struct Cell(char);

impl From<char> for Cell {
    fn from(c: char) -> Self {
        Cell(c)
    }
}

impl Cell {
    pub fn new_cursor(d: Direction) -> Self {
        use Direction::*;

        Cell::from(match d {
            North => '▲',
            South => '▼',
            East => '▶',
            West => '◀',
        })
    }

    pub fn as_cursor_direction(self) -> Option<Direction> {
        use Direction::*;

        match self.0 {
            '▲' => Some(North),
            '▼' => Some(South),
            '▶' => Some(East),
            '◀' => Some(West),
            _ => None,
        }
    }
}

impl PartialEq<char> for Cell {
    fn eq(&self, other: &char) -> bool {
        self.0 == *other
    }
}
