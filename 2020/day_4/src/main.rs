use std::{time::Instant, ops::RangeInclusive};
use std::fs;
use std::io::BufRead;
extern crate regex;
use regex::Regex;

fn digits_in_number(n: i32) -> i32 {
    let mut n = n as usize;
    let base = 10usize;
    let mut digit = 0;
    while n != 0 {
        n /= base;
        digit += 1;
    }
    digit
}

fn year_in_range(year: i32, range: RangeInclusive<i32>) -> bool {
    if digits_in_number(year) == 4 {
        if range.contains(&year) {
            return true
        }
    }
    false
}

fn birth_year_valid(byr: &str) -> bool {
    match byr.parse() {
        Ok(byr) => {
            return year_in_range(byr, 1920..=2020)
        },
        _ => false
    }
}

fn issue_year_valid(iyr: &str) -> bool {
    match iyr.parse() {
        Ok(iyr) => {
            return year_in_range(iyr, 2010..=2020)
        },
        _ => false
    }
}

fn expiration_year_valid(eyr: &str) -> bool {
    match eyr.parse() {
        Ok(eyr) => {
            return year_in_range(eyr, 2020..=2030)
        },
        _ => false
    }
}

fn height_valid(hgt: &str) -> bool {
    let last_chars = hgt.chars().rev().take(2).collect::<String>().to_lowercase();
    if last_chars == "mc" {
        let height: i32 = hgt[0..hgt.len() - 2].parse().unwrap();
        let range = 150..=193;
        if range.contains(&height) {
            return true
        }
    }
    else if last_chars == "ni" {
        let height: i32 = hgt[0..hgt.len() - 2].parse().unwrap();
        let range = 59..=76;
        if range.contains(&height) {
            return true
        }
    }
    false
}

fn hair_color_valid(hcl: &str) -> bool {
    let re = Regex::new(r"^#([A-Fa-f0-9]{6})$").unwrap();
    if re.is_match(hcl) {
        return true
    }
    false
}

fn eye_color_valid(ecl: &str) -> bool {
    match ecl {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false
    }
}

fn passport_id_valid(pid: &str) -> bool {
    let mut char_count = 0;
    for i in pid.chars() {
        if !i.is_digit(10) {
            return false
        }
        char_count += 1;
    }
    if char_count == 9 {
        return true
    }
    false
}

fn validate_passport(passport: &str) -> bool {
    let mut byr_valid = false;
    let mut iyr_valid = false;
    let mut eyr_valid = false;
    let mut hgt_valid = false;
    let mut hcl_valid = false;
    let mut ecl_valid = false;
    let mut pid_valid = false;
    for field in passport.split_whitespace() {
        if field.len() < 5 {
            return false
        }
        let field_value = &field[4..];
        match &field[0..3] {
            "byr" => {
                byr_valid = birth_year_valid(field_value);
            },
            "iyr" => {
                iyr_valid = issue_year_valid(field_value);
            },
            "eyr" => {
                eyr_valid = expiration_year_valid(field_value);
            },
            "hgt" => {
                hgt_valid = height_valid(field_value);
            },
            "hcl" => {
                hcl_valid = hair_color_valid(field_value);
            },
            "ecl" => {
                ecl_valid = eye_color_valid(field_value);
            },
            "pid" => {
                pid_valid = passport_id_valid(field_value);
            },
            &_ => ()
        }
    }
    if byr_valid &&
        iyr_valid &&
        eyr_valid &&
        hgt_valid &&
        hcl_valid &&
        ecl_valid &&
        pid_valid {
            return true
        }
    false
}

fn main() {
    let start = Instant::now();
    let input = get_input("input.txt");
    let solution = solve(input);
    println!("Found solution in {} microseconds", start.elapsed().as_micros());
    println!("The solution is {}", solution);
}

fn solve(input: Vec<String>) -> i32 {
    let mut passports: Vec<String> = Vec::new();
    for passport in input {
        if passport.contains("byr")
            && passport.contains("iyr")
            && passport.contains("eyr")
            && passport.contains("hgt")
            && passport.contains("hcl")
            && passport.contains("ecl")
            && passport.contains("pid") {
                passports.push(passport);
            }
    }
    let mut valid_passports = 0;
    for passport in passports {
        if validate_passport(&passport) {
            valid_passports += 1;
        }
    }
    valid_passports
}

fn get_input(filename: &str) -> Vec<String> {
    let mut contents: Vec<String> = Vec::new();
    let mut current_string = "".to_string();
    let file = fs::File::open(filename).expect("Failed to open input file");
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(line) = line {
            if line.trim().is_empty() {
                contents.push(current_string);
                current_string = "".to_string();
            }
            else {
                current_string.push_str(" ");
                current_string.push_str(&line);
            }
        }
    }
    if !current_string.is_empty() {
        contents.push(current_string);
    }
    contents
}