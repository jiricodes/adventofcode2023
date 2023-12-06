use std::{
    fs::File,
    io::{BufRead, BufReader}, time::Instant,
};

use num::Complex;

fn part_one(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let mut lines = BufReader::new(&file).lines();

    let times_line = lines.next().unwrap().unwrap();
    let times: Vec<u32> = times_line
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let distances_line = lines.next().unwrap().unwrap();
    let distances: Vec<u32> = distances_line
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    assert!(times.len() == distances.len());

    let mut product = 1;
    for i in 0..times.len() {
        let mut left = 1;
        let mut count  = 1;
        // maybe polynomial solver would be better?
        // brute way easy
        while left * (times[i] - left) <= distances[i] {
            left += 1;
        }
        while (left + count) * (times[i] - left - count) > distances[i] {
            count += 1;
        }
        println!("{}: {}", i, count);
        product *= count;
    }
    println!("Part 1: {}", product);

}

fn part_two(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let mut lines = BufReader::new(&file).lines();

    let times_line = lines.next().unwrap().unwrap();
    let mut time_string = String::from(times_line.split(": ").last().unwrap());
    time_string.retain(|c| !c.is_whitespace());
    let time: u64 = time_string.parse::<u64>().unwrap();

    let distances_line = lines.next().unwrap().unwrap();
    let mut distance_string = String::from(distances_line.split(": ").last().unwrap());
    distance_string.retain(|c| !c.is_whitespace());
    let distance: u64 = distance_string.parse::<u64>().unwrap();

    // ugh agh
    // poly fix
    // speed * time_left > distance
    // x * (b - x) > c
    // -1 * x2 + bx - c > 0

    // d = b^2 - 4ac
    // rp = -b / 2a
    // ip = abs(d)^0.5 / 2a

    let a: f64 = -1.0;
    let b: f64 = time as f64;
    let c: f64 = -1.0 * distance as f64;
    let d = b.powi(2) - 4.0 * a * c;
    let rp = -b / 2.0 * a;
    let ip = d.abs().sqrt() / 2.0 * a;

    if d < 0.0 {
        let x1 = Complex::new(rp, ip);
        let x2 = Complex::new(rp, -ip);
        println!("Complex: {} | {}", x1, x2);
    } else {
        let x1 = rp + ip;
       let x2 = rp - ip;
        println!("Real: {} | {}", x1, x2);
        println!("Part2: {}", x2.ceil() - x1.ceil());
        
    }
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
