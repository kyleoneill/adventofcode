use problem::{Problem, solve};
use std::collections::HashMap;

#[derive(Debug)]
enum Error {
    NoSolution
}

fn solve_1(input: &Vec<String>) -> Option<i32> {
    let mut seven_segment_display: HashMap<String, i32> = HashMap::new();
    let mut counter = 0;
    for line in input {
        let mut split = line.split("|");
        let unique_signal_patterns = split.next().unwrap();
        for pattern in unique_signal_patterns.split(" ") {
            let mut chars: Vec<char> = pattern.chars().collect();
            chars.sort_by(|a, b| b.cmp(a));
            let unique_segment = chars.iter().collect();
            match chars.len() {
                2 => seven_segment_display.insert(unique_segment, 1),
                4 => seven_segment_display.insert(unique_segment, 4),
                3 => seven_segment_display.insert(unique_segment, 7),
                7 => seven_segment_display.insert(unique_segment, 8),
                _ => None
            };
        }
        let output_value = split.next().unwrap();
        for pattern in output_value.split(" ") {
            let mut chars: Vec<char> = pattern.chars().collect();
            chars.sort_by(|a, b| b.cmp(a));
            let unique_segment: String = chars.iter().collect();
            if seven_segment_display.contains_key(&unique_segment) {
                counter += 1
            }
        }
    }
    Some(counter)
}

fn solve_2(input: &Vec<String>) -> Option<i32> {
    Some(0)
}

struct Day8;
impl Problem for Day8 {
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
    solve::<Day8>("input").unwrap();
}
