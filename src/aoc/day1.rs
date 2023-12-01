use crate::aoc::common::PuzzleResult;

const NUMBERS: &'static [&'static str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn part1(input_data: &str) -> PuzzleResult {
    input_data.split('\n').filter_map(|line| {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        for c in line.chars() {
            if c.is_numeric() {
                last = c.to_digit(10);
                if first.is_none() { first = last; }
            }
        }
        first.and_then(|a| last.and_then(|b| Some(a * 10 + b)))
    }).sum()
}

pub fn part2(input_data: &str) -> PuzzleResult {
    input_data.split('\n').filter_map(|line| {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        for start in 0..line.len() {
            let mut digit = if line.chars().nth(start)?.is_numeric() {
                line.chars().nth(start)?.to_digit(10)
            } else { None };
            for (i, num) in NUMBERS.iter().enumerate() {
                if line[start..line.len()].starts_with(num) { digit = Some((i as u32) + 1); }
            }
            if digit.is_some() {
                last = digit;
                if first.is_none() { first = last; }
            }
        }
        first.and_then(|a| last.and_then(|b| Some(a * 10 + b)))
    }).sum()
}

#[cfg(test)]
mod tests {
    use crate::aoc::common::PuzzleResult;
    use crate::aoc::day1;

    const INPUT_1: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    const INPUT_2: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

    #[test]
    fn part1() { assert_eq!(day1::part1(INPUT_1), PuzzleResult::Number(142)); }

    #[test]
    fn part2() { assert_eq!(day1::part2(INPUT_2), PuzzleResult::Number(281)); }
}