use advent_of_code_rust::year2024::day01;

#[test]
fn test_part1_example() {
    let input = "\
3   4
4   3
2   5
1   3
3   9
3   3
    ";
    assert_eq!(day01::part1(input), "11");
}

#[test]
fn test_part2_example() {
    let input = "\
3   4
4   3
2   5
1   3
3   9
3   3
    ";
    assert_eq!(day01::part2(input), "31");
}
