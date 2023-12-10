use std::collections::HashMap;

use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, line_ending, one_of},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

use crate::parse::parse_all_to;

pub fn part1(input: &str) -> Result<usize> {
    let state = parse_all_to(input, parse_state)?;
    Ok(state.steps_to_find("ZZZ"))
}

pub fn part2(input: &str) -> Result<usize> {
    let state = parse_all_to(input, parse_state)?;
    Ok(state.steps_for_part2())
}

#[derive(Debug)]
struct State<'a> {
    directions: Vec<Direction>,
    // map of node ID to (Left, Right) next node
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl<'a> State<'a> {
    pub fn steps_to_find(&self, target: &str) -> usize {
        let mut directions_cyle = self.directions.iter().cycle();
        let mut steps = 0;
        let mut key = "AAA";

        loop {
            if key == target {
                break;
            }
            let dir = directions_cyle.next().unwrap();
            let next = self.nodes.get(key).unwrap();
            key = match dir {
                Direction::Left => next.0,
                Direction::Right => next.1,
            };
            steps += 1;
        }
        steps
    }

    pub fn steps_for_part2(&self) -> usize {
        // Starting points for our "ghosts"
        let ghost_keys = self
            .nodes
            .keys()
            .filter(|k| k.ends_with("A"))
            .map(|k| *k)
            .collect::<Vec<&str>>();

        // Entries are how long each "ghost" took to find their first end point
        let mut end_steps = ghost_keys.into_iter().map(|k| {
            let mut directions_cyle = self.directions.iter().cycle();
            let mut steps = 0;
            let mut key = k;
            loop {
                if key.ends_with("Z") {
                    break;
                }
                let dir = directions_cyle.next().unwrap();
                let next = self.nodes.get(key).unwrap();
                key = match dir {
                    Direction::Left => next.0,
                    Direction::Right => next.1,
                };
                steps += 1;
            }
            steps
        });

        // Find the lowest common multiple of all of the ghosts' end points
        let n = end_steps.next().unwrap();
        let lcm = end_steps.fold(n, |n, m| lcm(n, m));

        lcm
    }
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(format!("cannot build Direction from '{}'", value)),
        }
    }
}

fn parse_state<'a>(input: &'a str) -> IResult<&str, State<'a>> {
    // Directions is a line of "LRLRLRLRLRLR"
    let parse_directions = terminated(
        many1(map_res(one_of("LR"), Direction::try_from)),
        line_ending,
    );

    let node_id = alpha1;
    // "AAA = (BBB, CCC)"
    let node = separated_pair(
        node_id,
        tag(" = "),
        delimited(
            char('('),
            separated_pair(node_id, tag(", "), node_id),
            char(')'),
        ),
    );

    let parse_nodes = separated_list1(line_ending, node);
    let (input, (directions, nodes)) =
        separated_pair(parse_directions, line_ending, parse_nodes)(input)?;
    Ok((
        input,
        State {
            directions,
            nodes: nodes.into_iter().collect(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, 6);
    }

    const INPUT2: &str = "LR

OOA = (OOB, XXX)
OOB = (XXX, OOZ)
OOZ = (OOB, XXX)
TTA = (TTB, XXX)
TTB = (TTC, TTC)
TTC = (TTZ, TTZ)
TTZ = (TTB, TTB)
XXX = (XXX, XXX)";

    #[test]
    fn test_part2_gives_correct_answer() {
        let res = part2(INPUT2).unwrap();
        assert_eq!(res, 6);
    }
}
