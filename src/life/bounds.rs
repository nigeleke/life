use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

use super::cell::Cell;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Bounds {
    #[default]
    Undefined,
    Defined(RangeInclusive<isize>, RangeInclusive<isize>),
}

impl Bounds {
    pub fn is_defined(&self) -> bool {
        matches!(self, Bounds::Defined(_, _))
    }

    pub fn encompass(&mut self, cell: &Cell) {
        *self = match self {
            Bounds::Undefined => {
                Bounds::Defined(cell.row()..=cell.row(), cell.column()..=cell.column())
            }
            Bounds::Defined(rows, columns) => {
                let (r_min, r_max) = (*rows.start(), *rows.end());
                let (c_min, c_max) = (*columns.start(), *columns.end());
                let rows = min(r_min, cell.row())..=max(r_max, cell.row());
                let columns = min(c_min, cell.column())..=max(c_max, cell.column());
                Bounds::Defined(rows, columns)
            }
        };
    }

    pub fn rows(&self) -> Option<&RangeInclusive<isize>> {
        match self {
            Bounds::Undefined => None,
            Bounds::Defined(rows, _) => Some(rows),
        }
    }

    pub fn columns(&self) -> Option<&RangeInclusive<isize>> {
        match self {
            Bounds::Undefined => None,
            Bounds::Defined(_, columns) => Some(columns),
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn default_bounds_are_undefined() {
        assert_eq!(Bounds::default(), Bounds::Undefined);
    }

    #[test]
    fn there_are_no_default_bounds_rows() {
        assert_eq!(Bounds::default().rows(), None);
    }

    #[test]
    fn there_are_no_default_bounds_columns() {
        assert_eq!(Bounds::default().columns(), None);
    }
}
