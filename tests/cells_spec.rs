mod cells {
    use std::path::Path;

    use life::prelude::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_be_creatable_when_empty() {
        let cells = Cells::try_from(".").expect("valid cells");
        assert!(cells.is_empty());
    }

    #[test]
    fn should_be_creatable_when_not_empty() {
        let cells = Cells::try_from("x").expect("valid cells");
        assert!(cells.contains(&Cell::new(0, 0)));
    }

    #[test]
    fn may_contain_many_rows() {
        let cells = Cells::try_from(
            r#"
. . .
. x .
. . .
"#,
        )
        .expect("valid cells");
        assert!(cells.contains(&Cell::new(1, 1)));
    }

    #[test]
    fn will_ignore_empty_rows() {
        let cells = Cells::try_from(
            r#"
. . .

. x .

. . .
"#,
        )
        .expect("valid cells");
        assert!(cells.contains(&Cell::new(1, 1)));
    }

    #[test]
    fn will_be_readable_from_a_file() {
        let path = Path::new("./tests/data/initial_world.life");
        let cells = Cells::try_from(path).expect("valid cells");
        assert!(cells.contains(&Cell::new(0, 0)));
        assert!(cells.contains(&Cell::new(0, 1)));
        assert!(cells.contains(&Cell::new(1, 0)));
        assert!(cells.contains(&Cell::new(1, 1)));
    }

    #[test]
    fn will_be_creatable_from_empty() {
        let cells = Cells::try_from("").expect("valid cells");
        assert!(cells.is_empty());
    }

    #[test]
    fn will_not_be_creatable_from_rows_of_different_lengths() {
        let error = Cells::try_from(
            r#"
            |. . . . .
            |. . .
            |. . . . ."#,
        )
        .expect_err("invalid cells");
        assert!(matches!(error, CellsError::InconsistentLineLengths));
    }

    #[test]
    fn will_not_be_creatable_from_missing_file() {
        let path = Path::new("./tests/data/no_world_file.life");
        let error = Cells::try_from(path).expect_err("invalid cells");
        assert!(matches!(error, CellsError::FileError(_)));
    }

    #[test]
    fn will_not_be_creatable_from_invalid_file() {
        let path = Path::new("./tests/data/invalid_world.life");
        let error = Cells::try_from(path).expect_err("invalid cells");
        assert!(matches!(error, CellsError::InconsistentLineLengths));
    }

    #[test]
    fn rotate_0_degrees() {
        let initial_cells = Cells::try_from(
            r#"
* * * *
. . * *
. . . *
"#,
        )
        .expect("valid cells");
        let expected_cells = initial_cells.clone();
        let actual_cells = initial_cells.rotate(0);
        assert_eq!(actual_cells, expected_cells);
    }

    #[test]
    fn rotate_90_degrees() {
        let initial_cells = Cells::try_from(
            r#"
* * * *
. . * *
. . . *
"#,
        )
        .expect("valid cells");
        let expected_cells = Cells::try_from(
            r#"
. . *
. . *
. * *
* * *
"#,
        )
        .expect("valid cells");

        let actual_cells = initial_cells.rotate(1);
        assert_eq!(actual_cells, expected_cells);
    }

    #[test]
    fn rotate_180_degrees() {
        let initial_cells = Cells::try_from(
            r#"
* * * *
. . * *
. . . *
"#,
        )
        .expect("valid cells");
        let expected_cells = Cells::try_from(
            r#"
* . . .
* * . .
* * * *
"#,
        )
        .expect("valid cells");

        let actual_cells = initial_cells.rotate(2);
        assert_eq!(actual_cells, expected_cells);
    }

    #[test]
    fn rotate_270_degrees() {
        let initial_cells = Cells::try_from(
            r#"
* * * *
. . * *
. . . *
"#,
        )
        .expect("valid cells");
        let expected_cells = Cells::try_from(
            r#"
* * *
* * .
* . .
* . .
"#,
        )
        .expect("valid cells");

        let actual_cells = initial_cells.rotate(3);
        assert_eq!(actual_cells, expected_cells);
    }

    #[test]
    fn rotate_360_degrees() {
        let initial_cells = Cells::try_from(
            r#"
* * * *
. . * *
. . . *
"#,
        )
        .expect("valid cells");
        let expected_cells = initial_cells.clone();

        let actual_cells = initial_cells.rotate(4);
        assert_eq!(actual_cells, expected_cells);
    }
}
