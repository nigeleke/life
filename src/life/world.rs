use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::path::Path;

use thiserror::*;

use super::bounds::Bounds;
use super::cell::Cell;
use super::cells::{Cells, CellsError};
use super::pattern::Pattern;

#[derive(Debug, Error)]
pub enum WorldError {
    #[error("cannot construct world: {0}")]
    BadPath(#[from] CellsError),
}

#[derive(Clone, Debug, PartialEq)]
pub struct World {
    live_cells: Cells,
    bounds: Option<Bounds>,
    viewport: Option<Bounds>,
}

impl World {
    pub fn random() -> Self {
        let mut cells = Cells::default();
        let bounds = Bounds::Defined(0..=20, 0..=20);
        bounds.rows().into_iter().for_each(|rs| {
            rs.clone().for_each(|r| {
                bounds.columns().into_iter().for_each(|cs| {
                    cs.clone().for_each(|c| {
                        if rand::random::<f32>() < 0.2 {
                            cells.add_cell(Cell::new(r, c));
                        }
                    })
                })
            })
        });
        World::from(cells)
    }

    pub fn live_cells(&self) -> &Cells {
        &self.live_cells
    }

    fn remove_off_worlders(&mut self) {
        if let Some(bounds) = self.bounds.as_ref() {
            let within_range = |r: &RangeInclusive<isize>, i: isize| r.contains(&i);
            let within_bounds = |c: &Cell| {
                within_range(bounds.rows().unwrap(), c.row())
                    && within_range(bounds.columns().unwrap(), c.column())
            };
            let cells_in_bounds = self
                .live_cells
                .iter()
                .filter_map(|c| within_bounds(c).then_some(*c))
                .collect::<HashSet<_>>();
            self.live_cells = Cells::new(cells_in_bounds);
        };
    }

    pub fn with_bounds(&mut self, bounds: Bounds) {
        self.bounds = Some(bounds);
        self.remove_off_worlders()
    }

    pub fn with_viewport(&mut self, viewport: Bounds) {
        self.viewport = Some(viewport);
    }

    fn is_live(&self, cell: &Cell) -> bool {
        self.live_cells.contains(cell)
    }

    fn neighbour_count(&self, cell: &Cell) -> usize {
        cell.neighbours()
            .iter()
            .filter(|cell| self.is_live(cell))
            .count()
    }

    pub fn next_generation(&mut self) {
        let mut cells_to_consider = HashSet::<Cell>::new();

        self.live_cells.iter().for_each(|c| {
            cells_to_consider.insert(*c);
            cells_to_consider.extend(c.neighbours());
        });

        let neighbour_counts = cells_to_consider
            .iter()
            .map(|c| (c, self.neighbour_count(c)))
            .collect::<Vec<_>>();

        let dying_cells = neighbour_counts
            .iter()
            .filter_map(|(cell, count)| (*count < 2 || *count > 3).then_some(**cell));

        let born_cells = neighbour_counts
            .iter()
            .filter_map(|(cell, count)| (*count == 3).then_some(**cell));

        self.live_cells.remove_cells(Cells::from_iter(dying_cells));
        self.live_cells.add_cells(Cells::from_iter(born_cells));
        self.remove_off_worlders()
    }

    pub fn is_empty(&self) -> bool {
        self.live_cells.is_empty()
    }
}

impl From<Cells> for World {
    fn from(value: Cells) -> Self {
        Self {
            live_cells: value,
            bounds: None,
            viewport: None,
        }
    }
}

impl TryFrom<&Path> for World {
    type Error = WorldError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let cells = Cells::try_from(value)?;
        Ok(Self::from(cells))
    }
}

impl TryFrom<&Pattern> for World {
    type Error = WorldError;

    fn try_from(value: &Pattern) -> Result<Self, Self::Error> {
        let cells = Cells::try_from(value.cells_str())?;
        Ok(Self::from(cells))
    }
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let default_bounds = self.live_cells.bounds();
        let bounds = self
            .viewport
            .as_ref()
            .or(self.bounds.as_ref())
            .unwrap_or(&default_bounds);

        if let (Some(rows), Some(columns)) = (bounds.rows(), bounds.columns()) {
            let pretty_row = |r: isize| {
                columns
                    .clone()
                    .map(|c| {
                        if self.is_live(&Cell::new(r, c)) {
                            "*"
                        } else {
                            " "
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            };

            let pretty_all = rows.clone().map(pretty_row).collect::<Vec<_>>().join("\n");
            pretty_all.fmt(f)
        } else {
            "".fmt(f)
        }
    }
}
