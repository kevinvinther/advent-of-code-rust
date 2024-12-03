use regex::Regex;

pub fn part1(input: &str) -> String {
    let re = Regex::new(r"mul\((?<d1>\d{1,3}),(?<d2>\d{1,3})\)+").unwrap();

    let sum: i32 = re
        .captures_iter(input)
        .map(|caps| {
            let digit1 = caps.name("d1").unwrap().as_str().parse::<i32>().unwrap();
            let digit2 = caps.name("d2").unwrap().as_str().parse::<i32>().unwrap();
            digit1 * digit2
        })
        .sum();

    sum.to_string()
}
pub fn part2(input: &str) -> String {
    let re = Regex::new(r"mul\((?<d1>\d{1,3}),(?<d2>\d{1,3})\)+|(?<do>don't\(\)|do\(\))").unwrap();

    let mut sum = 0;
    let mut cont = true;

    for caps in re.captures_iter(input) {
        if let Some(matched_action) = caps.name("do") {
            if matched_action.as_str() == "do()" {
                cont = true;
            } else {
                cont = false;
            }
        } else if let Some(d1) = caps.name("d1") {
            if cont {
                let digit1 = d1.as_str().parse::<i32>().unwrap();
                let digit2 = caps.name("d2").unwrap().as_str().parse::<i32>().unwrap();
                sum += digit1 * digit2;
            } // Otherwise, don't()
        }
    }

    sum.to_string()
}
