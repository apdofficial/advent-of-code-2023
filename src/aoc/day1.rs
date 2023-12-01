use crate::aoc::common::PuzzleResult;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref NUMBERS_MAP: HashMap<&'static str, &'static str> = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ]
    .iter()
    .copied()
    .collect();
}

pub fn part1(input_data: &str) -> PuzzleResult {
    input_data
        .split('\n')
        .filter_map(digits_to_number)
        .collect_vec()
        .iter()
        .sum()
}

pub fn part2(input_data: &str) -> PuzzleResult {
    input_data
        .split('\n')
        .map(text_to_text_digit)
        .filter_map(|x| digits_to_number(x.as_ref()))
        .collect_vec()
        .iter()
        .sum()
}

fn text_to_text_digit(line: &str) -> String {
    let mut line: String = line.to_owned();
    for (old, new) in NUMBERS_MAP.iter() {
        line = line.replace(old, new);
    }
    line
}

fn digits_to_number(line: &str) -> Option<u32> {
    let a = line
        .chars()
        .find(|x| x.is_numeric())
        .and_then(|x| x.to_digit(10));
    let b = line
        .chars()
        .rev()
        .find(|x| x.is_numeric())
        .and_then(|x| x.to_digit(10));
    if a.is_none() {
        return None;
    }
    return Some(
        format!("{}{}", a.unwrap(), b.unwrap())
            .parse::<u32>()
            .unwrap(),
    );
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
