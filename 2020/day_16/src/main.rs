use std::{collections::HashMap, time::Instant};
use std::fs;
use std::ops::Range;

struct Input {
    ticket_fields: HashMap<String, (Range<i32>, Range<i32>)>,
    my_ticket: Vec<i32>,
    nearby_tickets: Vec<Vec<i32>>
}

impl Input {
    #[allow(dead_code)]
    fn print_input(&self) {
        println!("Ticket Fields:");
        for (key, value) in self.ticket_fields.iter() {
            println!("{}: {:?}-{:?}", key, value.0, value.1);
        }
        println!("My Ticket: ");
        for field in &self.my_ticket {
            print!("{}, ", field);
        }
        println!("\nNearby Tickets: ");
        for ticket in &self.nearby_tickets {
            println!("\nTicket: ");
            for field in ticket {
                print!("{}, ", field);
            }
        }
    }
}

fn main() {
    let start = Instant::now();
    let input = get_input("input.txt");
    let solution = solve_part_two(input);
    println!("Found solution in {} microseconds", start.elapsed().as_micros());
    println!("The solution is {}", solution);
}

#[allow(dead_code)]
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

fn solve_part_two(input: Input) -> u64 {
    let valid_tickets = get_valid_tickets(&input);
    let field_order = get_field_order(&input, valid_tickets);
    let mut counter: u64 = 0;
    for (i, field) in field_order.iter().enumerate() {
        let name = field.keys().next().unwrap();
        if name.starts_with("depa") {
            let val = input.my_ticket[i] as u64;
            if counter == 0 {
                counter = val;
            }
            else {
                counter *= val;
            }
        }
    }
    counter
}

fn get_field_order(input: &Input, valid_tickets: Vec<Vec<i32>>) -> Vec<HashMap<String, (Range<i32>, Range<i32>)>> {
    let mut field_order: Vec<HashMap<String, (Range<i32>, Range<i32>)>> = Vec::new();
    for field in 0..valid_tickets[0].len() {
        let mut potential_fields = input.ticket_fields.clone();
        let mut invalids: Vec<String> = Vec::new();
        for ticket in 0..valid_tickets.len() {
            let value = valid_tickets[ticket][field];
            for (ticket_field, ranges) in potential_fields.iter() {
                if !(ranges.0.contains(&value) || ranges.1.contains(&value)) {
                    if !invalids.contains(&ticket_field) {
                        invalids.push(ticket_field.clone());
                    }
                }
            }
        }
        for val in invalids {
            potential_fields.remove_entry(&val);
        }
        field_order.push(potential_fields);
    }
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..field_order.len() {
            if field_order[i].len() == 1 {
                for j in 0..field_order.len() {
                    if i != j {
                        let asdf = field_order[i].keys().next().unwrap().clone();
                        match field_order[j].remove(&asdf) {
                            Some(_removed_val) => changed = true,
                            None => ()
                        }
                    }
                }
            }
        }
    }
    field_order
}

fn get_valid_tickets(input: &Input) -> Vec<Vec<i32>> {
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
    valid_tickets
}

fn parse_field_values(input: &str) -> Result<HashMap<String, (Range<i32>, Range<i32>)>, &'static str> {
    let split_line: Vec<&str> = input.split("\n").collect();
    let mut map: HashMap<String, (Range<i32>, Range<i32>)> = HashMap::new();
    for n in 0..split_line.len() {
        let split: Vec<&str> = split_line[n].split(":").collect();
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

fn parse_ticket(ticket: &str) -> Vec<i32> {
    ticket.split(",").map(|n| n.trim().parse().expect("Can't parse ticket")).collect()
}

fn get_input(filename: &str) -> Input {
    let file_text = fs::read_to_string(filename).unwrap();
    let mut split = file_text.split("\r\n\r\n");
    let fields = split.next().unwrap();
    let my_ticket = split.next().unwrap().split("\n").skip(1).next().unwrap();
    let other_tickets: Vec<Vec<i32>> = split.next().unwrap().split("\n").skip(1).map(|ticket| parse_ticket(ticket)).collect();
    Input {
        ticket_fields: parse_field_values(fields).unwrap(),
        my_ticket: parse_ticket(my_ticket),
        nearby_tickets: other_tickets
    }
}