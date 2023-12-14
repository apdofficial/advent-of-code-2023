use crate::aoc::util::{CharMap, count_diff};

fn summarize_pattern_notes(map: CharMap, pred: fn(usize) -> bool) -> usize {
    for i in 0..map.width() - 1 {
        let mut diffs = 0;
        let mut c0 = i as i64;
        let mut c1 = i + 1;
        while c0 >= 0 && c1 < map.width() {
            diffs += count_diff(&map.col_at(c0 as usize), &map.col_at(c1));
            c0 -= 1;
            c1 += 1;
        }
        if pred(diffs) { return i + 1; }
    }
    for i in 0..map.height() - 1 {
        let mut diffs = 0;
        let mut r0 = i as i64;
        let mut r1 = i + 1;
        while r0 >= 0 && r1 < map.height() {
            diffs += count_diff(&map[r0 as usize], &map[r1]);
            r0 -= 1;
            r1 += 1;
        }
        if pred(diffs) { return 100 * (i + 1); }
    }
    0
}

pub fn part1(input: &str) -> i64 {
    input
        .split("\n\n")
        .filter_map(CharMap::from_str)
        .map(|map|summarize_pattern_notes(map, |diff| diff == 0))
        .sum::<usize>() as i64
}

pub fn part2(input: &str) -> i64 {
    input
        .split("\n\n")
        .filter_map(CharMap::from_str)
        .map(|map|summarize_pattern_notes(map, |diff| diff == 1))
        .sum::<usize>() as i64
}

#[cfg(test)]
mod tests {
    use crate::aoc::day13;

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part1() { assert_eq!(day13::part1(INPUT), 405); }

    #[test]
    fn part2() { assert_eq!(day13::part2(INPUT), 400); }
}