use std::str::FromStr;
use std::string::ParseError;
use problem::{solve_main, Problem};

fn protected_add_to_vec_at_index(vec: &mut Vec<Vec<char>>, index: usize, c: char) {
    while vec.len() < index + 1 {
        vec.push(Vec::new());
    }
    vec[index].push(c);
}

struct Crates {
    crates: Vec<Vec<char>>
}

impl Crates {
    fn from_string_vec(input: &[String]) -> Self {
        let mut lines: Vec<String> = input.to_vec();
        lines.reverse();
        let mut crates: Vec<Vec<char>> = Vec::new();
        for line in lines {
            let chars: Vec<char> = line.chars().collect();
            for i in 0..chars.len() {
                if chars[i] == '[' {
                    let index = match i {
                        0 => 0,
                        _ => i / 4
                    };
                    protected_add_to_vec_at_index(&mut crates, index, chars[i + 1]);
                }
            }
        }
        Self { crates }
    }

    fn run_instruction_old(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.amount_to_move {
            let c = self.crates[instruction.source_index].pop().unwrap();
            self.crates[instruction.destination_index].push(c);
        }
    }

    fn run_instruction_new(&mut self, instruction: &Instruction) {
        let start_index = self.crates[instruction.source_index].len() - instruction.amount_to_move as usize;
        let drained: Vec<char> = self.crates[instruction.source_index].drain(start_index..).collect();
        for c in drained {
            self.crates[instruction.destination_index].push(c);
        }
    }

    fn get_top_crates(&self) -> String {
        let mut chars: Vec<char> = Vec::new();
        for stack in &self.crates {
            chars.push(stack[stack.len() - 1]);
        }
        chars.iter().collect()
    }
}

struct Instruction {
    amount_to_move: u32,
    source_index: usize,
    destination_index: usize
}

impl FromStr for Instruction {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split_whitespace().collect();
        Ok(Self { amount_to_move: split[1].parse().unwrap(), source_index: split[3].parse::<usize>().unwrap() - 1, destination_index: split[5].parse::<usize>().unwrap() - 1 })
    }
}

impl Instruction {
    fn from_string_vec(input: &[String]) -> Vec<Self> {
        let mut ret = Vec::new();
        for line in input {
            let instruction = Instruction::from_str(line).unwrap();
            ret.push(instruction);
        }
        ret
    }
}

fn read_input(input: &Vec<String>) -> (Crates, Vec<Instruction>) {
    // There has to be a better way to do this but I just finished hackathon and my brain ain't braining
    let mut begin_instructions_index: usize = 0;
    let mut counter = 0;
    for i in input {
        if begin_instructions_index == 0 {
            if i.is_empty() {
                begin_instructions_index = counter;
                break;
            }
            counter += 1;
        }
    }
    let crates = Crates::from_string_vec(&input[0..begin_instructions_index]);
    let instructions = Instruction::from_string_vec(&input[begin_instructions_index + 1..input.len()]);
    (crates, instructions)
}

struct Day4;

impl Problem for Day4 {
    type Input = Vec<String>;
    type PartOne = String;
    type PartTwo = String;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let (mut crates, instructions) = read_input(input);
        for instruction in instructions {
            crates.run_instruction_old(&instruction);
        }
        crates.get_top_crates()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let (mut crates, instructions) = read_input(input);
        for instruction in instructions {
            crates.run_instruction_new(&instruction);
        }
        crates.get_top_crates()
    }
}

fn main() {
    solve_main::<Day4>();
}
