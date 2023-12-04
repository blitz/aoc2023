mod day1;
mod day2;
mod day3;
mod day4;

use anyhow::{bail, Result};
use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Selects the solution to run.
    day: Option<u8>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let solutions: Vec<fn() -> Result<()>> =
        vec![day1::solve, day2::solve, day3::solve, day4::solve];

    println!("🎄 Advent of Code 2023 🎄");

    if let Some(day) = args.day {
        if let Some(f) = solutions.get(usize::from(day - 1)) {
            println!("🎅 Running Day {} ...", day.to_string().bold());
            f()
        } else {
            bail!("Invalid day or no solution yet!")
        }
    } else {
        for (day, f) in solutions.into_iter().enumerate() {
            println!("🎅 Running Day {} ...", (day + 1).to_string().bold());
            f()?;
        }

        Ok(())
    }
}
