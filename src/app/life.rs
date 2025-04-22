use std::io::Write;

use thiserror::Error;

use crate::life::{Cells, CellsError, Generations, World};

use super::arguments::Arguments;

#[derive(Debug, Error)]
pub enum LifeErrors {
    #[error("cannot create world")]
    CannotCreateWorld(#[from] CellsError),
}

pub struct Life(Generations);

impl Life {
    pub fn run(&mut self) {
        let generations = &mut self.0;
        print!("{}{}", ansi::CLEAR_SCREEN, ansi::HOME);
        println!("{}", generations.current());
        std::io::stdout().flush().unwrap();
        while let Some(generation) = generations.next() {
            print!("{}", ansi::HOME);
            println!("{}", generation);
            std::io::stdout().flush().unwrap();
            let duration = std::time::Duration::from_millis(200);
            std::thread::sleep(duration);
        }
    }
}

impl TryFrom<&Arguments> for Life {
    type Error = LifeErrors;

    fn try_from(value: &Arguments) -> Result<Self, Self::Error> {
        let mut world = if let Some(path) = value.world() {
            let cells = Cells::try_from(path.as_path())?;
            World::from(cells)
        } else {
            World::random()
        };

        if let Some(viewport) = value.viewport() {
            world.with_viewport(viewport.clone());
        }

        if let Some(bounds) = value.bounds() {
            world.with_bounds(bounds.clone());
        }

        let generations = Generations::new(world);
        Ok(Life(generations))
    }
}

mod ansi {
    pub const CLEAR_SCREEN: &str = "\x1b[2J";
    pub const HOME: &str = "\x1b[H";
}
