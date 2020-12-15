use std::{ops::Range, time::Instant};
use std::fs;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let start = Instant::now();
    let input = get_input("input.txt");
    let solution = solve(input);
    println!("Found solution in {} microseconds", start.elapsed().as_micros());
    println!("The solution is {}", solution);
}

fn generate_mask(s: &str) -> (u64, u64) {
    let mut mask = 0;
    let mut mask_mask = 0;
    for (i, c) in s.chars().rev().enumerate() {
        match c {
            '0' => {
                mask_mask = mask_mask | (1 << i);
            },
            '1' => {
                mask = mask | (1 << i);
                mask_mask = mask_mask | (1 << i);
            },
            'X' => (),
            _ => panic!("Undefined char in mask")
        }
    }
    (mask, mask_mask)
}

fn solve(input: Vec<String>) -> u64 {
    const MASK_RANGE: Range<usize> = 7..43;
    let mut str_mask = input[0][MASK_RANGE].to_string();
    let mut mask = generate_mask(&str_mask);
    let mut memory: HashMap<u64, u64> = HashMap::new(); //HashMap<address, value>
    for elm in input {
        match &elm[0..4] {
            "mask" => {
                str_mask = elm[MASK_RANGE].to_string();
                mask = generate_mask(&str_mask);
            },
            _ => {
                let split: Vec<&str> = elm.split_whitespace().collect();
                let address: u64 = split[0][4..split[0].len() - 1].parse().unwrap();
                let value: u64 = split[split.len() - 1].parse().unwrap();
                *memory.entry(address).or_insert(0) = mask.0 | (!mask.1 & value);
            }
        }
    }
    let mut counter = 0;
    for val in memory.values() {
        counter += val;
    }
    counter
}

fn get_input(filename: &str) -> Vec<String> {
    let mut contents: Vec<String> = Vec::new();
    let file = fs::File::open(filename).expect("Failed to open input file");
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(line) = line {
            contents.push(line);
        }
    }
    contents
}