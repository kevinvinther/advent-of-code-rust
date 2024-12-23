/// Reads a file input, given year and day.
pub fn read_input(year: u16, day: u8) -> String {
    let path = format!("inputs/year{}/day{:02}.txt", year, day);
    std::fs::read_to_string(path).expect("Failed to read input file")
}

pub fn remove_whitespace_from_str(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn str_to_numbers(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .filter_map(|part| part.parse::<i32>().ok())
        .collect()
}

pub fn parse_to_vec_of_vec(s: &str) -> Vec<Vec<i32>> {
    s.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.trim()
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

pub fn input_to_vec_vec_char(s: &str) -> Vec<Vec<char>> {
    s.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect()
}
