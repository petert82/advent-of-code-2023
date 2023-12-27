use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::{alpha1, char},
    combinator::{map, map_parser},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

use crate::parse::{number, parse_all_to};

pub fn part1(input: &str) -> Result<usize> {
    let steps = parse_all_to(input, parse_initialization_sequence)?;
    let res = steps.iter().map(Step::hash).sum();
    Ok(res)
}

#[derive(Debug)]
struct Step {
    hash: usize,
    instruction: Instruction,
}

#[derive(Debug)]
enum Instruction {
    Insert { label: String, focal_length: usize },
    Remove { label: String },
}

impl Step {
    pub fn hash(&self) -> usize {
        self.hash
    }
}

fn hash(input: &str) -> usize {
    input.chars().fold(0, |acc, c| {
        let val = c as usize;
        ((acc + val) * 17) % 256
    })
}

fn parse_initialization_sequence(input: &str) -> IResult<&str, Vec<Step>> {
    let step = map_parser(is_not(","), parse_step);
    separated_list1(char(','), step)(input)
}

fn parse_step(input: &str) -> IResult<&str, Step> {
    let hash = hash(input);
    map(parse_instruction, move |instruction| Step {
        hash,
        instruction,
    })(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    // e.g. "rn=1"
    let insert = map(
        separated_pair(alpha1, char('='), number),
        |(label, focal_length)| Instruction::Insert {
            label: String::from(label),
            focal_length,
        },
    );
    // e.g "cm-"
    let remove = map(terminated(alpha1, char('-')), |label| Instruction::Remove {
        label: String::from(label),
    });
    alt((insert, remove))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, 1320);
    }

    // #[test]
    // fn test_part2_gives_correct_answer() {
    //     let res = part2(INPUT).unwrap();
    //     assert_eq!(res, 64);
    // }
}
