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

pub fn part2(input: &str) -> Result<usize> {
    let steps = parse_all_to(input, parse_initialization_sequence)?;
    let state = steps.iter().fold(LensBoxes::new(), |mut state, step| {
        state.handle(&step.instruction);
        state
    });

    Ok(state.focusing_power())
}

#[derive(Debug)]
struct LensBoxes {
    boxes: [Vec<(String, usize)>; 256],
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

impl LensBoxes {
    pub fn new() -> Self {
        const DEFAULT: Vec<(String, usize)> = Vec::new();
        Self {
            boxes: [DEFAULT; 256],
        }
    }

    pub fn handle(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Insert {
                label,
                focal_length,
            } => self.insert(label, *focal_length),
            Instruction::Remove { label } => self.remove(label),
        }
    }

    pub fn focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(box_idx, b)| {
                b.iter()
                    .enumerate()
                    .map(|(lens_idx, (_, focal_length))| {
                        (box_idx + 1) * (lens_idx + 1) * focal_length
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn insert(&mut self, key: &str, focal_length: usize) {
        let box_nr = hash(key);
        if let Some(idx) = self.index_in_box(box_nr, key) {
            self.boxes[box_nr][idx] = (String::from(key), focal_length);
        } else {
            self.boxes[box_nr].push((String::from(key), focal_length));
        }
    }

    fn remove(&mut self, key: &str) {
        let box_nr = hash(key);
        if let Some(idx) = self.index_in_box(box_nr, key) {
            self.boxes[box_nr].remove(idx);
        };
    }

    fn index_in_box(&self, box_nr: usize, key: &str) -> Option<usize> {
        self.boxes[box_nr].iter().position(|(k, _)| k == key)
    }
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

    #[test]
    fn test_part2_gives_correct_answer() {
        let res = part2(INPUT).unwrap();
        assert_eq!(res, 145);
    }
}
