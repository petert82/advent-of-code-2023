use anyhow::Result;

pub fn part1(_input: &str) -> Result<usize> {
    Ok(1)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, 21);
    }
}
