use std::time::Instant;
use std::fs;
use std::io::BufRead;
use std::collections::VecDeque;

const PREAMBLE_LENGTH: usize = 25;

fn main() {
    let start = Instant::now();
    let input = get_input("input.txt");
    let solution = solve(input).unwrap();
    println!("Found solution in {} microseconds", start.elapsed().as_micros());
    println!("The solution is {}", solution);
}

fn add_smallest_largest(input: &Vec<u64>, start: usize, end: usize) -> u64 {
    let mut smallest = input[start];
    let mut largest = input[start];
    for n in start..end {
        if input[n] > largest {
            largest = input[n];
        }
        if input[n] < smallest {
            smallest = input[n];
        }
    }
    largest + smallest
}

fn solve(input: Vec<u64>) -> Result<u64, &'static str> {
    let invalid_number = get_invalid_number(&input).unwrap();
    let mut range_start = 0;
    let mut range_end = 1;
    let mut sum = input[range_start];
    while range_end < input.len() {
        if sum == invalid_number {
            return Ok(add_smallest_largest(&input, range_start, range_end));
        }
        if sum < invalid_number {
            sum += input[range_end];
            range_end += 1;
        }
        if sum > invalid_number {
            sum -= input[range_start];
            range_start += 1;
        }
    }
    Err("Failed to find a solution")
}

fn get_invalid_number(input: &Vec<u64>) -> Result<u64, &'static str> {
    let mut preamble: VecDeque<u64> = VecDeque::new();
    for n in 0..PREAMBLE_LENGTH {
        preamble.push_back(input[n]);
    }
    for n in PREAMBLE_LENGTH..input.len() {
        let current_number = input[n];
        let mut number_can_be_created = false;
        'outer: for i in 0..preamble.len() - 1 {
            for j in i + 1..preamble.len() {
                if preamble[i] + preamble[j] == current_number {
                    number_can_be_created = true;
                    break 'outer;
                }
            }
        }
        if !number_can_be_created {
            return Ok(current_number);
        }
        preamble.pop_front();
        preamble.push_back(current_number);
    }
    Err("Failed to find invalid number")
}

fn get_input(filename: &str) -> Vec<u64> {
    let mut contents: Vec<u64> = Vec::new();
    let file = fs::File::open(filename).expect("Failed to open input file");
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(line) = line {
            contents.push(line.parse().expect("Failed to convert input to u64"));
        }
    }
    contents
}