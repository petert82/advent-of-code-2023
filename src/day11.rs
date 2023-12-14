use std::fmt::Display;

use anyhow::{bail, Result};
use itertools::Itertools;

pub fn part1(input: &str) -> Result<usize> {
    let universe = Universe::try_from(input)?;
    Ok(universe.galaxy_pair_distances().iter().sum())
}

struct Universe {
    w: usize,
    h: usize,
    galaxies: Vec<Coord>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Universe {
    fn galaxy_pair_distances(&self) -> Vec<usize> {
        self.galaxies
            .iter()
            .tuple_combinations()
            .map(|(a, b)| a.distance_to(b))
            .collect()
    }
}

impl Coord {
    fn distance_to(&self, other: &Self) -> usize {
        (self.x as isize - other.x as isize).abs() as usize
            + (self.y as isize - other.y as isize).abs() as usize
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.w {
            for y in 0..self.h {
                if self.galaxies.contains(&Coord { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl TryFrom<&str> for Universe {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let mut map: Vec<Vec<char>> = vec![];

        for line in value.lines() {
            let mut line_empty = true;
            let mut line_chars = vec![];
            for c in line.chars() {
                match c {
                    '#' => line_empty = false,
                    '.' => {}
                    c => bail!("unexpected character in input: {}", c),
                }
                line_chars.push(c);
            }
            map.push(line_chars);
            if line_empty {
                let extra_line = (0..line.len()).map(|_| '.').collect();
                map.push(extra_line);
            }
        }

        map = transpose(map);
        let mut expanded_map: Vec<Vec<char>> = vec![];
        for row in map.iter() {
            let mut row_empty = true;
            let mut row_chars = vec![];
            for c in row.iter() {
                if *c == '#' {
                    row_empty = false;
                }
                row_chars.push(*c);
            }
            expanded_map.push(row_chars);
            if row_empty {
                let extra_row = (0..row.len()).map(|_| '.').collect();
                expanded_map.push(extra_row);
            }
        }
        expanded_map = transpose(expanded_map);
        expanded_map = transpose(expanded_map);

        let mut galaxies = vec![];
        for (y, row) in expanded_map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '#' {
                    galaxies.push(Coord { x, y });
                }
            }
        }

        Ok(Universe {
            w: expanded_map.first().unwrap().len(),
            h: expanded_map.len(),
            galaxies,
        })
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, 374);
    }

    // #[test]
    // fn test_part2_gives_correct_answer() {
    //     let res = part2(INPUT).unwrap();
    //     assert_eq!(res, 30);
    // }
}
