use anyhow::{bail, Context, Result};
use nom::{
    character::complete::line_ending,
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::terminated,
    Parser,
};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Puzzle {
    day: u16,
    part: u16,
}

impl Puzzle {
    pub fn run(&self) -> Result<Box<dyn Display>> {
        let input_file = format!("./input/day{}.txt", self.day);
        let input = std::fs::read_to_string(input_file).context("could not read input file")?;
        match (self.day, self.part) {
            (1, 1) => Ok(Box::new(crate::day1::part1(input.as_ref())?)),
            (1, 2) => Ok(Box::new(crate::day1::part2(input.as_ref())?)),
            (2, 1) => Ok(Box::new(crate::day2::part1(input.as_ref())?)),
            (2, 2) => Ok(Box::new(crate::day2::part2(input.as_ref())?)),
            (3, 1) => Ok(Box::new(crate::day3::part1(input.as_ref())?)),
            (3, 2) => Ok(Box::new(crate::day3::part2(input.as_ref())?)),
            (4, 1) => Ok(Box::new(crate::day4::part1(input.as_ref())?)),
            (4, 2) => Ok(Box::new(crate::day4::part2(input.as_ref())?)),
            (5, 1) => Ok(Box::new(crate::day5::part1(input.as_ref())?)),
            (5, 2) => Ok(Box::new(crate::day5::part2(input.as_ref())?)),
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

pub fn parse_all_to<'a, T>(
    input: &'a str,
    parser: impl Parser<&'a str, T, nom::error::Error<&'a str>>,
) -> Result<T> {
    let (_, res) = all_consuming(terminated(parser, opt(line_ending)))(input)
        .map_err(|e| e.to_owned())
        .context("failed to parse input")?;
    Ok(res)
}

pub fn parse_lines_to_vec<'a, T>(
    input: &'a str,
    parser: impl Parser<&'a str, T, nom::error::Error<&'a str>>,
) -> Result<Vec<T>> {
    let (_, res) = all_consuming(terminated(
        separated_list1(line_ending, parser),
        opt(line_ending),
    ))(input)
    .map_err(|e| e.to_owned())
    .context("failed to parse input")?;
    Ok(res)
}
