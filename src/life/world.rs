use std::{collections::HashSet, path::Path};

use thiserror::*;

use super::{
    bounds::Bounds,
    cell::Cell,
    cells::{Cells, CellsError},
    pattern::Pattern,
    position::Position,
};

#[derive(Debug, Error)]
pub enum WorldError {
    #[error("cannot construct world: {0}")]
    BadPath(#[from] CellsError),
}

#[derive(Clone, Debug, PartialEq)]
pub struct World {
    live_cells: Cells,
    bounds: Bounds,
    viewport: Bounds,
}

impl World {
    pub fn random() -> Self {
        let mut cells = Cells::default();
        let bounds = Bounds::new(0..=20, 0..=20);
        bounds.rows().clone().for_each(|r| {
            bounds.columns().clone().for_each(|c| {
                if rand::random::<f32>() < 0.2 {
                    cells.insert(Cell::new(r, c));
                }
            })
        });
        World::from(cells)
    }

    pub fn live_cells(&self) -> &Cells {
        &self.live_cells
    }

    fn remove_off_worlders(&mut self) {
        if self.bounds.is_defined() {
            let (rows, columns) = (self.bounds.rows(), self.bounds.columns());
            self.live_cells
                .retain(|c| rows.contains(&c.row()) && columns.contains(&c.column()));
        }
    }

    pub fn with_bounds(&mut self, bounds: &Bounds) {
        self.bounds = bounds.clone();
        self.remove_off_worlders();
    }

    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }

    pub fn with_viewport(&mut self, viewport: &Bounds) {
        self.viewport = viewport.clone();
    }

    pub fn viewport(&self) -> &Bounds {
        &self.viewport
    }

    fn is_live(&self, cell: &Cell) -> bool {
        self.live_cells.contains(cell)
    }

    fn neighbour_count(&self, cell: &Cell) -> usize {
        cell.neighbours().filter(|cell| self.is_live(cell)).count()
    }

    pub fn next_generation(&mut self) {
        let cells_to_consider =
            self.live_cells
                .iter()
                .copied()
                .fold(HashSet::new(), |mut acc, c| {
                    acc.insert(c);
                    acc.extend(c.neighbours());
                    acc
                });

        let cell_counts = cells_to_consider
            .into_iter()
            .map(|c| (c, self.neighbour_count(&c)))
            .collect::<Vec<_>>();

        cell_counts.iter().for_each(|(cell, count)| {
            let _ = match count {
                0 | 1 => self.live_cells.remove(cell),
                2 => false,
                3 => self.live_cells.insert(*cell),
                _ => self.live_cells.remove(cell),
            };
        });

        self.remove_off_worlders();
    }

    pub fn is_empty(&self) -> bool {
        self.live_cells.is_empty()
    }

    pub fn add_cells(&mut self, cells: Cells, offset: &Position) {
        cells.iter().map(|c| *c + *offset).for_each(|c| {
            self.live_cells.insert(c);
        });
        self.remove_off_worlders();
    }
}

impl From<Cells> for World {
    fn from(value: Cells) -> Self {
        Self {
            live_cells: value,
            bounds: Bounds::default(),
            viewport: Bounds::default(),
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
        let bounds = if self.viewport.is_defined() {
            &self.viewport
        } else if self.bounds.is_defined() {
            &self.bounds
        } else {
            &default_bounds
        };

        let rows = bounds.rows();
        let columns = bounds.columns();

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
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn will_create_world_from_valid_file() {
        let path = Path::new("./tests/data/initial_world.life");
        assert!(World::try_from(path).is_ok());
    }

    #[test]
    fn will_not_create_world_from_invalid_file() {
        let path = Path::new("./tests/data/file_does_not_exist.life");
        let error = World::try_from(path).unwrap_err();
        assert!(matches!(error, WorldError::BadPath(_)));
    }
}
