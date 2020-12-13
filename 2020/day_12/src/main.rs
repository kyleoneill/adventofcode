#![allow(dead_code)]
use std::{num::ParseIntError, str::FromStr, time::Instant};
use std::fs;
use std::io::BufRead;

enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn new_direction(current_dir: &Self, mag: i32) -> Self {
        let new_direction = (current_dir.to_i32() + mag + 360) % 360;
        Self::from_u32(new_direction as u32)
    }
    fn from_u32(value: u32) -> Self {
        match value {
            0 => Direction::North,
            90 => Direction::East,
            180 => Direction::South,
            270 => Direction::West,
            _ => panic!(format!("u32: {} does not match the direction varient", value))
        }
    }
    fn to_i32(&self) -> i32 {
        match self {
            Direction::North => 0,
            Direction::East => 90,
            Direction::South => 180,
            Direction::West => 270
        }
    }
}

struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position {
            x,
            y
        }
    }
    fn manhatten_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

struct WayPoint {
    position: Position
}

impl WayPoint {
    fn rotate_around_boat(&mut self, mut mag: i32) {
        if mag < 0 {
            mag += 360;
        }
        let old = Position::new(self.position.x, self.position.y);
        match mag {
            90 => {
                self.position.x = old.y;
                self.position.y = -old.x;
            },
            180 => {
                self.position.x = -old.x;
                self.position.y = -old.y;
            },
            270 => {
                self.position.x = -old.y;
                self.position.y = old.x;
            },
            _ => ()
        }
    }
}

struct Boat {
    position: Position,
    waypoint: WayPoint,
    facing_direction: Direction
}

impl Boat {
    fn new() -> Self {
        Boat {
            position: Position::new(0, 0),
            waypoint: WayPoint{position: Position::new(10, 1)},
            facing_direction: Direction::East
        }
    }
    //part one
    fn follow_instruction(&mut self, instruction: Instruction) {
        match instruction.instruction {
            'N' => self.position.y += instruction.magnitude,
            'S' => self.position.y -= instruction.magnitude,
            'E' => self.position.x += instruction.magnitude,
            'W' => self.position.x -= instruction.magnitude,
            'L' => {
                self.facing_direction = Direction::new_direction(&self.facing_direction, instruction.magnitude * -1);
            },
            'R' => {
                self.facing_direction = Direction::new_direction(&self.facing_direction, instruction.magnitude);
            },
            'F' => {
                match self.facing_direction {
                    Direction::North => self.position.y += instruction.magnitude,
                    Direction::South => self.position.y -= instruction.magnitude,
                    Direction::East => self.position.x += instruction.magnitude,
                    Direction::West => self.position.x -= instruction.magnitude
                }
            },
            _ => panic!("Undefined instruction")
        }
    }
    //part two
    fn follow_waypoint(&mut self, instruction: Instruction) {
        match instruction.instruction {
            'N' => self.waypoint.position.y += instruction.magnitude,
            'S' => self.waypoint.position.y -= instruction.magnitude,
            'E' => self.waypoint.position.x += instruction.magnitude,
            'W' => self.waypoint.position.x -= instruction.magnitude,
            'L' => {
                self.waypoint.rotate_around_boat(instruction.magnitude * -1);
            },
            'R' => {
                self.waypoint.rotate_around_boat(instruction.magnitude);
            },
            'F' => {
                self.position.x += self.waypoint.position.x * instruction.magnitude;
                self.position.y += self.waypoint.position.y * instruction.magnitude;
            },
            _ => panic!("Undefined instruction")
        }
    }
}

struct Instruction {
    instruction: char,
    magnitude: i32
}

impl FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction = &s[0..1];
        let magnitude = s[1..].parse().unwrap();
        Ok(Instruction {
            instruction: instruction.chars().next().unwrap(),
            magnitude
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

fn solve(instructions: Vec<Instruction>) -> i32 {
    let mut boat = Boat::new();
    for instruction in instructions {
        //boat.follow_instruction(instruction); // <-- part one solution
        boat.follow_waypoint(instruction); // <-- part two solution
    }
    boat.position.manhatten_distance()
}

fn get_input(filename: &str) -> Vec<Instruction> {
    let mut contents: Vec<Instruction> = Vec::new();
    let file = fs::File::open(filename).expect("Failed to open input file");
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(line) = line {
            contents.push(line.parse().expect("Failed to parse instruction"));
        }
    }
    contents
}