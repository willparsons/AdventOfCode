use std::collections::VecDeque;
use std::fs;

fn solution(mut fish_timers: VecDeque<usize>, range: usize) -> usize {
    for _i in 0..range {
        let num = fish_timers.pop_front().unwrap();
        fish_timers[6] += num;
        fish_timers.push_back(num);
    }

    sum_vec(fish_timers)
}

fn sum_vec(v: VecDeque<usize>) -> usize {
    v.iter().fold(0, |acc, x| acc + x)
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let data = contents.split(",").map(|x| {x.parse::<usize>().unwrap()}).collect::<Vec<usize>>();

    let fish_timers: VecDeque<usize> = (0..9).map(|x1| data.iter().filter(|&x2| { *x2 == x1 }).count()).collect();

    println!("{:?}", solution(fish_timers.clone(), 80));
    println!("{:?}", solution(fish_timers.clone(), 256));
}
