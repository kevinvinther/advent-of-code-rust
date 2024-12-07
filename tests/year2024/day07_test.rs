use advent_of_code_rust::year2024::day07;

#[test]
fn test_part1_example() {
    let input = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    assert_eq!(day07::part1(input), "3749");
}

// #[test]
// fn test_part2_example() {
//     let input = "\
//     ";
//     assert_eq!(day0x::part2(input), "xx");
// }
