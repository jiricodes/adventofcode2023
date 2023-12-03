use std::{fs::File, io::{BufRead, BufReader}, fmt::Display};
use std::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: u32,
    y: u32,
}

impl Pos {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y
        }
    }

    fn neighbours(&self, length: u32) -> Vec<Pos> {
        let mut ngbs: Vec<Pos> = Vec::new();
        // left
        if self.x > 0 {
            ngbs.push(Pos::new(self.x - 1, self.y));
        }
        // right
        ngbs.push(Pos::new(self.x + length, self.y)); 
        for t in self.x..(self.x + length) {
            // top
            if self.y > 0 {
                ngbs.push(Pos::new(self.x + t, self.y - 1)); 
            }
            // bot
            ngbs.push(Pos::new(self.x + t, self.y + 1)); 
        }

        ngbs
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Area {
    start: Pos,
    end: Pos,
}

impl Area {
    pub fn new(start: Pos, end: Pos) -> Self {
        Self {
            start,
            end
        }
    }

    pub fn is_within(&self, pos: Pos) -> bool {
        // self.start.x <= pos.x && self.start.y <= pos.y && 
        //                pos.x <= self.end.x && pos.y <= self.end.y
        self.start <= pos && pos < self.end
    }
}

#[derive(Debug, Default, Clone)]
struct Component {
    value: u32,
    neigbours_pos: Vec<Pos>,
}

impl Component {
    pub fn constrain_area(&mut self, area: Area) {
        self.neigbours_pos.retain(|&x| area.is_within(x));
    }
}


#[derive(Debug, Default, Clone)]
struct Schematic {
    components: Vec<Component>,
    symbols: Vec<Pos>,
    area: Area,
}

impl Schematic {
    fn constrain_area(&mut self) {
        for component in self.components.iter_mut() {
            component.constrain_area(self.area)
        }
    }

    pub fn parse_line(&mut self, line: &str) {
        self.area.end.x = line.len() as u32;
        self.area.end.y += 1;
        let chunks: Vec<&str> = line.split(".").collect();
        dbg!(chunks);
    }

    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            symbols: Vec::new(),
            area: Area::default(),
        }
    }

    pub fn calc_part_one(&mut self) -> u32 {
        self.constrain_area();
        let mut total: u32 = 0;
        for component in self.components.iter() {
            for ngb in component.neigbours_pos.iter() {
                if self.symbols.contains(ngb) {
                    total += component.value;
                }
            }
        
        }

        total
    }
}

fn part_one(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let lines  = BufReader::new(&file).lines();

    let mut schematic = Schematic::new();
    for line in lines {

        if let Ok(text) = line {
            schematic.parse_line(&text);
        }
    }
    dbg!(&schematic);

    println!("Part 1: {}", schematic.calc_part_one());
}

fn part_two(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let lines  = BufReader::new(&file).lines();
    let mut total: u32 = 0;
    for line in lines {
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
