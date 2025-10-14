use derive_new::new;
use itertools::Itertools as _;

use crate::geometry::span::Spanning;
use crate::geometry::{Position, Span};

#[derive(Debug, Default, new)]
pub struct Rect {
    pub cols: Span,
    pub rows: Span,
}

impl Spanning<Position> for Rect {
    fn extend_to_cover(&mut self, pos: Position) {
        self.cols.extend_to_cover(pos.col);
        self.rows.extend_to_cover(pos.row);
    }
}

impl IntoIterator for &Rect {
    type Item = Position;
    type IntoIter = impl Iterator<Item = Position>;

    fn into_iter(self) -> Self::IntoIter {
        self.cols
            .into_iter()
            .cartesian_product(&self.rows)
            .map(|(c, r)| Position::new(c, r))
    }
}
