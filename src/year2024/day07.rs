use rayon::prelude::*;

enum Operation {
    Addition,
    Multiplication,
    Concatenation,
}

fn solve(
    target: u64,
    nums: &Vec<u64>,
    operation: Operation,
    mut running_sum: u64,
    part_two: bool,
) -> bool {
    if nums.len() == 0 {
        return target == running_sum;
    }

    match operation {
        Operation::Multiplication => {
            running_sum = running_sum * nums[0];
        }
        Operation::Addition => {
            running_sum = running_sum + nums[0];
        }
        Operation::Concatenation => {
            let mut string_sum = running_sum.to_string();
            string_sum.push_str(&nums[0].to_string());

            running_sum = string_sum.parse().unwrap();
        }
    }
    let mut nums = nums.clone();
    nums.remove(0);

    if !part_two {
        return solve(target, &nums, Operation::Addition, running_sum, false)
            || solve(target, &nums, Operation::Multiplication, running_sum, false);
    } else {
        return solve(target, &nums, Operation::Addition, running_sum, true)
            || solve(target, &nums, Operation::Multiplication, running_sum, true)
            || solve(target, &nums, Operation::Concatenation, running_sum, true);
    }
}

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    let mut res: Vec<(u64, Vec<u64>)> = Vec::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split(":").collect();
        let target: u64 = split[0].parse().unwrap();
        let nums: Vec<u64> = split[1]
            .split_ascii_whitespace()
            .map(|value| value.parse().unwrap())
            .collect();

        res.push((target, nums));
    }

    res
}

pub fn part1(input: &str) -> String {
    let equations: Vec<(u64, Vec<u64>)> = parse(&input);
    let res: u64 = (0..equations.len() as u64)
        .into_par_iter()
        .map(|state| -> u64 {
            let (target, mut nums) = equations.clone()[state as usize].clone();
            let remove = nums.remove(0);
            let first = remove;

            if solve(target, &nums, Operation::Addition, first, false)
                || solve(target, &nums, Operation::Multiplication, first, false)
            {
                return target.clone();
            } else {
                return 0;
            }
        })
        .sum();

    res.to_string()
}
pub fn part2(input: &str) -> String {
    let equations: Vec<(u64, Vec<u64>)> = parse(&input);
    let res: u64 = (0..equations.len() as u64)
        .into_par_iter()
        .map(|state| -> u64 {
            let (target, mut nums) = equations.clone()[state as usize].clone();
            let remove = nums.remove(0);
            let first = remove;

            if solve(target, &nums, Operation::Addition, first, true)
                || solve(target, &nums, Operation::Multiplication, first, true)
                || solve(target, &nums, Operation::Concatenation, first, true)
            {
                return target.clone();
            } else {
                return 0;
            }
        })
        .sum();

    res.to_string()
}
