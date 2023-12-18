use std::collections::HashMap;

use anyhow::{bail, Result};
use nom::{
    character::complete::{char, one_of},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use rayon::prelude::*;

use crate::parse::{number, parse_lines_to_vec};

pub fn part1(input: &str) -> Result<usize> {
    let rows = parse_lines_to_vec(input, parse_row)?;
    let res = rows
        .par_iter()
        .map(|r| r.possible_arrangement_count())
        .sum();
    Ok(res)
}

pub fn part2(input: &str) -> Result<usize> {
    let rows = parse_lines_to_vec(input, parse_row)?;
    let res = rows
        .par_iter()
        .map(|r| r.multiply(5).possible_arrangement_count())
        .sum();
    Ok(res)
}

#[derive(Debug)]
struct Row {
    springs: Vec<Spring>,
    group_sizes: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Row {
    /// Makes a new Row with `factor` copies of `springs` and `group_sizes`.
    pub fn multiply(&self, factor: usize) -> Self {
        // Make `factor` copies of `springs` separated by '?' (Unknown condition Spring)
        let springs = itertools::Itertools::intersperse(
            (0..factor)
                .fold(vec![], |mut acc, _| {
                    acc.push(self.springs.clone());
                    acc
                })
                .into_iter(),
            vec![Spring::Unknown],
        )
        .flatten()
        .collect();
        // Make `factor` copies of the `group_sizes` list
        let group_sizes = (0..factor).fold(vec![], |mut acc, _| {
            acc.extend(self.group_sizes.iter().copied());
            acc
        });
        Self {
            springs,
            group_sizes,
        }
    }

    pub fn possible_arrangement_count(&self) -> usize {
        let mut cache = HashMap::new();
        self.count_arrangements(0, 0, &mut cache)
    }

    fn count_arrangements(
        &self,
        spring_idx: usize,
        group_idx: usize,
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(cached) = cache.get(&(spring_idx, group_idx)) {
            return *cached;
        }

        // check if the current group can be satisfied
        let groups_result = self.group_sizes.get(group_idx).map_or(0, |group_size| {
            // check we have enough springs left for the group
            if (spring_idx + group_size) > self.springs.len() {
                return 0;
            }
            // group must only contain damaged springs
            if (0..*group_size)
                .any(|i| self.springs.get(spring_idx + i) == Some(&Spring::Operational))
            {
                return 0;
            }
            // item after group can't be a Damaged spring (else we're still in a group)
            if self.springs.get(spring_idx + group_size) == Some(&Spring::Damaged) {
                return 0;
            }
            // we have a possibly valid group
            self.count_arrangements(spring_idx + group_size + 1, group_idx + 1, cache)
        });

        let spring_result = match self.springs.get(spring_idx) {
            // if we got through all of the springs/groups, we found a valid arrangement
            None => usize::from(group_idx >= self.group_sizes.len()),
            Some(Spring::Damaged) => 0,
            Some(_) => self.count_arrangements(spring_idx + 1, group_idx, cache),
        };

        cache.insert((spring_idx, group_idx), groups_result + spring_result);
        groups_result + spring_result
    }
}

impl TryFrom<char> for Spring {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            _ => bail!("Cannot create a Spring from '{}'", value),
        }
    }
}

fn parse_row(input: &str) -> IResult<&str, Row> {
    let spring = map_res(one_of(".#?"), Spring::try_from);
    let parse_springs = many1(spring);
    let parse_group_sizes = separated_list1(char(','), number);
    map(
        separated_pair(parse_springs, char(' '), parse_group_sizes),
        |(springs, group_sizes)| Row {
            springs,
            group_sizes,
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, 21);
    }

    #[test]
    fn test_part2_gives_correct_answer() {
        let res = part2(INPUT).unwrap();
        assert_eq!(res, 525152);
    }
}
