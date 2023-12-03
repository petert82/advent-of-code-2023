use anyhow::Result;

pub fn part1(input: &str) -> Result<usize> {
    let res = input
        .lines()
        .map(|line| parse_line_part1(line))
        .sum();
    Ok(res)
}

fn parse_line_part1(line: &str) -> usize {
    let mut digits = line
        .chars()
        .filter(|p| p.is_digit(10))
        .map(|d| d.to_digit(10).unwrap());

    let first = digits.next().unwrap();
    let last = digits.last();

    if let Some(last) = last {
        return (first as usize * 10) + last as usize;
    }

    (first as usize * 10) + first as usize
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
}