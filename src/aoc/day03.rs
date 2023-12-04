use std::ops::Range;
use lazy_static::lazy_static;
use regex::{Match, Regex};

lazy_static! {
    static ref NUMBER_RE: Regex = Regex::new(r"[0-9]+").unwrap();
    static ref GEAR_RE: Regex = Regex::new(r"[*]").unwrap();
}

pub fn part1(input: &str) -> u32 {
    let lines: Vec<_> = input.split('\n').collect();
    lines.iter().fold((0u32, 0usize), |(mut sum, line_nr), line| {
        NUMBER_RE.find_iter(line).for_each(|m| {
            if line_nr > 0 && any_neighbour_contains_symbol_at(lines[line_nr - 1], &m) || // above
                line_nr < lines.len() - 2 && any_neighbour_contains_symbol_at(lines[line_nr + 1], &m) || // below
                m.start().checked_sub(1).is_some_and(|x| line.chars().nth(x).is_some_and(is_symbol)) || // left
                line.chars().nth(m.end()).is_some_and(is_symbol) { // right
                sum += m.as_str().parse::<u32>().unwrap_or(0);
            }
        });
        (sum, line_nr + 1)
    }).0
}

pub fn part2(input: &str) -> u32 {
    let lines: Vec<_> = input.split('\n').collect();
    lines.iter().fold((0u32, 0usize), |(mut sum, line_nr), line| {
        GEAR_RE.find_iter(line).for_each(|gear| {
            let mut matches: Vec<Match> = vec![];
            if line_nr > 0 {
                NUMBER_RE.find_iter(lines[line_nr - 1])
                    .filter(|x| contains_gear(lines[line_nr], &gear, x))
                    .for_each(|x| matches.push(x));
            }
            NUMBER_RE.find_iter(lines[line_nr])
                .filter(|x| contains_gear(lines[line_nr], &gear, x))
                .for_each(|x| matches.push(x));
            if line_nr < lines.len() - 1 {
                NUMBER_RE.find_iter(lines[line_nr + 1])
                    .filter(|x| contains_gear(lines[line_nr], &gear, x))
                    .for_each(|x| matches.push(x));
            }
            if matches.len() == 2 {
                sum += matches.get(0).unwrap().as_str().parse::<u32>().unwrap_or(0) *
                    matches.get(1).unwrap().as_str().parse::<u32>().unwrap_or(0);
            }
        });
        (sum, line_nr + 1)
    }).0
}

fn contains_gear(line: &str, gear: &Match, candidate: &Match) -> bool {
    neighbours(line, candidate).any(|i| line.chars().nth(i).is_some_and(is_gear) && gear.start() == i)
}

fn is_gear(c: char) -> bool { c == '*' }

fn is_symbol(c: char) -> bool { !c.is_numeric() && c != '.' }

fn any_neighbour_contains_symbol_at(line: &str, candidate: &Match) -> bool {
    neighbours(line, candidate).any(|i| line.chars().nth(i).is_some_and(is_symbol))
}

fn neighbours(line: &str, m: &Match) -> Range<usize> {
    Range {
        start: m.start().checked_sub(1).unwrap_or(0),
        end: if m.end() < line.chars().count() - 2 { m.end() + 1 } else { m.range().end },
    }
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