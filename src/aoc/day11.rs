use itertools::Itertools;
use num::abs;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new_with_expansion(&self, empty_rows: &Vec<i64>, empty_cols: &Vec<i64>, factor: i64) -> Self {
        let mut new = self.clone();
        (0..self.x)
            .filter(|x| empty_cols.contains(&x))
            .for_each(|_| new.x += factor);
        (0..self.y)
            .filter(|y| empty_rows.contains(&y))
            .for_each(|_| new.y += factor);
        new.clone()
    }

    // https://en.wikipedia.org/wiki/Taxicab_geometry
    pub fn manhattan_distance(&self, other: &Point) -> i64 {
        abs(self.x - other.x) + abs(self.y - other.y)
    }
}

type Matrix2<T> = Vec<Vec<T>>;

fn filter_rows(map: &Matrix2<char>, pred: fn(&char) -> bool) -> Vec<i64> {
    map
        .iter()
        .enumerate()
        .filter(|(_, x)| x.iter().all(pred))
        .map(|(i, _)| i as i64)
        .collect_vec()
}

fn filter_cols(map: &Matrix2<char>, pred: fn(&char) -> bool) -> Vec<i64> {
    let mut cols: Vec<i64> = vec![];
    for y in 0..map.len() {
        let mut is_valid = true;
        for x in 0..map[y].len() {
            if !pred(&map[x][y]) {
                is_valid = false;
                break;
            }
        }
        if is_valid { cols.push(y as i64); }
    }
    cols
}

fn find_all(chars: &Matrix2<char>, pred: fn(&char) -> bool) -> Vec<Point> {
    let mut points: Vec<Point>= vec![];
    for (y, line) in chars.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if pred(c) {
                points.push(Point { x: x as i64, y: y as i64 });
            }
        }
    }
    points
}

pub fn part1(input: &str) -> i64 {
    let chars = input.split("\n").map(|line| line.chars().collect_vec()).collect();
    let empty_rows = filter_rows(&chars, |&x| x == '.');
    let empty_cols = filter_cols(&chars, |&x| x == '.');
    let galaxies = find_all(&chars, |&x| x == '#')
        .iter()
        .map(|x| x.new_with_expansion(&empty_rows, &empty_cols, 1))
        .collect_vec();
    (0..galaxies.len()).fold(0, |acc1, i| {
        acc1 + (i+1..galaxies.len()).fold(0, |acc2, j| {
            acc2 + galaxies[i].manhattan_distance(&galaxies[j])
        })
    })
}



pub fn part2(input: &str) -> i64 {
    let chars = input.split("\n").map(|line| line.chars().collect_vec()).collect();
    let empty_rows = filter_rows(&chars, |&x| x == '.');
    let empty_cols = filter_cols(&chars, |&x| x == '.');
    let galaxies = find_all(&chars, |&x| x == '#')
        .iter()
        .map(|x| x.new_with_expansion(&empty_rows, &empty_cols, 999999))
        .collect_vec();
    (0..galaxies.len()).fold(0, |acc1, i| {
        acc1 + (i+1..galaxies.len()).fold(0, |acc2, j| {
            acc2 + galaxies[i].manhattan_distance(&galaxies[j])
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
    fn part2() { assert_eq!(day11::part2(INPUT), 8200210); }
}
