use std::{str::FromStr, fmt::Error, collections::HashMap};

use problem::{Input, Problem, solve_main};
use anyhow::Result;

struct Rule {
    left: char,
    right: char,
    center: char
}

impl FromStr for Rule {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        Ok(Rule { left: chars[0], right: chars[1], center: chars[6] })
    }
}

struct PolymerFormula {
    polymer_template: String,
    pair_insertion_rules: Vec<Rule>
}

impl Input for PolymerFormula {
    fn parse<R: std::io::BufRead>(reader: R) -> Result<Self> {
        let mut input_iter = reader.lines();
        let polymer_template = input_iter.next().unwrap()?;
        let mut pair_insertion_rules: Vec<Rule> = Vec::new();
        input_iter.next().unwrap()?;
        for line in input_iter {
            let line = line?;
            pair_insertion_rules.push(line.parse().unwrap());
        }
        Ok(PolymerFormula { polymer_template, pair_insertion_rules })
    }
}

fn count(input: &PolymerFormula, c: char, steps: usize) -> usize {
    let rules: HashMap<(char, char), char> = input.pair_insertion_rules.iter().map(|r| ((r.left, r.right), r.center)).collect();
    let mut memoized: HashMap<(char, char, usize), usize> = HashMap::new();

    let mut total = 0;
    total
}

fn count_recursion() -> usize {
    0
}

struct Day14;
impl Problem for Day14 {
    type Input = PolymerFormula;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut polymer: Vec<char> = Vec::new();
        0
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        0
    }
}

fn main() {
    solve_main::<Day14>();
}
