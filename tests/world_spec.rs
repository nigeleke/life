mod world {
    use life::prelude::{Bounds, Cell, Cells, Position, World};
    use pretty_assertions::assert_eq;

    #[test]
    fn will_allow_an_initial_set_of_cells_to_be_provided() {
        let mut cells = Cells::default();
        cells.add_cell(Cell::new(0, 0));

        let world = World::from(cells);
        assert!(world.live_cells().contains(&Cell::new(0, 0)));
    }

    #[test]
    fn will_allow_world_limits_to_be_defined_when_all_enclosing() {
        let bounds = Bounds::new(0..=10, 0..=10);
        let cells = Cells::try_from(
            r#"
* * *
. . *
. * .
"#,
        )
        .expect("valid cells");
        let mut world = World::from(cells);
        world.with_bounds(&bounds);
        assert_eq!(world.bounds(), &bounds);
    }

    #[test]
    fn will_allow_world_limits_to_be_defined_when_restrictive() {
        let bounds = Bounds::new(0..=1, 0..=1);
        let initial_cells = Cells::try_from(
            r#"
* * *
. . *
. * .
"#,
        )
        .expect("valid cells");
        let expected_cells = Cells::try_from(
            r#"
* *
. .
"#,
        )
        .expect("valid cells");
        let mut world = World::from(initial_cells);
        world.with_bounds(&bounds);
        assert_eq!(world.live_cells(), &expected_cells);
    }

    #[test]
    fn will_allow_world_limits_to_be_defined_when_previously_defined() {
        let initial_bounds = Bounds::new(0..=9, 0..=9);
        let new_bounds = Bounds::new(0..=4, 0..=4);
        let mut world = World::from(Cells::default());
        world.with_bounds(&initial_bounds);
        world.with_bounds(&new_bounds);
        assert_eq!(world.bounds(), &new_bounds);
    }

    #[test]
    fn will_allow_additional_cells_to_be_added_when_unbounded_world() {
        let initial_cells = Cells::try_from(
            r#"
* * *
. . *
. * .
"#,
        )
        .expect("valid cells");
        let additional_cells = Cells::try_from(
            r#"
* * *
* . .
. * .
"#,
        )
        .expect("valid cells");
        let expected_cells = Cells::try_from(
            r#"
* * * . . .
. . * * * *
. * . * . .
. . . . * .
"#,
        )
        .expect("valid cells");
        let mut world = World::from(initial_cells);
        world.add_cells(additional_cells, &Position::new(1, 3));
        assert_eq!(world.live_cells(), &expected_cells);
    }

    #[test]
    fn will_allow_additional_cells_to_be_added_when_bounded_world() {
        let bounds = Bounds::new(0..=2, 0..=4);
        let initial_cells = Cells::try_from(
            r#"
* * *
. . *
. * .
"#,
        )
        .expect("valid cells");
        let additional_cells = Cells::try_from(
            r#"
* * *
* . .
. * .
"#,
        )
        .expect("valid cells");
        let expected_cells = Cells::try_from(
            r#"
* * * . .
. . * * *
. * . * .
"#,
        )
        .expect("valid cells");

        let mut world = World::from(initial_cells);
        world.with_bounds(&bounds);
        world.add_cells(additional_cells, &Position::new(1, 3));
        assert_eq!(world.live_cells(), &expected_cells);
    }

    #[test]
    fn will_allow_a_viewport_to_be_defined() {
        let viewport = Bounds::new(0..=10, 0..=10);
        let mut world = World::from(Cells::default());
        world.with_viewport(&viewport);
        assert_eq!(world.viewport(), &viewport);
    }

    #[test]
    fn will_be_pretty_printable_when_no_viewport_defined() {
        let cells = Cells::try_from("* . * .").expect("valid cells");
        let world = World::from(cells);
        assert_eq!(world.to_string(), "*   *".to_string());
    }

    #[test]
    fn will_be_pretty_printable_when_viewport_defined() {
        let initial_cells = Cells::try_from(
            r#"
. . . . . .
* . . . . .
* * . . . .
. . * . . .
* . * . . .
* * * . . .
"#,
        )
        .expect("valid cells");
        let mut world = World::from(initial_cells);
        world.with_viewport(&Bounds::new(2..=3, 1..=3));

        let expected = String::from(
            r#"*....
..*.."#,
        )
        .replace(".", " ")
        .replace("\r", "");
        assert_eq!(world.to_string(), expected);
    }
}
