mod generations {
    use life::prelude::{Cell, Cells, Generations, Pattern, World};
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn should_be_an_iterator() {
        let world = World::try_from(&Pattern::Block).expect("valid pattern");
        let mut generations = Generations::new(world.clone());
        let Some(new_world) = generations.next() else {
            panic!("invalid state")
        };
        assert_eq!(new_world, world);
    }

    #[test]
    fn should_iterate_to_the_next_generation() {
        let world = World::try_from(&Pattern::Toad).expect("valid pattern");
        let mut generations = Generations::new(world.clone());
        let Some(new_world) = generations.next() else {
            panic!("invalid state")
        };
        assert_ne!(new_world, world);
    }

    #[test]
    fn should_not_have_a_next_available_after_iterating_back_to_a_former_world() {
        let world = World::try_from(&Pattern::Toad).expect("valid pattern");
        let mut generations = Generations::new(world.clone());

        let Some(_) = generations.next() else {
            panic!("invalid state")
        };

        let Some(_) = generations.next() else {
            panic!("invalid state")
        };

        assert_eq!(generations.next(), None);
    }

    #[test]
    fn should_stop_when_the_world_is_dead() {
        let world = World::from(Cells::from_iter([Cell::new(0, 0)]));
        let mut generations = Generations::new(world);
        let Some(_) = generations.next() else {
            panic!("invalid state")
        };
        assert_eq!(generations.next(), None);
    }
}
