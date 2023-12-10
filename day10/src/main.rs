use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Mul},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const NORTH: Self = Self { x: 0, y: -1 };
    const SOUTH: Self = Self { x: 0, y: 1 };
    const WEST: Self = Self { x: -1, y: 0 };
    const EAST: Self = Self { x: 1, y: 0 };
    const ALL_DIRS: [Self; 4] = [Self::NORTH, Self::SOUTH, Self::WEST, Self::EAST];

    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul<i32> for Point {
    type Output = Point;
    fn mul(self, rhs: i32) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Pipe(Point, Point);

impl Pipe {
    const VERT: Self = Self(Point::NORTH, Point::SOUTH);
    const HORZ: Self = Self(Point::WEST, Point::EAST);
    const BEND_L: Self = Self(Point::NORTH, Point::EAST);
    const BEND_J: Self = Self(Point::NORTH, Point::WEST);
    const BEND_7: Self = Self(Point::SOUTH, Point::WEST);
    const BEND_F: Self = Self(Point::SOUTH, Point::EAST);

    fn conn_north(&self) -> bool {
        self.0 == Point::NORTH || self.1 == Point::NORTH
    }

    fn conn_south(&self) -> bool {
        self.0 == Point::SOUTH || self.1 == Point::SOUTH
    }

    fn conn_east(&self) -> bool {
        self.0 == Point::EAST || self.1 == Point::EAST
    }

    fn conn_west(&self) -> bool {
        self.0 == Point::WEST || self.1 == Point::WEST
    }

    fn get_other(&self, dir: Point) -> Point {
        if dir == self.0 {
            self.1
        } else {
            self.0
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Cell {
    kind: Option<Pipe>,
}

impl Cell {
    fn has_conn_to_opposite(&self, opposite: Point) -> bool {
        if let Some(pipe) = self.kind {
            match opposite {
                Point::NORTH => pipe.conn_south(),
                Point::SOUTH => pipe.conn_north(),
                Point::WEST => pipe.conn_east(),
                Point::EAST => pipe.conn_west(),
                _ => false,
            }
        } else {
            false
        }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        let kind = match value {
            '-' => Some(Pipe::HORZ),
            '|' => Some(Pipe::VERT),
            'L' => Some(Pipe::BEND_L),
            'J' => Some(Pipe::BEND_J),
            '7' => Some(Pipe::BEND_7),
            'F' => Some(Pipe::BEND_F),
            '.' | 'S' => None,
            _ => panic!("Unknown pipe kind"),
        };
        Self { kind }
    }
}

#[derive(Debug, Clone, Default)]
struct Map {
    grid: Vec<Cell>,
    start: Point,
    dimension: Point,
}

impl Map {
    fn add_cell(&mut self, c: char, pos: Point) {
        self.grid.push(Cell::from(c));
        if c == 'S' {
            self.start = pos;
        }
        self.dimension.x = self.dimension.x.max(pos.x + 1);
        self.dimension.y = self.dimension.y.max(pos.y + 1);
    }

    fn pipe_len(&self) -> usize {
        let mut current: Point = Point::default();
        let mut from_dir: Point = Point::default();
        let mut len: usize = 0;
        // find first pipe the stupid way
        for dir in Point::ALL_DIRS {
            current = self.start + dir;
            if let Some(cell) = self.get(current) {
                if cell.has_conn_to_opposite(dir) {
                    len += 1;
                    from_dir = dir * -1;
                    break;
                }
            }
        }
        while current != self.start {
            let cell = self.get(current).expect("failed to get connecting cell");
            let pipe = cell.kind.expect("failed to get connecting pipe");
            let new_dir = pipe.get_other(from_dir);
            current = current + new_dir;
            from_dir = new_dir * -1;
            len += 1;
        }
        len
    }

    fn get(&self, pos: Point) -> Option<&Cell> {
        let i = self.dimension.x * pos.y + pos.x;
        if i < 0 {
            None
        } else {
            self.grid.get(i as usize)
        }
    }
}

fn part_one(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let lines = BufReader::new(&file).lines();

    let mut map: Map = Map::default();
    let mut y: i32 = 0;
    for line in lines {
        if let Ok(text) = line {
            for (x, c) in text.chars().enumerate() {
                map.add_cell(c, Point::new(x as i32, y));
            }
            y += 1;
        }
    }
    println!("Part 1: {}", map.pipe_len() / 2);
}

fn part_two(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let lines = BufReader::new(&file).lines();

    let mut total = 0;
    for line in lines {
        if let Ok(text) = line {}
    }
    println!("Part 2: {}", total);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <filename> 1|2", args[0]);
        return;
    }
    if args[2] == "1" {
        part_one(&args[1]);
    } else if args[2] == "2" {
        part_two(&args[1]);
    } else {
        eprintln!("Usage: {} <filename> 1|2", args[0]);
        return;
    }
}
