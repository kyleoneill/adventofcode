use problem::{Problem, solve};

fn binary_to_int(input: &str) -> i32 {
    const BASE: i32 = 2;
    let mut num = 0;
    for (i, c) in input.chars().rev().enumerate() {
        if c == '1' {
            num += BASE.pow(i as u32);
        }
    }
    num
}

struct DiagnosticReport {
    gamma_rate: String,
    epsilon_rate: String,
    oxygen_generator_rating: String,
    co2_scrubber_rating: String
}

impl DiagnosticReport {
    fn new() -> Self {
        DiagnosticReport { gamma_rate: String::from(""), epsilon_rate: String::from(""), oxygen_generator_rating: String::from(""), co2_scrubber_rating: String::from("")}
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
    fn get_power_consumption(&self) -> i32 {
        binary_to_int(&self.gamma_rate) * binary_to_int(&self.epsilon_rate)
    }
    fn get_life_support_rating(&self) -> i32 {
        binary_to_int(&self.oxygen_generator_rating) * binary_to_int(&self.co2_scrubber_rating)
    }
}

fn solve_1(input: &Vec<String>) -> Option<i32> {
    let length = input.len();
    let width = input[0].chars().count();
    let mut diagnostic_report = DiagnosticReport::new();
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
            diagnostic_report.gamma_rate.push('1');
        }
        else {
            diagnostic_report.gamma_rate.push('0');
        }
    }
    diagnostic_report.set_epsilon_rate().unwrap();
    Some(diagnostic_report.get_power_consumption())
}

fn find_oxygen_co2_rating(input: &Vec<String>, is_oxygen_rating: bool) -> Option<String> {
    let mut filtered_binaries = input.clone();

    let width = input[0].chars().count();
    for n in 0..width {
        let length = filtered_binaries.len();
        let mut one = 0;
        let mut zero = 0;
        for m in 0..length {
            match filtered_binaries[m].chars().nth(n).unwrap() {
                '0' => zero += 1,
                '1' => one += 1,
                _ => return None
            }
        }
        let filtered_char: char;
        if is_oxygen_rating {
            filtered_char = if one >= zero { '1' } else { '0' };
        }
        else {
            filtered_char = if zero <= one { '0' } else { '1' };
        }
        filtered_binaries.retain(|x| x.chars().nth(n).unwrap() == filtered_char);
        if filtered_binaries.len() == 1 {
            break
        }
    }
    if filtered_binaries.len() > 1 {
        return None
    }
    Some(filtered_binaries[0].clone())
}

fn solve_2(input: &Vec<String>) -> Option<i32> {
    let mut diagnostic_report = DiagnosticReport::new();
    diagnostic_report.oxygen_generator_rating = find_oxygen_co2_rating(input, true).unwrap();
    diagnostic_report.co2_scrubber_rating = find_oxygen_co2_rating(input, false).unwrap();
    Some(diagnostic_report.get_life_support_rating())
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
