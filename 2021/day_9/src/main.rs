use problem::{Problem, solve};
use std::str::FromStr;

struct VentRow {
    values: Vec<i32>
}

impl FromStr for VentRow {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const RADIX: u32 = 10;
        let mut values: Vec<i32> = Vec::new();
        for c in s.chars() {
            values.push(c.to_digit(RADIX).unwrap() as i32);
        }
        Ok(VentRow{values})
    }
}

#[derive(Debug)]
enum Error {
    NoSolution
}

struct SmallestBasins {
    smallest: i32,
    second: i32,
    third: i32
}

impl SmallestBasins {
    fn new() -> Self {
        SmallestBasins { smallest: 0, second: 0, third: 0 }
    }
    fn solve(&self) -> i32 {
        self.smallest * self.second * self.third
    }
}

fn is_low_point(input: &Vec<VentRow>, i: usize, j: usize, val: &i32) -> bool {
    let mut is_low_point = true;
    // check up
    if i != 0 {
        if val >= &input[i - 1].values[j] {
            is_low_point = false;
        }
    }
    // check down
    if i != input.len() - 1 {
        if val >= &input[i + 1].values[j] {
            is_low_point = false;
        }
    }
    // check left
    if j != 0 {
        if val >= &input[i].values[j - 1] {
            is_low_point = false;
        }
    }
    // check right
    if j != input[0].values.len() - 1 {
        if val >= &input[i].values[j + 1] {
            is_low_point = false;
        }
    }
    is_low_point
}

fn solve_1(input: &Vec<VentRow>) -> Option<i32> {
    let mut risk_total = 0;
    for (i, row) in input.iter().enumerate() {
        for (j, val) in row.values.iter().enumerate() {
            if is_low_point(input, i, j, val) {
                risk_total += val + 1
            }
        }
    }
    Some(risk_total)
}

fn solve_2(input: &Vec<VentRow>) -> Option<i32> {
    let mut smallest_basins = SmallestBasins::new();
    for(i, row) in input.iter().enumerate() {
        for (j, val) in row.values.iter().enumerate() {
            if is_low_point(input, i, j, val) {
                // find basin, get area, check if it's within smallest 3
            }
        }
    }
    Some(smallest_basins.solve())
}

struct Day9;
impl Problem for Day9 {
    type Input = Vec<VentRow>;
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
    solve::<Day9>("test").unwrap();
}
