use super::cells::Cells;
use super::world::World;

use std::collections::HashSet;

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
        let world = self.previous.insert(cells).then_some(world);

        if let Some(world) = world.as_ref() {
            self.current = world.clone();
        }

        world
    }
}
