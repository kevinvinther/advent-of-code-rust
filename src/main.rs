use advent_of_code_rust as aoc;
use aoc::*;
use clap::Parser;
use std::collections::HashMap;

/// CLI arguments parser
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The year of the challenge (e.g., 2023)
    year: u16,

    /// The day of the challenge (e.g., 1)
    day: u8,
}

fn main() {
    // Parse arguments using clap
    let args = Cli::parse();

    // Register all solutions
    let solutions = register_all_solutions();

    // Convert year and day to strings for key
    let year = args.year.to_string();
    let day = format!("{:02}", args.day);
    let key = (year.as_str(), day.as_str());

    match solutions.get(&key) {
        Some((part1_fn, part2_fn)) => {
            let input = aoc::utils::read_input(args.year, args.day);

            // Run and print results for both parts
            println!("Part 1: {}", part1_fn(&input));
            println!("Part 2: {}", part2_fn(&input));
        }
        None => eprintln!(
            "Solutions for year {} day {} not yet implemented. :(",
            year, day
        ),
    }
}

/// Registers all solutions for both parts into a HashMap.
fn register_all_solutions(
) -> HashMap<(&'static str, &'static str), (fn(&str) -> String, fn(&str) -> String)> {
    let mut solutions: HashMap<(&str, &str), (fn(&str) -> String, fn(&str) -> String)> =
        HashMap::new();

    // Use the macro to register solutions
    register_solutions!(solutions, year2023::day01, year2024::day01, year2024::day02,);

    solutions
}
