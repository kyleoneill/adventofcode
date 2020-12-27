use std::time::Instant;
use std::fs;
use std::io::BufRead;

const MOD_VAL: u64 = 20201227;

fn main() {
    let start = Instant::now();
    let input = get_input("input.txt");
    let solution = solve(input);
    println!("Found solution in {} microseconds", start.elapsed().as_micros());
    println!("The solution is {}", solution);
}

fn transformation(subject_number: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject_number;
        value %= MOD_VAL;
    }
    value
}

fn get_loop_size(subject_number: u64, door_pk: u64, card_pk: u64) -> Result<(u64, u64), &'static str> {
    let mut loop_size = 1;
    let mut value = 1;
    loop {
        value *= subject_number;
        value %= MOD_VAL;
        if value == door_pk {
            return Ok((loop_size, card_pk))
        }
        if value == card_pk {
            return Ok((loop_size, door_pk))
        }
        loop_size += 1;
        if loop_size == std::u64::MAX {
            break;
        }
    }
    Err("Failed to find solution")
}

fn solve(input: Vec<u64>) -> u64 {
    let subject_number = 7;
    let (loop_size, public_key) = get_loop_size(subject_number, input[0], input[1]).unwrap();
    transformation(public_key, loop_size)
}

fn get_input(filename: &str) -> Vec<u64> {
    let mut contents: Vec<u64> = Vec::new();
    let file = fs::File::open(filename).expect("Failed to open input file");
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(line) = line {
            contents.push(line.parse().expect("Failed to convert input to i32"));
        }
    }
    contents
}