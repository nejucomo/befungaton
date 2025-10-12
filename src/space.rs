use std::collections::BTreeMap;
use std::ops::{Index, IndexMut};

use crate::{Cell, Position};

#[derive(Debug, Default)]
pub struct Space {
    cells: BTreeMap<Position, Cell>,
}

impl TryFrom<&str> for Space {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let myspace = Space {
            cells: BTreeMap::default(),
        };

        for (row, rowtext) in s.split('\n').enumerate() {
            for (col, c) in rowtext.chars().enumerate() {
                let pos = Position::try_new(col, row)?;
                let cell = Cell::try_from(c)?;
                myspace.cells[pos] = cell;
            }
        }

        Ok(myspace)
    }
}

// impl<T> Index<T> for Space
// where
//     Position: From<T>,
// {
//     type Output = Cell;

//     fn index(&self, index: T) -> &Self::Output {
//         &self.cells[&Position::from(index)]
//     }
// }

// impl<T> IndexMut<T> for Space
// where
//     Position: From<T>,
// {
//     fn index_mut(&mut self, index: T) -> &mut Self::Output {
//         self.cells.entry(Position::from(index)).or_default()
//     }
// }
