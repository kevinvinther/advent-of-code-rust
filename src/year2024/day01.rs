use crate::utils::*;
use std::collections::HashMap;

/// Part 1
/// Given a list of two integers, find the difference and add it up
///
/// Running time: O(n log n)
pub fn part1(input: &str) -> String {
    let numbers = str_to_numbers(input);
    let mut vec: [Vec<i32>; 2] = [Vec::new(), Vec::new()];
    let mut i = 0;

    while i < numbers.len() {
        vec[0].push(numbers[i]);
        vec[1].push(numbers[i + 1]);

        i += 2;
    }

    vec[0].sort();
    vec[1].sort();

    let mut res = 0;
    for i in 0..vec[0].len() {
        res += (vec[0][i] - vec[1][i]).abs();
    }

    res.to_string()
}

/// Part 2
/// Given a list of two integers, organize such that it is two lists, left and right.
/// For each elemtn in the left list, multiply its value with the number of occurences
/// of this element in the right list.
///
/// Running time: O(n)
pub fn part2(input: &str) -> String {
    let numbers: Vec<i32> = str_to_numbers(input);
    let mut right: HashMap<i32, i32> = HashMap::new();
    // 1 for odd indexing
    let mut i = 1;

    // Populate the hashmap
    while i < numbers.len() {
        *right.entry(numbers[i]).or_insert(0) += 1;
        i += 2;
    }

    // 0 for even indexing
    let mut j = 0;
    let mut res = 0;
    while j < numbers.len() {
        if let Some(&count) = right.get(&numbers[j]) {
            res += numbers[j] * count;
        }
        j += 2;
    }

    res.to_string()
}
