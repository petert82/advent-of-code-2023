use anyhow::Result;
use clap::Parser;

mod algorithm;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod parse;
mod point;
mod puzzle;

#[derive(Parser)]
struct Cli {
    /// e.g. '1-2' to run the solution for day 1, part 2
    #[arg(value_parser = str_to_puzzle)]
    puzzle: puzzle::Puzzle,
}

fn str_to_puzzle(s: &str) -> Result<puzzle::Puzzle, String> {
    puzzle::Puzzle::try_from(s)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("Running {}", cli.puzzle);

    let (result, duration) = cli.puzzle.run()?;
    println!("{} ({:?})", result, duration);

    Ok(())
}
