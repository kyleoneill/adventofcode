use std::{collections::HashMap, str::FromStr};

use problem::{solve_main, Problem, Coord};

#[derive(Debug)]
enum Operation {
    TurnOn,
    Toggle,
    TurnOff
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    lower_left: Coord<isize>,
    upper_right: Coord<isize>,
}

#[derive(Debug)]
struct ParseInstructionError;
impl FromStr for Instruction {
    type Err = ParseInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let first = split.next().expect("Line did not contain any tokens");
        let operation = match first {
            "turn" => {
                let second = split.next().expect("Failed to get a second token for turning lights on or off");
                match second {
                    "on" => Operation::TurnOn,
                    "off" => Operation::TurnOff,
                    _ => panic!("Got an invalid turn on/off token")
                }
            },
            "toggle" => Operation::Toggle,
            _ => panic!("First token '{}' while parsing a line was invalid", first)
        };
        let first_coord_string = split.next().expect("Failed to get first coord from line");
        let top_left: Coord<isize> = Coord::from_str(first_coord_string).expect("Failed to convert first coord to Coord");
        split.next().expect("Failed to get empty 'through' token when parsing a line");
        let second_coord_string = split.next().expect("Failed to get second coord from line");
        let bottom_right: Coord<isize> = Coord::from_str(second_coord_string).expect("Failed to convert second coord to Coord");
        Ok(Self{ operation, lower_left: top_left, upper_right: bottom_right })
    }
}

impl Instruction {
    fn from_input(input: &Vec<String>) -> Vec<Self> {
        let mut instructions: Vec<Self> = Vec::new();
        for line in input {
            instructions.push(line.parse::<Instruction>().expect("Failed to parse line into instruction"));
        }
        instructions
    }
}

#[derive(Debug)]
struct Rectangle {
    lower_left: Coord<isize>,
    upper_right: Coord<isize>,
    state: usize,
}

impl Rectangle {
    fn new() -> Self {
        Self { lower_left: Coord{ x: 0, y: 0 }, upper_right: { Coord{ x: 999, y: 999 } }, state: 0 }
    }
    fn area(&self) -> isize {
        let length = self.upper_right.x - self.lower_left.x + 1;
        let width = self.upper_right.y - self.lower_left.y + 1;
        length * width
    }
    fn apply_operation_part_one(&mut self, op: &Operation) {
        match op {
            Operation::TurnOn => { self.state = 1 },
            Operation::TurnOff => { self.state = 0 },
            Operation::Toggle => { self.state = 1 - self.state },
        }
    }
    fn clamp(&mut self, other: &Self) {
        self.lower_left.x = isize::max(self.lower_left.x, other.lower_left.x);
        self.lower_left.y = isize::max(self.lower_left.y, other.lower_left.y);
        self.upper_right.x = isize::min(self.upper_right.x, other.upper_right.x);
        self.upper_right.y = isize::min(self.upper_right.y, other.upper_right.y);
    }
}

#[allow(dead_code)]
fn part_one_slow(instructions: &Vec<Instruction>) -> usize {
    let mut grid: HashMap<Coord<isize>, bool> = HashMap::new();
    for instruction in instructions {
        for x in instruction.lower_left.x..=instruction.upper_right.x {
            for y in instruction.lower_left.y..=instruction.upper_right.y {
                let current_coord: Coord<isize> = Coord { x, y };
                match instruction.operation {
                    Operation::TurnOn => { grid.insert(current_coord, true); },
                    Operation::TurnOff => { grid.insert(current_coord, false); },
                    Operation::Toggle => {
                        let prev_val: bool = *grid.get(&current_coord).unwrap_or(&false);
                        grid.insert(current_coord, !prev_val);
                    }
                }
            }
        }
    }
    grid.values().filter(|x| **x == true).count()
}

