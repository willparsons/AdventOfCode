use std::cmp::{max, min};
use std::collections::HashMap;
use crate::coords::Coords;


#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub start: Coords,
    pub end: Coords,
}

impl Line {
    pub fn new(start: Coords, end: Coords) -> Self {
        Self { start, end }
    }

    pub fn parse(input: Vec<&str>) -> Self {
        let start = Coords::parse(input[0].split(",").collect::<Vec<&str>>());
        let end = Coords::parse(input[1].split(",").collect::<Vec<&str>>());

        Line::new(start, end)
    }

    pub fn min(&self) -> (usize, usize) {
        (min(self.start.x, self.end.x), min(self.start.y, self.end.y))
    }

    pub fn max(&self) -> (usize, usize) {
        (max(self.start.x, self.end.x), max(self.start.y, self.end.y))
    }

    pub fn mark(&self, grid: &mut Vec<Vec<usize>>, map: &mut HashMap<Coords, bool>, offset: &Coords) {
        if self.start.y == self.end.y {
            self.mark_horizontal(grid, map, offset);
        } else if self.start.x == self.end.x {
            self.mark_vertical(grid, map, offset);
        } else {
            self.mark_diagonal(grid, map, offset);
        }
    }

    fn mark_horizontal(&self, grid: &mut Vec<Vec<usize>>, map: &mut HashMap<Coords, bool>, offset: &Coords) {
        let offset_x = (*offset).x;
        let offset_y = (*offset).y;

        let min_x = min(self.start.x, self.end.x);
        let max_x = max(self.start.x, self.end.x);

        let mut x: usize;
        let mut y: usize;

        for val in min_x..=max_x {
            x = val - offset_x;
            y = (self.start.y - offset_y) as usize;

            grid[y][x] += 1;

            if grid[y][x] >= 2 {
                Line::add_to_danger_map(x, y, map);
            }
        }
    }

    fn mark_vertical(&self, grid: &mut Vec<Vec<usize>>, map: &mut HashMap<Coords, bool>, offset: &Coords) {
        let offset_x = (*offset).x;
        let offset_y = (*offset).y;

        let min_y = min(self.start.y, self.end.y);
        let max_y = max(self.start.y, self.end.y);

        let mut x: usize;
        let mut y: usize;

        for val in min_y..=max_y {
            x = (self.start.x - offset_x) as usize;
            y = (val - offset_y) as usize;

            grid[y][x] += 1;

            if grid[y][x] >= 2 {
                Line::add_to_danger_map(x, y, map);
            }
        }
    }

    fn mark_diagonal(&self, grid: &mut Vec<Vec<usize>>, map: &mut HashMap<Coords, bool>, offset: &Coords) {
        let offset_x = (*offset).x;
        let offset_y = (*offset).y;

        let x_diff = self.end.x as isize - self.start.x as isize;
        let y_diff = self.end.y as isize - self.start.y as isize;

        let mut x: usize;
        let mut y: usize;

        // not 45 degrees
        if x_diff.abs() != y_diff.abs() {
            return;
        }

        for i in 0..=x_diff.abs() {
            let i = i as usize;

            x = self.start.x - offset_x + i;
            y = self.start.y - offset_y + i;

            if x_diff < 0 {
                x -= i * 2;
            }

            if y_diff < 0 {
                y -= i * 2;
            }

            grid[y][x] += 1;

            if grid[y][x] >= 2 {
                Line::add_to_danger_map(x, y, map);
            }
        }
    }

    fn add_to_danger_map(x: usize, y: usize, map: &mut HashMap<Coords, bool>) {
        let coords = Coords::new(x, y);

        if !map.contains_key(&coords) {
            map.insert(coords, true);
        }
    }
}
