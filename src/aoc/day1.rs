use crate::aoc::common::PuzzleResult;

pub fn part1(input_data: &str) -> PuzzleResult {
    PuzzleResult::Text("Not implemented".into())
}

pub fn part2(input_data: &str) -> PuzzleResult {
    PuzzleResult::Text("Not implemented".into())
}

#[cfg(test)]
mod tests {
    use crate::aoc::common::PuzzleResult;
    use crate::aoc::day1;
    const INPUT_1: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    const INPUT_2: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
    #[test]
    fn part1() {
        let result = day1::part1(INPUT_1);
        assert_eq!(result, PuzzleResult::Number(142));
    }

    #[test]
    fn part2() {
        let result = day1::part2(INPUT_2);
        assert_eq!(result, PuzzleResult::Number(281));
    }
}
