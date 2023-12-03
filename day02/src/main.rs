use std::{fs::File, io::{BufRead, BufReader}, fmt::Display};
use std::fmt;

#[derive(Debug, Default)]
struct Clues {
    red: u32,
    green: u32,
    blue: u32
}

impl Clues {
    fn add_one(&mut self, input: &str) {
        let split: Vec<&str> = input.split(" ").collect();
        assert!(split.len() == 2);
        let value = split[0].parse::<u32>().unwrap();
        match split[1] {
            "red" => self.red = self.red.max(value),
            "green" => self.green = self.green.max(value),
            "blue" => self.blue = self.blue.max(value),
            _ => eprintln!("unknown color"),
        }
    }

    fn add_clue(&mut self, input: &str) {
        for color in input.split(", ") {
            self.add_one(color);
        }
    }

    pub fn add_line(&mut self, input: &str) {
        for chunk in input.split("; ") {
            self.add_clue(chunk);
        }
    }

    pub fn is_possible(&self, red_max: u32, green_max: u32, blue_max: u32) -> bool {
        self.red <= red_max && self.green <= green_max && self.blue <= blue_max 
    }

    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl Display for Clues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(red {}, green {}, blue {})", self.red, self.green, self.blue)
    }
}

fn part_one(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let lines  = BufReader::new(&file).lines();
    let mut total: u32 = 0;
    for line in lines {
        let mut clues = Clues::default();
        if let Ok(text) = line {
            let game: Vec<&str> = text.split(": ").collect();
            assert!(game.len() == 2);
            clues.add_line(game[1]);
            let n = game[0][5..].parse::<u32>().unwrap();
            println!("{}: {} {} {}", game[0], clues, clues.is_possible(12, 13, 14), n);
            if clues.is_possible(12, 13, 14) {
                total += n;
            }
        }
    }

    println!("Part 1: {}", total);
}

fn part_two(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let lines  = BufReader::new(&file).lines();
    let mut total: u32 = 0;
    for line in lines {
        let mut clues = Clues::default();
        if let Ok(text) = line {
            let game: Vec<&str> = text.split(": ").collect();
            assert!(game.len() == 2);
            clues.add_line(game[1]);
            total += clues.power();
            // println!("{}: {}", game[0], clues);
        }
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
