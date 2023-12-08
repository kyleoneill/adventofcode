use problem::{solve_main, Problem};
use std::collections::HashMap;

fn read_input(input: &Vec<String>) -> (Vec<char>, HashMap<String, (String, String)>) {
    let instructions = input[0].chars().collect();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for line in &input[2..] {
        let split: Vec<&str> = line.split("=").collect();
        let key = split[0].trim();
        let trimmed = split[1].trim();
        let left = &trimmed[1..=3];
        let right = &trimmed[6..=8];
        map.insert(key.to_owned(), (left.to_owned(), right.to_owned()));
    }
    (instructions, map)
}

fn iterate_instructions(instructions: &Vec<char>, map: &HashMap<String, (String, String)>) -> u32 {
    let mut steps = 0;
    let mut instruction_pointer = 0;
    let mut current_node = "AAA";
    loop {
        let (left, right) = map.get(current_node).expect("Failed to read map key");
        if instructions[instruction_pointer] == 'L' {
            current_node = left;
        }
        else {
            current_node = right;
        }
        // We took a step, also need to inc where we are in instructions
        // If we reached the end of the instructions, loop back
        steps += 1;
        instruction_pointer += 1;
        if instruction_pointer == instructions.len() {
            instruction_pointer = 0;
        }
        if current_node == "ZZZ" {
            break;
        }
    }
    steps
}

fn distance_to_end(instructions: &Vec<char>, map: &HashMap<String, (String, String)>, node: &str) -> u64 {
    let mut instruction_pointer = 0;
    let mut steps = 0;
    let mut current_node = node;
    loop {
        let instruction = instructions[instruction_pointer];
        if instruction == 'L' {
            current_node = &map[current_node].0;
        }
        else {
            current_node = &map[current_node].1;
        }
        steps += 1;
        if current_node.chars().last().unwrap() == 'Z' {
            break;
        }
        instruction_pointer += 1;
        if instruction_pointer == instructions.len() {
            instruction_pointer = 0;
        }
    }
    steps
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(mut first: u64, mut second: u64) -> u64 {
    while second != 0 {
        let t = second;
        second = first % second;
        first = t;
    }
    first
}

fn iterate_ghost(instructions: &Vec<char>, map: &HashMap<String, (String, String)>) -> u64 {
    // Brute forcing this would take forever, the solution is _enormous_, have to calculate it
    let current_nodes: Vec<&str> = map.keys().filter(|&n| n.chars().last().unwrap() == 'A').map(|n| n.as_str()).collect();
    current_nodes.iter().map(|n| distance_to_end(instructions, map, n)).fold(1, lcm)
}

struct Day8;

impl Problem for Day8 {
    type Input = Vec<String>;
    type PartOne = u32;
    type PartTwo = u64;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let (instructions, map) = read_input(input);
        iterate_instructions(&instructions, &map)
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let (instructions, map) = read_input(input);
        iterate_ghost(&instructions, &map)
    }
}

fn main() {
    solve_main::<Day8>();
}
