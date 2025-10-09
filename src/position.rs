use derive_new::new;

#[derive(Copy, Clone, Debug, new, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub col: isize,
    pub row: isize,
}

impl From<(isize, isize)> for Position {
    fn from((col, row): (isize, isize)) -> Self {
        Position { col, row }
    }
}

impl From<(usize, usize)> for Position {
    fn from((c, r): (usize, usize)) -> Self {
        let u2i = |u| isize::try_from(u).unwrap();
        Self::from((u2i(c), u2i(r)))
    }
}
