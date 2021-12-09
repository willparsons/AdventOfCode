fn part1(height_map: Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;

    for i in 0..height_map.len() {
        for j in 0..height_map[i].len() {
            let current = height_map[i][j];

            // look left
            if j > 0 {
                if current >= height_map[i][j - 1] {
                    continue;
                }
            }

            // look right
            if j < height_map[0].len() - 1 {
                if current >= height_map[i][j + 1] {
                    continue;
                }
            }

            // look up
            if i > 0 {
                if current >= height_map[i - 1][j] {
                    continue;
                }
            }

            // look down
            if i < height_map.len() - 1 {
                if current >= height_map[i + 1][j] {
                    continue;
                }
            }

            sum += current + 1;
        }
    }
    sum
}

fn main() {
    let height_map = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    println!("{}", part1(height_map));
}
