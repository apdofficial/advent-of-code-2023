use std::collections::HashSet;
use std::ops::Not;
use itertools::Itertools;

type Map = Vec<Vec<char>>;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
enum Direction { Left, Right, Up, Down }

impl Direction {
    fn is_valid_to(&self, c: char) -> bool {
        match c {
            '|' => *self == Direction::Up ||    *self == Direction::Down,
            '-' => *self == Direction::Left  || *self == Direction::Right,
            'L' => *self == Direction::Down ||  *self == Direction::Left,
            'J' => *self == Direction::Down ||  *self == Direction::Right,
            '7' => *self == Direction::Right || *self == Direction::Up,
            'F' => *self == Direction::Up ||    *self == Direction::Left,
            'S' => true,
            _ => false
        }
    }

    fn is_valid_from(&self, c: char) -> bool {
        match c {
            '|' => *self == Direction::Up ||    *self == Direction::Down,
            '-' => *self == Direction::Left  || *self == Direction::Right,
            'L' => *self == Direction::Up ||    *self == Direction::Right,
            'J' => *self == Direction::Up ||    *self == Direction::Left,
            '7' => *self == Direction::Left ||  *self == Direction::Down,
            'F' => *self == Direction::Down ||  *self == Direction::Right,
            'S' => true,
            _ => false
        }
    }
}

const DIRECTIONS: [(i64, i64, Direction); 4] = [
    (-1,  0,  Direction::Left),
    ( 1,  0,  Direction::Right),
    ( 0, -1,  Direction::Up),
    ( 0,  1,  Direction::Down)
];

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point { x: i64, y: i64 }

type Points = Vec<Point>;

impl Point {
    pub fn neighbours_in(&self, map: &Map) -> Points {
        let mut points: Vec<Point> = vec![];
        let c_curr = map[self.y as usize][self.x as usize];
        for &(d_x, d_y, direction) in DIRECTIONS.iter().filter(|(_, _, c)| c.is_valid_from(c_curr)).clone() {
            let y = self.y + d_y;
            if y.is_negative() || y >= map.len() as i64 { continue }
            let x = self.x + d_x;
            if x.is_negative() || x >= map.first().unwrap().len() as i64 { continue }
            if direction.is_valid_to(map[y as usize][x as usize]) {
                points.push(Point { x, y });
            }
        }
        points
    }
}

#[derive(Clone)]
struct Path {
    points: Vec<Point>
}

impl Path {

    pub fn new(start: Point) -> Self {
        Path { points: vec![start] }
    }

    pub fn is_start(&self, point: &Point) -> bool {
        self.points.first().unwrap().x == point.x && self.points.first().unwrap().y == point.y
    }

    pub fn push(&mut self, point: Point) {
        if self.is_cycle() { return; }
        if self.first().unwrap().x == point.x && self.first().unwrap().y == point.y && self.len() >= 3 {
            self.points.push(point)
        } else if !self.contains(&point) {
            self.points.push(point)
        }
    }

    pub fn is_cycle(&self) -> bool {
        self.first().is_some_and(|s| self.last().is_some_and(|e| s == e)) && self.len() > 1
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.points.contains(point)
    }

    pub fn last(&self) -> Option<&Point> {
        self.points.last()
    }

    pub fn first(&self) -> Option<&Point> {
        self.points.first()
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }
}

fn find_start(chars: &Map, start: char) -> Option<Point> {
    for (y, line) in chars.iter().enumerate() {
        if let Some((x, _)) = line.iter().find_position(|&&x| x == start) {
            return Some(Point { x: x as i64, y: y as i64 });
        }
    }
    return None;
}

fn find_all_paths_rec(map: &Map, paths: &mut Vec<Path>) {
    let neighbours = paths
        .last()
        .expect("Missing next path")
        .last()
        .expect("Missing next point in last path")
        .neighbours_in(&map)
        .into_iter()
        .collect_vec();
    let mut i = 0;
    for &next in neighbours.iter() {
        if paths.last().unwrap().contains(&next) { // already visited
            if paths.last().unwrap().is_start(&next) { // start (now the path becomes a cycle)
                paths.last_mut().unwrap().push(next);
            }
            continue;
        }
        if i > 0 {
            if let Some(&ref path) = paths.last() {
                paths.push(Path { points: path.points.iter().copied().collect_vec() });
            }
        }
        paths.last_mut().unwrap().push(next);
        if paths.last().expect("Missing next path").is_cycle() {
            return;
        }
        find_all_paths_rec(map, paths);
        i += 1;
    }
}

fn find_all_paths_from(map: &Map, start: char) -> Vec<Path> {
    let start = find_start(&map, start).expect("Could not find the the start.");
    let mut paths: Vec<Path> = vec![];
    paths.push(Path::new(start));
    find_all_paths_rec(map, &mut paths);
    paths
}

pub fn part1(input: &str) -> i64 {
    let chars = input
        .split("\n")
        .map(|x| x.chars().collect_vec()).filter(|x| !x.is_empty())
        .collect_vec();

    let Some(cycle) = find_all_paths_from(&chars, 'S')
        .into_iter()
        .filter(Path::is_cycle)
        .sorted_by(|a, b| Ord::cmp(&a.len(), &b.len()))
        .rev()
        .last() else { return -1; };

    return (cycle.len()  / 2) as i64
}

pub fn part2(input: &str) -> i64 {
    let map = input
        .split("\n")
        .map(|x| x.chars().collect_vec()).filter(|x| !x.is_empty())
        .collect_vec();

    let Some(cycle) = find_all_paths_from(&map, 'S')
        .into_iter()
        .filter(Path::is_cycle)
        .sorted_by(|a, b| Ord::cmp(&a.len(), &b.len()))
        .rev()
        .last() else { return -1; };

    let mut enclosed_tiles: HashSet<Point> = HashSet::new();

    for y in 0..map.len() {
        let mut is_inside = false;
        for x in 0..map[y].len() {
            let p = Point { x: x as i64, y: y as i64 };
            if cycle.contains(&p) {
                let mut c = map[y][x];
                if map[y][x] == 'S' {
                    // close the loop
                    c = if Direction::Left.is_valid_from(c) && Direction::Right.is_valid_from(c) { '-' }
                    else if Direction::Up.is_valid_from(c) && Direction::Down.is_valid_from(c) { '|' }
                    else if Direction::Left.is_valid_from(c) && Direction::Up.is_valid_from(c) { 'J' }
                    else if Direction::Left.is_valid_from(c) && Direction::Down.is_valid_from(c) { '7' }
                    else if Direction::Right.is_valid_from(c) && Direction::Up.is_valid_from(c) { 'L' }
                    else if Direction::Right.is_valid_from(c) && Direction::Down.is_valid_from(c) { 'F' }
                    else { '.' }
                }
                if c == '|' || c == 'L' || c == 'J' {
                    is_inside = is_inside.not();
                }
            } else  {
                if is_inside {
                    enclosed_tiles.insert(p);
                }
            }
        }
    }

    enclosed_tiles.len() as i64
}

#[cfg(test)]
mod tests {
    use crate::aoc::day10;

    const INPUT1: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const INPUT2: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    #[test]
    fn part1() { assert_eq!(day10::part1(INPUT1), 8); }

    #[test]
    fn part2() { assert_eq!(day10::part2(INPUT2), 4); }
}
