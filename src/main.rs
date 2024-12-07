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

            let duration_part1 = benchmark_solution(*part1_fn, &input);
            println!("Part 1 result: {}", part1_fn(&input));

            let duration_part2 = benchmark_solution(*part2_fn, &input);
            println!("Part 2 result: {}", part2_fn(&input));
            println!("--- Benchmark Time ---");
            println!("Part 1 time: {:?}", duration_part1);
            println!("Part 2 time: {:?}", duration_part2);
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
    register_solutions!(
        solutions,
        year2023::day01,
        year2024::day01,
        year2024::day02,
        year2024::day03,
        year2024::day04,
        year2024::day05,
        year2024::day06,
        year2024::day07,
    );

    solutions
}

fn benchmark_solution(part_fn: fn(&str) -> String, input: &str) -> std::time::Duration {
    let start = std::time::Instant::now();
    let _ = part_fn(input);
    start.elapsed()
}
