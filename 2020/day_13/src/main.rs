use std::time::Instant;
use std::fs;
use std::io::BufRead;

fn main() {
    let start = Instant::now();
    let input = get_input("input.txt");
    let solution = solve_part_two(input);
    println!("Found solution in {} microseconds", start.elapsed().as_micros());
    println!("The solution is {}", solution);
}

fn solve_part_one(input: Vec<String>) -> Result<i32, &'static str> {
    let earliest_timestamp: i32 = input[0].parse().unwrap();
    let mut actual_depart_time = earliest_timestamp;
    let busses = parse_busses(&input[1]);
    loop {
        for bus in &busses {
            if actual_depart_time % bus == 0 {
                return Ok((actual_depart_time - earliest_timestamp) * bus)
            }
        }
        actual_depart_time += 1;
        if actual_depart_time == std::i32::MAX {
            break;
        }
    }
    Err("Failed to find solution")
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0{
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn solve_part_two(input: Vec<String>) -> u64 {
    let busses: Vec<Option<i32>> = input[1].split(",").map(|s| s.parse::<i32>().ok()).collect();
    let targets: Vec<(i32, usize)> = busses.iter().enumerate().filter_map(|(i, bus)| bus.map(|b| (b, (b as usize - i % b as usize) % b as usize))).collect();
    println!("{:?}", targets);
    let mut t: u64 = 0;
    let mut stride: u64 = 1;
    for target in targets {
        while t % target.0 as u64 != target.1 as u64 {
            t += stride;
        }
        stride = lcm(stride, target.0 as u64);
    }
    return t;
}

fn parse_busses(busses: &str) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    let split = busses.split(",");
    for s in split {
        match s.parse::<i32>() {
            Ok(val) => output.push(val),
            Err(_e) => continue
        }
    }
    output
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