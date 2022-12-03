use std::str::FromStr;
use std::collections::HashSet;
use std::string::ParseError;
use problem::{solve_main, Problem};

struct Backpack {
    left_compartment: HashSet<char>,
    right_compartment: HashSet<char>,
    unified: HashSet<char>
}

impl FromStr for Backpack {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let half_point = s.chars().count() / 2;
        let (left, right) = s.split_at(half_point);
        let left_compartment = left.chars().collect();
        let right_compartment = right.chars().collect();
        let unified = s.chars().collect();
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
        let intersection: HashSet<_> = self.left_compartment.intersection(&self.right_compartment).collect();
        if intersection.len() == 1 {
            return Some(priority(&intersection.iter().next().unwrap()))
        }
        None
    }

    pub fn get_group_badge_priority(backpacks: &[Backpack]) -> Option<u32> {
        let first_intersection: HashSet<char> = backpacks[0].unified.intersection(&backpacks[1].unified).cloned().collect();
        first_intersection.intersection(&backpacks[2].unified).map(priority).next()
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
