use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPairs,
    Triple,
    FullHouse,
    Quad,
    Penta,
}

impl From<&[u8; 5]> for HandKind {
    fn from(value: &[u8; 5]) -> Self {
        match value {
            [5, 5, 5, 5, 5] => Self::Penta,
            [4, 4, 4, 4, 1] => Self::Quad,
            [3, 3, 3, 2, 2] => Self::FullHouse,
            [3, 3, 3, 1, 1] => Self::Triple,
            [2, 2, 2, 2, 1] => Self::TwoPairs,
            [2, 2, 1, 1, 1] => Self::OnePair,
            [1, 1, 1, 1, 1] => Self::HighCard,
            _ => panic!("unknown hand"),
        }
    }
}

impl From<&([u8; 5], u8)> for HandKind {
    fn from(value: &([u8; 5], u8)) -> Self {
        let values = value.0;
        let jokers = value.1;
        match jokers {
            5 | 4 => Self::Penta,
            3 => {
                if values[4] == 2 {
                    Self::Penta
                } else {
                    Self::Quad
                }
            }
            2 => match values[2] {
                3 => Self::Penta,
                2 => Self::Quad,
                1 => Self::Triple,
                _ => panic!("2 jokers promo fail"),
            },
            1 => match values[0] {
                4 => Self::Penta,
                3 => Self::Quad,
                2 => {
                    if values[2] == 2 {
                        Self::FullHouse
                    } else {
                        Self::Triple
                    }
                }
                1 => Self::OnePair,
                _ => panic!("1 joker promo fail"),
            },
            0 => Self::from(&values),
            _ => panic!("too many jokers"),
        }
    }
}

#[derive(Debug, Eq)]
struct Hand {
    values: [u8; 5],
    kind: HandKind,
    bet: u32,
}

impl Hand {
    fn new(values: [u8; 5], bet: u32) -> Self {
        let mut counts: [u8; 5] = [0; 5];
        for i in 0..5 {
            let val = values[i];
            counts[i] = values
                .iter()
                .fold(0, |acc, x| if *x == val { acc + 1 } else { acc });
        }
        counts.sort();
        counts.reverse();
        Self {
            values,
            kind: HandKind::from(&counts),
            bet,
        }
    }

    fn new2(values: [u8; 5], bet: u32) -> Self {
        let mut counts: [u8; 5] = [0; 5];
        for i in 0..5 {
            let val = values[i];
            counts[i] = values
                .iter()
                .fold(0, |acc, x| if *x == val { acc + 1 } else { acc });
        }
        let jokers: u8 = values
            .iter()
            .fold(0, |acc, x| if *x == 1 { acc + 1 } else { acc });
        counts.sort();
        counts.reverse();
        Self {
            values,
            kind: HandKind::from(&(counts, jokers)),
            bet,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.values == other.values
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.kind == other.kind {
            self.values.cmp(&other.values)
        } else {
            self.kind.cmp(&other.kind)
        }
    }
}

fn part_one(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let lines = BufReader::new(&file).lines();

    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        if let Ok(text) = line {
            let cards = text.split(" ").nth(0).unwrap();
            let bet: u32 = text.split(" ").nth(1).unwrap().parse::<u32>().unwrap();
            let mut cards_vals: [u8; 5] = [0; 5];
            for (i, c) in cards.chars().enumerate() {
                match c {
                    'A' => cards_vals[i] = 13,
                    'K' => cards_vals[i] = 12,
                    'Q' => cards_vals[i] = 11,
                    'J' => cards_vals[i] = 10,
                    'T' => cards_vals[i] = 9,
                    '9' => cards_vals[i] = 8,
                    '8' => cards_vals[i] = 7,
                    '7' => cards_vals[i] = 6,
                    '6' => cards_vals[i] = 5,
                    '5' => cards_vals[i] = 4,
                    '4' => cards_vals[i] = 3,
                    '3' => cards_vals[i] = 2,
                    '2' => cards_vals[i] = 1,
                    _ => panic!("unknown char"),
                }
            }
            hands.push(Hand::new(cards_vals, bet));
        }
    }
    hands.sort();
    dbg!(&hands);
    let total = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (i as u32 + 1) * x.bet);
    println!("Part 1: {}", total);
}

fn part_two(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let lines = BufReader::new(&file).lines();

    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        if let Ok(text) = line {
            let cards = text.split(" ").nth(0).unwrap();
            let bet: u32 = text.split(" ").nth(1).unwrap().parse::<u32>().unwrap();
            let mut cards_vals: [u8; 5] = [0; 5];
            for (i, c) in cards.chars().enumerate() {
                match c {
                    'A' => cards_vals[i] = 13,
                    'K' => cards_vals[i] = 12,
                    'Q' => cards_vals[i] = 11,
                    'T' => cards_vals[i] = 10,
                    '9' => cards_vals[i] = 9,
                    '8' => cards_vals[i] = 8,
                    '7' => cards_vals[i] = 7,
                    '6' => cards_vals[i] = 6,
                    '5' => cards_vals[i] = 5,
                    '4' => cards_vals[i] = 4,
                    '3' => cards_vals[i] = 3,
                    '2' => cards_vals[i] = 2,
                    'J' => cards_vals[i] = 1,
                    _ => panic!("unknown char"),
                }
            }
            let hand = Hand::new2(cards_vals, bet);
            if cards.contains("J") {
                dbg!(&hand);
            }
            hands.push(hand);
        }
    }
    hands.sort();
    // dbg!(&hands);
    let total = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (i as u32 + 1) * x.bet);
    println!("Part 1: {}", total);
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
