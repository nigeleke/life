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

    pub fn next_generation(&mut self) -> Option<&World> {
        self.current.next_generation();
        let is_unique = self.previous.insert(self.current.live_cells().clone());
        is_unique.then_some(&self.current)
    }
}
