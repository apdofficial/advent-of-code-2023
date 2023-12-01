use crate::aoc::common::PuzzleResult;

const NUMBERS: &'static [(&'static str, &'static str)] = &[
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "f4r"),
    ("five", "f5e"),
    ("six", "s6x"),
    ("seven", "s7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
];

pub fn part1(input_data: &str) -> PuzzleResult {
    input_data
        .split('\n')
        .filter_map(to_number)
        .sum()
}

pub fn part2(input_data: &str) -> PuzzleResult {
    input_data
        .split('\n')
        .filter_map(|x| {
            let mut line: String = x.to_owned();
            NUMBERS.iter().for_each(|(old, new)| {
                line = line.replace(old, new)
            });
            to_number(line.as_ref())
        })
        .sum()
}

fn to_number(line: &str) -> Option<u32> {
    let (first, last) = line.chars()
        .fold((Option::<u32>::None, Option::<u32>::None), |mut acc, x| {
            if x.is_numeric() {
                acc.1 = x.to_digit(10);
                if acc.0.is_none() { acc.0 = acc.1; }
            }
            acc
        });
    if let (Some(a), Some(b)) = (first, last) {
        return Some(format!("{}{}", a, b).parse::<u32>().unwrap())
    }
    None
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