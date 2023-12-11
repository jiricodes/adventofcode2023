use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Mul},
};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn manhattan(&self, other: Self) -> usize {
        ((other.x - self.x).abs() + (other.y - self.y).abs()) as usize
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

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}; {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Default)]
struct Map {
    galaxies: Vec<Point>,
    dimension: Point,
}

impl Map {
    fn expand(&mut self, coefficient: i32) {
        let mut empty_rows: Vec<i32> = (0..self.dimension.y).collect();
        let mut empty_cols: Vec<i32> = (0..self.dimension.x).collect();
        for galaxy in self.galaxies.iter() {
            if let Some(i) = empty_rows.iter().position(|&y| y == galaxy.y) {
                empty_rows.remove(i);
            }
            if let Some(i) = empty_cols.iter().position(|&x| x == galaxy.x) {
                empty_cols.remove(i);
            }
        }
        for galaxy in self.galaxies.iter_mut() {
            let xpand: i32 = empty_cols.iter().fold(0, |acc, &x| if galaxy.x > x { acc + 1 } else { acc });
            galaxy.x += xpand * (coefficient - 1);
            let ypand: i32 = empty_rows.iter().fold(0, |acc, &y| if galaxy.y > y { acc + 1 } else { acc });
            galaxy.y += ypand * (coefficient - 1);
        }
        self.dimension.x += empty_cols.len() as i32;
        self.dimension.y += empty_rows.len() as i32;
    }

    fn all_manhattans(&self) -> usize {
        let combs = self.galaxies.clone().into_iter().combinations(2);
        let mut total: usize = 0;
        for combo in combs {
            let manhattan = combo[0].manhattan(combo[1]);
            // println!("{} -> {}: {}", combo[0], combo[1], manhattan);
            total += manhattan;
        }
       total 
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
                if c == '#' {

                    map.galaxies.push(Point::new(x as i32, y));
                }
                map.dimension.x = map.dimension.x.max(x as i32 + 1);
                map.dimension.y = map.dimension.y.max(y as i32 + 1);
            }
            y += 1;
        }
    }
    map.expand(2);
    println!("Part 1: {}", map.all_manhattans());
}

fn part_two(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let lines = BufReader::new(&file).lines();

    let mut map: Map = Map::default();
    let mut y: i32 = 0;
    for line in lines {
        if let Ok(text) = line {
            for (x, c) in text.chars().enumerate() {
                if c == '#' {

                    map.galaxies.push(Point::new(x as i32, y));
                }
                map.dimension.x = map.dimension.x.max(x as i32 + 1);
                map.dimension.y = map.dimension.y.max(y as i32 + 1);
            }
            y += 1;
        }
    }
    map.expand(1_000_000);
    println!("Part 2: {}", map.all_manhattans());
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
