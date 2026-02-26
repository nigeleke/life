/// Arbitary [Position] on a board.
/// @param row
/// @param column
#[derive(Clone, Copy)]
pub struct Position {
    row: isize,
    column: isize,
}

impl Position {
    pub fn new(row: isize, column: isize) -> Self {
        Self { row, column }
    }

    pub fn row(&self) -> isize {
        self.row
    }

    pub fn column(&self) -> isize {
        self.column
    }
}