#[allow(dead_code)]
fn part_two_slow(instructions: &Vec<Instruction>) -> usize {
    let mut grid: HashMap<Coord<isize>, usize> = HashMap::new();
    for instruction in instructions {
        for x in instruction.lower_left.x..=instruction.upper_right.x {
            for y in instruction.lower_left.y..=instruction.upper_right.y {
                let current_coord: Coord<isize> = Coord { x, y };
                let prev_val: usize = *grid.get(&current_coord).unwrap_or(&0);
                match instruction.operation {
                    Operation::TurnOn => { grid.insert(current_coord, prev_val + 1); },
                    Operation::TurnOff => {
                        match prev_val.checked_sub(1) {
                            Some(new_val) => { grid.insert(current_coord, new_val); }
                            None => ()
                        };
                    },
                    Operation::Toggle => { grid.insert(current_coord, prev_val + 2); }
                }
            }
        }
    }
    grid.values().sum()
}

// Attempt to get a more efficient solution, not finished yet
fn part_one_fast(instructions: &Vec<Instruction>) -> isize {
    // This views the problem as having a start of one rectangle and each instruction is an input of one rectangle, which can break the starting rectangle into up to 5 new ones.
    // When a given existing rectangle is checked against an instruction, that rectangle can be thought of as being broken into rectangles A through E, like
    // +-------------------+
    // |        E          |
    // +-----+------+------+
    // | B   |  C   |   D  |     
    // +-----+------+------+       
    // |        A          |
    // +-------------------+
    // Where the overall shape is our starting rectangle, C is the rectangle which will have its state changed, and all other rectangles
    // will be new ones which inherit the state of the starting rectangle
    
    // Maybe having issues due to too many recursive rectangles? Might be running for forever, might need some base case? A way to reduce # of items in the vec?
    let mut rectangles: Vec<Rectangle> = vec![Rectangle::new()];
    for instruction in instructions {
        let mut new_rectangles: Vec<Rectangle> = Vec::new();
        for current_rectangle in rectangles {
            let mut rec_a = Rectangle{ lower_left: current_rectangle.lower_left, upper_right: Coord{ x: current_rectangle.upper_right.x, y: instruction.lower_left.y - 1 }, state: current_rectangle.state };
            let mut rec_b = Rectangle{ lower_left: Coord{ x: current_rectangle.lower_left.x, y: instruction.lower_left.y }, upper_right: Coord{ x: instruction.lower_left.x - 1, y: instruction.upper_right.y }, state: current_rectangle.state };
            let mut rec_c = Rectangle{ lower_left: instruction.lower_left, upper_right: instruction.upper_right, state: current_rectangle.state };
            let mut rec_d = Rectangle{ lower_left: Coord{ x: instruction.upper_right.x + 1, y: instruction.lower_left.y }, upper_right: Coord{ x: current_rectangle.upper_right.x, y: instruction.upper_right.y }, state: current_rectangle.state };
            let mut rec_e = Rectangle{ lower_left: Coord{ x: current_rectangle.lower_left.x, y: instruction.upper_right.y + 1 }, upper_right: current_rectangle.upper_right, state: current_rectangle.state };

            rec_c.apply_operation_part_one(&instruction.operation);

            rec_a.clamp(&current_rectangle);
            rec_b.clamp(&current_rectangle);
            rec_c.clamp(&current_rectangle);
            rec_d.clamp(&current_rectangle);
            rec_e.clamp(&current_rectangle);

            if rec_a.area() > 0 {
                new_rectangles.push(rec_a);
            }
            if rec_b.area() > 0 {
                new_rectangles.push(rec_b);
            }
            if rec_c.area() > 0 {
                new_rectangles.push(rec_c);
            }
            if rec_d.area() > 0 {
                new_rectangles.push(rec_d);
            }
            if rec_e.area() > 0 {
                new_rectangles.push(rec_e);
            }
        }
        rectangles = new_rectangles;
    }
    rectangles.iter().filter(|rec| rec.state == 1).map(|rec| rec.area()).sum()
}

struct Day6;

impl Problem for Day6 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    // Both of these solutions work, but they are incredibly slow. Part 1 is ~5s on debug mode,
    // part 2 is ~6.5s on debug mode. There must be a more clever solution here, likely working
    // with ranges instead of individual cells in the grid
    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let instructions: Vec<Instruction> = Instruction::from_input(input);
        part_one_slow(&instructions)
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let instructions: Vec<Instruction> = Instruction::from_input(input);
        part_two_slow(&instructions)
    }
}

fn main() {
    solve_main::<Day6>();
}
