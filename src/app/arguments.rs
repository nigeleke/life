use clap::*;
use std::{ops::RangeInclusive, path::PathBuf};

use crate::prelude::{Bounds, Pattern};

fn parse_range(s: &str) -> Result<RangeInclusive<isize>, String> {
    let min_max = s.split("..").collect::<Vec<_>>();
    if min_max.len() != 2 {
        return Err("bounds format rowmin..rowmax,colmin..colmax".into());
    }
    let min = min_max[0].parse::<isize>().map_err(|e| e.to_string())?;
    let max = min_max[1].parse::<isize>().map_err(|e| e.to_string())?;
    Ok(min..=max)
}

fn parse_bounds(s: &str) -> Result<Bounds, String> {
    let ranges = s.split(",").collect::<Vec<_>>();
    if ranges.len() != 2 {
        return Err("bounds format rowmin..rowmax,colmin..colmax".into());
    };

    let rows = parse_range(ranges[0])?;
    let columns = parse_range(ranges[1])?;
    Ok(Bounds::Defined(rows, columns))
}

#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
pub struct Arguments {
    #[command(flatten)]
    source: Source,

    /// The initial viewport "minRow..maxRow,minColumn..maxColumn" (default is all)
    #[arg(long, value_parser = parse_bounds)]
    viewport: Option<Bounds>,

    /// The initial world boundary "minRow..maxRow,minColumn..maxColumn" (default is unbounded)
    #[arg(long, value_parser = parse_bounds)]
    bounds: Option<Bounds>,
}

#[derive(Clone, Debug, Args)]
#[group(required = false, multiple = false)]
struct Source {
    /// Path to a world file
    #[arg(long)]
    world: Option<PathBuf>,

    /// Name of a predefined pattern
    #[arg(long)]
    pattern: Option<Pattern>,
}

impl Arguments {
    pub fn world(&self) -> Option<&PathBuf> {
        self.source.world.as_ref()
    }

    pub fn pattern(&self) -> Option<&Pattern> {
        self.source.pattern.as_ref()
    }

    pub fn viewport(&self) -> Option<&Bounds> {
        self.viewport.as_ref()
    }

    pub fn bounds(&self) -> Option<&Bounds> {
        self.bounds.as_ref()
    }
}
