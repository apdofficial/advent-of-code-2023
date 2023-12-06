struct Race {
    duration: u64,
    distance: u64,
}

impl Race {
    pub fn winning_combinations_count(&self) -> u32 {
        (1..self.duration - 1)
            .filter(|x| (self.duration - x) * x > self.distance)
            .count() as u32
    }
}

pub fn part1(input: &str) -> u32 {
    let mut races: Vec<Race> = vec![];
    let Some((line1, line2)) = input.split_once("\n") else { return 0; };
    let Some(("Time", times)) = line1.split_once(":") else { return 0; };
    times.split_whitespace().for_each(|x| {
        races.push(Race {
            duration: x.parse::<u64>().unwrap_or(0),
            distance: 0,
        });
    });
    let Some(("Distance", distances))  = line2.split_once(":") else { return 0; };
    distances.split_whitespace().enumerate().for_each(|(i, x)| {
        races[i].distance = x.parse::<u64>().unwrap_or(0);
    });
    races.iter().map(Race::winning_combinations_count).product()
}

pub fn part2(input: &str) -> u32 {
    let Some((line1, line2)) = input.split_once("\n") else { return 0; };
    let duration = line1
        .split_once(":")
        .and_then(|(_, x)| x.replace(" ", "").parse::<u64>().ok()).unwrap_or(0);
    let distance = line2
        .split_once(":")
        .and_then(|(_, x)| x.replace(" ", "").replace("\n", "").parse::<u64>().ok()).unwrap_or(0);
    Race { duration, distance }.winning_combinations_count()
}

#[cfg(test)]
mod tests {
    use crate::aoc::day06;

    const INPUT: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn part1() { assert_eq!(day06::part1(INPUT), 288); }

    #[test]
    fn part2() { assert_eq!(day06::part2(INPUT), 71503); }
}
