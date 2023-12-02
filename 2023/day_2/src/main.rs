use problem::{solve_main, Problem};
use std::collections::HashMap;
use std::str::FromStr;

struct Day2;

pub struct Set {
    pub red: u32,
    pub blue: u32,
    pub green: u32
}

impl Set {
    pub fn new(red: u32, blue: u32, green: u32) -> Self {
        Self { red, blue, green }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseSetErr;

impl FromStr for Set {
    type Err = ParseSetErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        let number_colors = s.trim().split(",");
        for pairing in number_colors {
            let split: Vec<&str> = pairing.trim().split(" ").collect();
            match split[1] {
                "red" => red = split[0].parse::<u32>().expect("Failed to convert str to u32"),
                "blue" => blue = split[0].parse::<u32>().expect("Failed to convert str to u32"),
                "green" => green = split[0].parse::<u32>().expect("Failed to convert str to u32"),
                _ => return Err(ParseSetErr)
            }
        }
        Ok(Set::new(red, blue, green))
    }
}

fn build_game_map(input: &Vec<String>) -> HashMap<u32, Vec<Set>> {
    let mut map: HashMap<u32, Vec<Set>> = HashMap::new();
    for line in input {
        let first_split: Vec<&str> = line.split(":").collect();
        let second_split: Vec<&str> = first_split[0].split(" ").collect();
        let game_index = second_split[1].parse::<u32>().expect("Failed to convert game index into a u32");
        let string_sets: Vec<&str> = first_split[1].split(";").collect();
        let mut sets: Vec<Set> = Vec::new();
        for str_set in string_sets {
            let converted = str_set.parse::<Set>().expect("Failed to convert string to Set");
            sets.push(converted);
        }
        map.insert(game_index, sets);
    }
    map
}

fn count_possible_games(games: HashMap<u32, Vec<Set>>) -> u32 {
    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;
    let mut sum = 0;
    for game in games {
        let mut possible = true;
        for set in game.1 {
            if set.red > max_red || set.green > max_green || set.blue > max_blue {
                possible = false;
                break;
            }
        }
        if possible {
            sum += game.0;
        }
    }
    sum
}

fn get_minimum_cubes(games: HashMap<u32, Vec<Set>>) -> u32 {
    let mut sum = 0;
    for game in games {
        let mut largest_red: u32 = 0;
        let mut largest_blue: u32 = 0;
        let mut largest_green: u32 = 0;
        for set in game.1 {
            if set.red > largest_red {
                largest_red = set.red;
            }
            if set.green > largest_green {
                largest_green = set.green;
            }
            if set.blue > largest_blue {
                largest_blue = set.blue;
            }
        }
        let power = largest_red * largest_blue * largest_green;
        sum += power;
    }
    sum
}

impl Problem for Day2 {
    type Input = Vec<String>;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let game_map = build_game_map(input);
        count_possible_games(game_map)
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let game_map = build_game_map(input);
        get_minimum_cubes(game_map)
    }
}

fn main() {
    solve_main::<Day2>();
}
