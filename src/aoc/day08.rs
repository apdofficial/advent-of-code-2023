use std::ops::Not;
use std::string::String;
use itertools::Itertools;
use regex::Regex;

type Map = std::collections::HashMap<String, (String, String)>;

#[derive(Debug)]
struct Network {
    instructions: Vec<char>,
    map: Map,
}

impl Network {
    pub fn from_str(input: &str) -> Option<Self> {
        let (instructions_str, map_str) = input.split_once("\n\n")?;
        let mut map: Map = std::collections::HashMap::new();
        map_str.split('\n').filter(|x| x.trim().is_empty().not()).for_each(|line| {
            let re = Regex::new(r#"(\w+)"#).unwrap();
            let mut it = re.find_iter(line).map(|m| m.as_str());
            map.insert(it.next().unwrap().into(), (it.next().unwrap().into(), it.next().unwrap().into()));
        });
        Some(Network { instructions: instructions_str.chars().collect_vec(), map })
    }

    pub fn count_steps(&self, start: String, end: fn(&str) -> bool) -> u64 {
        let mut current = (start.to_string(), self.map.get(&start).expect("Missing start node!").clone());
        let mut steps = 0;
        for direction in self.instructions.iter().cycle() {
            let next = match direction {
                'L' => current.1.0.clone(),
                'R' => current.1.1.clone(),
                _ => unreachable!()
            };
            current = (next.clone(), self.map.get(&next).expect("Missing node!").clone());
            steps += 1;
            if end(current.0.as_str()) { break; }
        }
        steps
    }
}

pub fn part1(input: &str) -> i64 {
    let Some(network) = Network::from_str(input) else { return 0; };
    network.count_steps("AAA".into(), |x| x == "ZZZ") as i64
}

pub fn part2(input: &str) -> i64 {
    let Some(network) = Network::from_str(input) else { return 0; };
    network.map
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|start| network.count_steps(start.into(), |end| end.ends_with('Z')))
        .fold(1, num::integer::lcm) as i64
}

#[cfg(test)]
mod tests {
    use crate::aoc::day08;

    const INPUT1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part1() { assert_eq!(day08::part1(INPUT1), 6); }

    #[test]
    fn part2() { assert_eq!(day08::part2(INPUT2), 6); }
}
