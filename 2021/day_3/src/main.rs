use std::str::FromStr;
use problem::{Problem, solve};

fn binary_to_int(input: &str) -> i32 {
    const base: i32 = 2;
    let mut num = 0;
    for (i, c) in input.chars().rev().enumerate() {
        if c == '1' {
            num += base.pow(i as u32);
        }
    }
    num
}

struct PowerConsumption {
    gamma_rate: String,
    epsilon_rate: String
}

impl PowerConsumption {
    fn new() -> Self {
        PowerConsumption { gamma_rate: String::from(""), epsilon_rate: String::from("")}
    }
    fn set_epsilon_rate(&mut self) -> Result<(), Error> {
        for c in self.gamma_rate.chars() {
            match c {
                '0' => self.epsilon_rate.push('1'),
                '1' => self.epsilon_rate.push('0'),
                _ => return Err(Error::FailParse)
            }
        }
        Ok(())
    }
    fn solution(&self) -> i32 {
        binary_to_int(&self.gamma_rate) * binary_to_int(&self.epsilon_rate)
    }
}

fn solve_1(input: &Vec<String>) -> Option<i32> {
    let length = input.len();
    let width = input[0].chars().count();
    let mut power_consumption = PowerConsumption::new();
    for n in 0..width {
        let mut one = 0;
        let mut zero = 0;
        for m in 0..length {
            match input[m].chars().nth(n).unwrap() {
                '0' => zero += 1,
                '1' => one += 1,
                _ => return None
            }
        }
        if one > zero {
            power_consumption.gamma_rate.push('1');
        }
        else {
            power_consumption.gamma_rate.push('0');
        }
    }
    power_consumption.set_epsilon_rate().unwrap();
    Some(power_consumption.solution())
}

fn solve_2(input: &Vec<String>) -> Option<i32> {
    todo!()
}

#[derive(Debug)]
enum Error {
    NoSolution,
    FailParse
}

struct Day3;
impl Problem for Day3 {
    type Input = Vec<String>;
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
    solve::<Day3>("input").unwrap();
}
