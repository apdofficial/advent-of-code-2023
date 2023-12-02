use clap::Parser;
use reqwest::header::COOKIE;
use tokio::main;
pub mod aoc;
use aoc::{day01, day02};
use crate::aoc::common::PuzzleResult;

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
}

#[main]
async fn main() -> Result<(), String> {
    let args: Args = Args::parse();
    if args.year != 2023 {
        return Err(String::from("Invalid year"));
    }

    if args.day == 0 {
        // solve all implemented days
        for day in 1..=2 {
            let input_puzzle_result = fetch_input_data(args.year, day, &args.token).await;
            if input_puzzle_result.is_err() {
                return Err(input_puzzle_result.err().unwrap());
            }
            let input_puzzle = input_puzzle_result.ok().unwrap();
            let result = solve_day(day, &input_puzzle)?;
            println!("{:?}", result);
        }
    } else {
        let input_puzzle_result = fetch_input_data(args.year, args.day, &args.token).await;
        if input_puzzle_result.is_err() {
            return Err(input_puzzle_result.err().unwrap());
        }
        let input_puzzle = input_puzzle_result.ok().unwrap();
        let result = solve_day(args.day, &input_puzzle)?;
        println!("{:?}", result);
    }

    Ok(())
}

fn solve_day(day: u8, input_data: &str) -> Result<DayResult, String> {
    match day {
        1 => Ok(DayResult {
            day,
            part1: PuzzleResult::Number(day01::part1(&input_data)),
            part2: PuzzleResult::Number(day01::part2(&input_data)),
        }),
        2 => Ok(DayResult {
            day,
            part1: PuzzleResult::Number(day02::part1(&input_data)),
            part2: PuzzleResult::Number(day02::part2(&input_data)),
        }),
        _ => Err(String::from("invalid day")),
    }
}

async fn fetch_input_data(year: u16, day: u8, session_token: &str) -> Result<String, String> {
    let response = reqwest::Client::new()
        .get(std::format!(
            "https://adventofcode.com/{year}/day/{day}/input"
        ))
        .header(COOKIE, format!("session={session_token}"))
        .send()
        .await;
    if response.is_err() {
        return Err(format!(
            "failed to fetch the puzzle input text: {}",
            response.err().unwrap()
        ));
    }
    let input = response.ok().unwrap().text().await;
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
