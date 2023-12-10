use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn next_val(vals: &[i32]) -> i32 {
    let mut new_line: Vec<i32> = Vec::new();
    for i in 0..vals.len() - 1 {
        new_line.push(vals[i + 1] - vals[i]);
    }
    assert!(new_line.len() == vals.len() - 1);
    let last: i32 = *vals.iter().last().unwrap();
    let last_diff = if new_line.iter().all(|&x| x == 0) {
        0
    } else {
        next_val(&new_line)
    };
    last + last_diff
}

fn prev_val(vals: &[i32]) -> i32 {
    let mut new_line: Vec<i32> = Vec::new();
    for i in 0..vals.len() - 1 {
        new_line.push(vals[i + 1] - vals[i]);
    }
    assert!(new_line.len() == vals.len() - 1);
    let first: i32 = *vals.iter().next().unwrap();
    let first_diff = if new_line.iter().all(|&x| x == 0) {
        0
    } else {
        prev_val(&new_line)
    };
    first - first_diff
}

fn part_one(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let lines = BufReader::new(&file).lines();

    let mut total = 0;
    for line in lines {
        if let Ok(text) = line {
            let vals: Vec<i32>= text.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
            total += next_val(&vals);
        }
    }
    println!("Part 1: {}", total);
}

fn part_two(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let lines = BufReader::new(&file).lines();

    let mut total = 0;
    for line in lines {
        if let Ok(text) = line {
            let vals: Vec<i32>= text.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
            total += prev_val(&vals);
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
