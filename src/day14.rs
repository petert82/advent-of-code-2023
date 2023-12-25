use crate::parse::parse_all_to;
use anyhow::Result;
use nom::character::complete::{line_ending, one_of};
use nom::combinator::{map, opt};
use nom::multi::{many1, separated_list1};
use nom::sequence::terminated;
use nom::IResult;
use std::fmt::{Display, Formatter};

pub fn part1(input: &str) -> Result<usize> {
    let platform = parse_all_to(input, parse_platform)?;
    println!("{}", platform);
    Ok(1)
}

struct Platform {
    w: usize,
    h: usize,
    rocks: Vec<Vec<Option<Rock>>>,
}

#[derive(Debug, PartialEq, Eq)]
enum Rock {
    Round,
    Square,
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.rocks.iter().enumerate() {
            for (x, maybe_rock) in row.iter().enumerate() {
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
