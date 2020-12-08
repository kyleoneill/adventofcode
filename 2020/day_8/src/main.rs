use std::{num::ParseIntError, str::FromStr, time::Instant};
use std::fs;
use std::io::BufRead;

#[derive(PartialEq, Eq, Clone)]
enum Instruction {
    Nop,
    Acc,
    Jmp
}

#[derive(Clone)]
struct OpCode {
    instruction: Instruction,
    value: i32,
    visited: bool
}

impl FromStr for OpCode {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.trim().split_whitespace().collect();
        let instruction = match split[0] {
            "nop" => Instruction::Nop,
            "acc" => Instruction::Acc,
            "jmp" => Instruction::Jmp,
            _ => panic!("Undefined instruction")
        };
        Ok(OpCode {
            instruction,
            value: split[1].parse::<i32>().unwrap(),
            visited: false
        })
    }
}

fn main() {
    let start = Instant::now();
    let input = get_input("input.txt");
    let solution = fix_boot_loop(input);
    println!("Found solution in {} microseconds", start.elapsed().as_micros());
    println!("The solution is {}", solution);
}

fn fix_boot_loop(input: Vec<OpCode>) -> i32 {
    let mut last_instruction_modified_position = 0;
    loop {
        let mut loop_set = input.clone();
        //Swap next nop/jmp instruction
        for pos in last_instruction_modified_position..loop_set.len() {
            match loop_set[pos].instruction {
                Instruction::Nop => {
                    loop_set[pos].instruction = Instruction::Jmp;
                    last_instruction_modified_position = pos;
                    break;
                },
                Instruction::Jmp => {
                    loop_set[pos].instruction = Instruction::Nop;
                    last_instruction_modified_position = pos;
                    break;
                },
                _ => ()
            }
        }
        //make sure a single-instruction infinite loop wasn't created
        let op = &loop_set[last_instruction_modified_position];
        if !(op.instruction == Instruction::Jmp && op.value == 0) {
            //test modified instruction set, return acc if it did not loop
            let res =  does_input_loop(loop_set);
            if !res.0 {
                return res.1;
            } 
        }
        last_instruction_modified_position += 1;
    }
}

fn does_input_loop(mut input: Vec<OpCode>) -> (bool, i32) {
    let mut acc = 0;
    let mut stack_position = 0;
    loop {
        if stack_position == input.len() {
            return (false, acc);
        }
        let op: &mut OpCode = &mut input[stack_position];
        if op.visited {
            break;
        }
        match op.instruction {
            Instruction::Nop => {
                stack_position += 1;
            },
            Instruction::Acc => {
                acc += op.value;
                stack_position += 1;
            },
            Instruction::Jmp => {
                if op.value.is_positive() {
                    stack_position += op.value as usize;
                }
                else {
                    stack_position -= (op.value * -1) as usize;
                }
            }
        };
        op.visited = true;
    }
    (true, acc)
}

fn get_input(filename: &str) -> Vec<OpCode> {
    let mut contents: Vec<OpCode> = Vec::new();
    let file = fs::File::open(filename).expect("Failed to open input file");
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(line) = line {
            if !line.is_empty() {
                contents.push(line.parse().expect("Failed to convert input to OpCode"));
            }
        }
    }
    contents
}