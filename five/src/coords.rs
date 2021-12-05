#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coords {
    pub x: usize,
    pub y: usize,
}

impl Coords {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn parse(input: Vec<&str>) -> Self {
        let x = input[0].parse::<usize>().unwrap();
        let y = input[1].parse::<usize>().unwrap();

        Coords::new(x, y)
    }
}