use std::cmp::{min, max};
use std::fs;

#[derive(Debug, Copy, Clone)]
struct Coords {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Line {
    start: Coords,
    end: Coords,
}

impl Coords {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn parse(input: Vec<&str>) -> Self {
        let x = input[0].parse::<i32>().unwrap();
        let y = input[1].parse::<i32>().unwrap();

        Coords::new(x, y)
    }
}

impl Line {
    fn new(start: Coords, end: Coords) -> Self {
        Self { start, end }
    }

    fn parse(input: Vec<&str>) -> Self {
        let start = Coords::parse(input[0].split(",").collect::<Vec<&str>>());
        let end = Coords::parse(input[1].split(",").collect::<Vec<&str>>());

        Line::new(start, end)
    }

    fn min(&self) -> (i32, i32) {
        (min(self.start.x, self.end.x), min(self.start.y, self.end.y))
    }

    fn max(&self) -> (i32, i32) {
        (max(self.start.x, self.end.x), max(self.start.y, self.end.y))
    }

    fn mark(&self, grid: &mut Vec<Vec<usize>>, offset: &(i32, i32)) {
        let offset_x = (*offset).0;
        let offset_y = (*offset).1;

        let min_x = min(self.start.x, self.end.x);
        let max_x = max(self.start.x, self.end.x);

        let min_y = min(self.start.y, self.end.y);
        let max_y = max(self.start.y, self.end.y);

        // horizontal
        if self.start.y == self.end.y {
            for x in min_x..=max_x {
                grid[(self.start.y - offset_y) as usize][(x - offset_x) as usize] += 1;
            }
        }
        // vertical
        else if self.start.x == self.end.x {
            for y in min_y..=max_y {
                grid[(y - offset_y) as usize][(self.start.x - offset_x) as usize] += 1;
            }
        }
        // diagonal
        else {
            let x_diff = self.end.x - self.start.x;
            let y_diff = self.end.y - self.start.y;

            println!("{:?}, {:?}", self.start, self.end);

            // not 45 degrees
            if x_diff.abs() != y_diff.abs() {
                return;
            }

            for i in 0..=x_diff.abs() {
                let mut x = self.start.x - offset_x + i;
                let mut y = self.start.y - offset_y + i;

                if x_diff < 0 {
                    x -= i * 2;
                }

                if y_diff < 0 {
                    y -= i * 2;
                }

                grid[y as usize][x as usize] += 1;
            }
        }
    }
}

fn read_file(filename: &str) -> String {
    return fs::read_to_string(filename).unwrap();
}

fn main() {
    let contents = read_file("input.txt");

    let lines: Vec<String> = contents.split("\r\n").map(|x| { x.to_string() }).collect();

    let mut line_list: Vec<Line> = Vec::new();

    let mut min_coord: (i32, i32) = (i32::MAX, i32::MAX);
    let mut max_coord: (i32, i32) = (i32::MIN, i32::MIN);

    for line in lines {
        let l = line.split(" -> ").collect::<Vec<&str>>();
        let l = Line::parse(l);

        line_list.push(l);

        min_coord.0 = min(min_coord.0, l.min().0);
        min_coord.1 = min(min_coord.1, l.min().1);

        max_coord.0 = max(max_coord.0, l.max().0);
        max_coord.1 = max(max_coord.1, l.max().1);
    }

    // e.g: if smallest coord is 15,15 and largest is 30,30
    // then we want a grid that can support 0,0 -> 15,15
    // which is a 16,16 array
    let grid_size = ((max_coord.0 - min_coord.0) + 1, (max_coord.1 - min_coord.1) + 1);

    // we cannot guarantee that the grid starts from 0,0
    // so offset is used for mapping coordinates to the grid
    let offset = min_coord;

    let mut danger_grid = vec![vec![0; grid_size.0 as usize]; grid_size.1 as usize];

    for line in line_list {
        line.mark(&mut danger_grid, &offset);
    }

    let mut points = 0;
    for row in &danger_grid {
        println!("{:?}", row);
        for col in row {
            if *col >= 2 as usize {
                points += 1;
            }
        }
    }

    println!("{}", points);
}