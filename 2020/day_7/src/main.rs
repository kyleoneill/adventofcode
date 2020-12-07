use std::{time::Instant, num::ParseIntError};
use std::fs;
use std::io::BufRead;
use std::str::FromStr;

/*
------------------------------
my solution is slow and sub-optimal for part one. Read about Graph Thoery and use a HashMap instead of Vec<Bag>
------------------------------
*/

fn remove_last_char_if(mut s: String, c: char) -> String {
    if s.chars().last().unwrap() == c {
        s.truncate(s.len() - 1);
    }
    s
}

struct Bag {
    name: String,
    contents: Vec<ContainedBag>
}

impl FromStr for Bag {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_bag_and_policy: Vec<&str> = s.split("contain").collect();
        let mut built_name: String = split_bag_and_policy[0].trim().to_owned();
        built_name = remove_last_char_if(built_name, 's');
        let contents = ContainedBag::vector_from_str(split_bag_and_policy[1]);
        Ok(Bag {
            name: built_name,
            contents
        })
    }
}

struct ContainedBag {
    name: String,
    amount: i32
}

impl FromStr for ContainedBag {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s  = s.trim();
        if s == "no other bags." {
            return Ok(ContainedBag {
                name: "empty".to_string(),
                amount: 0
            })
        }
        let mut split: Vec<&str> = s.split_whitespace().collect();
        let amount: i32 = FromStr::from_str(split.remove(0)).unwrap();
        let mut name = split.join(" ");
        name = remove_last_char_if(name, '.');
        name = remove_last_char_if(name, 's');
        Ok(ContainedBag {
            name,
            amount
        })
    }
}

impl ContainedBag {
    fn vector_from_str(s: &str) -> Vec<Self> {
        let mut output: Vec<Self> = Vec::new();
        let policies = s.split(",");
        for policy in policies {
            output.push(policy.parse().unwrap());
        }
        output
    }
}

fn main() {
    let start = Instant::now();
    let input = get_input("input.txt");
    let solution = solve_part_two(&input, "shiny gold bag") - 1;
    println!("Found solution in {} microseconds", start.elapsed().as_micros());
    println!("The solution is {}", solution);
}

fn solve_part_one(input: Vec<Bag>) -> i32 {
    let mut possible_bags: Vec<String> = vec!["shiny gold bag".to_string()];
    loop {
        let starting_len = possible_bags.len();
        for bag in &input {
            for contained_bag in &bag.contents {
                if possible_bags.contains(&contained_bag.name) {
                    if !possible_bags.contains(&bag.name) {
                        possible_bags.push(bag.name.clone());
                    }
                }
            }
        }
        if possible_bags.len() == starting_len {
            break;
        }
    }
    (possible_bags.len() as i32) - 1
}

fn solve_part_two(input: &Vec<Bag>, bag_name: &str) -> i32 {
    let mut counter = 1;
    let container = input.iter().find(|bag| bag.name == bag_name).expect(&format!("Couldn't find {}", bag_name));
    for contained_bag in &container.contents {
        if contained_bag.name == "empty" {
            continue
        }
        counter += contained_bag.amount * solve_part_two(&input, &contained_bag.name);
    }
    counter
}

fn get_input(filename: &str) -> Vec<Bag> {
    let mut contents: Vec<Bag> = Vec::new();
    let file = fs::File::open(filename).expect("Failed to open input file");
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(line) = line {
            contents.push(line.trim().parse().expect("Failed to convert input to i32"));
        }
    }
    contents
}