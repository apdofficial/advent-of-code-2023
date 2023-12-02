use std::cmp;

pub fn part1(input: &str) -> u32 {
    input.split("\n").fold(0, |mut sum, line| {
        let Some((game, cubes)) = line.split_once(":") else { return sum; };
        if cubes.split(";").flat_map(|x| x.split(",")).all(|chunk| match chunk.trim().split_once(" ") {
                Some((c, "red")) => c.parse::<u32>().unwrap_or(99) <= 12,
                Some((c, "green")) => c.parse::<u32>().unwrap_or(99) <= 13,
                Some((c, "blue")) => c.parse::<u32>().unwrap_or(99) <= 14,
                _ => false
            }) {
            sum += game.trim().split_once(" ").and_then(|(_, b)| Some(b.parse::<u32>().unwrap_or(0))).unwrap_or(0);
        }
        sum
    })
}

pub fn part2(input: &str) -> u32 {
    input.split("\n").fold(0, |sum, line| {
        let Some((_, cubes)) = line.split_once(":") else { return sum; };
        let mut min = (0u32, 0u32, 0u32);
        cubes.split(";").flat_map(|x| x.split(",")).for_each(|chunk| match chunk.trim().split_once(" ") {
            Some((c, "red")) => min.0 = cmp::max(min.0, c.parse::<u32>().unwrap_or(0)),
            Some((c, "green")) => min.1 = cmp::max(min.1, c.parse::<u32>().unwrap_or(0)),
            Some((c, "blue")) => min.2 = cmp::max(min.2, c.parse::<u32>().unwrap_or(0)),
            _ => ()
        });
        sum + (min.0 * min.1 * min.2)
    })
}

#[cfg(test)]
mod tests {
    use crate::aoc::day02;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n
                         Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n
                         Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n
                         Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n
                         Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1() { assert_eq!(day02::part1(INPUT), 8); }

    #[test]
    fn part2() { assert_eq!(day02::part2(INPUT), 2286); }
}