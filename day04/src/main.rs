use std::collections::HashSet;
use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

fn part_one(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let lines = BufReader::new(&file).lines();

    let mut result: u32 = 0;
    for line in lines {
        if let Ok(text) = line {
            let game = text.split(": ").nth(1).unwrap();
            let win_chunk = game.split(" | ").nth(0).unwrap();
            let our_chunk = game.split(" | ").nth(1).unwrap();

            let winning_nums: HashSet<u32> = win_chunk
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            let our_nums: HashSet<u32> = our_chunk
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            let correct = winning_nums.intersection(&our_nums);
            let count = correct.count() as u32;
            if count > 0 {
                let value = u32::pow(2, count - 1);
                result += value;
            }
        }
    }

    println!("Part 1: {}", result);
}

fn part_two(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let lines = BufReader::new(&file).lines();

    let mut collection: [u32; 300] = [0; 300];
    for (i, line) in lines.enumerate() {
        if let Ok(text) = line {
            collection[i] += 1;
            let game = text.split(": ").nth(1).unwrap();
            let win_chunk = game.split(" | ").nth(0).unwrap();
            let our_chunk = game.split(" | ").nth(1).unwrap();

            let winning_nums: HashSet<u32> = win_chunk
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            let our_nums: HashSet<u32> = our_chunk
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            let correct = winning_nums.intersection(&our_nums);
            let count = correct.count();
            if count > 0 {
                for j in i + 1..i + count + 1 {
                    collection[j] += collection[i];
                }
            }
        }
    }

    let result: u32 = collection.iter().sum();

    println!("Part 2: {}", result);
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
