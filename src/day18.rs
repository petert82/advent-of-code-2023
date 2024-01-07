use anyhow::Result;
use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{alphanumeric1, char, one_of},
    combinator::{map, map_res},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

use crate::{
    algorithm::shoelace,
    parse::{number, parse_lines_to_vec},
    point::Point,
};

pub fn part1(input: &str) -> Result<usize> {
    solve(input, ParseMode::Part1)
}

pub fn part2(input: &str) -> Result<usize> {
    // I'm sure there's a more optimal way to solve this,
    // but brute force works in a second or so, so... ¯\_(ツ)_/¯
    solve(input, ParseMode::Part2)
}

fn solve(input: &str, parse_mode: ParseMode) -> Result<usize> {
    let parse_fn = match parse_mode {
        ParseMode::Part1 => parse_part1_instruction,
        ParseMode::Part2 => parse_part2_instruction,
    };
    let instructions = parse_lines_to_vec(input, parse_fn)?;
    let coords = apply_instructions(&instructions);
    let enclosed_point_count = shoelace::enclosed_area(&coords);
    Ok(coords.len() + enclosed_point_count as usize - 1)
}

/// Starting at (0,0) applies the given `instructions` to find all of the trench coordinates.
/// (0,0) should end up in the returned list twice (at the start and the end)
fn apply_instructions(instructions: &[Instruction]) -> Vec<Coord> {
    let mut pos = Coord(0, 0);
    vec![pos]
        .into_iter()
        .chain(instructions.iter().flat_map(|instruction| {
            (0..instruction.length)
                .map(|_| {
                    pos = instruction.apply_to(pos);
                    pos
                })
                .collect::<Vec<_>>()
        }))
        .collect()
}

impl Instruction {
    fn apply_to(&self, coord: Coord) -> Coord {
        let Coord(x, y) = coord;
        match self.dir {
            Direction::Up => Coord(x, y - 1),
            Direction::Down => Coord(x, y + 1),
            Direction::Left => Coord(x - 1, y),
            Direction::Right => Coord(x + 1, y),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    dir: Direction,
    length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord(i64, i64);

enum ParseMode {
    Part1,
    Part2,
}

fn parse_part1_instruction(input: &str) -> IResult<&str, Instruction> {
    let parse_direction = map_res(one_of("UDLR"), |c| {
        Direction::try_from((ParseMode::Part1, c))
    });
    let parse_colour = delimited(tag("(#"), alphanumeric1, char(')'));

    map(
        tuple((
            terminated(parse_direction, char(' ')),
            terminated(number, char(' ')),
            parse_colour,
        )),
        |(dir, length, _colour)| Instruction { dir, length },
    )(input)
}

fn parse_part2_instruction(input: &str) -> IResult<&str, Instruction> {
    // L 10 (#3e6430)
    // Parsers for the "L 10 (#" that we need to ignore
    let ignore_dir = one_of("UDLR");
    let ignore_num = terminated(number, tag(" (#"));
    let line_start = separated_pair(ignore_dir, char(' '), ignore_num);

    // The first five hexadecimal digits encode the distance in meters as a five-digit
    // hexadecimal number. The last hexadecimal digit encodes the direction to dig.
    let parse_length = map_res(take_while_m_n(5, 5, is_hex_digit), from_hex);
    let parse_direction = map_res(one_of("0123"), |c| {
        Direction::try_from((ParseMode::Part2, c))
    });

    map(
        tuple((
            line_start,
            parse_length,
            terminated(parse_direction, char(')')),
        )),
        |(_ignore, length, dir)| Instruction { dir, length },
    )(input)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn from_hex(input: &str) -> Result<usize, std::num::ParseIntError> {
    usize::from_str_radix(input, 16)
}

impl TryFrom<(ParseMode, char)> for Direction {
    type Error = String;

    fn try_from(value: (ParseMode, char)) -> std::result::Result<Self, Self::Error> {
        let (parse_mode, value) = value;
        match parse_mode {
            ParseMode::Part1 => match value {
                'U' => Ok(Self::Up),
                'D' => Ok(Self::Down),
                'L' => Ok(Self::Left),
                'R' => Ok(Self::Right),
                value => Err(format!("invalid direction: {}", value)),
            },
            // 0 means R, 1 means D, 2 means L, and 3 means U
            ParseMode::Part2 => match value {
                '0' => Ok(Self::Right),
                '1' => Ok(Self::Down),
                '2' => Ok(Self::Left),
                '3' => Ok(Self::Up),
                value => Err(format!("invalid direction: {}", value)),
            },
        }
    }
}

impl Point<i64> for Coord {
    fn x(&self) -> i64 {
        self.0
    }

    fn y(&self) -> i64 {
        self.1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, 62);
    }

    #[test]
    fn test_part2_gives_correct_answer() {
        let res = part2(INPUT).unwrap();
        assert_eq!(res, 952408144115);
    }
}
