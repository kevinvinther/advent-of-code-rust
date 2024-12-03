use advent_of_code_rust::year2024::day03;

#[test]
fn test_part1_example() {
    let input = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    ";
    assert_eq!(day03::part1(input), "161");
}

#[test]
fn test_part2_example() {
    let input = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    ";
    assert_eq!(day03::part2(input), "48");
}
