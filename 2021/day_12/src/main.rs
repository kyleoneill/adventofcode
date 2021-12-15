use std::str::FromStr;

use problem::{Problem, solve_main};
use anyhow::{Result, Error};

#[derive(Debug)]
struct CaveConnection {
    start: String,
    end: String
}

impl FromStr for CaveConnection {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("-");
        Ok(CaveConnection{ start: split.next().unwrap().to_string(), end: split.next().unwrap().to_string() })
    }
}

struct Day11;
impl Problem for Day11 {
    type Input = Vec<CaveConnection>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        for connection in input {
            dbg!(connection);
        }
        0
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        0
    }
}

fn main() {
    solve_main::<Day11>();
}
