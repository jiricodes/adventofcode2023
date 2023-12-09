use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    thread::current,
    time::Instant,
};

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
    let mut inst_cycle = instructions.chars().cycle();
    let _ = lines.next().unwrap();
    let mut total: usize = 0;
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut current: Vec<String> = Vec::new();
    for line in lines {
        if let Ok(text) = line {
            let key: &str = text.split(" = ").nth(0).unwrap();
            let ways: Vec<&str> = text.split(" = ").nth(1).unwrap().split(", ").collect();
            let _ = map.insert(
                String::from(key),
                (String::from(&ways[0][1..]), String::from(&ways[1][0..3])),
            );

            if key.chars().last().unwrap() == 'A' {
                current.push(String::from(key));
            }
        }
    }

    println!("Number of paths: {}", current.len());
    let start = Instant::now();

    let mut cycles = 0;
    while !current.iter().all(|x| x.chars().last().unwrap() == 'Z') {
        total += 1;
        if total % 1_000_000_000 == 0 {
            cycles += 1;
            //dbg!(&current);
            let duration = start.elapsed();
            println!(
                "Segment {}s | {:.3} ips",
                duration.as_secs(),
                1_000_000_000.0 * cycles as f64 / duration.as_secs_f64()
            );
        }
        let dir = inst_cycle.next().unwrap();
        for i in 0..current.len() {
            let vals = map.get(&current[i]).unwrap();
            if dir == 'R' {
                current[i] = vals.1.clone();
            } else {
                current[i] = vals.0.clone();
            }
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
