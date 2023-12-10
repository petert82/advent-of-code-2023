use anyhow::{Context, Result};
use nom::{
    character::complete::{digit1, line_ending},
    combinator::{all_consuming, map_res, opt},
    multi::separated_list1,
    sequence::terminated,
    IResult, Parser,
};

pub fn parse_all_to<'a, T>(
    input: &'a str,
    parser: impl Parser<&'a str, T, nom::error::Error<&'a str>>,
) -> Result<T> {
    let (_, res) = all_consuming(terminated(parser, opt(line_ending)))(input)
        .map_err(|e| e.to_owned())
        .context("failed to parse input")?;
    Ok(res)
}

pub fn parse_lines_to_vec<'a, T>(
    input: &'a str,
    parser: impl Parser<&'a str, T, nom::error::Error<&'a str>>,
) -> Result<Vec<T>> {
    let (_, res) = all_consuming(terminated(
        separated_list1(line_ending, parser),
        opt(line_ending),
    ))(input)
    .map_err(|e| e.to_owned())
    .context("failed to parse input")?;
    Ok(res)
}

pub fn number(digits: &str) -> IResult<&str, usize> {
    map_res(digit1, |n: &str| n.parse::<usize>())(digits)
}
