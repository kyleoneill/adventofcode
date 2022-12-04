use std::ops::RangeInclusive;
use std::str::FromStr;
use std::string::ParseError;
use problem::{solve_main, Problem};

struct Pair {
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>
}

impl FromStr for Pair {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_str, second_str) = s.split_once(',').unwrap();
        let (first_lower, first_upper) = first_str.split_once('-').unwrap();
        let (second_lower, second_upper) = second_str.split_once('-').unwrap();
        let first = RangeInclusive::new(first_lower.parse().unwrap(),first_upper.parse().unwrap());
        let second = RangeInclusive::new(second_lower.parse().unwrap(), second_upper.parse().unwrap());
        Ok(Self { first, second  })
    }
}

impl Pair {
    fn one_contains_another(&self) -> bool {
        if self.first.start() <= self.second.start() && self.first.end() >= self.second.end() {
            return true
        }
        else if self.second.start() <= self.first.start() && self.second.end() >= self.first.end() {
            return true
        }
        false
    }

    fn contains_overlap(&self) -> bool {
        // This is what I came up with first, but it can be simplified
        // if self.first.contains(&self.second.start()) || self.first.contains(&self.second.end()) {
        //     return true
        // }
        // else if self.second.contains(&self.first.start()) || self.second.contains(&self.first.end()) {
        //     return true
        // }
        // false
        self.first.start() <= self.second.end() && self.first.end() >= self.second.start()
    }
}

struct Day4;

impl Problem for Day4 {
    type Input = Vec<String>;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut sum = 0;
        for i in input {
            let pair = Pair::from_str(i).unwrap();
            if pair.one_contains_another() {
                sum += 1;
            }
        }
        sum
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut sum = 0;
        for i in input {
            let pair = Pair::from_str(i).unwrap();
            if pair.contains_overlap() {
                sum += 1;
            }
        }
        sum
    }
}

fn main() {
    solve_main::<Day4>();
}
