use crate::utils::input_to_vec_vec_char;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn find_pairs(input: &Vec<Vec<char>>) -> HashMap<char, Vec<(i32, i32)>> {
    let mut chars: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if input[row][col].is_alphanumeric() {
                let row: i32 = match row.try_into() {
                    Ok(v) => v,
                    Err(_) => panic!("Row value {} too large for i32", row),
                };
                let col: i32 = match col.try_into() {
                    Ok(v) => v,
                    Err(_) => panic!("col value {} too large for i32", col),
                };
                chars
                    .entry(input[row as usize][col as usize])
                    .or_insert(Vec::new())
                    .push((row, col));
            }
        }
    }

    chars
}

pub fn part1(input: &str) -> String {
    let input: Vec<Vec<char>> = input_to_vec_vec_char(input);
    let alphamap = find_pairs(&input);
    let top = input.len() as i32;
    let right = input[0].len() as i32;

    let mut pairs: Vec<(char, ((i32, i32), (i32, i32)))> = Vec::new();
    let mut initial_positions = HashSet::new();

    for (_c, alpha) in &alphamap {
        for &coord in alpha {
            initial_positions.insert(coord);
        }
    }

    for (c, alpha) in alphamap {
        for combo in alpha.iter().combinations(2) {
            let p1 = *combo[0];
            let p2 = *combo[1];
            pairs.push((c, (p1, p2)));
        }
    }

    let mut antinodes = HashSet::new();

    for (_, (p1, p2)) in pairs {
        let a1 = antinode(p1, p2);
        let a2 = antinode(p2, p1);

        // p1.0 = row, p1.1 = col
        if a1.0 >= 0 && a1.0 < top && a1.1 >= 0 && a1.1 < right {
            antinodes.insert(a1);
        }
        if a2.0 >= 0 && a2.0 < top && a2.1 >= 0 && a2.1 < right {
            antinodes.insert(a2);
        }
    }

    antinodes.len().to_string()
}

fn antinode(p1: (i32, i32), p2: (i32, i32)) -> (i32, i32) {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    (x2 + (x2 - x1), y2 + (y2 - y1))
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut x = a.abs();
    let mut y = b.abs();
    while y != 0 {
        let temp = x % y;
        x = y;
        y = temp;
    }
    x
}

pub fn part2(input: &str) -> String {
    let input: Vec<Vec<char>> = input_to_vec_vec_char(input);
    let antenna_positions = find_pairs(&input);

    let max_rows = input.len() as i32;
    let max_cols = input[0].len() as i32;

    let mut antinodes = HashSet::new();

    for (_freq, antennas) in antenna_positions.iter() {
        if antennas.len() < 2 {
            continue;
        }

        for combo in antennas.iter().combinations(2) {
            let (row1, col1) = *combo[0];
            let (row2, col2) = *combo[1];

            let delta_row = row2 - row1;
            let delta_col = col2 - col1;
            let gcd_val = gcd(delta_row, delta_col);

            let step_row = delta_row / gcd_val;
            let step_col = delta_col / gcd_val;

            let mut t = 0;
            loop {
                let current_row = row1 + t * step_row;
                let current_col = col1 + t * step_col;
                if current_row < 0
                    || current_row >= max_rows
                    || current_col < 0
                    || current_col >= max_cols
                {
                    break;
                }
                antinodes.insert((current_row, current_col));
                t += 1;
            }

            t = -1;
            loop {
                let current_row = row1 + t * step_row;
                let current_col = col1 + t * step_col;
                if current_row < 0
                    || current_row >= max_rows
                    || current_col < 0
                    || current_col >= max_cols
                {
                    break;
                }
                antinodes.insert((current_row, current_col));
                t -= 1;
            }
        }
    }

    antinodes.len().to_string()
}
