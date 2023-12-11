use anyhow::Result;

pub fn part1(_input: &str) -> Result<usize> {
    Ok(1)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    const INPUT2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn test_part1_gives_correct_answer() {
        let cases = vec![(INPUT1, 4), (INPUT2, 8)];

        for (input, expect) in cases {
            let res = part1(input).unwrap();
            assert_eq!(res, expect);
        }
    }

    // #[test]
    // fn test_part2_gives_correct_answer() {
    //     let res = part2(INPUT).unwrap();
    //     assert_eq!(res, 2);
    // }
}
