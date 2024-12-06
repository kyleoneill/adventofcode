use std::{collections::HashMap, str::FromStr};

use problem::{solve_main, Problem};

#[derive(Debug, Clone)]
enum Data {
    Literal(u16),
    Wire(String)
}

impl Data {
    fn resolve(&self, map: &mut HashMap<String, Operation>) -> u16 {
        match self {
            Data::Literal(val) => *val,
            Data::Wire(wire_name) => {
                let val = get_value_of_wire(map, wire_name.as_str());
                // Memoize the data we got so we don't need to recompute this wire later
                map.insert(wire_name.to_owned(), Operation::Pass(Data::Literal(val)));
                return val
            },
        }
    }
}

#[derive(Debug)]
struct ParseDataError;
impl FromStr for Data {
    type Err = ParseDataError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u16>() {
            Ok(num) => Ok(Data::Literal(num)),
            Err(_) => Ok(Data::Wire(s.to_owned()))
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    And(Data, Data),
    Or(Data, Data),
    LeftShift(Data, Data),
    RightShift(Data, Data),
    BitwiseComplement(Data),
    Pass(Data)
}

fn parse_input(input: &Vec<String>) -> HashMap<String, Operation> {
    let mut map: HashMap<String, Operation> = HashMap::new();
    for line in input {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() == 3 {
            let left = Data::from_str(tokens[0]).unwrap();
            // Output should always be a wire, never a literal
            let output_wire = tokens[2].to_owned();
            let operation = Operation::Pass(left);
            map.insert(output_wire, operation);
        }
        else if tokens.len() == 5 {
            let output_wire = tokens[4].to_owned();
            let left = Data::from_str(tokens[0]).unwrap();
            let right = Data::from_str(tokens[2]).unwrap();
            let operation = match tokens[1] {
                "AND" => Operation::And(left, right),
                "OR" => Operation::Or(left, right),
                "LSHIFT" => Operation::LeftShift(left, right),
                "RSHIFT" => Operation::RightShift(left, right),
                _ => panic!("Got an undefined operation")
            };
            map.insert(output_wire, operation);
        }
        else if tokens.len() == 4 {
            let output_wire = tokens[3].to_owned();
            let data = Data::from_str(tokens[1]).unwrap();
            let operation = Operation::BitwiseComplement(data);
            map.insert(output_wire, operation);
        }
        else {
            panic!("Got an undefined number of tokens")
        }
    }
    map
}

fn get_value_of_wire(map: &mut HashMap<String, Operation>, desired_wire: &str) -> u16 {
    let operation = map.get(desired_wire).expect("Failed to find wire in map").clone();
    match operation {
        Operation::And(first_wire, second_wire) => {
            let first_wire_val = first_wire.resolve(map);
            let second_wire_val = second_wire.resolve(map);
            first_wire_val & second_wire_val
        },
        Operation::Or(first_wire, second_wire) => {
            let first_wire_val = first_wire.resolve(map);
            let second_wire_val = second_wire.resolve(map);
            first_wire_val | second_wire_val
        },
        Operation::LeftShift(left, right) => {
            let left_resolved = left.resolve(map);
            let right_resolved = right.resolve(map);
            left_resolved << right_resolved
        },
        Operation::RightShift(left, right) => {
            let left_resolved = left.resolve(map);
            let right_resolved = right.resolve(map);
            left_resolved >> right_resolved
        },
        Operation::BitwiseComplement(data) => {
            let resolved = data.resolve(map);
            !resolved
        },
        Operation::Pass(data) => {
            data.resolve(map)
        }
    }
}

struct Day7;

impl Problem for Day7 {
    type Input = Vec<String>;
    type PartOne = u16;
    type PartTwo = u16;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut map = parse_input(input);
        get_value_of_wire(&mut map, "a")
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut map = parse_input(input);

        // Have to clone the input here as we need an unmodified map to run a second time
        let mut cloned_map = map.clone();
        let value_from_a = get_value_of_wire(&mut cloned_map, "a");

        map.insert("b".to_owned(), Operation::Pass(Data::Literal(value_from_a)));
        get_value_of_wire(&mut map, "a")
    }
}

fn main() {
    solve_main::<Day7>();
}
