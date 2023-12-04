const NUMBERS: &'static [&'static str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

struct Calibration{ val: u32 }

impl Calibration {
    pub fn new(line: &str, consider_text: bool) -> Self {
        let(first, last) = (0..line.len()).fold((0,0), | (mut first, mut last), start| {
            let mut digit = if line.chars().nth(start).unwrap_or('-').is_numeric() {
                line.chars().nth(start).unwrap_or('-').to_digit(10).unwrap_or(0)
            } else { 0 };
            if consider_text {
                for (i, num) in NUMBERS.iter().enumerate() {
                    if line[start..line.len()].starts_with(num) { digit = (i as u32) + 1; }
                }
            }
            if digit != 0 {
                last = digit;
                if first == 0 { first = last; }
            }
            (first, last)
        });
        Calibration { val: first * 10 + last }
    }
}

pub fn part1(input_data: &str) -> u32 {
    input_data.split('\n').map(|line| Calibration::new(line, false))
        .map(|calibration| calibration.val)
        .sum()
}

pub fn part2(input_data: &str) -> u32 {
    input_data.split('\n').map(|line| Calibration::new(line, true))
        .map(|calibration| calibration.val)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::aoc::day01;

    const INPUT_1: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    const INPUT_2: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

    #[test]
    fn part1() { assert_eq!(day01::part1(INPUT_1), 142); }

    #[test]
    fn part2() { assert_eq!(day01::part2(INPUT_2), 281); }
}