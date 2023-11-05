// Advent of Code 2022 in Rust
use std::env;
mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

const USAGE: &str = "\nNo argument given, which day's exercise to run.
Usage: e.g. `cargo run --1` for day 1\n";

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(day) = args.get(1) {
        match day.parse::<usize>() {
            Ok(1) => day1::run(),
            Ok(2) => day2::run(),
            Ok(3) => day3::run(),
            Ok(4) => day4::run(),
            Ok(5) => day5::run(),
            Ok(6) => day6::run(),

            _ => println!("{}", USAGE),
        }
    } else {
        println!("{}", USAGE);
    }
}
