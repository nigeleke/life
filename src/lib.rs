mod app;
mod life;

pub mod prelude {
    pub use super::app::{Arguments, Life};
    pub use super::life::{Bounds, Cell, Cells, CellsError, Generations, Pattern, Position, World};
}
