use crate::utils::*;
use std::cmp;
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

    let mut i = 0;

    let mut res = 0;

    while i < vec[0].len() {
        res += cmp::max(vec[0][i], vec[1][i]) - cmp::min(vec[0][i], vec[1][i]);
        i += 1
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

    // Populate the hashmap
    let mut i = 1;
    while i < numbers.len() {
        *right.entry(numbers[i]).or_insert(0) += 1;
        i += 2;
    }

    // Notice 0
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
