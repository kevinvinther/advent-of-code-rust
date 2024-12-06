use crate::utils::input_to_vec_vec_char;
use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn solve(input: &Vec<Vec<char>>, part2: bool) -> usize {
    let (mut row, mut col) = find_hat(&input, '^');
    let mut direction = Direction::Up; // Default direction is up
    let mut set: HashMap<(usize, usize), Vec<Direction>> = HashMap::new();

    loop {
        if part2 && set.contains_key(&(row, col)) {
            if set[&(row, col)].contains(&direction) {
                return 1;
            }
        }
        match direction {
            Direction::Up => {
                if input[row - 1][col] == '#' {
                    set.entry((row, col))
                        .or_insert(Vec::new())
                        .push(direction.clone());
                    direction = Direction::Right; // 90 Degrees
                } else {
                    set.entry((row, col))
                        .or_insert(Vec::new())
                        .push(direction.clone());
                    if row - 1 == 0 {
                        set.entry((row - 1, col))
                            .or_insert(Vec::new())
                            .push(direction.clone());
                        return set.len();
                    } else {
                        row -= 1;
                    }
                }
            }
            Direction::Down => {
                if input[row + 1][col] == '#' {
                    set.entry((row, col))
                        .or_insert(Vec::new())
                        .push(direction.clone());
                    direction = Direction::Left; // 90 Degrees
                } else {
                    set.entry((row, col))
                        .or_insert(Vec::new())
                        .push(direction.clone());
                    row += 1;
                    if row + 1 >= input.len() {
                        set.entry((row, col))
                            .or_insert(Vec::new())
                            .push(direction.clone());
                        // MAYBE FIX?
                        return set.len();
                    }
                }
            }
            Direction::Left => {
                if input[row][col - 1] == '#' {
                    set.entry((row, col))
                        .or_insert(Vec::new())
                        .push(direction.clone());
                    direction = Direction::Up; // 90 Degrees
                } else {
                    set.entry((row, col))
                        .or_insert(Vec::new())
                        .push(direction.clone());
                    if col - 1 == 0 {
                        set.entry((row, col - 1))
                            .or_insert(Vec::new())
                            .push(direction.clone());
                        return set.len();
                    } else {
                        col -= 1;
                    }
                }
            }
            Direction::Right => {
                if input[row][col + 1] == '#' {
                    set.entry((row, col))
                        .or_insert(Vec::new())
                        .push(direction.clone());
                    direction = Direction::Down; // 90 Degrees
                } else {
                    set.entry((row, col))
                        .or_insert(Vec::new())
                        .push(direction.clone());
                    col += 1;
                    if col + 1 >= input[0].len() {
                        set.entry((row, col + 1))
                            .or_insert(Vec::new())
                            .push(direction.clone());
                        return set.len();
                    }
                }
            }
        }
    }
}

pub fn part1(input: &str) -> String {
    let input = input_to_vec_vec_char(input);

    solve(&input, false).to_string()
}

fn find_hat(input: &Vec<Vec<char>>, hat: char) -> (usize, usize) {
    for (i, row) in input.into_iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            if *element == hat {
                return (i, j);
            }
        }
    }

    panic!("Could not find hat!");
}

pub fn part2(input: &str) -> String {
    let mut res = 0;
    let mut input = input_to_vec_vec_char(input);

    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if input[row][col] != '#' && input[row][col] != '^' {
                input[row][col] = '#';
                if solve(&input, true) == 1 {
                    res += 1;
                }
                input[row][col] = '.';
            }
        }
    }

    res.to_string()
}
