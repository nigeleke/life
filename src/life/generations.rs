use std::collections::HashSet;

use super::{cells::Cells, world::World};

#[derive(Debug)]
pub struct Generations {
    current: World,
    previous: HashSet<Cells>,
}

impl Generations {
    pub fn new(world: World) -> Self {
        Self {
            current: world,
            previous: HashSet::default(),
        }
    }

    pub fn current(&self) -> &World {
        &self.current
    }
}

impl std::iter::Iterator for Generations {
    type Item = World;

    fn next(&mut self) -> Option<Self::Item> {
        let mut world = self.current.clone();
        world.next_generation();

        let cells = world.live_cells().clone();
        let inserted = self.previous.insert(cells);
        let world = inserted.then_some(world);

        if let Some(world) = world.as_ref() {
            self.current = world.clone();
        }

        world
    }
}
