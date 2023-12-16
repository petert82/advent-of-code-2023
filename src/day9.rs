use anyhow::Result;
use itertools::Itertools;
use nom::{
    character::complete::{char, digit1},
    combinator::{map_res, opt, recognize},
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};

use crate::parse::parse_lines_to_vec;

pub fn part1(input: &str) -> Result<i64> {
    solve(input, ExtrapolateDir::Forwards)
}

pub fn part2(input: &str) -> Result<i64> {
    solve(input, ExtrapolateDir::Backwards)
}

#[derive(Clone, Copy)]
enum ExtrapolateDir {
    Forwards,
    Backwards,
}

fn solve(input: &str, dir: ExtrapolateDir) -> Result<i64> {
    let rows = parse_lines_to_vec(input, parse_line)?;
    let res = rows.into_iter().map(|r| process_row(r, dir)).sum();
    Ok(res)
}

fn process_row(row: Vec<i64>, dir: ExtrapolateDir) -> i64 {
    let mut diff_rows = vec![row];
    let mut found_end = false;

    while !found_end {
        let row = get_diffs(diff_rows.last().unwrap());
        found_end = is_all_zeros(&row);
        diff_rows.push(row);
    }

    diff_rows
        .iter()
        .rev()
        .map(|v| match dir {
            ExtrapolateDir::Forwards => *v.last().unwrap(),
            ExtrapolateDir::Backwards => *v.first().unwrap(),
        })
        .reduce(|acc, l| match dir {
            ExtrapolateDir::Forwards => l + acc,
            ExtrapolateDir::Backwards => l - acc,
        })
        .unwrap()
}

fn get_diffs(row: &[i64]) -> Vec<i64> {
    row.iter().tuple_windows().map(|(n, m)| *m - *n).collect()
}

fn is_all_zeros(row: &[i64]) -> bool {
    row.iter().all(|n| *n == 0)
}

fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
    let number = map_res(
        recognize(preceded(opt(char('-')), many1(digit1))),
        |n: &str| n.parse::<i64>(),
    );
    separated_list1(char(' '), number)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, 114);
    }

    #[test]
    fn test_part2_gives_correct_answer() {
        let res = part2(INPUT).unwrap();
        assert_eq!(res, 2);
    }
}
