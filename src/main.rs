use tokio::main;
use clap::Parser;
use reqwest::header::COOKIE;
pub mod aoc;
use aoc::{common, day1};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    year: u16,
    #[arg(short, long)]
    day: u8,
    #[arg(short, long)]
    token: String
}
#[derive(Debug)]
struct DayResult {
    part1: common::PuzzleResult,
    part2: common::PuzzleResult
}

#[main]
async fn main() -> Result<(), String> {
    let cli : Args = Args::parse();
    let input_data = fetch_input_data(cli.year, cli.day, &cli.token).await.unwrap();
    println!("{}", input_data);

    if cli.year != 2022 {
        return Err(String::from("invalid year"));
    }

    let result = match cli.day {
        1 => Ok(DayResult {
            part1: day1::part1(&input_data),
            part2: day1::part2(&input_data)
        }),
        _ => Err(String::from("invalid day"))
    }?;

    println!("{:?}", result);

    Ok(())
}

async fn fetch_input_data(
    year: u16,
    day: u8,
    session_token: &str
) -> reqwest::Result<String> {
    let url = std::format!("https://adventofcode.com/{year}/day/{day}/input");
    reqwest::Client::new().get(url)
        .header(COOKIE, format!("session={session_token}"))
        .send()
        .await?
        .text()
        .await
}
