use problem::{CSV, Problem, solve};
use std::collections::HashMap;

fn solve_1(input: &CSV<i32>) -> Option<i32> {
    let lowest_position = input.values.iter().min().unwrap().clone() as usize;
    let highest_position = input.values.iter().max().unwrap().clone() as usize;
    let mut cheapest_fuel = i32::MAX;
    for n in lowest_position..=highest_position {
        let mut fuel = 0;
        for m in input.values.iter() {
            let difference = n as i32 - m;
            fuel += difference.abs();
        }
        if fuel < cheapest_fuel { cheapest_fuel = fuel }
    }
    Some(cheapest_fuel)
}

fn solve_2(input: &CSV<i32>) -> Option<i32> {
    let lowest_position = input.values.iter().min().unwrap().clone() as usize;
    let highest_position = input.values.iter().max().unwrap().clone() as usize;
    let mut cheapest_fuel = i32::MAX;
    let mut memoized: HashMap<i32, i32> = HashMap::new();
    for n in lowest_position..=highest_position {
        let mut fuel = 0;
        for m in input.values.iter() {
            let difference = (n as i32 - m).abs();
            let fuel_to_burn = match memoized.contains_key(&difference) {
                true => memoized.get(&difference).unwrap().clone(),
                false => {
                    let new_fuel = calculate_fuel(difference);
                    memoized.insert(difference, new_fuel);
                    new_fuel
                }
            };
            fuel += fuel_to_burn;
        }
        if fuel < cheapest_fuel { cheapest_fuel = fuel }
    }
    Some(cheapest_fuel)
}

fn calculate_fuel(steps: i32) -> i32 {
    let mut counter = 0;
    for n in 1..=steps {
        counter += n;
    }
    counter
}

#[derive(Debug)]
enum Error {
    NoSolution
}

struct Day7;
impl Problem for Day7 {
    type Input = CSV<i32>;
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
    solve::<Day7>("input").unwrap();
}
