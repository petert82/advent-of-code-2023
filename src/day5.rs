use anyhow::{Context, Result};
use itertools::Itertools;
use nom::bytes::complete::{is_a, is_not, tag};
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::{all_consuming, map, map_res, opt};
use nom::multi::separated_list1;
use nom::sequence::{terminated, tuple};
use nom::{IResult, Parser};
use rayon::prelude::*;
use std::sync::Arc;

pub fn part1(input: &str) -> Result<usize> {
    let state = parse_lines::<State>(input, parse_state)?;
    Ok(state.part1())
}

pub fn part2(input: &str) -> Result<usize> {
    let state = parse_lines::<State>(input, parse_state)?;
    Ok(state.part2())
}

#[derive(Debug)]
struct State {
    seeds: Vec<usize>,
    seed_ranges: Vec<SeedRange>,
    start_map: Arc<Map>,
}

#[derive(Debug)]
struct SeedRange {
    min: usize,
    max: usize,
}

#[derive(Debug)]
struct Map {
    #[allow(dead_code)]
    name: String,
    ranges: Vec<MapRange>,
    next: Option<Arc<Map>>,
}

#[derive(Debug)]
struct MapRange {
    src_start: usize,
    dst_start: usize,
    size: usize,
}

impl State {
    pub fn new(seeds: Vec<usize>, start_map: Arc<Map>) -> Self {
        let seed_ranges = seeds
            .iter()
            .tuples()
            .map(|(start, len)| SeedRange {
                min: *start,
                max: *start + *len - 1,
            })
            .collect();
        Self {
            seeds,
            start_map,
            seed_ranges,
        }
    }

    pub fn part1(&self) -> usize {
        self.seeds
            .iter()
            .map(|s| self.start_map.lookup(*s))
            .min()
            .unwrap()
    }

    pub fn part2(&self) -> usize {
        self.seed_ranges
            .par_iter()
            .map(|range| {
                (range.min..range.max)
                    .into_par_iter()
                    .map(|s| self.start_map.lookup(s))
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap()
    }
}

impl Map {
    pub fn new(name: String, ranges: Vec<MapRange>, next: Option<Arc<Map>>) -> Self {
        Self { name, ranges, next }
    }

    pub fn lookup(&self, n: usize) -> usize {
        let m = self.ranges.iter().find_map(|r| r.lookup(n)).unwrap_or(n);
        match &self.next {
            Some(next) => next.lookup(m),
            None => m,
        }
    }
}

impl MapRange {
    pub fn new(src_start: usize, dst_start: usize, size: usize) -> Self {
        Self {
            src_start,
            dst_start,
            size,
        }
    }

    pub fn lookup(&self, n: usize) -> Option<usize> {
        let src_end = self.src_start + self.size - 1;
        if n < self.src_start || n > src_end {
            return None;
        }
        let offset = n - self.src_start;
        Some(self.dst_start + offset)
    }
}

fn number(digits: &str) -> IResult<&str, usize> {
    map_res(digit1, |n: &str| n.parse::<usize>())(digits)
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = tag("seeds: ")(input)?;
    separated_list1(space1, number)(input)
}

fn parse_map_range(input: &str) -> IResult<&str, MapRange> {
    map(
        tuple((
            terminated(number, tag(" ")),
            terminated(number, tag(" ")),
            number,
        )),
        |(dst_start, src_start, size)| MapRange::new(src_start, dst_start, size),
    )(input)
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, name) = terminated(is_not(" "), tag(" map:"))(input)?;
    let (input, _) = line_ending(input)?;
    let (input, ranges) = separated_list1(line_ending, parse_map_range)(input)?;
    Ok((input, Map::new(name.to_owned(), ranges, None)))
}

fn parse_state(input: &str) -> IResult<&str, State> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, _) = is_a("\r\n")(input)?;
    let (input, maps) = separated_list1(is_a("\r\n"), parse_map)(input)?;

    let mut start_map: Option<Arc<Map>> = None;
    let mut prev_map: Option<Arc<Map>> = None;

    for mut map in maps.into_iter().rev() {
        if prev_map.is_some() {
            map.next = Some(Arc::clone(&prev_map.take().unwrap()));
        }
        let map_arc = Arc::new(map);
        start_map = Some(Arc::clone(&map_arc));
        prev_map = Some(Arc::clone(&map_arc));
    }

    Ok((input, State::new(seeds, start_map.unwrap())))
}

fn parse_lines<'a, T>(
    input: &'a str,
    parser: impl Parser<&'a str, T, nom::error::Error<&'a str>>,
) -> Result<T> {
    let (_, res) = all_consuming(terminated(parser, opt(line_ending)))(input)
        .map_err(|e| e.to_owned())
        .context("failed to parse input")?;
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, 35);
    }

    #[test]
    fn test_part2_gives_correct_answer() {
        let res = part2(INPUT).unwrap();
        assert_eq!(res, 46);
    }

    #[test]
    fn test_can_lookup_a_value_in_a_map_range() {
        let mr = MapRange::new(98, 50, 2);

        assert_eq!(mr.lookup(97), None);
        assert_eq!(mr.lookup(98), Some(50));
        assert_eq!(mr.lookup(99), Some(51));
        assert_eq!(mr.lookup(100), None);
    }

    #[test]
    fn test_can_lookup_a_value_in_a_single_map() {
        let ranges = vec![MapRange::new(98, 50, 2), MapRange::new(50, 52, 48)];
        let m = Map::new("map-1".to_string(), ranges, None);
        assert_eq!(m.lookup(50), 52);
        assert_eq!(m.lookup(79), 81);
        assert_eq!(m.lookup(98), 50);
        assert_eq!(m.lookup(99), 51);
        assert_eq!(m.lookup(100), 100);
    }

    #[test]
    fn test_can_lookup_a_value_in_a_linked_map() {
        let ranges2 = vec![
            MapRange::new(15, 0, 37),
            MapRange::new(52, 37, 2),
            MapRange::new(0, 39, 15),
        ];
        let m2 = Map::new("map-2".to_string(), ranges2, None);
        let ranges1 = vec![MapRange::new(98, 50, 2), MapRange::new(50, 52, 48)];
        let m1 = Map::new("map-1".to_string(), ranges1, Some(Arc::new(m2)));
        assert_eq!(m1.lookup(79), 81);
        assert_eq!(m1.lookup(14), 53);
    }
}
