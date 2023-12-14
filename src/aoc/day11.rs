use itertools::Itertools;
use super::util::{CharMap, Point};

impl Point {
    pub fn new_with_expansion(&self, empty_rows: &Vec<i64>, empty_cols: &Vec<i64>, factor: i64) -> Self {
        let mut new = self.clone();
        (0..self.x)
            .filter(|x| empty_cols.contains(&x))
            .for_each(|_| new.x += factor);
        (0..self.y)
            .filter(|y| empty_rows.contains(&y))
            .for_each(|_| new.y += factor);
        new
    }
}

pub fn part1(input: &str) -> i64 {
    let Some(map) = CharMap::from_str(input) else { return 0; };
    let empty_rows = map.filter_rows(|&x| x == '.');
    let empty_cols = map.filter_cols(|&x| x == '.');
    let galaxies = map.find_all(|&x| x == '#')
        .iter()
        .map(|x| x.new_with_expansion(&empty_rows, &empty_cols, 1))
        .collect_vec();
    (0..galaxies.len()).fold(0, |acc1, i| {
        acc1 + (i+1..galaxies.len()).fold(0, |acc2, j| {
            acc2 + galaxies[i].manhattan(&galaxies[j])
        })
    })
}

pub fn part2(input: &str) -> i64 {
    let Some(map) = CharMap::from_str(input) else { return 0; };
    let empty_rows = map.filter_rows(|&x| x == '.');
    let empty_cols = map.filter_cols(|&x| x == '.');
    let galaxies = map.find_all(|&x| x == '#')
        .iter()
        .map(|x| x.new_with_expansion(&empty_rows, &empty_cols, 999999))
        .collect_vec();
    (0..galaxies.len()).fold(0, |acc1, i| {
        acc1 + (i+1..galaxies.len()).fold(0, |acc2, j| {
            acc2 + galaxies[i].manhattan(&galaxies[j])
        })
    })
}

#[cfg(test)]
mod tests {
    use crate::aoc::day11;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part1() { assert_eq!(day11::part1(INPUT), 374); }

    #[test]
    fn part2() { assert_eq!(day11::part2(INPUT), 82000210); }
}
