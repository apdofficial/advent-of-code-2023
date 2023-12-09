use std::ops::{AddAssign, Range};
use itertools::Itertools;
use regex::{Match, Regex};

const GEAR_RE: &str = r"[*]";
const NUMBER_RE: &str = r"[0-9]+";
const SYMBOL_RE: &str = r"[^0-9.]";

struct Schematics {
    lines: Vec<String>,
}

impl Schematics {
    pub fn new(input: &str) -> Self {
        let mut schematics = Schematics { lines: vec![] };
        schematics.lines.extend(input.split('\n').map(|x| x.into()));
        schematics
    }

    pub fn fold<F, T: AddAssign + Copy>(&self, re: &str, init: T,  mut f: F) -> T where F: FnMut(T, usize, &Match) -> T {
        let mut acc = init;
        let mut line_nr = 0;
        let reg = Regex::new(re).unwrap();
        self.lines.iter().for_each(|line| {
            reg.find_iter(line).for_each(|m| acc = f(acc, line_nr, &m) );
            line_nr += 1;
        });
        acc
    }

    pub fn find_neighbours(&self, re: &str, line_nr: usize, m: &Match) -> Vec<Match> {
        let reg = Regex::new(re).unwrap();
        let mut matches = reg.find_iter(self.lines[line_nr].as_str()).collect_vec();
        if line_nr > 0 { matches.extend(reg.find_iter(self.lines[line_nr - 1].as_str())); }
        if line_nr < self.lines.len() - 1 { matches.extend(reg.find_iter(self.lines[line_nr + 1].as_str())); }
        matches.iter().filter(|x| self.all_neighbours(line_nr, x).any(|i| m.start() <= i && i <= m.end()-1)).copied().collect()
    }

    fn all_neighbours(&self, line_nr: usize, m: &Match) -> Range<usize> {
        Range {
            start: m.start().checked_sub(1).unwrap_or(0),
            end: if m.end() < self.lines[line_nr].as_str().chars().count() - 2 { m.end() + 1 } else { m.range().end },
        }
    }
}
pub fn part1(input: &str) -> i64 {
    let schematics = Schematics::new(input);
    schematics.fold(NUMBER_RE, 0, |mut sum, line_nr, m| {
        let matches = schematics.find_neighbours(SYMBOL_RE, line_nr, m);
        if matches.len() > 0 {
            sum += m.as_str().parse::<u64>().unwrap_or(0);
        }
        sum
    }) as i64
}

pub fn part2(input: &str) -> i64 {
    let schematics = Schematics::new(input);
    schematics.fold(GEAR_RE, 0, |mut sum, line_nr, m| {
        let matches = schematics.find_neighbours(NUMBER_RE, line_nr, m);
        if matches.len() == 2 {
            sum += matches.get(0).unwrap().as_str().parse::<u64>().unwrap_or(0) *
                matches.get(1).unwrap().as_str().parse::<u64>().unwrap_or(0);
        }
        sum
    })  as i64
}

#[cfg(test)]
mod tests {
    use crate::aoc::day03;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part1() { assert_eq!(day03::part1(INPUT), 4361); }

    #[test]
    fn part2() { assert_eq!(day03::part2(INPUT), 467835); }
}