mod coords;
mod line;

use std::cmp::{min, max};
use std::collections::HashMap;
use std::fs;
use crate::coords::Coords;
use crate::line::Line;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<String> = contents.split("\r\n").map(|x| { x.to_string() }).collect();

    let mut line_list: Vec<Line> = Vec::new();
    let mut danger_map: HashMap<Coords, bool> = HashMap::new();

    let mut min_coord = Coords::new(usize::MAX, usize::MAX);
    let mut max_coord = Coords::new(usize::MIN, usize::MIN);

    for line in lines {
        let start_and_end = line.split(" -> ").collect::<Vec<&str>>();
        let l = Line::parse(start_and_end);

        line_list.push(l);

        min_coord.x = min(min_coord.x, l.min().0);
        min_coord.y = min(min_coord.y, l.min().1);

        max_coord.x = max(max_coord.x, l.max().0);
        max_coord.y = max(max_coord.y, l.max().1);
    }

    // e.g: if smallest coord is 15,15 and largest is 30,30
    // then we want a grid that can support 0,0 -> 15,15
    // which is a 16,16 array
    let grid_size = ((max_coord.x - min_coord.x) + 1, (max_coord.y - min_coord.y) + 1);

    // we cannot guarantee that the grid starts from 0,0
    // so offset is used for mapping coordinates to the grid
    let offset = min_coord;

    let mut danger_grid = vec![vec![0; grid_size.0 as usize]; grid_size.1 as usize];

    for line in line_list {
        line.mark(&mut danger_grid, &mut danger_map, &offset);
    }

    println!("{}", danger_map.len());
}