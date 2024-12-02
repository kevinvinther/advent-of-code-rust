use advent_of_code_rust::year2024::day02;

#[test]
fn test_part1_example() {
    let input = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
    ";
    assert_eq!(day02::part1(input), "2");
}

#[test]
fn test_part2_example() {
    let input = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
    ";
    assert_eq!(day02::part2(input), "4");
}
