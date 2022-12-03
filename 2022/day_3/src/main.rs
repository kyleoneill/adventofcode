use std::str::FromStr;
use std::collections::HashMap;
use std::string::ParseError;
use problem::{solve_main, Problem};

struct Compartment {
    compartment: HashMap<char, u32>
}

impl Compartment {
    pub fn from_str(input: &str) -> Self {
        let mut compartment: HashMap<char, u32> = HashMap::new();
        for c in input.chars() {
            *compartment.entry(c).or_insert(0) += 1;
        }
        Self { compartment }
    }
}

struct Backpack {
    left_compartment: Compartment,
    right_compartment: Compartment,
    unified: Compartment
}

impl FromStr for Backpack {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let half_point = s.chars().count() / 2;
        let (left, right) = s.split_at(half_point);
        let left_compartment = Compartment::from_str(left);
        let right_compartment = Compartment::from_str(right);
        let unified = Compartment::from_str(s);
        Ok(Self{ left_compartment, right_compartment, unified })
    }
}

impl Backpack {
    pub fn from_str_vec(input: &Vec<String>) -> Vec<Self> {
        let mut output = Vec::new();
        for i in input {
            let backpack = Backpack::from_str(&i).unwrap();
            output.push(backpack);
        }
        output
    }

    pub fn get_priority(&self) -> Option<u32> {
        for key in self.left_compartment.compartment.keys() {
            if self.right_compartment.compartment.contains_key(key) {
                return Some(priority(key));
            }
        }
        None
    }

    pub fn get_group_badge_priority(backpacks: &[Backpack]) -> Option<u32> {
        for b in backpacks {

        }
        Some(0)
    }
}

/// This cursed function converts a char into an ascii position and then shifts it
pub fn priority(x: &char) -> u32 {
    let x = *x as u32;
    if x >= 'a' as u32 && x <= 'z' as u32 {
        // Subtracting by 'a' brings the char code into the scope we want it.
        // Adding 1 removes the zero based index this introduces
        // e.g. g is 147 so this would print g as (147 - 141 + 1) which is 7
        (x - 'a' as u32) + 1
    } else {
        // Add 27 (26 + 1 to remove zero based index) rather than just the 1 as the prompt
        // wants 'A' to be 27
        (x - 'A' as u32) + 27
    }
}

struct Day3;

impl Problem for Day3 {
    type Input = Vec<String>;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let backpacks = Backpack::from_str_vec(input);
        let mut sum = 0;
        for backpack in backpacks {
            sum += backpack.get_priority().expect("Did not find any overlap when comparing backpack compartments");
        }
        sum
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let backpacks = Backpack::from_str_vec(input);
        let mut sum = 0;
        for b in (0..backpacks.len()).step_by(3) {
            sum += Backpack::get_group_badge_priority(&backpacks[b..b+3]).expect("Could not find badge for group of elves");
        }
        sum
    }
}

fn main() {
    solve_main::<Day3>();
}
