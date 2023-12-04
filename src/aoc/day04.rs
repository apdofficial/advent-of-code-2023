use itertools::Itertools;

struct Card {
    winning: Vec<u32>,
    actual: Vec<u32>,
}

impl Card {
    pub fn new(line: &str) -> Option<Self> {
        let Some((_, card)) = line.split_once(":") else { return None; };
        let Some((winning_str, actual_str)) = card.split_once("|") else { return None; };
        return Some(Card {
            winning: winning_str.split_whitespace().filter_map(|x| x.parse::<u32>().ok()).collect(),
            actual: actual_str.split_whitespace().filter_map(|x| x.parse::<u32>().ok()).collect(),
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
    input.split("\n").filter_map(Card::new).map(|card| card.geometric_points()).sum()
}

pub fn part2(input: &str) -> u32 {
    let cards = input.split("\n").filter_map(Card::new).map(|card| card.count_matches()).collect_vec();
    let mut card_count = vec![1; cards.len()];
    for i in 0..cards.len() {
        for k in i..i + cards[i]{
            card_count[k+1] += card_count[i];
        }
    }
    card_count.iter().sum()
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