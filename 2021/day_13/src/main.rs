use std::str::FromStr;
use std::collections::{VecDeque, HashMap};

use problem::{Problem, solve_main};
use anyhow::{Result, Error};

struct Day13;
impl Problem for Day13 {
    type Input = Vec<i32>;
    type PartOne = usize;
    type PartTwo = usize;

    // breadth first search

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        0
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        0
    }
}

fn main() {
    solve_main::<Day13>();
}
