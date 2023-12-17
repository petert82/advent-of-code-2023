use std::{
    collections::{HashMap, VecDeque},
    sync::OnceLock,
};

use anyhow::Result;
use nom::{
    bytes::complete::is_a,
    character::complete::{char, digit1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use regex::Regex;

use crate::parse::parse_lines_to_vec;

pub fn part1(input: &str) -> Result<usize> {
    let rows = parse_lines_to_vec(input, parse_row)?;
    let res = rows
        .iter()
        .map(|r| r.get_possible_arrangements().len())
        .sum();
    Ok(res)
}

#[derive(Debug)]
struct Row {
    record: String,
    pattern: Regex,
}

impl Row {
    pub fn get_possible_arrangements(&self) -> Vec<String> {
        let mut templates = VecDeque::from([self.record.clone()]);
        let mut arrangements = Vec::new();
        while let Some(template) = templates.pop_front() {
            // Find a '?' and generate the two possible replacements
            if let Some(pos) = template.chars().position(|c| c == '?') {
                let mut new_template1 = template.clone();
                new_template1.replace_range(pos..pos + 1, ".");
                let mut new_template2 = template.clone();
                new_template2.replace_range(pos..pos + 1, "#");
                templates.push_back(new_template1);
                templates.push_back(new_template2);
            } else {
                // `template` contains no question marks, check if it's valid
                if self.pattern.is_match(template.as_str()) {
                    arrangements.push(template);
                }
            }
        }
        arrangements
    }
}

fn parse_pattern(input: &str) -> IResult<&str, Regex> {
    let (input, numbers) = separated_list1(char(','), digit1)(input)?;
    // We're changing "1,1,3" into a regex like:
    //   ^\.*[#]{1}\.+[#]{1}\.+[#]{3}(?:\.+$|$)?
    let re_parts = numbers
        .into_iter()
        .map(|n| format!("[#]{{{}}}", n))
        .collect::<Vec<_>>();
    let re_str = format!(r"^\.*{}(?:\.+$|$)", re_parts.join(r"\.+"));

    // todo: cache previously seen regexes and re-use
    Ok((input, Regex::new(re_str.as_str()).unwrap()))
}

fn parse_row(input: &str) -> IResult<&str, Row> {
    let record = map(is_a(".#?"), String::from);
    map(
        separated_pair(record, char(' '), parse_pattern),
        |(record, pattern)| Row { record, pattern },
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
}
