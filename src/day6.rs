use anyhow::Result;
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{digit1, line_ending},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

use crate::puzzle::parse_all_to;

pub fn part1(input: &str) -> Result<usize> {
    let mut state = parse_all_to(input, parse_state)?;
    let res = state.part1();
    Ok(res)
}

#[derive(Debug)]
struct State {
    races: Vec<Race>,
    max_duration: usize,
    acceleration: usize,
    time: usize,
}

#[derive(Debug)]
struct Race {
    duration: usize,
    distance: usize,
    win_count: usize,
}

impl State {
    pub fn new(races: Vec<Race>) -> Self {
        let max_duration = races.iter().map(|r| r.duration).max().unwrap();
        Self {
            races,
            max_duration,
            acceleration: 1,
            time: 0,
        }
    }

    pub fn part1(&mut self) -> usize {
        self.time += 1;
        let mut v: usize;
        for t in 0..=self.max_duration {
            v = t * self.acceleration;
            self.races.iter_mut().for_each(|r| r.step(t, v));
        }

        self.races.iter().map(|r| r.win_count).product()
    }
}

impl Race {
    pub fn new(duration: usize, distance: usize) -> Self {
        Self {
            duration,
            distance,
            win_count: 0,
        }
    }

    pub fn step(&mut self, t: usize, v: usize) {
        if t > self.duration {
            return;
        }
        if self.is_win_at(v, t) {
            self.win_count += 1;
        }
    }

    fn is_win_at(&self, v: usize, t: usize) -> bool {
        if t >= self.duration {
            return false;
        }
        let remaining_t = self.duration - t;
        (v * remaining_t) > self.distance
    }
}

fn number(digits: &str) -> IResult<&str, usize> {
    map_res(digit1, |n: &str| n.parse::<usize>())(digits)
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    let parse_times = preceded(
        terminated(tag("Time:"), is_a(" ")),
        separated_list1(is_a(" "), number),
    );
    let parse_distances = preceded(
        terminated(tag("Distance:"), is_a(" ")),
        separated_list1(is_a(" "), number),
    );
    let (input, (times, distances)) =
        separated_pair(parse_times, line_ending, parse_distances)(input)?;
    let races = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(duration, distance)| Race::new(duration, distance))
        .collect();
    Ok((input, races))
}

fn parse_state(input: &str) -> IResult<&str, State> {
    map(parse_races, State::new)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, 288);
    }
}
