use std::{
    fs::File,
    io::{BufRead, BufReader}, time::Instant,
};

#[derive(Debug, Default)]
struct Encoder {
    in_start: u64,
    out_start: u64,
    range: u64,
}

impl Encoder {
    pub fn new(in_start: u64, out_start: u64, range: u64) -> Self {
        Self {
            in_start,
            out_start,
            range,
        }
    }

    pub fn encode(&self, value: u64) -> Option<u64> {
        if value >= self.in_start && value < self.in_start + self.range {
            Some(self.out_start + (value - self.in_start))
        } else {
            None
        }
    }

    pub fn combine(&self, other: &Self) -> Vec<Encoder> {
        let mut ret: Vec<Encoder> = Vec::new();
        // |-------|
        //      |-----
        ret
    }
}

#[derive(Debug, Default)]
struct EncoderMap {
    map: Vec<Encoder>,
}

impl EncoderMap {
    pub fn add_line(&mut self, line: &str) {
        let values: Vec<u64> = line
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        self.map.push(Encoder::new(values[1], values[0], values[2]));
    }

    pub fn reset(&mut self) {
        self.map.clear();
    }

    pub fn encode(&self, values: &mut [u64]) {
        'outer: for value in values {
            for enc in self.map.iter() {
                if let Some(v) = enc.encode(*value) {
                    *value = v;
                    continue 'outer;
                }
            }
        }
    }
}

fn part_one(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let mut lines = BufReader::new(&file).lines();

    let seeds = lines.next().unwrap().unwrap();
    let mut values: Vec<u64> = seeds
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut encoder_map = EncoderMap::default();

    for line in lines {
        if let Ok(text) = line {
            if text.len() > 0 {
                if text.chars().next().unwrap().is_alphabetic() {
                    encoder_map.encode(values.as_mut_slice());
                    encoder_map.reset();
                    // dbg!(&values);
                    // println!("{}", text);
                } else {
                    encoder_map.add_line(&text);
                }
            }
        }
    }
    encoder_map.encode(values.as_mut_slice());
    // dbg!(&values);

    println!("Part 1: {}", values.iter().min().unwrap());
}

fn part_two(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let mut lines = BufReader::new(&file).lines();

    let seeds = lines.next().unwrap().unwrap();
    let ranges: Vec<u64> = seeds
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    // dbg!(&ranges);

    let mut values: Vec<u64> = Vec::new();

    for i in (0..ranges.len()).step_by(2) {
        let mut range = (ranges[i]..ranges[i] + ranges[i + 1]).collect();
        values.append(&mut range);
    }

    println!("Values length: {}", values.len());

    let mut encoder_map = EncoderMap::default();

    for line in lines {
        if let Ok(text) = line {
            if text.len() > 0 {
                if text.chars().next().unwrap().is_alphabetic() {
                    let start = Instant::now();
                    encoder_map.encode(values.as_mut_slice());
                    let duration = start.elapsed();
                    encoder_map.reset();
                    // dbg!(&values);
                    println!("Segment {}s", duration.as_secs());
                } else {
                    encoder_map.add_line(&text);
                }
            }
        }
    }
    encoder_map.encode(values.as_mut_slice());
    // dbg!(&values);

    println!("Part 2: {}", values.iter().min().unwrap());
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
