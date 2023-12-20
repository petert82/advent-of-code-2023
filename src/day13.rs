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
    let res = patterns
        .iter()
        .map(|p| {
            let v = p
                .reflection_index(MirrorAlignment::Vertical)
                .map_or(0, |i| i);
            let h = p
                .reflection_index(MirrorAlignment::Horizontal)
                .map_or(0, |i| i * 100);
            h + v
        })
        .sum();
    Ok(res)
}

#[derive(Debug)]
struct Pattern {
    w: usize,
    h: usize,
    rows: Vec<Vec<char>>,
}

enum MirrorAlignment {
    Horizontal,
    Vertical,
}

impl Pattern {
    pub fn reflection_index(&self, dir: MirrorAlignment) -> Option<usize> {
        let rows = match dir {
            MirrorAlignment::Horizontal => self.rows.clone(),
            MirrorAlignment::Vertical => {
                let transposed: Vec<Vec<_>> = (0..self.w)
                    .map(|col| (0..self.h).map(|row| self.rows[row][col]).rev().collect())
                    .collect();
                transposed
            }
        };

        for i in 0..rows.len() - 1 {
            for offset in 0..=i + 1 {
                let candidate1 = if offset > i {
                    None
                } else {
                    rows.get(i - offset)
                };
                let candidate2 = rows.get(i + offset + 1);
                if candidate1.is_some() && candidate2.is_some() && candidate1 != candidate2 {
                    // two unequal candidate rows: not in a reflection
                    break;
                }

                if candidate1.is_some() && candidate2.is_some() && candidate1 == candidate2 {
                    // two equal candidate rows: in a possible reflection
                    continue;
                }

                // One of the candidates was None, we didn't break yet, so must have got to
                // the end of a reflection
                return Some(i + 1);
            }
        }
        None
    }
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

    const INPUT1: &str = "#.##..##.
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

    const INPUT2: &str = "##..#.#......#..#
.....#..#.#######
...########...##.
..#.#.#.###...##.
..#.#..#.#.#.#..#
..##...#.##.#.##.
###.####..##.#...
....#..#.#..#.##.
....#.###..##....
..##.#..##.######
...##..#.#.##....
...#.#..#........
###....##..######
..#...##..##.#..#
..#.......#...##.";

    #[test]
    fn test_part1_gives_correct_answer() {
        let cases = vec![(INPUT1, 405), (INPUT2, 1)];
        for (input, expect) in cases {
            let res = part1(input).unwrap();
            assert_eq!(res, expect);
        }
    }

    // #[test]
    // fn test_part2_gives_correct_answer() {
    //     let res = part2(INPUT).unwrap();
    //     assert_eq!(res, 525152);
    // }
}
