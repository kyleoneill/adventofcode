use std::collections::HashMap;

use problem::{CSV, One, Problem, solve};

struct Day15;

impl Problem for Day15 {
    type Input = One<CSV<u32>>;
    type Part1Output = u32;
    type Part2Output = u32;
    type Error = ();

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        Ok(memory_game(&input.0.values, 2020))
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        Ok(memory_game(&input.0.values, 30000000))
    }
}

fn memory_game(input: &Vec<u32>, final_number: u32) -> u32 {
    let mut tracker: HashMap<u32, u32> = HashMap::new(); //Tracker<number (key), last turn seen (value)>
    for n in 0..input.len() - 1 {
        tracker.insert(input[n], n as u32);
    }
    let mut prev_number = input[input.len() - 1];
    for n in input.len()..final_number as usize {
        match tracker.get(&prev_number).cloned() {
            Some(val) => {
                tracker.insert(prev_number, n as u32 - 1);
                prev_number = (n as u32 - 1) - val;
            },
            None => {
                tracker.insert(prev_number, n as u32 - 1);
                prev_number = 0;
            }
        }
    }
    prev_number
}

fn main() {
    solve::<Day15>("input.txt").unwrap();
}
