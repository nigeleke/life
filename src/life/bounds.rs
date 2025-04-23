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

// /** Extract [Bounds] from a [Config]. The [Config] must be the root for the [Bounds] definition. The definition requires each of
//   * `minRow`, `maxRow`, `minColumn` and `maxColumn` to be defined, e.g.,
//   *
//   * ```
//   * mybounds {
//   *   minRow: 0
//   *   maxRow: 40
//   *   minColumn: 0
//   *   maxColumn: 80
//   * }
//   * ```
//   *
//   * @param config
//   *   The root [Config] object
//   * @return
//   *   The [Bounds] object derived from the [Config] settings.
//   */
// def from(config: Config) =
//   def intFor(s: String) = config.getInt(s)
//   Bounds(
//     intFor("minRow") to intFor("maxRow"),
//     intFor("minColumn") to intFor("maxColumn")
//   )
