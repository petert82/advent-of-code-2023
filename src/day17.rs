use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use anyhow::{anyhow, Result};

pub fn part1(input: &str) -> Result<usize> {
    let costs = parse_costs(input)?;

    let get_neighbours = |vertex: &Vertex| -> Vec<Vertex> {
        let poss_new = if vertex.forward_count < 3 {
            vec![
                vertex.forward(&costs),
                vertex.left(&costs),
                vertex.right(&costs),
            ]
        } else {
            vec![vertex.left(&costs), vertex.right(&costs)]
        };
        poss_new.iter().filter_map(|v| *v).collect()
    };

    let get_cost = |to: &Vertex| -> usize {
        let Coord(x, y) = to.pos.1;
        costs.values[y][x]
    };

    let is_dest = |vertex: &Vertex| -> bool {
        let Coord(x, y) = vertex.pos.1;
        x == costs.w - 1 && y == costs.h - 1
    };

    let starts = vec![
        Vertex::new(Direction::East, 0, 0, 0),
        Vertex::new(Direction::South, 0, 0, 0),
    ];

    dijkstra(starts, &get_neighbours, &get_cost, &is_dest).ok_or(anyhow!("could not find path"))
}

fn parse_costs(input: &str) -> Result<Costs> {
    let mut h = 0;
    let mut w = 0;
    let mut rows = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        h += 1;
        w = 0;
        for c in line.chars() {
            w += 1;
            let val = c.to_digit(10).ok_or(anyhow!("parsing failed"))? as usize;
            row.push(val);
        }
        rows.push(row);
    }
    Ok(Costs { w, h, values: rows })
}

fn dijkstra(
    starts: Vec<Vertex>,
    get_neighbours: &dyn Fn(&Vertex) -> Vec<Vertex>,
    get_cost: &dyn Fn(&Vertex) -> usize,
    is_dest: &dyn Fn(&Vertex) -> bool,
) -> Option<usize> {
    let mut distances = HashMap::new();
    let mut to_visit = BinaryHeap::new();
    let mut visited = HashSet::new();

    for start in starts.into_iter() {
        distances.insert(start, 0);
        to_visit.push(Reverse(Visit {
            vertex: start,
            distance: 0,
        }));
    }

    while let Some(Reverse(Visit { vertex, distance })) = to_visit.pop() {
        if !visited.insert(vertex) {
            continue;
        }

        if is_dest(&vertex) {
            return Some(distance);
        }

        for neighbour in get_neighbours(&vertex) {
            let cost = get_cost(&neighbour);
            let new_distance = distance + cost;
            let is_closer = distances
                .get(&neighbour)
                .map_or(true, |prev| new_distance < *prev);

            if is_closer {
                distances.insert(neighbour, new_distance);
                to_visit.push(Reverse(Visit {
                    vertex: neighbour,
                    distance: new_distance,
                }));
            }
        }
    }

    None
}

struct Costs {
    w: usize,
    h: usize,
    values: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord(usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector(Direction, Coord);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vertex {
    pos: Vector,
    forward_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Visit {
    vertex: Vertex,
    distance: usize,
}

impl Vertex {
    pub fn new(d: Direction, x: usize, y: usize, forward_count: usize) -> Self {
        Self {
            pos: Vector(d, Coord(x, y)),
            forward_count,
        }
    }

    pub fn forward(&self, grid: &Costs) -> Option<Self> {
        let Self {
            pos: Vector(direction, coord),
            forward_count,
        } = *self;
        let Coord(x, y) = coord;

        let new_coord = match direction {
            Direction::North if y > 0 => Some(Coord(x, y - 1)),
            Direction::East if x < grid.w - 1 => Some(Coord(x + 1, y)),
            Direction::South if y < grid.h - 1 => Some(Coord(x, y + 1)),
            Direction::West if x > 0 => Some(Coord(x - 1, y)),
            _ => None,
        };

        match new_coord {
            None => None,
            Some(coord) => Some(Self {
                pos: Vector(direction, coord),
                forward_count: forward_count + 1,
            }),
        }
    }

    pub fn left(&self, grid: &Costs) -> Option<Self> {
        let Self {
            pos: Vector(direction, coord),
            forward_count: _,
        } = *self;
        let new_direction = match direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Self {
            pos: Vector(new_direction, coord),
            forward_count: 0,
        }
        .forward(grid)
    }

    pub fn right(&self, grid: &Costs) -> Option<Self> {
        let Self {
            pos: Vector(direction, coord),
            forward_count: _,
        } = *self;
        let new_direction = match direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Self {
            pos: Vector(new_direction, coord),
            forward_count: 0,
        }
        .forward(grid)
    }
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, 102);
    }

    // #[test]
    // fn test_part2_gives_correct_answer() {
    //     let res = part2(INPUT).unwrap();
    //     assert_eq!(res, 51);
    // }
}
