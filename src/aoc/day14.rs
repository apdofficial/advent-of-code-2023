use std::collections::HashMap;
use crate::aoc::util::CharMap;

fn tilt_north(map: &mut CharMap) {
    for col in 0..map.width() {
        let mut empty_row: Option<usize> = None;
        for mut row in 0..map.height() {
            if map[row][col] == 'O' && empty_row.is_some() {
                map[row][col] = '.';
                map[empty_row.unwrap()][col] = 'O';
                row = empty_row.unwrap() + 1; // start again from +1 relative to last empty
                empty_row = None;
            }
            if map[row][col] == '#' && empty_row.is_some() {
                empty_row = None;
            }
            if map[row][col] == '.' && empty_row.is_none() {
                empty_row = Some(row);
            }
        }
    }
}

fn cycle(map: &mut CharMap) {
    tilt_north(map); // N
    map.rotate_clockwise();
    tilt_north(map); // W
    map.rotate_clockwise();
    tilt_north(map); // E
    map.rotate_clockwise();
    tilt_north(map); // S
    map.rotate_clockwise();
}

pub fn part1(input: &str) -> i64 {
    let Some(mut map) = CharMap::from_str(input) else { return 0 };
    tilt_north(&mut map);
    map.find_all(|&x| x == 'O').iter().map(|p| map.height() as i64 - p.y).sum()
}

pub fn part2(input: &str) -> i64 {
    let mut cache: HashMap<CharMap, usize> = HashMap::new();
    let Some(mut map) = CharMap::from_str(input) else { return 0 };
    for i in 0..1_000_000_000 {
        cycle(&mut map);
        if cache.contains_key(&map) { // detected cycle
            let Some(nth_cycle) = cache.get(&map) else { return 0; };
            let cycle_len = i - nth_cycle;
            let spins_left = (1_000_000_000 - 1 - nth_cycle) % cycle_len;
            for _ in 0..spins_left {
                cycle(&mut map); // compute the remaining cycles excluding the periods
            }
            break;
        }
        cache.insert(map.clone(), i);
    }
    map.find_all(|&x| x == 'O').iter().map(|p| map.height() as i64 - p.y).sum()
}

#[cfg(test)]
mod tests {
    use crate::aoc::day14;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn part1() { assert_eq!(day14::part1(INPUT), 136); }

    #[test]
    fn part2() { assert_eq!(day14::part2(INPUT), 64); }
}