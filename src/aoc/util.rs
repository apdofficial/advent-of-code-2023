use std::ops::{Add, Sub};
use itertools::Itertools;
use num::abs;
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Point{
    pub(crate) x: i64,
    pub(crate) y: i64
}

#[allow(dead_code)]
impl Point {

    // https://en.wikipedia.org/wiki/Taxicab_geometry
    pub(crate)  fn manhattan(&self, other: &Point) -> i64 { abs(self.x - other.x) + abs(self.y - other.y) }

    pub(crate)  fn delta(&self, dx: i64, dy: i64) -> Point { Point { x: self.x + dx, y: self.y + dy } }

    pub(crate)  fn left(&self) -> Self { self.delta(-1, 0) }

    pub(crate)  fn right(&self) -> Self { self.delta(1, 0) }

    pub(crate)  fn up(&self) -> Self { self.delta(0, -1) }

    pub(crate)  fn down(&self) -> Self { self.delta(0, 1) }
}

#[allow(dead_code)]
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[allow(dead_code)]
impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) struct Path {
    points: Vec<Point>
}

#[allow(dead_code)]
impl Path {

    pub(crate)  fn new(start: Point) -> Self {
        Path { points: vec![start] }
    }

    pub(crate)  fn push(&mut self, point: Point) {
        if self.is_cycle() { return; }
        if self.first().unwrap().x == point.x && self.first().unwrap().y == point.y && self.len() >= 3 {
            self.points.push(point)
        } else if !self.contains(&point) {
            self.points.push(point)
        }
    }

    pub(crate)  fn is_cycle(&self) -> bool {
        self.first().is_some_and(|s| self.last().is_some_and(|e| s == e)) && self.len() > 1
    }

    pub(crate)  fn contains(&self, point: &Point) -> bool {
        self.points.contains(point)
    }

    pub(crate)  fn last(&self) -> Option<&Point> {
        self.points.last()
    }

    pub(crate)  fn first(&self) -> Option<&Point> {
        self.points.first()
    }

    pub(crate)  fn len(&self) -> usize {
        self.points.len()
    }
}

#[allow(dead_code)]
pub(crate) struct CharMap {
    map: Vec<Vec<char>>
}

#[allow(dead_code)]
impl CharMap {

    pub(crate)  fn from_str(input: &str, delimiter: char) -> Self {
        CharMap { map: input.split(delimiter).map(|x| x.chars().collect_vec()).collect() }
    }

    pub(crate) fn at(&self, point: &Point) -> Option<char> {
        if point.y >= self.width() as i64 { return None; }
        if point.x >= self.height() as i64 { return None; }
        Some(self.map[point.y as usize][point.x as usize])
    }

    pub(crate) fn width(&self) -> usize { self.map.len() }

    pub(crate) fn height(&self) -> usize { self.map.first().unwrap_or(&vec![]).len() }

    pub(crate) fn filter_rows(&self, pred: fn(&char) -> bool) -> Vec<i64> {
        self.map
            .iter()
            .enumerate()
            .filter(|(_, x)| x.iter().all(pred))
            .map(|(i, _)| i as i64)
            .collect_vec()
    }

    pub(crate) fn filter_cols(&self, pred: fn(&char) -> bool) -> Vec<i64> {
        let mut cols: Vec<i64> = vec![];
        for y in 0..self.map.len() {
            let mut is_valid = true;
            for x in 0..self.map[y].len() {
                if !pred(&self.map[x][y]) {
                    is_valid = false;
                    break;
                }
            }
            if is_valid { cols.push(y as i64); }
        }
        cols
    }

    pub(crate) fn find_all(&self, pred: fn(&char) -> bool) -> Vec<Point> {
        let mut points: Vec<Point>= vec![];
        for (y, line) in self.map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if pred(c) {
                    points.push(Point { x: x as i64, y: y as i64 });
                }
            }
        }
        points
    }
}