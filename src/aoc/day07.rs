use std::cmp::Ordering;
use itertools::Itertools;

const CARDS: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPairs = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

impl HandType {
    pub fn new(cards: [char; 5]) -> Self {
        let mut card_matches: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
        cards.iter().for_each(|x| *card_matches.entry(x.clone()).or_insert(0) += 1);
        match card_matches.len() {
            1 => HandType::FiveOfAKind,
            2 if card_matches.values().any(|&x| x == 4) => HandType::FourOfAKind,
            2 if card_matches.values().any(|&x| x == 3) => HandType::FullHouse,
            3 if card_matches.values().any(|&x| x == 3) => HandType::ThreeOfAKind,
            3 if card_matches.values().any(|&x| x == 2) => HandType::TwoPairs,
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

struct Hand {
    hand_type: HandType,
    cards: [char; 5],
    bid: u64,
}

impl Hand {
    pub fn from_str(line: &str, joker: bool) -> Option<Self> {
        let (cards_str, bid_str) = line.split_once(" ")?;
        let mut chars = cards_str.chars();
        let cards: [char;5] = [chars.next()?, chars.next()?, chars.next()?, chars.next()?, chars.next()?];
        let hand_type = if joker && cards.iter().any(|&x| 'J' == x) {
            CARDS.iter().copied().map(|c| HandType::new(cards.map(|x| if x == 'J' { c } else { x }))).max()?
        } else {
            HandType::new(cards)
        };
        Some(Hand { hand_type, cards, bid: bid_str.parse::<u64>().ok()? })
    }
}

fn card_value(card: char, joker: bool) -> u8 {
    match card {
        '0'..='9' => card.to_digit(10).unwrap() as u8,
        'T' => 10,
        'J' => if joker { 0 } else { 11 },
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

fn cmp(a: &Hand, b: &Hand, joker: bool) -> Ordering {
    if a.hand_type != b.hand_type { return a.hand_type.cmp(&b.hand_type); }
    for (i, &card) in a.cards.iter().enumerate() {
        if card_value(card, joker) != card_value(b.cards[i], joker) {
            return card_value(card, joker).cmp(&card_value(b.cards[i], joker));
        }
    }
    Ordering::Equal
}

fn solve(input: &str, jack: bool) -> u64 {
    input.split('\n')
        .filter_map(|line| Hand::from_str(line, jack))
        .sorted_by(|a, b| cmp(a, b, jack))
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1) as u64)
        .sum()
}

pub fn part1(input: &str) -> u64 { solve(input, false) }

pub fn part2(input: &str) -> u64 { solve(input, true) }

#[cfg(test)]
mod tests {
    use crate::aoc::day07;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part1() { assert_eq!(day07::part1(INPUT), 6440); }

    #[test]
    fn part2() { assert_eq!(day07::part2(INPUT), 5905); }
}
