mod cell {
    use life::prelude::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_die_when_fewer_than_two_live_neighbours_as_if_by_under_population() {
        let cells = Cells::try_from(
            r#"
    . . .
    . x .
    . . .
    "#,
        )
        .expect("valid cells");
        let mut world = World::from(cells);
        world.next_generation();
        assert!(world.is_empty())
    }

    #[test]
    fn should_die_when_more_than_three_live_neighbours_as_if_by_over_population() {
        let initial_cells = Cells::try_from(
            r#"
    x . . . x
    x x . x x
    . . x . .
    x x . x x
    x . . . x
    "#,
        )
        .expect("valid cells");
        let expected_cells = Cells::try_from(
            r#"
    x x . x x
    x x x x x
    . . . . .
    x x x x x
    x x . x x
    "#,
        )
        .expect("valid cells");
        let mut world = World::from(initial_cells);
        world.next_generation();
        assert_eq!(world, World::from(expected_cells));
    }

    #[test]
    fn should_survive_when_two_or_three_live_neighbours() {
        let initial_cells = Cells::try_from(
            r#"
    x x . . . .
    x x . . . .
    . . . . x .
    . . . . x .
    . . . . x .
    "#,
        )
        .expect("valid cells");
        let expected_cells = Cells::try_from(
            r#"
    x x . . . .
    x x . . . .
    . . . . . .
    . . . x x x
    . . . . . .
    "#,
        )
        .expect("valid cells");
        let mut world = World::from(initial_cells);
        world.next_generation();
        assert_eq!(world, World::from(expected_cells));
    }

    #[test]
    fn be_born_when_three_live_neighbours_as_if_by_reproduction() {
        let initial_cells = Cells::try_from(
            r#"
    . x .
    . x .
    . x .
    "#,
        )
        .expect("valid cells");
        let expected_cells = Cells::try_from(
            r#"
    . . .
    x x x
    . . .
    "#,
        )
        .expect("valid cells");
        let mut world = World::from(initial_cells);
        world.next_generation();
        assert_eq!(world, World::from(expected_cells));
    }
}
