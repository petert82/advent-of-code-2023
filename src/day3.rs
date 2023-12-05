use anyhow::Result;
use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> Result<usize> {
    let grid = Grid::from(input);
    let numbers: Vec<usize> = grid
        .parse_numbers()
        .into_iter()
        .filter(|num| num.has_adjacent_symbol(&grid))
        .map(|n| n.into())
        .collect();
    Ok(numbers.iter().sum())
}

pub fn part2(input: &str) -> Result<usize> {
    // Get numbers
    let grid = Grid::from(input);
    let numbers: Vec<Number> = grid.parse_numbers();

    // Make a map of coord -> number
    let num_coords: HashMap<Coord, usize> = numbers.iter().fold(HashMap::new(), |mut acc, n| {
        let n_usize: usize = n.into();
        for c in n.num_coords.iter() {
            acc.insert(c.clone(), n_usize);
        }
        acc
    });
    // Get gear coords
    // For each gear, make set of numbers in adjacent cells, if list len is 2 keep and multiply members
    let sum: usize = grid
        .parse_gear_coords()
        .iter()
        .map(|c| {
            grid.adjacent_coords(c.clone())
                .iter()
                .map(|c| num_coords.get(c).cloned())
                .flatten()
                .collect::<HashSet<usize>>()
        })
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums.iter().product::<usize>())
        .sum();
    Ok(sum)
}

type Coord = (usize, usize);

#[derive(Debug)]
struct Grid {
    w: usize,
    h: usize,
    rows: Vec<Vec<Cell>>,
}

#[derive(Debug, PartialEq)]
enum Cell {
    Symbol,
    Gear,
    Empty,
    Digit(char),
}

#[derive(Debug)]
struct Number {
    digits: String,
    num_coords: HashSet<Coord>,
    adjacent_coords: HashSet<Coord>,
}

impl Number {
    pub fn new() -> Self {
        Self {
            digits: String::new(),
            num_coords: HashSet::new(),
            adjacent_coords: HashSet::new(),
        }
    }

    pub fn push_digit(&mut self, digit: char) -> &mut Self {
        self.digits.push(digit);
        self
    }

    pub fn push_adjacent_coords(&mut self, coord: Coord, coords: HashSet<Coord>) -> &mut Self {
        self.num_coords.insert(coord);
        self.adjacent_coords
            .extend(coords.iter().filter(|c| !self.num_coords.contains(c)));
        self.adjacent_coords.remove(&coord);
        self
    }

    pub fn has_adjacent_symbol(&self, grid: &Grid) -> bool {
        self.adjacent_coords
            .iter()
            .any(|coord| grid.is_symbol(coord.clone()))
    }
}

impl From<Number> for usize {
    fn from(value: Number) -> Self {
        value.digits.parse().unwrap()
    }
}

impl From<&Number> for usize {
    fn from(value: &Number) -> Self {
        value.digits.parse().unwrap()
    }
}

impl Grid {
    pub fn parse_numbers(&self) -> Vec<Number> {
        let mut numbers = Vec::new();
        let mut curr_num: Option<Number> = None;
        for (y, row) in self.rows.iter().enumerate() {
            if curr_num.is_some() {
                numbers.push(curr_num.take().unwrap());
            }
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Gear | Cell::Symbol | Cell::Empty => {
                        if curr_num.is_some() {
                            numbers.push(curr_num.take().unwrap());
                        }
                    }
                    Cell::Digit(c) => {
                        curr_num
                            .get_or_insert_with(Number::new)
                            .push_digit(c.clone())
                            .push_adjacent_coords((x, y), self.adjacent_coords((x, y)));
                    }
                }
            }
        }
        numbers
    }

    pub fn parse_gear_coords(&self) -> Vec<Coord> {
        let mut gear_coords = Vec::new();
        for (y, row) in self.rows.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if Cell::Gear == *cell {
                    gear_coords.push((x, y));
                }
            }
        }
        gear_coords
    }

    pub fn is_symbol(&self, (x, y): Coord) -> bool {
        let Some(row) = self.rows.get(y) else {
            return false;
        };
        let Some(cell) = row.get(x) else {
            return false;
        };
        match cell {
            Cell::Gear => true,
            Cell::Symbol => true,
            _ => false,
        }
    }

    pub fn adjacent_coords(&self, (x, y): Coord) -> HashSet<Coord> {
        let mut adj = HashSet::new();
        // TL
        if x > 0 && y > 0 {
            adj.insert((x - 1, y - 1));
        }
        // T
        if y > 0 {
            adj.insert((x, y - 1));
        }
        // TR
        if x < self.w - 1 && y > 0 {
            adj.insert((x + 1, y - 1));
        }
        // L
        if x > 0 {
            adj.insert((x - 1, y));
        }
        // R
        if x < self.w - 1 {
            adj.insert((x + 1, y));
        }
        // BL
        if x > 0 && y < self.h - 1 {
            adj.insert((x - 1, y + 1));
        }
        // B
        if y < self.h - 1 {
            adj.insert((x, y + 1));
        }
        // BR
        if x < self.w - 1 && y < self.h - 1 {
            adj.insert((x + 1, y + 1));
        }
        adj
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut w = 0;
        let mut rows = Vec::new();
        for line in value.lines() {
            let cols: Vec<Cell> = line.chars().map(|c| c.into()).collect();
            if w == 0 {
                w = cols.len();
            }
            rows.push(cols);
        }

        Grid {
            w,
            h: rows.len(),
            rows,
        }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        if value.is_digit(10) {
            return Cell::Digit(value);
        }
        match value {
            '*' => Cell::Gear,
            '.' => Cell::Empty,
            _ => Cell::Symbol,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, 4361);
    }

    #[test]
    fn test_part2_gives_correct_answer() {
        let res = part2(INPUT).unwrap();
        assert_eq!(res, 467835);
    }
}
