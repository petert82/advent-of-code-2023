use anyhow::Result;
use regex::Regex;
use std::sync::OnceLock;

pub fn part1(input: &str) -> Result<usize> {
    let res = input.lines().map(parse_line_part1).sum();
    Ok(res)
}

fn parse_line_part1(line: &str) -> usize {
    let mut digits = line
        .chars()
        .filter(|p| p.is_ascii_digit())
        .map(|d| d.to_digit(10).unwrap());

    let first = digits.next().unwrap();
    let last = digits.last();

    if let Some(last) = last {
        return (first as usize * 10) + last as usize;
    }

    (first as usize * 10) + first as usize
}

pub fn part2(input: &str) -> Result<usize> {
    let res = input.lines().map(parse_line_part2).sum();
    Ok(res)
}

static FWD_RE: OnceLock<Regex> = OnceLock::new();
static BWD_RE: OnceLock<Regex> = OnceLock::new();

fn parse_line_part2(line: &str) -> usize {
    let re1 = FWD_RE.get_or_init(|| {
        Regex::new(r"1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine").unwrap()
    });
    let re2 = BWD_RE.get_or_init(|| {
        Regex::new(r"enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|9|8|7|6|5|4|3|2|1").unwrap()
    });

    // Find the first number on the line
    let first: usize = re1
        .find(line)
        .map(|m| match m.as_str() {
            "1" | "one" => 1,
            "2" | "two" => 2,
            "3" | "three" => 3,
            "4" | "four" => 4,
            "5" | "five" => 5,
            "6" | "six" => 6,
            "7" | "seven" => 7,
            "8" | "eight" => 8,
            "9" | "nine" => 9,
            _ => panic!("bad regex match"),
        })
        .unwrap();

    // Find the last number on the line
    let reverse_line: String = line.chars().rev().collect();
    let last: usize = re2
        .find(&reverse_line)
        .map(|m| match m.as_str() {
            "1" | "eno" => 1,
            "2" | "owt" => 2,
            "3" | "eerht" => 3,
            "4" | "ruof" => 4,
            "5" | "evif" => 5,
            "6" | "xis" => 6,
            "7" | "neves" => 7,
            "8" | "thgie" => 8,
            "9" | "enin" => 9,
            _ => panic!("bad regex match"),
        })
        .unwrap();

    (first * 10) + last
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT1).unwrap();
        assert_eq!(res, 142);
    }

    const INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part2_gives_correct_answer() {
        let res = part2(INPUT2).unwrap();
        assert_eq!(res, 281);
    }
}
