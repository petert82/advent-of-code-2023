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
    solve(input, false)
}

pub fn part2(input: &str) -> Result<usize> {
    solve(input, true)
}

fn solve(input: &str, find_smudges: bool) -> Result<usize> {
    let patterns = parse_lines_to_vec(input, parse_pattern)?;
    let res = patterns
        .iter()
        .map(|p| {
            let v = p
                .reflection_index(MirrorAlignment::Vertical, find_smudges)
                .map_or(0, |i| i);
            let h = p
                .reflection_index(MirrorAlignment::Horizontal, find_smudges)
                .map_or(0, |i| i * 100);
            h + v
        })
        .sum();
    Ok(res)
}

#[derive(Debug)]
struct Pattern {
    /// Rows from the input with each row converted to a usize, where enabled bits
    /// represent '#'s.
    rows: Vec<usize>,
    /// Columns from the input represented in the same way as in `rows`
    cols: Vec<usize>,
}

enum MirrorAlignment {
    Horizontal,
    Vertical,
}

impl Pattern {
    pub fn reflection_index(&self, dir: MirrorAlignment, find_smudges: bool) -> Option<usize> {
        let rows = match dir {
            MirrorAlignment::Horizontal => &self.rows,
            MirrorAlignment::Vertical => &self.cols,
        };

        for i in 0..rows.len() - 1 {
            let mut found_smudge = false;
            for offset in 0..=i + 1 {
                let candidate1 = if offset > i {
                    None
                } else {
                    rows.get(i - offset)
                };
                let candidate2 = rows.get(i + offset + 1);

                if let (Some(a), Some(b)) = (candidate1, candidate2) {
                    if a != b {
                        // If we're looking for smudges, then rows that only differ by
                        // one bit can be considered equal
                        if find_smudges {
                            let diff_bits = (a ^ b).count_ones();
                            if diff_bits == 1 {
                                found_smudge = true;
                                continue;
                            }
                        }
                        // two unequal candidate rows: not in a reflection
                        break;
                    } else {
                        // two equal candidate rows: in a possible reflection
                        continue;
                    }
                }

                if find_smudges && !found_smudge {
                    // if looking for smudges and didn't find one and we're here
                    // we found the none-smudge reflection, so ignore it.
                    break;
                }

                // One of the candidates was None, we didn't break yet, so must have got to
                // the end of a reflection
                return Some(i + 1);
            }
        }
        None
    }
}

/// Requires that no lines in the input are longer than 64 characters
fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    let character = one_of(".#");
    let row = many1(character);
    // terminated by an optional line ending, because the last pattern
    // in the list doesn't have a newline after it.
    map(
        terminated(separated_list1(line_ending, row), opt(line_ending)),
        |char_rows| {
            let rows = char_rows
                .iter()
                .map(|chars| hashes_to_bits(&chars[..]))
                .collect::<Vec<_>>();

            let w = char_rows[0].len();
            let h = char_rows.len();
            let char_cols: Vec<Vec<_>> = (0..w)
                .map(|col| (0..h).map(|row| char_rows[row][col]).rev().collect())
                .collect();
            let cols = char_cols
                .iter()
                .map(|chars| hashes_to_bits(&chars[..]))
                .collect::<Vec<_>>();

            Pattern { rows, cols }
        },
    )(input)
}

/// Takes a row of characters from the input and converts to
/// a binary value where '#' characters are represented by 1
/// and '.' characters are 0
fn hashes_to_bits(chars: &[char]) -> usize {
    let mut val: usize = 0;
    for c in chars.iter() {
        val <<= 1;
        if *c == '#' {
            val |= 1;
        }
    }
    val
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

    #[test]
    fn test_part2_gives_correct_answer() {
        let res = part2(INPUT1).unwrap();
        assert_eq!(res, 400);
    }
}
