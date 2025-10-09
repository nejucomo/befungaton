use std::collections::BTreeMap;
use std::ops::{Index, IndexMut};

use crate::{Cell, Position};

#[derive(Debug, Default)]
pub struct Space(BTreeMap<Position, Cell>);

impl From<&str> for Space {
    fn from(s: &str) -> Self {
        let mut myspace = Space(BTreeMap::default());

        for (row, rowtext) in s.split('\n').enumerate() {
            for (col, c) in rowtext.chars().enumerate() {
                myspace[(col, row)] = Cell::from(c);
            }
        }

        myspace
    }
}

impl<T> Index<T> for Space
where
    Position: From<T>,
{
    type Output = Cell;

    fn index(&self, index: T) -> &Self::Output {
        &self.0[&Position::from(index)]
    }
}

impl<T> IndexMut<T> for Space
where
    Position: From<T>,
{
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        self.0.entry(Position::from(index)).or_default()
    }
}
