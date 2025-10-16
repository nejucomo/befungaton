#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;

use crate::errors::SpaceFromStringError;
use crate::geometry::{Position, Rect, Spanning as _};
use crate::{Cell, Cursor, DEFAULT_CELL, Glyph as _, Physical, Widget};

/// The full state space of an interpreter instance
#[derive(Debug, Default)]
pub struct Space {
    cells: BTreeMap<Position, Cell>,
    span: Rect,
}

impl Space {
    /// Step each [Cursor], and then apply the effects of the new [Cursor] [Position]s
    pub fn step_cursors(&mut self) {
        let mut deltas = vec![];

        for (&pos, mutcell) in self.cells.iter_mut() {
            while let Some(cursor) = mutcell.pop_cursor() {
                let newpos = pos + cursor.direction;
                deltas.push((newpos, cursor));
            }
        }

        for (pos, cursor) in deltas {
            self.insert(pos, cursor);
        }
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
}

impl TryFrom<&str> for Space {
    type Error = SpaceFromStringError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use crate::geometry::Direction::East;

        let mut space = Space::default();

        for (row, rowtext) in s.split('\n').enumerate() {
            for (col, c) in rowtext.chars().enumerate() {
                let pos = Position::new_conv(col, row);
                let widget = Widget::try_from(c).map_err(|ic| SpaceFromStringError(pos, ic))?;
                space.insert(pos, widget);
            }
        }

        space.insert((0, 0), Cursor::new(East));

        Ok(space)
    }
}

impl FromStr for Space {
    type Err = SpaceFromStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.span.rows {
            let mut line = "".to_string();
            for col in &self.span.cols {
                line.push(self.get_cell((col, row)).glyph());
            }
            writeln!(f, "{}", line.trim_end())?;
        }

        Ok(())
    }
}
