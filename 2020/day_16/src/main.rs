use std::{collections::HashMap, time::Instant};
use std::fs;
use std::io::BufRead;
use std::ops::Range;

// use std::{collections::HashSet, io, num::ParseIntError, str::FromStr};

// use problem::{CSV, Input, Problem, solve};

// struct Day16;
// impl Problem for Day16 {
//     type Input = Info;
//     type Part1Output = u32;
//     type Part2Output = u64;
//     type Error = ();

//     fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        
//     }

//     fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        
//     }
// }

struct Input {
    ticket_fields: HashMap<String, (Range<i32>, Range<i32>)>,
    my_ticket: Vec<i32>,
    nearby_tickets: Vec<Vec<i32>>
}

fn main() {
    solve::<Day16>("input").unwrap();
}

fn solve_part_one(input: Input) -> i32 {
    let mut invalid_values: Vec<i32> = Vec::new();
    for ticket in input.nearby_tickets {
        for field in ticket {
            let mut valid = false;
            for val in  input.ticket_fields.values() {
                if val.0.contains(&field) || val.1.contains(&field) {
                    valid = true;
                    break;
                }
            }
            if !valid {
                invalid_values.push(field);
            }
        }
    }
    invalid_values.iter().sum()
}

fn solve_part_two(input: Input) -> i32 {
    let mut valid_tickets: Vec<Vec<i32>> = Vec::new();
    for ticket in &input.nearby_tickets {
        let mut ticket_valid =  true;
        for field in ticket {
            let mut field_valid = false;
            for val in  input.ticket_fields.values() {
                if val.0.contains(&field) || val.1.contains(&field) {
                    field_valid = true;
                    break;
                }
            }
            if !field_valid {
                ticket_valid = false;
                break;
            }
        }
        if ticket_valid {
            valid_tickets.push(ticket.clone());
        }
    }
    let mut field_order: Vec<&str> = Vec::new();
    for ticket in valid_tickets
    {
        
    }
    0
}

fn parse_field_values(input: &Vec<String>) -> Result<HashMap<String, (Range<i32>, Range<i32>)>, &'static str> {
    if input.len() < 20 {
        return Err("Invalid input");
    }
    else {
        let mut map: HashMap<String, (Range<i32>, Range<i32>)> = HashMap::new();
        for n in 0..20 {
            let split: Vec<&str> = input[n].split(":").collect();
            let range_split: Vec<&str> = split[1].split_whitespace().collect();
            let lower_range: Vec<&str> = range_split[0].split("-").collect();
            let higher_range: Vec<&str> = range_split[2].split("-").collect();
            map.insert(split[0].to_string(), (
                Range {start: lower_range[0].parse().unwrap(), end: lower_range[1].parse::<i32>().unwrap() + 1},
                Range {start: higher_range[0].parse().unwrap(), end: higher_range[1].parse::<i32>().unwrap() + 1}
            ));
        }
        Ok(map)
    }
}

fn parse_ticket(ticket: &str) -> Vec<i32> {
    ticket.split(",").map(|n| n.parse().expect("Can't parse ticket")).collect()
}

fn get_input(filename: &str) -> Input {
    let file = fs::File::open(filename).expect("Failed to open input file");
    let lines: Vec<String> = std::io::BufReader::new(file).lines().map(|l| l.expect("Could not parse line")).collect();
    let mut other_tickets: Vec<Vec<i32>> = Vec::new();
    for t in 25..lines.len() {
        other_tickets.push(parse_ticket(&lines[t]));
    }
    Input {
        ticket_fields: parse_field_values(&lines).unwrap(),
        my_ticket: parse_ticket(&lines[22]),
        nearby_tickets: other_tickets
    }
}