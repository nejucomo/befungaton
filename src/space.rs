#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;

use crate::errors::{IOParseError, ParseError};
use crate::geometry::{Position, Rect};
use crate::{Cell, Cursor, DEFAULT_CELL, Glyph as _, Physical, Widget};

/// The full state space of an interpreter instance
#[derive(Debug, Default)]
pub struct Space {
    cells: BTreeMap<Position, Cell>,
    span: Rect,
}

impl Space {
    /// Step each [Cursor], and then apply the effects of the new [Cursor] [Position]s
    pub fn step_cursors(&mut self) -> Vec<Position> {
        let mut ps = vec![];
        let mut cursors = vec![];

        for (&pos, mutcell) in self.cells.iter_mut() {
            while let Some(cursor) = mutcell.pop_cursor() {
                ps.push(pos + cursor.direction);
                cursors.push(cursor);
            }
        }

        for (&pos, cursor) in ps.iter().zip(cursors) {
            self.insert(pos, cursor);
        }

        ps
    }

    fn insert<P, O>(&mut self, pos: P, object: O)
    where
        Position: From<P>,
        O: Physical,
    {
        let pos = Position::from(pos);
        self.cells.entry(pos).or_default().insert(object);
        self.span.extend_to_cover(pos);
    }

    fn get_cell<P>(&self, pos: P) -> &Cell
    where
        Position: From<P>,
    {
        self.cells.get(&Position::from(pos)).unwrap_or(DEFAULT_CELL)
    }

    /// Get a mutable reference to the cell, if present
    #[cfg(test)]
    pub fn mut_cell<P>(&mut self, pos: P) -> Option<&mut Cell>
    where
        Position: From<P>,
    {
        self.cells.get_mut(&Position::from(pos))
    }
}

impl TryFrom<std::path::PathBuf> for Space {
    type Error = IOParseError;

    fn try_from(path: std::path::PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(path.as_path())
    }
}

impl TryFrom<&std::path::Path> for Space {
    type Error = IOParseError;

    fn try_from(path: &std::path::Path) -> Result<Self, Self::Error> {
        let f = std::fs::File::open(path)?;
        Self::try_from(f)
    }
}

impl TryFrom<std::fs::File> for Space {
    type Error = IOParseError;

    fn try_from(mut f: std::fs::File) -> Result<Self, Self::Error> {
        use std::io::Read as _;

        let mut src = String::new();
        f.read_to_string(&mut src)?;
        let sp = Self::try_from(src)?;
        Ok(sp)
    }
}

impl TryFrom<String> for Space {
    type Error = ParseError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl TryFrom<&str> for Space {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use crate::geometry::Direction::East;

        let mut space = Space::default();

        for (row, rowtext) in s.split('\n').enumerate() {
            for (col, c) in rowtext.chars().enumerate() {
                let pos = Position::new_conv(col, row);
                let widget = Widget::try_from(c).map_err(|ic| ParseError(pos, ic))?;
                space.insert(pos, widget);
            }
        }

        space.insert((0, 0), Cursor::new(East));

        Ok(space)
    }
}

impl FromStr for Space {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.span.rows() {
            let mut line = "".to_string();
            for col in self.span.cols() {
                line.push(self.get_cell((col, row)).glyph());
            }
            writeln!(f, "{}", line.trim_end())?;
        }

        Ok(())
    }
}
