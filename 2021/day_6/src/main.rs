use problem::{CSV, Problem, solve};
use std::collections::HashMap;

/// simulate_fish and calculate_fish do the exact same thing, but calculate_fish
/// is significantly more efficient. Solving the problem for part 2, 256 days, is not possible
/// for simulate_fish. Keeping the inefficient function here for future reference.
#[allow(dead_code)]
fn simulate_fish(input: &CSV<i32>, days: i32) -> i32 {
    let mut initial_state = input.values.clone();
    for _day in 0..days {
        for n in 0..initial_state.len() {
            if initial_state[n] == 0 {
                initial_state[n] = 6;
                initial_state.push(8);
            }
            else {
                initial_state[n] -= 1;
            }
        }
    }
    initial_state.len() as i32
}

fn calculate_fish(input: &CSV<i32>, days: i32) -> i64 {
    let mut fish: HashMap<i32, i64> = HashMap::new();
    for val in &input.values {
        let count = fish.entry(val.clone()).or_insert(0);
        *count += 1;
    }
    for _day in 0..days {
        let zero_count = fish.entry(0).or_insert(0);
        let num_of_zeros = zero_count.clone();
        *zero_count = 0;
        for n in 1..=8 {
            let count = *fish.entry(n).or_insert(0);
            let fish_below = fish.entry(n - 1).or_insert(0);
            *fish_below = count; // try *count
            *fish.get_mut(&n).unwrap() = 0;
        }
        let six_count = fish.entry(6).or_insert(0);
        *six_count += num_of_zeros;
        let eight_count = fish.entry(8).or_insert(0);
        *eight_count += num_of_zeros;

    }
    let mut sum = 0;
    for n in 0..=8 {
        let count = fish.entry(n).or_insert(0);
        sum += *count;
    }
    sum
}

fn solve_1(input: &CSV<i32>) -> Option<i64> {
    Some(calculate_fish(input, 80))
}

fn solve_2(input: &CSV<i32>) -> Option<i64> {
    Some(calculate_fish(input, 256))
}

#[derive(Debug)]
enum Error {
    NoSolution
}

struct Day6;
impl Problem for Day6 {
    type Input = CSV<i32>;
    type Part1Output = i64;
    type Part2Output = i64;
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
    solve::<Day6>("input").unwrap();
}
