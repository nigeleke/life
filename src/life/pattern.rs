use clap::ValueEnum;

#[derive(Clone, Debug, ValueEnum)]
#[clap(rename_all = "snake_case")]
pub enum Pattern {
    Random,
    Beacon,
    Beehive,
    Blinker,
}

impl Pattern {
    pub fn name(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}
