use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

use super::cell::Cell;

#[derive(Clone, Debug, PartialEq)]
pub struct Bounds {
    rows: RangeInclusive<isize>,
    columns: RangeInclusive<isize>,
}

impl Bounds {
    pub fn new(rows: RangeInclusive<isize>, columns: RangeInclusive<isize>) -> Self {
        Self { rows, columns }
    }

    pub fn is_defined(&self) -> bool {
        !(self.rows.is_empty() || self.columns.is_empty())
    }

    pub fn encompass(&mut self, cell: &Cell) {
        let Self { rows, columns } = self;
        let (r_min, r_max) = (*rows.start(), *rows.end());
        let (c_min, c_max) = (*columns.start(), *columns.end());
        let rows = min(r_min, cell.row())..=max(r_max, cell.row());
        let columns = min(c_min, cell.column())..=max(c_max, cell.column());
        *self = Self::new(rows, columns)
    }

    pub fn rows(&self) -> &RangeInclusive<isize> {
        &self.rows
    }

    pub fn columns(&self) -> &RangeInclusive<isize> {
        &self.columns
    }
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            rows: RangeInclusive::new(0, -1),
            columns: RangeInclusive::new(0, -1),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_bounds_are_undefined() {
        assert!(!Bounds::default().is_defined())
    }

    #[test]
    fn default_bounds_rows_are_undefined() {
        assert!(Bounds::default().rows().is_empty());
    }

    #[test]
    fn default_bounds_columns_are_undefined() {
        assert!(Bounds::default().columns().is_empty());
    }
}
