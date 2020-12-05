use std::time::Instant;
use std::fs;
use std::io::BufRead;

fn lower_half(max: i32, min: i32) -> i32 {
    ((max + min + 1) / 2) - 1
}

fn upper_half(max: i32, min: i32) -> i32 {
    (max + min + 1) / 2
}

fn get_seat_row(boarding_pass: &Vec<char>) -> Result<i32, &'static str> {
    let mut max_row = 127;
    let mut min_row = 0;
    for row in 0..=6 {
        match boarding_pass[row] {
            'F' => {
                max_row = lower_half(max_row, min_row);
                if row == 6 {
                    return Ok(max_row)
                }
            },
            'B' => {
                min_row = upper_half(max_row, min_row);
                if row == 6 {
                    return Ok(min_row)
                }
            },
            _ => panic!("Failed to get seat row for boarding pass")
        }
    }
    Err("Failed to find a seat row")
}

fn get_seat_column(boarding_pass: &Vec<char>) -> Result<i32, &'static str> {
    let mut max_column = 7;
    let mut min_column = 0;
    for column in 7..=9 {
        match boarding_pass[column] {
            'R' => {
                min_column = upper_half(max_column, min_column);
                if column == 9 {
                    return Ok(min_column)
                }
            },
            'L' => {
                max_column = lower_half(max_column, min_column);
                if column == 9 {
                    return Ok(max_column)
                }
            },
            _ => panic!("Failed to get seat column for boarding pass")
        }
    }
    Err("Failed to find a seat column")
}

fn get_seat_id(boarding_pass: Vec<char>) -> i32 {
    let seat_row = get_seat_row(&boarding_pass).expect("Failed to get seat row");
    let seat_column = get_seat_column(&boarding_pass).expect("Failed to get seat column");
    (seat_row * 8) + seat_column
}

fn solve_part_one(input: Vec<String>) -> i32 {
    let mut highest_seat_id = 0;
    for boarding_pass in input {
        let seat_id = get_seat_id(boarding_pass.chars().collect::<Vec<char>>());
        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }
    highest_seat_id
}

fn solve_part_two(input: Vec<String>) -> Result<i32, &'static str> {
    let mut boarding_passes: Vec<i32> = Vec::new();
    for boarding_pass in input {
        let seat_id = get_seat_id(boarding_pass.chars().collect::<Vec<char>>());
        boarding_passes.push(seat_id);
    }
    boarding_passes.sort();
    for pass in 10..=boarding_passes.len() - 10 {
        if boarding_passes[pass - 1] + 1 != boarding_passes[pass] {
            return Ok(boarding_passes[pass - 1] + 1)
        }
    }
    Err("Failed to solve part two")
}

fn main() {
    let start = Instant::now();
    let input = get_input("input.txt");
    let solution = solve_part_two(input).expect("Failed to solve part two");
    println!("Found solution in {} microseconds", start.elapsed().as_micros());
    println!("The solution is {}", solution);
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
