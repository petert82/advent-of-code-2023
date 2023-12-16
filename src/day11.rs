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
    fn new(galaxies: Vec<Coord>) -> Self {
        let w = galaxies.iter().map(|c| c.x).max().unwrap() + 1;
        let h = galaxies.iter().map(|c| c.y).max().unwrap() + 1;
        Self { w, h, galaxies }
    }

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
        for y in 0..self.h {
            for x in 0..self.w {
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
        let h = value.lines().count();
        let w = value.lines().next().unwrap().len();
        let mut map: Vec<Vec<char>> = vec![];
        let mut x_offsets = vec![0usize; w];
        let mut y_offsets = vec![0usize; h];

        // Find empty rows
        for (y, line) in value.lines().enumerate() {
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
                increment_offsets(&mut y_offsets, y);
            }
        }

        // Find empty columns
        map = transpose(map);
        for (x, row) in map.iter().enumerate() {
            let mut row_empty = true;
            for c in row.iter() {
                if *c == '#' {
                    row_empty = false;
                }
            }
            if row_empty {
                increment_offsets(&mut x_offsets, x);
            }
        }

        // Map the
        let mut galaxies = vec![];
        for (y, row) in value.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == '#' {
                    let x_offset = x_offsets[x];
                    let y_offset = y_offsets[y];
                    galaxies.push(Coord {
                        x: x + x_offset,
                        y: y + y_offset,
                    });
                }
            }
        }

        Ok(Universe::new(galaxies))
    }
}

fn increment_offsets(offsets: &mut Vec<usize>, from: usize) {
    for i in (from + 1)..offsets.len() {
        if let Some(offset) = offsets.get_mut(i) {
            *offset += 1;
        }
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
