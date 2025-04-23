use thiserror::Error;

use super::arguments::Arguments;
use crate::life::{Generations, World, WorldError};

#[derive(Debug, Error)]
pub enum LifeError {
    #[error("cannot create world")]
    CannotCreateWorld(#[from] WorldError),
}

#[derive(Debug)]
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
    type Error = LifeError;

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

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::*;

    #[test]
    fn can_be_created_from_valid_program_args() {
        let args = "app --pattern=beehive".split_whitespace();
        let args = Arguments::parse_from(args);
        assert!(Life::try_from(&args).is_ok())
    }

    #[test]
    fn will_be_randomly_created_if_no_source_args() {
        let args = "app".split_whitespace();
        let args = Arguments::parse_from(args);
        assert!(Life::try_from(&args).is_ok())
    }

    #[test]
    fn will_run_to_completion() {
        let args = "app --pattern=beehive".split_whitespace();
        let args = Arguments::parse_from(args);
        let mut app = Life::try_from(&args).expect("valid life");
        app.run();
        assert!(true);
    }

    #[test]
    fn will_run_to_completion_within_viewport() {
        let args = "app --pattern=beehive --viewport=-1..10,-1..10".split_whitespace();
        let args = Arguments::parse_from(args);
        let mut app = Life::try_from(&args).expect("valid life");
        app.run();
        assert!(true);
    }

    #[test]
    fn will_run_to_completion_within_world_bounds() {
        let args = "app --pattern=beehive --bounds=-1..10,-1..10".split_whitespace();
        let args = Arguments::parse_from(args);
        let mut app = Life::try_from(&args).expect("valid life");
        app.run();
        assert!(true);
    }
}
