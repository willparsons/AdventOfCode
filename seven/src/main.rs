use std::cmp::min;
use std::fs;

fn _calc(crab: isize, target: isize) -> usize {
    let mut sum = 0;

    let diff = (crab - target).abs();

    for i in 1..=diff {
        sum += i;
    }

    sum as usize
}

fn part1(crabs: Vec<usize>) -> usize {
    let mut fuel_usage = 0;
    let pos = crabs[crabs.len()/2];

    for crab in crabs {
        fuel_usage += (crab as isize - pos as isize).abs() as usize;
    }

    fuel_usage
}

fn part2(crabs: Vec<usize>) -> usize {
    let min_point = *crabs.iter().min().unwrap();
    let max_point = *crabs.iter().max().unwrap();

    let mut fuel_usage = usize::MAX;

    for i in min_point..=max_point {
        let mut fuel = 0;
        for crab in crabs.iter() {
            fuel += _calc(*crab as isize, i as isize);
        }
        fuel_usage = min(fuel_usage, fuel);
    }

    fuel_usage
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut crabs = contents.split(",").map(|x| {x.parse::<usize>().unwrap()}).collect::<Vec<usize>>();
    crabs.sort_unstable();

    println!("{}", part1(crabs.clone()));
    println!("{}", part2(crabs.clone()));
}
