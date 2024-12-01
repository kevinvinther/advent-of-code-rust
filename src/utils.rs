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
