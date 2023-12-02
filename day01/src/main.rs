use std::{fs::File, io::{BufRead, BufReader}};

fn part_one(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let lines  = BufReader::new(&file).lines();
    let mut total: u32 = 0;
    for line in lines {
        if let Ok(text) = line {
            let mut leftmost: u32 = 69;
            let mut rightmost: u32 = 69;
            for letter in text.chars() {
                if let Some(digit) = letter.to_digit(10) {
                    if leftmost == 69 {
                        leftmost = digit;
                    }
                    rightmost = digit;
                }
            }
            assert!(leftmost != 69, "leftmost failed");
            assert!(rightmost != 69, "leftmost failed");
            total += leftmost * 10 + rightmost;
        }
    }

    println!("Part 1: {}", total);
}

fn part_two(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let lines  = BufReader::new(&file).lines();
    let mut total: u32 = 0;
    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for line in lines {
        if let Ok(text) = line {
            let mut leftmost: (i32, u32) = (-1, 69);
            let mut rightmost: (i32, u32) = (-1, 69);
            for (index, letter) in text.chars().enumerate() {
                if let Some(digit) = letter.to_digit(10) {
                    if leftmost.1 == 69 {
                        leftmost.1 = digit;
                        leftmost.0 = index as i32;
                    }
                    rightmost.1 = digit;
                    rightmost.0 = index as i32;
                }
            }
            for (i, d) in digits.iter().enumerate() {
                if let Some(position) = text.find(d) {
                    if leftmost.0 == -1 || leftmost.0 > position as i32 {
                        leftmost.0 = position as i32;
                        leftmost.1 = i as u32 + 1;
                    }
                }
            }
            for (i, d) in digits.iter().enumerate() {
                if let Some(position) = text.rfind(d) {
                    if rightmost.0 == -1 || rightmost.0 < position as i32 {
                        rightmost.0 = position as i32;
                        rightmost.1 = i as u32 + 1;
                    }
                }
            }
            println!("{} | {} | {} @{}; {} @{}", text, leftmost.1 * 10 + rightmost.1, leftmost.1, leftmost.0, rightmost.1, rightmost.0);
            assert!(leftmost.1 != 69, "leftmost failed");
            assert!(rightmost.1 != 69, "leftmost failed");
            total += leftmost.1 * 10 + rightmost.1;
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
