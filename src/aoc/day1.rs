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

    const INPUT_DATA: &str = "";

    #[test]
    fn part1() {
        let result = day1::part1(INPUT_DATA);
        assert_eq!(PuzzleResult::Number(0), result);
    }

    #[test]
    fn part2() {
        let result = day1::part2(INPUT_DATA);
        assert_eq!(PuzzleResult::Number(0), result);
    }
}
