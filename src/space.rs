#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;

use crate::errors::SpaceFromStringError;
use crate::geometry::{Position, Rect, Spanning as _};
use crate::{Cell, Cursor, Glyph as _, Physical, Widget};

#[derive(Debug, Default)]
pub struct Space {
    cells: BTreeMap<Position, Cell>,
    span: Rect,
}

impl Space {
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

    fn insert<P>(&mut self, pos: Position, object: P)
    where
        P: Physical,
    {
        self.cells.entry(pos).or_default().insert(object);
        self.span.extend_to_cover(pos);
    }
}

impl TryFrom<&str> for Space {
    type Error = SpaceFromStringError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use crate::geometry::Direction::East;

        let mut space = Space::default();

        for (row, rowtext) in s.split('\n').enumerate() {
            for (col, c) in rowtext.chars().enumerate() {
                let pos = Position::try_new(col, row).unwrap();
                let widget = Widget::try_from(c).map_err(|ic| SpaceFromStringError(pos, ic))?;
                space.insert(pos, widget);
            }
        }

        space.insert(Position::new(0, 0), Cursor::new(East));

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
        let mut lastrow = self.span.rows.start;

        for pos in &self.span {
            if lastrow != pos.row {
                // New line:
                writeln!(f)?;
            }
            lastrow = pos.row;

            self.cells
                .get(&pos)
                .map(|c| c.glyph())
                .unwrap_or(' ')
                .fmt(f)?;
        }
        writeln!(f)?;

        Ok(())
    }
}
