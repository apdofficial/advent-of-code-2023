use std::collections::VecDeque;
use itertools::Itertools;

struct Card {
    winning: Vec<u32>,
    actual: Vec<u32>,
}

impl Card {
    pub fn new(line: &str) -> Option<Self> {
        let (winning, actual) = line.split_once(":")?.1.split_once("|")?;
        return Some(Card {
            winning: winning.split_whitespace().filter_map(|x| x.parse::<u32>().ok()).collect(),
            actual: actual.split_whitespace().filter_map(|x| x.parse::<u32>().ok()).collect(),
        });
    }

    pub fn geometric_points(&self) -> u32 {
        self.count_matches().checked_sub(1).and_then(|x| Some(u32::pow(2, x as u32))).unwrap_or(0)
    }

    pub fn count_matches(&self) -> usize {
        self.winning.iter().filter(|x| self.actual.contains(x)).count()
    }
}

pub fn part1(input: &str) -> u32 {
    input.split("\n").filter_map(Card::new)
        .map(|card| card.geometric_points())
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let mut solved = VecDeque::<u32>::new(); // for dynamic programing
    input.split("\n").filter_map(Card::new)
        .map(|card| card.count_matches())
        .collect_vec()
        .into_iter()
        .rev()
        .for_each(|count| {
            solved.push_front(1 + (0..count).map(|i| solved[i]).sum::<u32>());
        });
    solved.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::aoc::day04;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1() { assert_eq!(day04::part1(INPUT), 13); }

    #[test]
    fn part2() { assert_eq!(day04::part2(INPUT), 30); }
}