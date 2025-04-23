use clap::ValueEnum;

#[derive(Clone, Debug, PartialEq, ValueEnum)]
#[clap(rename_all = "snake_case")]
pub enum Pattern {
    Beacon,
    Beehive,
    Blinker,
    Block,
    Boat,
    Glider,
    GosperGliderGun,
    Loaf,
    PentaDecathlon,
    Pulsar,
    SpaceshipLightweight,
    Toad,
    Tub,
}

impl Pattern {
    pub fn cells_str(&self) -> &str {
        match self {
            Pattern::Beacon => include_str!("../../assets/patterns/beacon.life"),
            Pattern::Beehive => include_str!("../../assets/patterns/beehive.life"),
            Pattern::Blinker => include_str!("../../assets/patterns/blinker.life"),
            Pattern::Block => include_str!("../../assets/patterns/block.life"),
            Pattern::Boat => include_str!("../../assets/patterns/boat.life"),
            Pattern::Glider => include_str!("../../assets/patterns/glider.life"),
            Pattern::GosperGliderGun => {
                include_str!("../../assets/patterns/gosper_glider_gun.life")
            }
            Pattern::Loaf => include_str!("../../assets/patterns/loaf.life"),
            Pattern::PentaDecathlon => include_str!("../../assets/patterns/penta_decathlon.life"),
            Pattern::Pulsar => include_str!("../../assets/patterns/pulsar.life"),
            Pattern::SpaceshipLightweight => {
                include_str!("../../assets/patterns/spaceship_lightweight.life")
            }
            Pattern::Toad => include_str!("../../assets/patterns/toad.life"),
            Pattern::Tub => include_str!("../../assets/patterns/tub.life"),
        }
    }
}
