use std::time::Instant;
use std::fs;
use std::io::BufRead;

fn main() {
    let start = Instant::now();
    let input = get_input("input.txt");
    let solution = solve_for_three(input);
    if solution != -1 {
        println!("Found solution in {} microseconds", start.elapsed().as_micros());
        println!("The solution is {}", solution);
    }
    else {
        println!("Failed to find solution");
    }
}

fn solve_for_three(mut input: Vec<i32>) -> i32 {
    while input.len() > 0 {
        let expense = input.pop().expect("Could not pop off top expense");
        let solution = solve_for_two(&input, 2020 - expense);
        if solution.0 != -1 {
            return expense * solution.0 * solution.1
        }
    }
    -1
}

fn solve_for_two(input: &Vec<i32>, solution_num: i32) -> (i32, i32) {
    let mut i = 0;
    while i < input.len() {
        let slice = input.as_slice();
        let expense = slice[i];
        for item in slice[(i + 1)..].iter() {
            if (item + expense) == solution_num {
                return (item.to_owned(), expense)
            }
        }
        i += 1;
    }
    (-1, -1)
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