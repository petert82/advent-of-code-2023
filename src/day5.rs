use anyhow::{Context, Result};
use nom::bytes::complete::{is_a, is_not, tag};
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::{all_consuming, map, map_res, opt};
use nom::multi::separated_list1;
use nom::sequence::{terminated, tuple};
use nom::{IResult, Parser};
use std::rc::Rc;

pub fn part1(input: &str) -> Result<usize> {
    let state = parse_lines::<State>(input, parse_state)?;
    Ok(1)
}

#[derive(Debug)]
struct State {
    seeds: Vec<usize>,
    start_map: Rc<Map>,
}

#[derive(Debug)]
struct Map {
    name: String,
    ranges: Vec<MapRange>,
    next: Option<Rc<Map>>,
}

#[derive(Debug)]
struct MapRange {
    src_start: usize,
    dst_start: usize,
    size: usize,
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
        |(dst_start, src_start, size)| MapRange {
            src_start,
            dst_start,
            size,
        },
    )(input)
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, name) = terminated(is_not(" "), tag(" map:"))(input)?;
    let (input, _) = line_ending(input)?;
    let (input, ranges) = separated_list1(line_ending, parse_map_range)(input)?;
    Ok((
        input,
        Map {
            name: name.to_owned(),
            ranges,
            next: None,
        },
    ))
}

fn parse_state(input: &str) -> IResult<&str, State> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, _) = is_a("\r\n")(input)?;
    let (input, maps) = separated_list1(is_a("\r\n"), parse_map)(input)?;

    let mut start_map: Option<Rc<Map>> = None;
    let mut prev_map: Option<Rc<Map>> = None;

    for mut map in maps.into_iter().rev() {
        if prev_map.is_some() {
            map.next = Some(Rc::clone(&prev_map.take().unwrap()));
        }
        let map_rc = Rc::new(map);
        start_map = Some(Rc::clone(&map_rc));
        prev_map = Some(Rc::clone(&map_rc));
    }

    Ok((
        input,
        State {
            seeds,
            start_map: start_map.unwrap(),
        },
    ))
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
}
