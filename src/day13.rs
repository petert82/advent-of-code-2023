use anyhow::Result;
use nom::{
    character::complete::{line_ending, one_of},
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

use crate::parse::parse_lines_to_vec;

pub fn part1(input: &str) -> Result<usize> {
    let patterns = parse_lines_to_vec(input, parse_pattern)?;
    Ok(1)
}

#[derive(Debug)]
struct Pattern {
    w: usize,
    h: usize,
    rows: Vec<Vec<char>>,
}

fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    let character = one_of(".#");
    let row = many1(character);
    // terminated by an optional line ending, because the last pattern
    // in the list doesn't have a newline after it.
    map(
        terminated(separated_list1(line_ending, row), opt(line_ending)),
        |rows| {
            let w = rows[0].len();
            Pattern {
                w,
                h: rows.len(),
                rows,
            }
        },
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, 405);
    }

    // #[test]
    // fn test_part2_gives_correct_answer() {
    //     let res = part2(INPUT).unwrap();
    //     assert_eq!(res, 525152);
    // }
}
