struct Game {
    id: u64,
    red: Vec<u64>,
    green: Vec<u64>,
    blue: Vec<u64>,
}

impl Game {
    pub fn new(line: &str) -> Option<Self> {
        let (game, cubes) = line.split_once(":")?;
        let id = game.trim().split_once(" ")?.1.parse::<u64>().ok()?;
        let mut game = Game { id, red: vec![], green: vec![], blue: vec![] };
        cubes.split(";").flat_map(|x| x.split(",")).for_each(|chunk| {
            match chunk.trim().split_once(" ") {
                Some((c, "red")) => game.red.push(c.parse::<u64>().unwrap_or(0)),
                Some((c, "green")) => game.green.push(c.parse::<u64>().unwrap_or(0)),
                Some((c, "blue")) => game.blue.push(c.parse::<u64>().unwrap_or(0)),
                _ => {}
            };
        });
        Some(game)
    }
}

pub fn part1(input: &str) -> i64 {
    input.split("\n").filter_map(Game::new)
        .filter(|g| g.red.iter().all(|&x| x <= 12) && g.green.iter().all(|&x| x <= 13) && g.blue.iter().all(|&x| x <= 14))
        .map(|g| g.id)
        .sum::<u64>() as i64
}

pub fn part2(input: &str) -> i64 {
    input.split("\n").filter_map(Game::new)
        .map(|g| g.red.iter().max().unwrap_or(&0) * g.green.iter().max().unwrap_or(&0) * g.blue.iter().max().unwrap_or(&0))
        .sum::<u64>() as i64
}

#[cfg(test)]
mod tests {
    use crate::aoc::day02;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1() { assert_eq!(day02::part1(INPUT), 8); }

    #[test]
    fn part2() { assert_eq!(day02::part2(INPUT), 2286); }
}