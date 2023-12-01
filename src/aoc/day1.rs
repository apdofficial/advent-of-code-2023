const NUMBERS: &'static [&'static str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn part1(input_data: &str) -> u32 {
    input_data.split('\n').fold(0, |acc1, line| {
        let (first, last) = line.chars().fold((0,0), |mut acc2, c| {
            if c.is_numeric() {
                acc2.1 = c.to_digit(10).unwrap_or(0);
                if acc2.0 == 0 { acc2.0 = acc2.1; }
            }
            acc2
        });
        acc1 + (first * 10 + last)
    })
}

pub fn part2(input_data: &str) -> u32 {
    input_data.split('\n').fold(0, |acc1, line| {
        let(first, last) = (0..line.len()).fold((0,0), |mut acc2, start| {
            let mut digit = if line.chars().nth(start).unwrap_or('-').is_numeric() {
                line.chars().nth(start).unwrap_or('-').to_digit(10).unwrap_or(0)
            } else { 0 };
            for (i, num) in NUMBERS.iter().enumerate() {
                if line[start..line.len()].starts_with(num) { digit = (i as u32) + 1; }
            }
            if digit != 0 {
                acc2.1 = digit;
                if acc2.0 == 0{ acc2.0 = acc2.1; }
            }
            acc2
        });
        acc1 + (first * 10 + last)
    })
}

#[cfg(test)]
mod tests {
    use crate::aoc::day1;

    const INPUT_1: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    const INPUT_2: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

    #[test]
    fn part1() { assert_eq!(day1::part1(INPUT_1), 142); }

    #[test]
    fn part2() { assert_eq!(day1::part2(INPUT_2), 281); }
}