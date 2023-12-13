use std::{
    cell::OnceCell,
    collections::{HashMap, HashSet},
    fmt::Display,
};

use anyhow::{bail, Result};

pub fn part1(input: &str) -> Result<usize> {
    let grid = Grid::try_from(input)?;
    println!("{}", grid);
    Ok(grid.dist_to_farthest_point())
}

struct Grid {
    start: Coord,
    w: usize,
    h: usize,
    pipes: HashMap<Coord, Pipe>,
    loop_coords: OnceCell<Vec<Coord>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord(i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}

impl Grid {
    pub fn new(start: Coord, w: usize, h: usize, pipes: HashMap<Coord, Pipe>) -> Self {
        Self {
            start,
            w,
            h,
            pipes,
            loop_coords: OnceCell::new(),
        }
    }

    pub fn loop_coords(&self) -> &Vec<Coord> {
        self.loop_coords.get_or_init(|| {
            let start_pipe = self.pipes.get(&self.start).unwrap();
            // Choose one of the start pipe's exits (doesn't matter which)
            let mut pipe_pos = start_pipe.exits(self.start).into_iter().next().unwrap();
            let mut prev_pos = self.start;
            let mut pipe_coords = vec![self.start];

            while pipe_pos != self.start {
                pipe_coords.push(pipe_pos);
                let curr = pipe_pos;
                pipe_pos = self
                    .pipes
                    .get(&pipe_pos)
                    .unwrap()
                    .other_exit(pipe_pos, prev_pos)
                    .unwrap();
                prev_pos = curr;
            }
            pipe_coords
        })
    }

    pub fn dist_to_farthest_point(&self) -> usize {
        self.loop_coords().len() / 2
    }
}

impl Coord {
    /// Is the coordinate in the grid?
    pub fn is_valid(&self) -> bool {
        self.0 >= 0 && self.1 >= 0
    }

    /// Gets valid neighbouring coordinates
    pub fn neighbours(&self) -> Vec<Coord> {
        vec![
            Coord(self.0, self.1 - 1),
            Coord(self.0, self.1 + 1),
            Coord(self.0 - 1, self.1),
            Coord(self.0 + 1, self.1),
        ]
        .into_iter()
        .filter(Self::is_valid)
        .collect()
    }
}

impl Pipe {
    /// For a Pipe at `pipe_pos`, gets the coordinates of the valid exits
    /// i.e. exits that are not out of bounds of the grid
    pub fn exits(&self, pipe_pos: Coord) -> Vec<Coord> {
        let deltas = match self {
            Self::NS => vec![(0, -1), (0, 1)],
            Self::EW => vec![(1, 0), (-1, 0)],
            Self::NE => vec![(0, -1), (1, 0)],
            Self::NW => vec![(0, -1), (-1, 0)],
            Self::SW => vec![(0, 1), (-1, 0)],
            Self::SE => vec![(0, 1), (1, 0)],
        };
        deltas
            .iter()
            .map(|(dx, dy)| Coord(pipe_pos.0 + dx, pipe_pos.1 + dy))
            .filter(|c| c.is_valid())
            .collect()
    }

    /// For a Pipe at `pipe_pos`, with one exit at `exit`, gets the coordinates
    /// of the other exit (assuming it would exit to a valid grid coordinate.
    pub fn other_exit(&self, pipe_pos: Coord, exit: Coord) -> Option<Coord> {
        self.exits(pipe_pos)
            .into_iter()
            .filter(|e| *e != exit)
            .next()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}x{} grid. Start is at {}\n\n",
            self.w, self.h, self.start
        )?;
        for y in 0..self.h {
            for x in 0..self.w {
                match self.pipes.get(&Coord(x as i32, y as i32)) {
                    None => write!(f, ".")?,
                    Some(pipe) => write!(f, "{}", pipe)?,
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Self::NS => '║',
            Self::EW => '═',
            Self::NE => '╚',
            Self::NW => '╝',
            Self::SW => '╗',
            Self::SE => '╔',
        };
        write!(f, "{}", char)
    }
}

impl TryFrom<&str> for Grid {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let mut start = None;
        let mut pipes = HashMap::new();
        let mut w = 0;
        let mut h = 0;
        for (y, line) in value.lines().enumerate() {
            h += 1;
            if w == 0 {
                w = line.chars().count()
            }
            for (x, char) in line.chars().enumerate() {
                let mut pipe = None;
                match char {
                    'S' => start = Some(Coord(x as i32, y as i32)),
                    '|' => pipe = Some(Pipe::NS),
                    '-' => pipe = Some(Pipe::EW),
                    'L' => pipe = Some(Pipe::NE),
                    'J' => pipe = Some(Pipe::NW),
                    '7' => pipe = Some(Pipe::SW),
                    'F' => pipe = Some(Pipe::SE),
                    '.' => {}
                    _ => bail!("unexpected character {}", char),
                }
                if pipe.is_some() {
                    pipes.insert(Coord(x as i32, y as i32), pipe.take().unwrap());
                }
            }
        }

        let Some(start) = start else {
            bail!("no start position found in input");
        };

        // Work out which kind of pipe is at the start
        let start_connecting_coords = start
            .neighbours()
            .into_iter()
            .map(|c| (c, pipes.get(&c)))
            .filter(|(c, p)| {
                if let Some(p) = p {
                    return p.exits(*c).iter().any(|e| *e == start);
                }
                false
            })
            .map(|(c, _p)| c)
            .collect::<HashSet<Coord>>();
        let start_pipe = vec![Pipe::NS, Pipe::EW, Pipe::NE, Pipe::NW, Pipe::SE, Pipe::SW]
            .into_iter()
            .filter(|p| {
                let exits = p.exits(start);
                if exits.len() != 2 {
                    return false;
                }
                exits
                    .into_iter()
                    .all(|e| start_connecting_coords.contains(&e))
            })
            .next()
            .expect("expected to have a start pipe");
        pipes.insert(start, start_pipe);

        Ok(Grid::new(start, w, h, pipes))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    const INPUT2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn test_part1_gives_correct_answer() {
        let cases = vec![(INPUT1, 4), (INPUT2, 8)];

        for (input, expect) in cases {
            let res = part1(input).unwrap();
            assert_eq!(res, expect);
        }
    }

    // #[test]
    // fn test_part2_gives_correct_answer() {
    //     let res = part2(INPUT).unwrap();
    //     assert_eq!(res, 2);
    // }
}
