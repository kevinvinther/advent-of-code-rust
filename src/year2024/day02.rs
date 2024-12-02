use crate::utils::*;

pub fn part1(input: &str) -> String {
    let input = parse_to_vec_of_vec(input);

    let res = input.iter().filter(|line| is_valid(line)).count();

    res.to_string()
}

fn is_valid(line: &[i32]) -> bool {
    if line.len() <= 1 {
        return true;
    }
    let direction = (line[1] - line[0]).signum();
    for i in 1..line.len() {
        let diff = line[i] - line[i - 1];
        if diff.abs() < 1 || diff.abs() > 3 || diff.signum() != direction {
            return false;
        }
    }
    true
}

pub fn part2(input: &str) -> String {
    let input = parse_to_vec_of_vec(input);

    let safe_count = input
        .iter()
        .filter(|line| is_valid(line) || is_valid_with_one_removal(line))
        .count();

    safe_count.to_string()
}

fn is_valid_with_one_removal(line: &[i32]) -> bool {
    for i in 0..line.len() {
        let mut modified = line.to_vec();
        modified.remove(i);
        if is_valid(&modified) {
            return true;
        }
    }
    false
}
