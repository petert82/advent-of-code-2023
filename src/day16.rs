use std::{collections::HashMap, fmt::Display};

use anyhow::{bail, Result};

pub fn part1(input: &str) -> Result<usize> {
    let state = parse_state(input)?;
    println!("{}", state);
    Ok(1)
}

struct State {
    w: usize,
    h: usize,
    entities: HashMap<Coord, Entity>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entity {
    SplitterVertical,
    SplitterHorizontal,
    MirrorTLBR,
    MirrorTRBL,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                if let Some(entity) = self.entities.get(&Coord { x, y }) {
                    write!(f, "{}", entity)?;
                } else {
                    write!(f, "{}", '.')?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::SplitterVertical => '|',
            Self::SplitterHorizontal => '-',
            Self::MirrorTLBR => '\\',
            Self::MirrorTRBL => '/',
        };
        write!(f, "{}", c)
    }
}

fn parse_state(input: &str) -> Result<State> {
    let mut h = 0;
    let mut w = 0;
    let mut entities = HashMap::new();
    for (y, row) in input.lines().enumerate() {
        h += 1;
        w = 0;
        for (x, c) in row.chars().enumerate() {
            w += 1;
            if c == '.' {
                continue;
            }
            let entity = match c {
                '|' => Entity::SplitterVertical,
                '-' => Entity::SplitterHorizontal,
                '\\' => Entity::MirrorTLBR,
                '/' => Entity::MirrorTRBL,
                _ => bail!("Unexpected character: {}", c),
            };
            entities.insert(Coord { x, y }, entity);
        }
    }
    Ok(State { w, h, entities })
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, 46);
    }

    // #[test]
    // fn test_part2_gives_correct_answer() {
    //     let res = part2(INPUT).unwrap();
    //     assert_eq!(res, 145);
    // }
}
