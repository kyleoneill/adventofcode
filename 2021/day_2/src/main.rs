use std::str::FromStr;
use problem::{Problem, solve};

enum Direction {
    Up,
    Down,
    Forward
}

struct Instruction {
    direction: Direction,
    magnitude: i32
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.trim().split(' ').collect();
        let direction: Direction = match split[0] {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => return Err(Error::FailParse)
        };
        let magnitude: i32 = split[1].parse().unwrap();
        Ok(Instruction { direction, magnitude })
    }
}

struct Position {
    horizontal: i32,
    vertical: i32,
    aim: i32
}

impl Position {
    fn new() -> Self {
        Position { horizontal: 0, vertical: 0, aim: 0 }
    }
    fn solution(&self) -> i32 {
        self.vertical * self.horizontal
    }
}

fn solve_1(values: &Vec<Instruction>) -> Option<i32> {
    let mut position = Position::new();
    for instruction in values {
        match instruction.direction {
            Direction::Forward => position.horizontal += instruction.magnitude,
            Direction::Up => position.vertical -= instruction.magnitude,
            Direction::Down => position.vertical += instruction.magnitude
        }
    }
    Some(position.solution())
}

fn solve_2(values: &Vec<Instruction>) -> Option<i32> {
    let mut position = Position::new();
    for instruction in values {
        match instruction.direction {
            Direction::Up => position.aim -= instruction.magnitude,
            Direction::Down => position.aim += instruction.magnitude,
            Direction::Forward => {
                position.horizontal += instruction.magnitude;
                position.vertical += position.aim * instruction.magnitude
            }
        }
    }
    Some(position.solution())
}

#[derive(Debug)]
enum Error {
    NoSolution,
    FailParse
}

struct Day2;
impl Problem for Day2 {
    type Input = Vec<Instruction>;
    type Part1Output = i32;
    type Part2Output = i32;
    type Error = Error;

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        let result = solve_1(input).ok_or(Error::NoSolution)?;
        Ok(result)
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        let result = solve_2(input).ok_or(Error::NoSolution)?;
        Ok(result)
    }
}

fn main() {
    solve::<Day2>("input").unwrap();
}
