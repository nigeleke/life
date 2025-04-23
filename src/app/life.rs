use thiserror::Error;

use super::arguments::Arguments;
use crate::life::{Generations, World, WorldError};

#[derive(Debug, Error)]
pub enum LifeErrors {
    #[error("cannot create world")]
    CannotCreateWorld(#[from] WorldError),
}

pub struct Life(Generations);

impl Life {
    pub fn run(&mut self) {
        let generations = &mut self.0;
        print!("{}{}", ansi::CLEAR_SCREEN, ansi::HOME);
        println!("{}", generations.current());
        for generation in generations.by_ref() {
            print!("{}", ansi::HOME);
            println!("{}", generation);
            let duration = std::time::Duration::from_millis(200);
            std::thread::sleep(duration);
        }
    }
}

impl TryFrom<&Arguments> for Life {
    type Error = LifeErrors;

    fn try_from(value: &Arguments) -> Result<Self, Self::Error> {
        let mut world = if let Some(path) = value.world() {
            World::try_from(path.as_path())?
        } else if let Some(pattern) = value.pattern() {
            World::try_from(pattern)?
        } else {
            World::random()
        };

        if let Some(viewport) = value.viewport() {
            world.with_viewport(viewport);
        }

        if let Some(bounds) = value.bounds() {
            world.with_bounds(bounds);
        }

        let generations = Generations::new(world);
        Ok(Life(generations))
    }
}

mod ansi {
    pub const CLEAR_SCREEN: &str = "\x1b[2J";
    pub const HOME: &str = "\x1b[H";
}
