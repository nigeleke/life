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

    pub fn neighbours(&self) -> Vec<Cell> {
        (-1..=1)
            .flat_map(|delta_row| {
                (-1..=1)
                    .filter(move |delta_column| (delta_row != 0 || *delta_column != 0))
                    .map(move |delta_column| *self + Position::new(delta_row, delta_column))
            })
            .collect::<Vec<_>>()
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
