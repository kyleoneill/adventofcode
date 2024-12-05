use std::{collections::HashMap, str::FromStr};

use problem::{solve_main, Problem, Coord};

#[derive(Debug)]
enum Operation {
    TurnOn,
    Toggle,
    TurnOff
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    top_left: Coord<usize>,
    bottom_right: Coord<usize>,
}

#[derive(Debug)]
struct ParseInstructionError;
impl FromStr for Instruction {
    type Err = ParseInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let first = split.next().expect("Line did not contain any tokens");
        let operation = match first {
            "turn" => {
                let second = split.next().expect("Failed to get a second token for turning lights on or off");
                match second {
                    "on" => Operation::TurnOn,
                    "off" => Operation::TurnOff,
                    _ => panic!("Got an invalid turn on/off token")
                }
            },
            "toggle" => Operation::Toggle,
            _ => panic!("First token '{}' while parsing a line was invalid", first)
        };
        let first_coord_string = split.next().expect("Failed to get first coord from line");
        let top_left: Coord<usize> = Coord::from_str(first_coord_string).expect("Failed to convert first coord to Coord");
        split.next().expect("Failed to get empty 'through' token when parsing a line");
        let second_coord_string = split.next().expect("Failed to get second coord from line");
        let bottom_right: Coord<usize> = Coord::from_str(second_coord_string).expect("Failed to convert second coord to Coord");
        Ok(Self{ operation, top_left, bottom_right })
    }
}

impl Instruction {
    fn from_input(input: &Vec<String>) -> Vec<Self> {
        let mut instructions: Vec<Self> = Vec::new();
        for line in input {
            instructions.push(line.parse::<Instruction>().expect("Failed to parse line into instruction"));
        }
        instructions
    }
}

fn part_one_slow(instructions: &Vec<Instruction>) -> usize {
    let mut grid: HashMap<Coord, bool> = HashMap::new();
    for instruction in instructions {
        for x in instruction.top_left.x..=instruction.bottom_right.x {
            for y in instruction.top_left.y..=instruction.bottom_right.y {
                let current_coord: Coord<usize> = Coord { x, y };
                match instruction.operation {
                    Operation::TurnOn => { grid.insert(current_coord, true); },
                    Operation::TurnOff => { grid.insert(current_coord, false); },
                    Operation::Toggle => {
                        let prev_val: bool = *grid.get(&current_coord).unwrap_or(&false);
                        grid.insert(current_coord, !prev_val);
                    }
                }
            }
        }
    }
    grid.values().filter(|x| **x == true).count()
}

fn part_two_slow(instructions: &Vec<Instruction>) -> usize {
    let mut grid: HashMap<Coord, usize> = HashMap::new();
    for instruction in instructions {
        for x in instruction.top_left.x..=instruction.bottom_right.x {
            for y in instruction.top_left.y..=instruction.bottom_right.y {
                let current_coord: Coord<usize> = Coord { x, y };
                let prev_val: usize = *grid.get(&current_coord).unwrap_or(&0);
                match instruction.operation {
                    Operation::TurnOn => { grid.insert(current_coord, prev_val + 1); },
                    Operation::TurnOff => {
                        match prev_val.checked_sub(1) {
                            Some(new_val) => { grid.insert(current_coord, new_val); }
                            None => ()
                        };
                    },
                    Operation::Toggle => { grid.insert(current_coord, prev_val + 2); }
                }
            }
        }
    }
    grid.values().sum()
}

struct Day6;

impl Problem for Day6 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    // Both of these solutions work, but they are incredibly slow. Part 1 is ~5s on debug mode,
    // part 2 is ~6.5s on debug mode. There must be a more clever solution here, likely working
    // with ranges instead of individual cells in the grid
    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let instructions: Vec<Instruction> = Instruction::from_input(input);
        part_one_slow(&instructions)
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let instructions: Vec<Instruction> = Instruction::from_input(input);
        part_two_slow(&instructions)
    }
}

fn main() {
    solve_main::<Day6>();
}
