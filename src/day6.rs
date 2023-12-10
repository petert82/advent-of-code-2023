use anyhow::Result;
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{digit1, line_ending},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

use crate::parse::{number, parse_all_to};

pub fn part1(input: &str) -> Result<usize> {
    let mut state = parse_all_to(input, parse_state_part1)?;
    let res = state.calculate_win_counts().product();
    Ok(res)
}

pub fn part2(input: &str) -> Result<usize> {
    let mut state = parse_all_to(input, parse_state_part2)?;
    let res = state.calculate_win_counts().next().unwrap();
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

    pub fn calculate_win_counts(&mut self) -> impl Iterator<Item = usize> + '_ {
        self.time += 1;
        let mut v: usize;
        for t in 0..=self.max_duration {
            v = t * self.acceleration;
            self.races.iter_mut().for_each(|r| r.step(t, v));
        }
        self.races.iter().map(|r| r.win_count)
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

fn parse_races_part1(input: &str) -> IResult<&str, Vec<Race>> {
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

fn parse_state_part1(input: &str) -> IResult<&str, State> {
    map(parse_races_part1, State::new)(input)
}

fn parse_race_part2(input: &str) -> IResult<&str, Vec<Race>> {
    let parse_time = map_res(
        preceded(
            terminated(tag("Time:"), is_a(" ")),
            separated_list1(is_a(" "), digit1),
        ),
        |parts| parts.join("").parse::<usize>(),
    );
    let parse_distance = map_res(
        preceded(
            terminated(tag("Distance:"), is_a(" ")),
            separated_list1(is_a(" "), digit1),
        ),
        |parts| parts.join("").parse::<usize>(),
    );
    let (input, (duration, distance)) =
        separated_pair(parse_time, line_ending, parse_distance)(input)?;
    let race = Race::new(duration, distance);
    Ok((input, vec![race]))
}

fn parse_state_part2(input: &str) -> IResult<&str, State> {
    map(parse_race_part2, State::new)(input)
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

    #[test]
    fn test_part2_gives_correct_answer() {
        let res = part2(INPUT).unwrap();
        assert_eq!(res, 71503);
    }
}
