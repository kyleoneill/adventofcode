use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let result_line = line?;
        let values: Vec<i32> = result_line.split(',').map(|x| x.parse()).collect::<Result<_,_>>().unwrap();
        let mut solution = Vec::new();
        'outer: for noun in 0..100 {
            'inner: for verb in 0..100 {
                let mut modified_values = values.clone();
                modified_values[1] = noun;
                modified_values[2] = verb;
                let result = run_opcode(modified_values);
                if result == 19690720 {
                    solution.push(noun);
                    solution.push(verb);
                    break 'outer;
                }
            }
        }
        println!("The noun is: {}", solution[0].to_string());
        println!("The verb is: {}", solution[1].to_string());
        println!("The solution is: {}", (100 * solution[0] + solution[1]).to_string());
    }
    Ok(())
}

fn run_opcode(mut program: Vec<i32>) -> i32 {
    let mut current_opcode = 0;
    loop {
        let first: usize = program[current_opcode + 1] as usize;
        let second: usize = program[current_opcode + 2] as usize;
        let result: usize = program[current_opcode + 3] as usize;
        if program[current_opcode] == 1 {
            program[result] = program[first] + program[second];
            current_opcode += 4;
        }
        else if program[current_opcode] == 2 {
            program[result] = program[first] * program[second];
            current_opcode += 4;
        }
        else if program[current_opcode] == 99 {
            current_opcode += 1;
            break;
        }
        else {
            panic!("Opcode at position {} is not valid", current_opcode.to_string());
        }
    }
    program[0]
}
