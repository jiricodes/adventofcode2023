use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};
use num::integer::lcm;

fn part_one(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let mut lines = BufReader::new(&file).lines();

    let instructions = lines.next().unwrap().unwrap();
    let mut inst_cycle = instructions.chars().cycle();
    let _ = lines.next().unwrap();
    let mut total = 0;
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for line in lines {
        if let Ok(text) = line {
            let key: &str = text.split(" = ").nth(0).unwrap();
            let ways: Vec<&str> = text.split(" = ").nth(1).unwrap().split(", ").collect();
            let _ = map.insert(
                String::from(key),
                (String::from(&ways[0][1..]), String::from(&ways[1][0..3])),
            );
        }
    }
    let mut current = String::from("AAA");
    let end = "ZZZ";

    while current != end {
        total += 1;
        let vals = map.get(&current).unwrap();
        let dir = inst_cycle.next().unwrap();
        if dir == 'R' {
            current = vals.1.clone();
        } else {
            current = vals.0.clone();
        }
    }

    println!("Part 1: {}", total);
}

fn part_two(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let mut lines = BufReader::new(&file).lines();

    let instructions = lines.next().unwrap().unwrap();
    let _ = lines.next().unwrap();
    let mut map: HashMap<[u8; 3], ([u8; 3], [u8; 3])> = HashMap::new();
    let mut start_nodes: Vec<[u8; 3]> = Vec::new();
    for line in lines {
        if let Ok(text) = line {
            let key: [u8; 3] = text
                .split(" = ")
                .nth(0)
                .unwrap()
                .as_bytes()
                .try_into()
                .unwrap();
            if key[2] == 65 {
                start_nodes.push(key);
            }
            let ways: Vec<&str> = text.split(" = ").nth(1).unwrap().split(", ").collect();
            let left: [u8; 3] = ways[0][1..].as_bytes().try_into().unwrap();
            let right: [u8; 3] = ways[1][0..3].as_bytes().try_into().unwrap();
            let _ = map.insert(key, (left, right));
        }
    }

    let mut totals: Vec<u32> = Vec::new();
    for start_node in start_nodes {
        let mut current = start_node;
        let mut total: u32 = 0;
        let mut inst_cycle = instructions.chars().cycle();
        while current[2] != 90 {
            total += 1;

            let dir = inst_cycle.next().unwrap();
            let vals = map.get(&current).unwrap();
            if dir == 'R' {
                current = vals.1;
            } else {
                current = vals.0;
            }
        }
        totals.push(total);
    }

    // dbg!(totals);

    println!("Part 2: {}", totals.into_iter().reduce(lcm).unwrap());
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
