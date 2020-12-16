use std::{cmp::max, time::Instant};
use std::fs;
use std::io::BufRead;

fn main() {
    let start = Instant::now();
    let mut input = get_input("input.txt");
    input.push(0);
    input.sort();
    input.push(input[input.len() - 1] + 3);
    let solution = solve_part_two(&input);
    println!("Found solution in {} microseconds", start.elapsed().as_micros());
    println!("The solution is {}", solution);
}

fn solve_part_two(input: &[i32]) -> i64 {
    let mut cache = vec![0; input.len()];
    cache[0] = 1;
    for (i, adapter) in input.iter().enumerate().skip(1) {
        let start = max(i as isize - 3, 0) as usize;
        cache[i] = cache[start..i].iter().enumerate().filter_map(|(index, a)| if adapter - input[start + index] <= 3 {Some(a)} else {None}).sum();
    }
    cache[cache.len() - 1]
}

fn solve_part_one(mut input: Vec<i32>) -> i32 {
    input.push(input[input.len() - 1] + 3); //Add the device which is +3 of our highest adapter
    let mut diff_one = 0;
    let mut diff_three = 0;
    let mut joltage = 0;
    for adapter in input {
        let diff = adapter - joltage;
        if diff > 0 && diff < 4 {
            if diff == 1 {diff_one += 1;}
            else if diff == 3 {diff_three += 1;}
            joltage = adapter;
        }
    }
    diff_one * diff_three
}

fn get_input(filename: &str) -> Vec<i32> {
    let mut contents: Vec<i32> = Vec::new();
    let file = fs::File::open(filename).expect("Failed to open input file");
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(line) = line {
            contents.push(line.parse().expect("Failed to convert input to i32"));
        }
    }
    contents
}