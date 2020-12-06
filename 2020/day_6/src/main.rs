use std::time::Instant;
use std::fs;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let start = Instant::now();
    let input = get_input("input.txt");
    let solution = solve_part_two(input);
    println!("Found solution in {} microseconds", start.elapsed().as_micros());
    println!("The solution is {}", solution);
}

fn solve_part_one(input: Vec<Vec<String>>) -> i32 {
    let mut sum = 0;
    for group in input {
        let mut group_responses: Vec<char> = Vec::new();
        for person in group {
            for response in person.chars() {
                if !group_responses.contains(&response) {
                    group_responses.push(response);
                    sum += 1;
                }
            }
        }
    }
    sum
}

fn solve_part_two(input: Vec<Vec<String>>) -> i32 {
    let mut sum = 0;
    for group in input {
        let mut group_responses = HashMap::new();
        for person in &group {
            for response in person.chars() {
                let counter = group_responses.entry(response).or_insert(0);
                *counter += 1;
            }
        }
        for key in group_responses.keys() {
            if group_responses.get(key) == Some(&(group.len() as i32)) {
                sum += 1;
            }
        }
    }
    sum
}

fn get_input(filename: &str) -> Vec<Vec<String>> {
    let mut contents: Vec<Vec<String>> = Vec::new();
    let mut current_group: Vec<String> = Vec::new();
    let file = fs::File::open(filename).expect("Failed to open input file");
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(line) = line {
            if line.trim().is_empty() {
                contents.push(current_group);
                current_group = Vec::new();
            }
            else {
                current_group.push(line);
            }
        }
    }
    if !current_group.len() != 0 {
        contents.push(current_group);
    }
    contents
}