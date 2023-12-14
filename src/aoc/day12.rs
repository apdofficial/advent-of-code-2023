use std::collections::HashMap;
use itertools::Itertools;

const DAMAGED: char = '#';
const OPERATIONAL: char = '.';

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    s: usize,
    n: usize,
}

impl Position {
    pub fn new() -> Self { Position { s: 0, n: 0 } }
}

type CountCache = HashMap<Position, usize>;

struct ConditionRecord {
    springs: Vec<char>,
    numbers: Vec<usize>,
}

impl ConditionRecord {
    pub fn number(&self, pos: &Position) -> usize { self.numbers[pos.n] }

    pub fn spring(&self, pos: &Position) -> char { self.springs[pos.s] }

    pub fn find_first(&self, pos: &Position, pred: fn(char) -> bool) -> Option<Position> {
        let mut pos = pos.clone();
        while pos.s < self.springs.len() {
            if pred(self.spring(&pos)) { return Some(pos); }
            pos.s += 1;
        }
        None
    }
}

type ConditionRecords = Vec<ConditionRecord>;

fn count_arrangements(rec: &ConditionRecord, pos: Position, cache: &mut CountCache) -> usize {
    if rec.numbers.len() == pos.n {
        if (pos.s..rec.springs.len()).any(|i| rec.springs[i] == DAMAGED) {
            return 0;
        }
        return 1;
    }
    let Some(pos) = rec.find_first(&pos, |c| c != OPERATIONAL) else {
        return 0;
    };
    if pos.s + rec.number(&pos) > rec.springs.len() {
        return 0;
    }
    if cache.contains_key(&pos) {
        return *cache.get(&pos).unwrap_or(&0);
    };
    let mut count = 0;
    if (pos.s..pos.s + rec.number(&pos)).all(|i| rec.springs[i] != OPERATIONAL) {
        if pos.s + rec.number(&pos) == rec.springs.len() || rec.springs[pos.s + rec.number(&pos)] != DAMAGED {
            count += count_arrangements(rec, Position { s: pos.s + rec.number(&pos) + 1, n: pos.n + 1 }, cache);
        }
    }
    if rec.spring(&pos) != DAMAGED {
        count += count_arrangements(rec, Position { s: pos.s + 1, n: pos.n }, cache);
    }
    cache.insert(pos, count);
    count
}

fn load_input(input: &str) -> ConditionRecords {
    input.split('\n').filter_map(|line| {
        let (springs, numbers_srt) = line.split_once(" ")?;
        let numbers = numbers_srt
            .split(',')
            .map(|x| x.parse::<usize>().ok().unwrap_or(0)).collect_vec();
        Some(ConditionRecord { springs: springs.chars().collect_vec(), numbers })
    }).collect_vec()
}

pub fn part1(input: &str) -> i64 {
    let mut cache = CountCache::new();
    load_input(input)
        .iter()
        .map(|record| {
            cache.clear();
            count_arrangements(record, Position::new(), &mut cache)
        })
        .sum::<usize>() as i64
}

pub fn part2(input: &str) -> i64 {
    let mut cache = CountCache::new();
    load_input(input)
        .iter()
        .map(|x| {
            let mut a = x.springs.clone();
            a.append(&mut [vec!['?'], x.springs.clone()].concat());
            a.append(&mut [vec!['?'], x.springs.clone()].concat());
            a.append(&mut [vec!['?'], x.springs.clone()].concat());
            a.append(&mut [vec!['?'], x.springs.clone()].concat());
            let mut b = x.numbers.clone();
            b.append(&mut x.numbers.clone());
            b.append(&mut x.numbers.clone());
            b.append(&mut x.numbers.clone());
            b.append(&mut x.numbers.clone());
            let record = ConditionRecord { springs: a, numbers: b };
            cache.clear();
            count_arrangements(&record, Position::new(), &mut cache)
        })
        .sum::<usize>() as i64
}

#[cfg(test)]
mod tests {
    use crate::aoc::day12;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part1() { assert_eq!(day12::part1(INPUT), 21); }

    #[test]
    fn part2() { assert_eq!(day12::part2(INPUT), 525152); }
}