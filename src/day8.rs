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

    // #[test]
    // fn test_part2_gives_correct_answer() {
    //     let res = part2(INPUT).unwrap();
    //     assert_eq!(res, 5905);
    // }
}
