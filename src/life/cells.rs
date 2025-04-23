use std::{
    collections::{HashSet, hash_set},
    iter::Rev,
    ops::RangeInclusive,
    path::Path,
};

use hashable::HashableHashSet;
use thiserror::*;

use super::{bounds::Bounds, cell::Cell};

#[derive(Debug, Error)]
pub enum CellsError {
    #[error("inconsistent line lengths")]
    InconsistentLineLengths,

    #[error("io error: {0}")]
    FileError(#[from] std::io::Error),
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Cells(HashableHashSet<Cell>);

impl Cells {
    pub fn new(cells: HashSet<Cell>) -> Self {
        Cells(HashableHashSet::from_iter(cells))
    }

    pub fn bounds(&self) -> Bounds {
        let mut bounds = Bounds::default();
        self.0.iter().for_each(|cell| {
            bounds.encompass(cell);
        });
        bounds
    }

    pub fn contains(&self, cell: &Cell) -> bool {
        self.0.contains(cell)
    }

    pub fn add_cell(&mut self, cell: Cell) {
        self.0.insert(cell);
    }

    pub fn add_cells(&mut self, cells: Cells) {
        cells.iter().for_each(|c| self.add_cell(*c));
    }

    pub fn remove_cells(&mut self, cells: Cells) {
        cells.iter().for_each(|c| {
            let _ = self.0.remove(c);
        });
    }

    pub fn iter(&self) -> hash_set::Iter<Cell> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn rotate(self, n: isize) -> Self {
        const ORDER: isize = 4;
        let modulus = n % ORDER;
        let cycle = if modulus >= 0 {
            modulus
        } else {
            ORDER + modulus
        };
        let bounds = self.bounds();
        let row = bounds.rows().expect("bound rows");
        let row_reversed = row.clone().rev();
        let column = bounds.columns().expect("bound columns");
        let column_reversed = column.clone().rev();

        let rev_index = |range: &RangeInclusive<isize>, rev: &Rev<RangeInclusive<isize>>| {
            let enumerated_range = Vec::from_iter(range.clone().enumerate());
            let rev = Vec::from_iter(rev.clone());
            move |original: isize| {
                let index = enumerated_range
                    .iter()
                    .find_map(|(i, c)| (*c == original).then_some(i))
                    .unwrap();
                rev[*index]
            }
        };

        let row_reversed = rev_index(row, &row_reversed);
        let column_reversed = rev_index(column, &column_reversed);

        let mut result = Cells::default();
        self.0.iter().for_each(|c| {
            let new_cell = match cycle {
                0 => *c,
                1 => Cell::new(c.column(), row_reversed(c.row())),
                2 => Cell::new(row_reversed(c.row()), column_reversed(c.column())),
                3 => Cell::new(column_reversed(c.column()), c.row()),
                _ => unreachable!(),
            };
            result.add_cell(new_cell);
        });
        result
    }
}

impl TryFrom<&str> for Cells {
    type Error = CellsError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use std::collections::HashSet;

        let mut lengths = HashSet::new();
        let mut cells = HashSet::new();

        for (ri, line) in value
            .lines()
            .map(|l| l.trim().replace(" ", ""))
            .filter(|l| !l.is_empty())
            .enumerate()
        {
            lengths.insert(line.len());
            (lengths.len() <= 1)
                .then_some(())
                .ok_or(CellsError::InconsistentLineLengths)?;

            for (ci, c) in line.chars().enumerate() {
                if c != '.' {
                    cells.insert(Cell::new(ri as isize, ci as isize));
                }
            }
        }

        Ok(Cells(HashableHashSet::from_iter(cells)))
    }
}

impl TryFrom<&Path> for Cells {
    type Error = CellsError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let content = std::fs::read_to_string(value)?;
        Cells::try_from(content.as_str())
    }
}

impl FromIterator<Cell> for Cells {
    fn from_iter<T: IntoIterator<Item = Cell>>(iter: T) -> Self {
        let cells = HashableHashSet::from_iter(iter);
        Self(cells)
    }
}

impl std::fmt::Display for Cells {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn cells_can_be_displayed() {
        let cells = Cells::from_iter([Cell::new(42, 31), Cell::new(31, 42)]);
        assert_eq!(cells.to_string(), "Cell(42, 31), Cell(31, 42)");
    }
}
