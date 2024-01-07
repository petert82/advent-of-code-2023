use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, one_of},
    combinator::{map, map_res},
    sequence::{delimited, terminated, tuple},
    IResult,
};

use crate::{
    algorithm::shoelace,
    parse::{number, parse_lines_to_vec},
    point::Point,
};

pub fn part1(input: &str) -> Result<usize> {
    let instructions = parse_lines_to_vec(input, parse_part1_instruction)?;
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
struct Coord(i32, i32);

fn parse_part1_instruction(input: &str) -> IResult<&str, Instruction> {
    let parse_direction = map_res(one_of("UDLR"), Direction::try_from);
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

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            value => Err(format!("invalid direction: {}", value)),
        }
    }
}

impl Point<i32> for Coord {
    fn x(&self) -> i32 {
        self.0
    }

    fn y(&self) -> i32 {
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

    // #[test]
    // fn test_part2_gives_correct_answer() {
    //     let res = part2(INPUT).unwrap();
    //     assert_eq!(res, 102);
    // }
}
