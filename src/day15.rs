use anyhow::Result;
use nom::{
    bytes::complete::is_not, character::complete::char, combinator::map, multi::separated_list1,
    IResult,
};

use crate::parse::parse_all_to;

pub fn part1(input: &str) -> Result<usize> {
    let steps = parse_all_to(input, parse_initialization_sequence)?;
    let res = steps.iter().map(Step::hash).sum();
    Ok(res)
}

#[derive(Debug)]
struct Step {
    value: String,
}

impl Step {
    pub fn hash(&self) -> usize {
        self.value.chars().fold(0, |acc, c| {
            let val = c as usize;
            ((acc + val) * 17) % 256
        })
    }
}

fn parse_initialization_sequence(input: &str) -> IResult<&str, Vec<Step>> {
    let step = map(is_not(","), |s| Step {
        value: String::from(s),
    });
    separated_list1(char(','), step)(input)
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
