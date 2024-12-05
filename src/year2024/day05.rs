use std::collections::HashMap;

#[derive(PartialEq)]
enum Part {
    Part1,
    Part2,
}

pub fn part1(input: &str) -> String {
    solve(Part::Part1, input)
}

fn solve(part: Part, input: &str) -> String {
    let mut part1 = 0;
    let mut part2 = 0;

    let input = input.split("\n\n").collect::<Vec<&str>>();

    let mut priorities: HashMap<i32, Vec<i32>> = HashMap::new();

    input[0].lines().for_each(|line| {
        let mut parts = line.split("|").map(|item| item.parse::<i32>().unwrap());
        let value = parts.next().unwrap();
        let key = parts.next().unwrap();

        priorities.entry(key).or_insert_with(Vec::new).push(value);
    });

    let lists: Vec<Vec<i32>> = input[1]
        .trim()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|item| item.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    for list in lists {
        let mut valid = true;
        for (i, x) in list.iter().enumerate() {
            for (j, y) in list.iter().enumerate() {
                if let Some(t) = priorities.get(x) {
                    if i < j && t.contains(y) {
                        valid = false;
                    }
                }
            }
        }
        if valid {
            part1 += list[list.len() / 2];
        } else {
            let mut sorted_list = list.clone();
            // Sort by asks "which should come first from two elements?"
            // Using Ordering::Less, Greater, Equal
            sorted_list.sort_by(|a, b| {
                priorities
                    .get(b) // Given `a` and `b`, look for what should come after `b`
                    .unwrap_or(&vec![])
                    .contains(a) // If `b` contains `a`, then `a` should come after `b`
                    .cmp(
                        // If this returns `true`, and the beforehand returns `false` we get Ordering::Less, indicating `a` should come before `b`
                        // If other way around, we get Ordering::Greater
                        // If both are true or false, we get equal. However, with the problem this shouldn't be the case.
                        &priorities.get(a).unwrap_or(&vec![]).contains(b),
                    )
            });

            part2 += sorted_list[sorted_list.len() / 2];
        }
    }

    if part == Part::Part1 {
        return part1.to_string();
    } else {
        return part2.to_string();
    }
}

pub fn part2(input: &str) -> String {
    solve(Part::Part2, input)
}
