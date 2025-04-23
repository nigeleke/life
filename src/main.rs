#![feature(coverage_attribute)]

use clap::Parser;
use life::prelude::*;

#[coverage(off)]
fn main() -> Result<(), String> {
    let args = Arguments::parse();
    let mut life = Life::try_from(&args).map_err(|e| e.to_string())?;
    life.run();
    Ok(())
}
