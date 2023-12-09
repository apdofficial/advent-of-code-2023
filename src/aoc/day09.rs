use itertools::Itertools;

fn extrapolate(numbers: &mut Vec<i64>, part2: bool) -> i64 {
    if numbers.iter().all(|&x| x == 0i64) { return 0; }
    let mut tmp: Vec<i64> = vec![];
    for n in 1..numbers.len() {
        tmp.push(numbers[n] - numbers[n-1]);
    }
    if part2 {
        numbers.first().unwrap_or(&0i64) - extrapolate(&mut tmp, part2)
    } else {
        numbers.last().unwrap_or(&0i64) + extrapolate(&mut tmp, part2)
    }
}

pub fn part1(input: &str) -> i64 {
    input.split("\n")
        .map(|line| line
            .split_whitespace()
            .filter_map(|x| x.parse::<i64>().ok())
            .collect_vec()
        )
        .map(|mut x| extrapolate(&mut x, false))
        .sum()
}

pub fn part2(input: &str) -> i64 {
    input.split("\n")
        .map(|line| line
            .split_whitespace()
            .filter_map(|x| x.parse::<i64>().ok())
            .collect_vec()
        )
        .map(|mut x| extrapolate(&mut x, true))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::aoc::day09;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part1() { assert_eq!(day09::part1(INPUT), 114); }

    #[test]
    fn part2() { assert_eq!(day09::part2(INPUT), 2); }
}
