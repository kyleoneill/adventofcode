use std::time::Instant;
use std::fs;
use std::io::BufRead;

fn main() {
    let start = Instant::now();
    let input = get_input("input.txt");
    let solution = solve(input);
    println!("Found solution in {} microseconds", start.elapsed().as_micros());
    println!("The solution is {}", solution);
}

fn solve(input: Vec<Vec<char>>) -> u32 {
    // let problem_one = run_slope(input, 1, 3);
    // problem_one
    let mut solution: u32 = 0;
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    for slope in 0..slopes.len() {
        let hit_trees = run_slope(&input, slopes[slope].1, slopes[slope].0);
        if solution == 0 {
            solution = hit_trees;
        }
        else {
            solution *= hit_trees;
        }
    }
    solution
}

fn run_slope(input: &Vec<Vec<char>>, slope_down: usize, slope_right: usize) -> u32 {
    let mut hit_trees = 0;
    let row_length = input[0].len();
    let mut current_column = 0;
    let mut current_row = 0;
    loop {
        if current_column >= row_length {
            current_column -= row_length;
        }
        if input[current_row][current_column].eq(&'#') {
            hit_trees += 1;
        }
        current_column += slope_right;
        current_row += slope_down;
        if current_row >= input.len() {
            break;
        }
        continue;
    }
    hit_trees
}

fn get_input(filename: &str) -> Vec<Vec<char>> {
    let mut contents: Vec<Vec<char>> = Vec::new();
    let file = fs::File::open(filename).expect("Failed to open input file");
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(line) = line {
            contents.push(line.chars().collect());
        }
    }
    contents
}