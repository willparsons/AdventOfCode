use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input.lines()
        .map(|l| {
            let s = l.split("|")
                .map(|s| s.trim())
                .collect::<Vec<_>>();
            (s[0], s[1])
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;

    let segments = HashMap::from([
        (2, 1),
        (4, 4),
        (3, 7),
        (7, 8),
    ]);

    for (_, output) in parse_input(input) {
        let digits: Vec<&str> = output.split(" ").collect();
        for digit in digits {
            if let Some(_) = segments.get(&(digit.len() as i32)) {
                sum += 1;
            }
        }
    }

    sum
}

fn main() {
    let data = include_str!("../input.txt");
    println!("{}", part1(data));
}
