use std::collections::HashSet;

use problem::{solve_main, Problem};

fn calculate_factors(input: usize) -> HashSet<usize> {
    let mut small_factors: Vec<usize> = Vec::new();
    let mut large_factors: Vec<usize> = Vec::new();

    let range_end = ((input as f64).sqrt().ceil() as usize) + 1;

    for i in 1..range_end {
        if input % i == 0 {
            small_factors.push(i);
        }
    }

    for i in &small_factors {
        if input != *i * *i {
            large_factors.push(input / i);
        }
    }

    let mut out: HashSet<usize> = HashSet::new();
    for n in small_factors {
        out.insert(n);
    }
    for n in large_factors {
        out.insert(n);
    }
    out
}

struct Day20;

impl Problem for Day20 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let desired_presents = input[0].parse::<usize>().unwrap();

        let mut i = 1;
        loop {
            let factors = calculate_factors(i);
            if factors.iter().sum::<usize>() * 10 >= desired_presents {
                return i;
            }
            i += 1;
        }
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let desired_presents = input[0].parse::<usize>().unwrap();

        let mut i = 1;
        loop {
            let factors = calculate_factors(i);
            let presents: usize = factors.iter().filter(|n| (i / **n) <= 50).sum::<usize>() * 11;
            if presents >= desired_presents {
                return i;
            }
            i += 1;
        }
    }
}

fn main() {
    solve_main::<Day20>();
}
