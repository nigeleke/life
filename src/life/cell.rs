use super::position::Position;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cell {
    row: isize,
    column: isize,
}

impl Cell {
    pub fn new(row: isize, column: isize) -> Self {
        Self { row, column }
    }

    pub fn row(&self) -> isize {
        self.row
    }

    pub fn column(&self) -> isize {
        self.column
    }

    pub fn neighbours(&self) -> impl Iterator<Item = Cell> + '_ {
        #[rustfmt::skip]
        const DELTAS: [(isize, isize); 8] = [
            (-1, -1), (-1, 0), (-1, 1),
            ( 0, -1),          ( 0, 1),
            ( 1, -1), ( 1, 0), ( 1, 1),
        ];

        DELTAS
            .into_iter()
            .map(move |(dr, dc)| *self + Position::new(dr, dc))
    }
}

impl std::ops::Add<Position> for Cell {
    type Output = Self;

    fn add(self, rhs: Position) -> Self::Output {
        Self::new(self.row + rhs.row(), self.column + rhs.column())
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cell({}, {})", self.row, self.column)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn cell_can_be_displayed() {
        let cell = Cell::new(42, 31);
        assert_eq!(cell.to_string(), "Cell(42, 31)".to_string());
    }
}
