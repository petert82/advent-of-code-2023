use anyhow::{bail, Context, Result};
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

#[derive(Debug, Clone)]
pub struct Puzzle {
    day: u16,
    part: u16,
}

impl Puzzle {
    pub fn run(&self) -> Result<(Box<dyn Display>, Duration)> {
        let input_file = format!("./input/day{}.txt", self.day);
        let input = std::fs::read_to_string(input_file).context("could not read input file")?;
        let start = Instant::now();
        Ok((self.run_part(input.as_ref())?, start.elapsed()))
    }

    fn run_part(&self, input: &str) -> Result<Box<dyn Display>> {
        match (self.day, self.part) {
            (1, 1) => Ok(Box::new(crate::day1::part1(input)?)),
            (1, 2) => Ok(Box::new(crate::day1::part2(input)?)),
            (2, 1) => Ok(Box::new(crate::day2::part1(input)?)),
            (2, 2) => Ok(Box::new(crate::day2::part2(input)?)),
            (3, 1) => Ok(Box::new(crate::day3::part1(input)?)),
            (3, 2) => Ok(Box::new(crate::day3::part2(input)?)),
            (4, 1) => Ok(Box::new(crate::day4::part1(input)?)),
            (4, 2) => Ok(Box::new(crate::day4::part2(input)?)),
            (5, 1) => Ok(Box::new(crate::day5::part1(input)?)),
            (5, 2) => Ok(Box::new(crate::day5::part2(input)?)),
            (6, 1) => Ok(Box::new(crate::day6::part1(input)?)),
            (6, 2) => Ok(Box::new(crate::day6::part2(input)?)),
            (7, 1) => Ok(Box::new(crate::day7::part1(input)?)),
            (7, 2) => Ok(Box::new(crate::day7::part2(input)?)),
            (8, 1) => Ok(Box::new(crate::day8::part1(input)?)),
            (8, 2) => Ok(Box::new(crate::day8::part2(input)?)),
            (9, 1) => Ok(Box::new(crate::day9::part1(input)?)),
            (9, 2) => Ok(Box::new(crate::day9::part2(input)?)),
            (10, 1) => Ok(Box::new(crate::day10::part1(input)?)),
            (10, 2) => Ok(Box::new(crate::day10::part2(input)?)),
            (11, 1) => Ok(Box::new(crate::day11::part1(input)?)),
            (11, 2) => Ok(Box::new(crate::day11::part2(input)?)),
            (12, 1) => Ok(Box::new(crate::day12::part1(input)?)),
            (12, 2) => Ok(Box::new(crate::day12::part2(input)?)),
            (13, 1) => Ok(Box::new(crate::day13::part1(input)?)),
            (13, 2) => Ok(Box::new(crate::day13::part2(input)?)),
            (14, 1) => Ok(Box::new(crate::day14::part1(input)?)),
            (14, 2) => Ok(Box::new(crate::day14::part2(input)?)),
            (15, 1) => Ok(Box::new(crate::day15::part1(input)?)),
            // (15, 2) => Ok(Box::new(crate::day15::part2(input)?)),
            _ => bail!("day {} part {} is not implemented", self.day, self.part),
        }
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "day {} part {}", self.day, self.part)
    }
}

impl TryFrom<&str> for Puzzle {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts = s.split_once('-');
        if let Some((day, part)) = parts {
            let day = day.parse::<u16>();
            let part = part.parse::<u16>();
            if day.is_err() || part.is_err() {
                return Err("Day and part must be integers".into());
            }
            let day = day.unwrap();
            if !(1..=25).contains(&day) {
                return Err("Day must be between 1 and 25".into());
            }
            let part = part.unwrap();
            if !(1..=2).contains(&part) {
                return Err("Part must be either 1 or 2".into());
            }
            Ok(Self { day, part })
        } else {
            Err("puzzle must be given in the format {day}-{part}".to_string())
        }
    }
}
