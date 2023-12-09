use std::ops::{Not, Range};
use rayon::prelude::*;

type RangeValue = f64;
type SeedValue = u64;

struct MapEntry {
    source: Range<RangeValue>,
    destination: Range<RangeValue>,
}

impl MapEntry {
    pub fn new(line: &str) -> Option<Self> {
        let mut items = line.trim().splitn(3, " ");
        let destination = items.next()?.parse::<SeedValue>().ok()?;
        let source = items.next()?.parse::<SeedValue>().ok()?;
        let length = items.next()?.parse::<SeedValue>().ok()?;
        Some(MapEntry {
            source: Range { start: source as RangeValue, end: (source + length ) as RangeValue },
            destination: Range { start: destination as RangeValue, end: (destination + length ) as RangeValue },
        })
    }

    pub fn contains(&self, input: RangeValue) -> bool {
        self.source.contains(&input)
    }

    pub fn convert(&self, x: RangeValue) -> RangeValue {
        if self.contains(x).not() {
            x
        } else {
            self.destination.start + (x - self.source.start) * (self.destination.end - self.destination.start) / (self.source.end - self.source.start)
        }
    }
}

type MapEntries = Vec<MapEntry>;

struct Map {
    entries: MapEntries
}

impl Map {
    pub fn new() -> Self { Map { entries: vec![] } }

    pub fn append(&mut self, line: &str) {
        if let Some(entry) = MapEntry::new(line) { self.entries.push(entry) }
    }

    pub fn convert(&self, input: RangeValue) -> RangeValue {
        for entry in self.entries.iter() {
            if entry.contains(input) { return entry.convert(input); }
        }
        input
    }
}

struct Alamac {
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map
}

impl Alamac {
    pub fn new(input: &str) -> Self {
        let mut alamac = Alamac {
            seed_to_soil: Map::new(),
            soil_to_fertilizer: Map::new(),
            fertilizer_to_water: Map::new(),
            water_to_light: Map::new(),
            light_to_temperature: Map::new(),
            temperature_to_humidity: Map::new(),
            humidity_to_location: Map::new(),
        };
        for section in input.split("\n\n") {
            match section.split_once(":") {
                Some(("seed-to-soil map", data)) =>
                    data.trim().split('\n').for_each(|x| { alamac.seed_to_soil.append(x) }),
                Some(("soil-to-fertilizer map", data)) =>
                    data.trim().split('\n').for_each(|x| alamac.soil_to_fertilizer.append(x)),
                Some(("fertilizer-to-water map", data)) =>
                    data.trim().split('\n').for_each(|x| alamac.fertilizer_to_water.append(x)),
                Some(("water-to-light map", data)) =>
                    data.trim().split('\n').for_each(|x| alamac.water_to_light.append(x)),
                Some(("light-to-temperature map", data)) =>
                    data.trim().split('\n').for_each(|x| alamac.light_to_temperature.append(x)),
                Some(("temperature-to-humidity map", data)) =>
                    data.trim().split('\n').for_each(|x| alamac.temperature_to_humidity.append(x)),
                Some(("humidity-to-location map", data)) =>
                    data.trim().split('\n').for_each(|x| alamac.humidity_to_location.append(x)),
                _ => {}
            }
        }
        alamac
    }

    pub fn get_location(&self, seed: SeedValue) -> SeedValue {
        let soil = self.seed_to_soil.convert(seed as RangeValue);
        let fertilizer = self.soil_to_fertilizer.convert(soil);
        let water = self.fertilizer_to_water.convert(fertilizer);
        let light = self.water_to_light.convert(water);
        let temperature = self.light_to_temperature.convert(light);
        let humidity = self.temperature_to_humidity.convert(temperature);
        self.humidity_to_location.convert(humidity) as SeedValue
    }
}

struct ValueSeeds {
    seeds: Vec<SeedValue>
}

impl ValueSeeds {
    pub fn new(input: &str) -> Self {
        let mut seeds = ValueSeeds { seeds: vec![] };
        input.split_once("\n\n").map(|(section, _)| {
            if let Some(("seeds", data)) = section.split_once(":") {
                data.trim().split_whitespace().filter_map(|x| x.parse::<SeedValue>().ok()).for_each(|x| seeds.seeds.push(x))
            }
        });
        seeds
    }
}

struct RangeSeeds {
    seeds: Vec<Range<SeedValue>>
}

impl RangeSeeds {
    pub fn new(input: &str) -> Self {
        let mut seeds = RangeSeeds { seeds: vec![] };
        input.split_once("\n\n").map(|(section, _)| {
            if let Some(("seeds", data)) = section.split_once(":") {
                let mut iter = data.trim().split_whitespace();
                let mut start_str = iter.next();
                let mut length_str = iter.next();
                while start_str.is_some() && length_str.is_some() {
                    let start = start_str.unwrap().parse::<SeedValue>().unwrap_or(0);
                    let length = length_str.unwrap().parse::<SeedValue>().unwrap_or(0);
                    seeds.seeds.push(Range { start, end: start + length });
                    start_str = iter.next();
                    length_str = iter.next();
                }
            }
        });
        seeds
    }
}

pub fn part1(input: &str) -> i64 {
    let seeds = ValueSeeds::new(input);
    let alamac = Alamac::new(input);
    seeds.seeds.into_par_iter()
        .map(|x| alamac.get_location(x))
        .min()
        .unwrap_or(0) as i64
}

pub fn part2(input: &str) -> i64 {
    let seeds = RangeSeeds::new(input);
    let alamac = Alamac::new(input);
    seeds.seeds.into_par_iter()
        .flat_map(|range| range.clone().into_par_iter().map(|i| alamac.get_location(i)))
        .min()
        .unwrap_or(0) as i64
}

#[cfg(test)]
mod tests {
    use crate::aoc::day05;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part1() { assert_eq!(day05::part1(INPUT), 35); }

    #[test]
    fn part2() { assert_eq!(day05::part2(INPUT), 46); } // too high 6082853
}