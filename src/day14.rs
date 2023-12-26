use crate::parse::parse_all_to;
use anyhow::Result;
use nom::character::complete::{line_ending, one_of};
use nom::combinator::{map, opt};
use nom::multi::{many1, separated_list1};
use nom::sequence::terminated;
use nom::IResult;
use std::fmt::{Display, Formatter};

pub fn part1(input: &str) -> Result<usize> {
    let mut platform = parse_all_to(input, parse_platform)?;
    platform.slide_north();
    Ok(platform.calculate_load())
}

struct Platform {
    w: usize,
    h: usize,
    rocks: Vec<Vec<Option<Rock>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Round,
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpanState {
    Blocked,
    Clear,
}

impl Platform {
    pub fn slide_north(&mut self) {
        for x in 0..self.w {
            let mut span_state: Option<SpanState> = None;
            let mut last_rock_idx: Option<usize> = None;
            for y in 0..self.h {
                let cur_rock = self.rocks[y][x];
                match (cur_rock, span_state) {
                    (None, None) => {
                        span_state = Some(SpanState::Clear);
                    }
                    (Some(_), None) => {
                        last_rock_idx = Some(y);
                        span_state = Some(SpanState::Blocked);
                    }
                    (None, Some(SpanState::Clear)) => continue,
                    (Some(Rock::Round), Some(SpanState::Clear)) => {
                        let rock = self.rocks[y][x].take();
                        let rock_idx = last_rock_idx.map_or(0, |i| i + 1);
                        self.rocks[rock_idx][x] = rock;
                        span_state = Some(SpanState::Clear);
                        last_rock_idx = Some(rock_idx);
                    }
                    (Some(Rock::Square), Some(SpanState::Clear)) => {
                        last_rock_idx = Some(y);
                        span_state = Some(SpanState::Blocked);
                    }
                    (None, Some(SpanState::Blocked)) => {
                        span_state = Some(SpanState::Clear);
                    }
                    (Some(_), Some(SpanState::Blocked)) => {
                        last_rock_idx = Some(y);
                    }
                }
            }
        }
    }

    pub fn calculate_load(&self) -> usize {
        self.rocks
            .iter()
            .enumerate()
            .map(|(y, row)| {
                let load_per_rock = self.h - y;
                row.iter()
                    .filter_map(|rock| match rock {
                        Some(Rock::Round) => Some(load_per_rock),
                        _ => None,
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.rocks.iter() {
            for maybe_rock in row.iter() {
                if let Some(rock) = maybe_rock {
                    write!(f, "{}", rock)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Rock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Round => 'O',
                Self::Square => '#',
            }
        )
    }
}

fn parse_rock(input: &str) -> IResult<&str, Option<Rock>> {
    map(one_of("O#."), |c| match c {
        'O' => Some(Rock::Round),
        '#' => Some(Rock::Square),
        _ => None,
    })(input)
}

fn parse_platform(input: &str) -> IResult<&str, Platform> {
    let rock_row = many1(parse_rock);
    map(
        terminated(separated_list1(line_ending, rock_row), opt(line_ending)),
        |rocks| {
            let w = rocks[0].len();
            Platform {
                w,
                h: rocks.len(),
                rocks,
            }
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, 136);
    }

    // #[test]
    // fn test_part2_gives_correct_answer() {
    //     let res = part2(INPUT).unwrap();
    //     assert_eq!(res, 525152);
    // }
}
