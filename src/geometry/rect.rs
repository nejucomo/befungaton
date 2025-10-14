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
        self.rows
            .into_iter()
            .cartesian_product(&self.cols)
            .map(|(r, c)| Position::new(c, r))
    }
}

#[test]
fn test_rect_iter_order() {
    use crate::geometry::Span;

    let r = Rect {
        cols: Span::from(0..2),
        rows: Span::from(0..2),
    };

    let actual: Vec<Position> = r.into_iter().collect();

    let expected = vec![
        Position::new(0, 0),
        Position::new(1, 0),
        Position::new(0, 1),
        Position::new(1, 1),
    ];

    assert_eq!(actual, expected);
}
