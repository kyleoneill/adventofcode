use std::{time::Instant, num::ParseIntError};
use std::fs;
use std::io::BufRead;
use std::str::FromStr;

struct PasswordPolicy {
    letter_range: (i32, i32),
    letter: char,
    password: String
}

impl PasswordPolicy {
    pub fn is_valid_problem_one(&self) -> bool {
        let mut char_counter = 0;
        for pass_char in self.password.chars() {
            if pass_char.eq(&self.letter) {
                char_counter += 1;
            }
            if char_counter > self.letter_range.1 {
                return false
            }
        }
        if char_counter < self.letter_range.0 {
            return false
        }
        true
    }
    pub fn is_valid_problem_two(&self) -> bool {
        let mut counter = 0;
        let password: Vec<char> = self.password.chars().collect();
        if password[(self.letter_range.0 - 1) as usize] == self.letter {
            counter += 1;
        }
        if password[(self.letter_range.1 - 1) as usize] == self.letter {
            counter += 1;
        }
        if counter == 1 {
            return true
        }
        false
    }
}

impl FromStr for PasswordPolicy {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_policy: Vec<&str> = s.split_whitespace().collect();
        let number_range: Vec<&str> = split_policy[0].split_terminator('-').collect();
        Ok(PasswordPolicy {
            letter_range: (number_range[0].parse().unwrap(), number_range[1].parse().unwrap()),
            letter: split_policy[1].chars().next().unwrap(),
            password: split_policy[2].to_owned()
        })
    }
}

fn main() {
    let start = Instant::now();
    let input = get_input("input.txt");
    let solution = solve(input);
    println!("Found solution in {} microseconds", start.elapsed().as_micros());
    println!("The solution is {}", solution);
}

fn solve(input: Vec<PasswordPolicy>) -> i32 {
    let mut correct_passwords = 0;
    for policy in input {
        // if policy.is_valid_problem_one() {
        //     correct_passwords += 1;
        // }
        if policy.is_valid_problem_two() {
            correct_passwords += 1;
        }
    }
    correct_passwords
}

fn get_input(filename: &str) -> Vec<PasswordPolicy> {
    let mut contents: Vec<PasswordPolicy> = Vec::new();
    let file = fs::File::open(filename).expect("Failed to open input file");
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(line) = line {
            contents.push(line.parse().expect("Could not parse password policy"));
        }
    }
    contents
}