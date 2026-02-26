mod patterns {
    use life::prelude::{Cells, Pattern, World};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_provide_beacon() {
        let world = World::try_from(&Pattern::Beacon).expect("valid pattern");
        let expected = Cells::try_from(
            r#"
* * . .
* . . *
. . * *
"#,
        )
        .expect("valid cells");
        assert_eq!(world.live_cells(), &expected)
    }

    #[test]
    fn should_provide_beehive() {
        let world = World::try_from(&Pattern::Beehive).expect("valid pattern");
        let expected = Cells::try_from(
            r#"
. * * .
* . . *
. * * .
"#,
        )
        .expect("valid cells");
        assert_eq!(world.live_cells(), &expected)
    }

    #[test]
    fn should_provide_blinker() {
        let world = World::try_from(&Pattern::Blinker).expect("valid pattern");
        let expected = Cells::try_from("* * *").expect("valid cells");
        assert_eq!(world.live_cells(), &expected)
    }

    #[test]
    fn should_provide_block() {
        let world = World::try_from(&Pattern::Block).expect("valid pattern");
        let expected = Cells::try_from(
            r#"
* *
* *
"#,
        )
        .expect("valid cells");
        assert_eq!(world.live_cells(), &expected)
    }

    #[test]
    fn should_provide_boat() {
        let world = World::try_from(&Pattern::Boat).expect("valid pattern");
        let expected = Cells::try_from(
            r#"
* * .
* . *
. * .
"#,
        )
        .expect("valid cells");
        assert_eq!(world.live_cells(), &expected)
    }

    #[test]
    fn should_provide_glider() {
        let world = World::try_from(&Pattern::Glider).expect("valid pattern");
        let expected = Cells::try_from(
            r#"
* * *
. . *
. * .
"#,
        )
        .expect("valid cells");
        assert_eq!(world.live_cells(), &expected)
    }

    #[test]
    fn should_provide_gospar_glider_gun() {
        let world = World::try_from(&Pattern::GosperGliderGun).expect("valid pattern");
        let expected = Cells::try_from(
            r#"
. . . . . . . . . . . . . . . . . . . . . . . . * . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . . . . . * . * . . . . . . . . . . .
. . . . . . . . . . . . * * . . . . . . * * . . . . . . . . . . . . * *
. . . . . . . . . . . * . . . * . . . . * * . . . . . . . . . . . . * *
* * . . . . . . . . * . . . . . * . . . * * . . . . . . . . . . . . . .
* * . . . . . . . . * . . . * . * * . . . . * . * . . . . . . . . . . .
. . . . . . . . . . * . . . . . * . . . . . . . * . . . . . . . . . . .
. . . . . . . . . . . * . . . * . . . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . * * . . . . . . . . . . . . . . . . . . . . . .
"#,
        )
        .expect("valid cells");
        assert_eq!(world.live_cells(), &expected)
    }

    #[test]
    fn should_provide_loaf() {
        let world = World::try_from(&Pattern::Loaf).expect("valid pattern");
        let expected = Cells::try_from(
            r#"
. * * .
* . . *
. * . *
. . * .
"#,
        )
        .expect("valid cells");
        assert_eq!(world.live_cells(), &expected)
    }

    #[test]
    fn should_provide_penta_decathlon() {
        let world = World::try_from(&Pattern::PentaDecathlon).expect("valid pattern");
        let expected = Cells::try_from(
            r#"
* * *
* . *
* * *
* * *
* * *
* . *
* * *
"#,
        )
        .expect("valid cells");
        assert_eq!(world.live_cells(), &expected)
    }

    #[test]
    fn should_provide_penta_pulsar() {
        let world = World::try_from(&Pattern::Pulsar).expect("valid pattern");
        let expected = Cells::try_from(
            r#"
. . * * * . . . * * * . .
. . . . . . . . . . . . .
* . . . . * . * . . . . *
* . . . . * . * . . . . *
* . . . . * . * . . . . *
. . * * * . . . * * * . .
. . . . . . . . . . . . .
. . * * * . . . * * * . .
* . . . . * . * . . . . *
* . . . . * . * . . . . *
* . . . . * . * . . . . *
. . . . . . . . . . . . .
. . * * * . . . * * * . .
"#,
        )
        .expect("valid cells");
        assert_eq!(world.live_cells(), &expected)
    }

    #[test]
    fn should_provide_spaceship_lightweight() {
        let world = World::try_from(&Pattern::SpaceshipLightweight).expect("valid pattern");
        let expected = Cells::try_from(
            r#"
* . . * .
. . . . *
* . . . *
. * * * *
"#,
        )
        .expect("valid cells");
        assert_eq!(world.live_cells(), &expected)
    }

    #[test]
    fn should_provide_toad() {
        let world = World::try_from(&Pattern::Toad).expect("valid pattern");
        let expected = Cells::try_from(
            r#"
. * * *
* * * .
"#,
        )
        .expect("valid cells");
        assert_eq!(world.live_cells(), &expected)
    }

    #[test]
    fn should_provide_tub() {
        let world = World::try_from(&Pattern::Tub).expect("valid pattern");
        let expected = Cells::try_from(
            r#"
. * .
* . *
. * .
"#,
        )
        .expect("valid cells");
        assert_eq!(world.live_cells(), &expected)
    }
}
