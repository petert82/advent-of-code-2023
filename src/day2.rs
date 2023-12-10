use anyhow::{bail, Result};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, line_ending};
use nom::combinator::{all_consuming, map, opt};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

use crate::parse::number;

pub fn part1(input: &str) -> Result<usize> {
    let Ok((_, games)) = parse_games(input) else {
        bail!("could not parse input");
    };
    let res = games
        .iter()
        .filter(|g| g.all_reveals_less_than(13, 14, 15))
        .map(|g| g.id)
        .sum();
    Ok(res)
}

pub fn part2(input: &str) -> Result<usize> {
    let Ok((_, games)) = parse_games(input) else {
        bail!("could not parse input");
    };
    let res = games
        .iter()
        .map(|gm| {
            let (r, g, b) = gm.min_cube_set();
            r * g * b
        })
        .sum();
    Ok(res)
}

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    reveals: Vec<Reveal>,
}

impl Game {
    pub fn all_reveals_less_than(&self, r: usize, g: usize, b: usize) -> bool {
        self.reveals.iter().all(|rv| rv.has_less_than(r, g, b))
    }

    pub fn min_cube_set(&self) -> (usize, usize, usize) {
        let mut max_r = 0;
        let mut max_g = 0;
        let mut max_b = 0;

        for rv in self.reveals.iter() {
            if let Some(r) = rv.r {
                if r > max_r {
                    max_r = r;
                }
            }
            if let Some(g) = rv.g {
                if g > max_g {
                    max_g = g;
                }
            }
            if let Some(b) = rv.b {
                if b > max_b {
                    max_b = b;
                }
            }
        }

        (max_r, max_g, max_b)
    }
}

#[derive(Debug, PartialEq)]
struct Reveal {
    r: Option<usize>,
    g: Option<usize>,
    b: Option<usize>,
}

impl Reveal {
    pub fn has_less_than(&self, r: usize, g: usize, b: usize) -> bool {
        let r_less = match self.r {
            Some(n) => n < r,
            None => true,
        };
        let g_less = match self.g {
            Some(n) => n < g,
            None => true,
        };
        let b_less = match self.b {
            Some(n) => n < b,
            None => true,
        };
        r_less && b_less && g_less
    }
}

fn reveal(input: &str) -> IResult<&str, Reveal> {
    // '3 green, 4 blue, 1 red'
    // There's always at least one colour, but all three don't have to be present
    let red = separated_pair(number, char(' '), tag("red"));
    let green = separated_pair(number, char(' '), tag("green"));
    let blue = separated_pair(number, char(' '), tag("blue"));
    let (input, parts) = separated_list1(tag(", "), alt((red, green, blue)))(input)?;
    let mut reveal = Reveal {
        r: None,
        g: None,
        b: None,
    };
    for (n, col) in parts {
        match col {
            "red" => reveal.r = Some(n),
            "green" => reveal.g = Some(n),
            "blue" => reveal.b = Some(n),
            _ => unreachable!(),
        }
    }
    Ok((input, reveal))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    // 'Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green'
    map(
        separated_pair(
            preceded(tag("Game "), number),
            tag(": "),
            separated_list1(tag("; "), reveal),
        ),
        |(id, reveals)| Game { id, reveals },
    )(input)
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    all_consuming(terminated(
        separated_list1(line_ending, parse_game),
        opt(line_ending),
    ))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT1).unwrap();
        assert_eq!(res, 8);
    }

    #[test]
    fn test_part2_gives_correct_answer() {
        let res = part2(INPUT1).unwrap();
        assert_eq!(res, 2286);
    }

    #[test]
    fn test_can_parse_a_list_of_games() {
        let input = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red";
        assert_eq!(
            parse_games(input),
            Ok((
                "",
                vec![
                    Game {
                        id: 4,
                        reveals: vec![
                            Reveal {
                                r: Some(3),
                                g: Some(1),
                                b: Some(6)
                            },
                            Reveal {
                                r: Some(6),
                                g: Some(3),
                                b: None,
                            },
                            Reveal {
                                r: Some(14),
                                g: Some(3),
                                b: Some(15)
                            },
                        ]
                    },
                    Game {
                        id: 5,
                        reveals: vec![
                            Reveal {
                                r: Some(6),
                                g: Some(3),
                                b: Some(1)
                            },
                            Reveal {
                                r: Some(1),
                                g: None,
                                b: Some(2)
                            }
                        ]
                    }
                ]
            ))
        );
    }
}
