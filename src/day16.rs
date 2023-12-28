use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

use anyhow::{bail, Result};

pub fn part1(input: &str) -> Result<usize> {
    let mut state = parse_state(input)?;
    state.energize(Beam(Coord { x: 0, y: 0 }, Direction::Right));
    Ok(state.energized_tile_count())
}

pub fn part2(input: &str) -> Result<usize> {
    let mut state = parse_state(input)?;
    let w = state.w;
    let h = state.h;
    let max_energized = (0..w)
        // Iterate through beam starting postions at the top and bottom..
        .flat_map(|x| {
            vec![
                Beam(Coord { x, y: 0 }, Direction::Down),
                Beam(Coord { x, y: h - 1 }, Direction::Up),
            ]
        })
        // ... and left and right
        .chain((0..h).flat_map(|y| {
            vec![
                Beam(Coord { x: 0, y }, Direction::Right),
                Beam(Coord { x: w - 1, y }, Direction::Right),
            ]
        }))
        // work out how many tiles are energized by each initial beam
        .map(|b| {
            state.energize(b);
            state.energized_tile_count()
        })
        .max()
        .unwrap();
    Ok(max_energized)
}

struct State {
    w: usize,
    h: usize,
    entities: HashMap<Coord, Entity>,
    energized_tiles: HashSet<Coord>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entity {
    /// |
    SplitterVertical,
    /// -
    SplitterHorizontal,
    /// \
    MirrorTLBR,
    /// /
    MirrorTRBL,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam(Coord, Direction);

impl State {
    pub fn new(w: usize, h: usize, entities: HashMap<Coord, Entity>) -> Self {
        Self {
            w,
            h,
            entities,
            energized_tiles: HashSet::new(),
        }
    }

    pub fn energize(&mut self, initial_beam: Beam) {
        self.energized_tiles.clear();
        let mut seen_beams = HashSet::new();
        let mut beams_to_process: VecDeque<Beam> = VecDeque::from([initial_beam]);

        while let Some(beam) = beams_to_process.pop_front() {
            seen_beams.insert(beam);
            self.energized_tiles.insert(beam.0);
            if let Some(entity) = self.entities.get(&beam.0) {
                self.intersect(beam, *entity)
                    .into_iter()
                    .filter(|b| !seen_beams.contains(b))
                    .for_each(|b| beams_to_process.push_back(b));
            } else if let Some(new_beam) = self.extend_beam(beam) {
                if !seen_beams.contains(&new_beam) {
                    beams_to_process.push_back(new_beam);
                }
            }
        }
    }

    pub fn energized_tile_count(&self) -> usize {
        self.energized_tiles.len()
    }

    fn intersect(&self, beam: Beam, entity: Entity) -> Vec<Beam> {
        let mut new_beams = Vec::new();
        let Beam(Coord { x, y }, beam_direction) = beam;
        match (entity, beam_direction) {
            // |
            (Entity::SplitterVertical, Direction::Up) => {
                if y > 0 {
                    new_beams.push(Beam(Coord { x, y: y - 1 }, Direction::Up));
                }
            }
            (Entity::SplitterVertical, Direction::Down) => {
                if y < self.h - 1 {
                    new_beams.push(Beam(Coord { x, y: y + 1 }, Direction::Down));
                }
            }
            (Entity::SplitterVertical, Direction::Left | Direction::Right) => {
                if y > 0 {
                    new_beams.push(Beam(Coord { x, y: y - 1 }, Direction::Up));
                }
                if y < self.h - 1 {
                    new_beams.push(Beam(Coord { x, y: y + 1 }, Direction::Down));
                }
            }
            // -
            (Entity::SplitterHorizontal, Direction::Up | Direction::Down) => {
                if x > 0 {
                    new_beams.push(Beam(Coord { x: x - 1, y }, Direction::Left));
                }
                if x < self.w - 1 {
                    new_beams.push(Beam(Coord { x: x + 1, y }, Direction::Right));
                }
            }
            (Entity::SplitterHorizontal, Direction::Left) => {
                if x > 0 {
                    new_beams.push(Beam(Coord { x: x - 1, y }, Direction::Left));
                }
            }
            (Entity::SplitterHorizontal, Direction::Right) => {
                if x < self.w - 1 {
                    new_beams.push(Beam(Coord { x: x + 1, y }, Direction::Right));
                }
            }
            // \
            (Entity::MirrorTLBR, Direction::Up) => {
                if x > 0 {
                    new_beams.push(Beam(Coord { x: x - 1, y }, Direction::Left));
                }
            }
            (Entity::MirrorTLBR, Direction::Down) => {
                if x < self.w - 1 {
                    new_beams.push(Beam(Coord { x: x + 1, y }, Direction::Right));
                }
            }
            (Entity::MirrorTLBR, Direction::Left) => {
                if y > 0 {
                    new_beams.push(Beam(Coord { x, y: y - 1 }, Direction::Up));
                }
            }
            (Entity::MirrorTLBR, Direction::Right) => {
                if y < self.h - 1 {
                    new_beams.push(Beam(Coord { x, y: y + 1 }, Direction::Down));
                }
            }
            // /
            (Entity::MirrorTRBL, Direction::Up) => {
                if x < self.w - 1 {
                    new_beams.push(Beam(Coord { x: x + 1, y }, Direction::Right));
                }
            }
            (Entity::MirrorTRBL, Direction::Down) => {
                if x > 0 {
                    new_beams.push(Beam(Coord { x: x - 1, y }, Direction::Left));
                }
            }
            (Entity::MirrorTRBL, Direction::Left) => {
                if y < self.h - 1 {
                    new_beams.push(Beam(Coord { x, y: y + 1 }, Direction::Down));
                }
            }
            (Entity::MirrorTRBL, Direction::Right) => {
                if y > 0 {
                    new_beams.push(Beam(Coord { x, y: y - 1 }, Direction::Up));
                }
            }
        }
        new_beams
    }

    fn extend_beam(&self, beam: Beam) -> Option<Beam> {
        let Beam(Coord { x, y }, beam_direction) = beam;
        match beam_direction {
            Direction::Up if y > 0 => Some(Beam(Coord { x, y: y - 1 }, Direction::Up)),
            Direction::Down if y < self.h - 1 => Some(Beam(Coord { x, y: y + 1 }, Direction::Down)),
            Direction::Left if x > 0 => Some(Beam(Coord { x: x - 1, y }, Direction::Left)),
            Direction::Right if x < self.h - 1 => {
                Some(Beam(Coord { x: x + 1, y }, Direction::Right))
            }
            _ => None,
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                if self.energized_tiles.contains(&Coord { x, y }) {
                    write!(f, "#")?;
                } else if let Some(entity) = self.entities.get(&Coord { x, y }) {
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

impl Display for Beam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arrow = match self.1 {
            Direction::Up => '↑',
            Direction::Down => '↓',
            Direction::Left => '←',
            Direction::Right => '→',
        };
        write!(f, "{} ({}, {})", arrow, self.0.x, self.0.y)
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
    Ok(State::new(w, h, entities))
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

    #[test]
    fn test_part2_gives_correct_answer() {
        let res = part2(INPUT).unwrap();
        assert_eq!(res, 51);
    }
}
