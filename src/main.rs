use std::time::{Duration, Instant};
use clap::Parser;
use reqwest::header::COOKIE;
use tokio::main;
pub mod aoc;
use crate::aoc::common::PuzzleResult;
use aoc::{
    day01, day02, day03, day04, day05, day06,
    day07, day08, day09, day10, day11, day12,
    day13, day14, day15
};


#[derive(Parser, Debug)]
struct Args {
    /// Year for which to solve the Advent of Code
    #[arg(short, long, default_value_t = 2023)]
    year: u16,
    /// Day for which to solve the Advent of Code, 0 means solve all days
    #[arg(short, long, default_value_t = 0)]
    day: u8,
    /// Token to access your puzzle input
    #[arg(short, long)]
    token: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct DayResult {
    day: u8,
    part1: PuzzleResult,
    part2: PuzzleResult,
    runtime: Duration,
}

#[main]
async fn main() -> Result<(), String> {
    let args: Args = Args::parse();
    if args.year != 2023 {
        return Err(String::from("Invalid year"));
    }
    if args.day == 0 {
        // solve all implemented days
        for day in 1..=15 {
            let input_puzzle_result = fetch_input_data(args.year, day, &args.token).await;
            if input_puzzle_result.is_err() { return Err(input_puzzle_result.err().unwrap()); }
            let input_puzzle = input_puzzle_result.ok().unwrap();
            let result = solve_day(day, &input_puzzle)?;
            println!("{:?}", result);
        }
    } else {
        let input_puzzle_result = fetch_input_data(args.year, args.day, &args.token).await;
        if input_puzzle_result.is_err() { return Err(input_puzzle_result.err().unwrap()); }
        let input_puzzle = input_puzzle_result.ok().unwrap();
        let result = solve_day(args.day, &input_puzzle)?;
        println!("{:?}", result);
    }
    Ok(())
}

fn solve_day(day: u8, input_data: &str) -> Result<DayResult, String> {
    let now = Instant::now();
    match day {
        1 => Ok(DayResult {
            day,
            part1: PuzzleResult::Number(day01::part1(&input_data)),
            part2: PuzzleResult::Number(day01::part2(&input_data)),
            runtime: now.elapsed(),
        }),
        2 => Ok(DayResult {
            day,
            part1: PuzzleResult::Number(day02::part1(&input_data)),
            part2: PuzzleResult::Number(day02::part2(&input_data)),
            runtime: now.elapsed(),
        }),
        3 => Ok(DayResult {
            day,
            part1: PuzzleResult::Number(day03::part1(&input_data)),
            part2: PuzzleResult::Number(day03::part2(&input_data)),
            runtime: now.elapsed(),
        }),
        4 => Ok(DayResult {
            day,
            part1: PuzzleResult::Number(day04::part1(&input_data)),
            part2: PuzzleResult::Number(day04::part2(&input_data)),
            runtime: now.elapsed(),
        }),
        5 => Ok(DayResult {
            day,
            part1: PuzzleResult::Number(day05::part1(&input_data)),
            part2: PuzzleResult::Number(day05::part2(&input_data)),
            runtime: now.elapsed(),
        }),
        6 => Ok(DayResult {
            day,
            part1: PuzzleResult::Number(day06::part1(&input_data)),
            part2: PuzzleResult::Number(day06::part2(&input_data)),
            runtime: now.elapsed(),
        }),
        7 => Ok(DayResult {
            day,
            part1: PuzzleResult::Number(day07::part1(&input_data)),
            part2: PuzzleResult::Number(day07::part2(&input_data)),
            runtime: now.elapsed(),
        }),
        8 => Ok(DayResult {
            day,
            part1: PuzzleResult::Number(day08::part1(&input_data)),
            part2: PuzzleResult::Number(day08::part2(&input_data)),
            runtime: now.elapsed(),
        }),
        9 => Ok(DayResult {
            day,
            part1: PuzzleResult::Number(day09::part1(&input_data)),
            part2: PuzzleResult::Number(day09::part2(&input_data)),
            runtime: now.elapsed(),
        }),
        10 => Ok(DayResult {
            day,
            part1: PuzzleResult::Number(day10::part1(&input_data)),
            part2: PuzzleResult::Number(day10::part2(&input_data)),
            runtime: now.elapsed(),
        }),
        11 => Ok(DayResult {
            day,
            part1: PuzzleResult::Number(day11::part1(&input_data)),
            part2: PuzzleResult::Number(day11::part2(&input_data)),
            runtime: now.elapsed(),
        }),
        12 => Ok(DayResult {
            day,
            part1: PuzzleResult::Number(day12::part1(&input_data)),
            part2: PuzzleResult::Number(day12::part2(&input_data)),
            runtime: now.elapsed(),
        }),
        13 => Ok(DayResult {
            day,
            part1: PuzzleResult::Number(day13::part1(&input_data)),
            part2: PuzzleResult::Number(day13::part2(&input_data)),
            runtime: now.elapsed(),
        }),
        14 => Ok(DayResult {
            day,
            part1: PuzzleResult::Number(day14::part1(&input_data)),
            part2: PuzzleResult::Number(day14::part2(&input_data)),
            runtime: now.elapsed(),
        }),
        15 => Ok(DayResult {
            day,
            part1: PuzzleResult::Number(day15::part1(&input_data)),
            part2: PuzzleResult::Number(day15::part2(&input_data)),
            runtime: now.elapsed(),
        }),
        _ => Err(String::from("invalid day")),
    }
}

async fn fetch_input_data(year: u16, day: u8, session_token: &str) -> Result<String, String> {
    let input = reqwest::Client::new()
        .get(std::format!(
            "https://adventofcode.com/{year}/day/{day}/input"
        ))
        .header(COOKIE, format!("session={session_token}"))
        .send()
        .await
        .unwrap()
        .text()
        .await;
    if input.is_err() {
        return Err(format!(
            "failed to fetch the puzzle input text: {}",
            input.err().unwrap()
        ));
    }
    let input_text = input.ok().unwrap();
    if input_text == "404 Not Found\n" {
        return Err(input_text);
    }
    Ok(input_text)
}
