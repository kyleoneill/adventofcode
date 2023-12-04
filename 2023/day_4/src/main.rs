use problem::{solve_main, Problem};
use std::str::FromStr;
use std::collections::HashMap;

struct Day4;

struct Scratchcard {
    card_number: u32,
    pub winning_numbers: Vec<u32>,
    pub card_numbers: Vec<u32>
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;

impl FromStr for Scratchcard {
    type Err = ParseCardError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut winning_numbers: Vec<u32> = Vec::new();
        let mut card_numbers: Vec<u32> = Vec::new();
        let split: Vec<&str> = s.split("|").collect();
        let second_split: Vec<&str> = split[0].split(":").collect();
        let third_split: Vec<&str> = second_split[0].split(" ").collect();
        
        let card_number = third_split[third_split.len() - 1].parse::<u32>().expect("Failed to read str num to u32");
        let unconverted_winning_numbers: Vec<&str> = second_split[1].trim().split(" ").collect();
        for win_num in unconverted_winning_numbers  {
            if win_num.chars().all(|c| c.is_whitespace()) {
                continue;
            }
            let num = win_num.parse::<u32>().expect("Failed to convert str to u32");
            winning_numbers.push(num);
        }
        let unconverted_card_numbers: Vec<&str> = split[1].trim().split(" ").collect();
        for normal_num in unconverted_card_numbers  {
            if normal_num.chars().all(|c| c.is_whitespace()) {
                continue;
            }
            let num = normal_num.parse::<u32>().expect("Failed to convert str to u32");
            card_numbers.push(num);
        }
        Ok(Self { card_number, winning_numbers, card_numbers: card_numbers })
    }
}

impl Scratchcard {
    pub fn count_points(&self) -> u32 {
        let mut winning_points = 0;
        for num in &self.card_numbers {
            if self.winning_numbers.contains(&num) {
                if winning_points == 0 {
                    winning_points = 1;
                }
                else {
                    winning_points *= 2;
                }
            }
        }
        winning_points
    }
    pub fn process_cards(&self, map: &mut HashMap<u32, u32>) {
        let times_to_run = match map.contains_key(&self.card_number) {
            true => map.get(&self.card_number).unwrap().clone(),
            false => 1
        };

        let mut num_matching_numbers = 0;
        for num in &self.card_numbers {
            if self.winning_numbers.contains(&num) {
                num_matching_numbers += 1;
            }
        }

        let card_to_go_to = self.card_number + num_matching_numbers;
        for n in self.card_number + 1..=card_to_go_to {
            *map.entry(n).or_insert(0) += times_to_run;
        }
    }
}

impl Problem for Day4 {
    type Input = Vec<String>;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut sum = 0;
        for line in input {
            let card = Scratchcard::from_str(line).expect("Failed to read string to scratchcard");
            sum += card.count_points();
        }
        sum
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut card_map: HashMap<u32, u32> = HashMap::new();
        for line in input {
            let card = Scratchcard::from_str(line).expect("Failed to read str to scratchcard");
            *card_map.entry(card.card_number).or_insert(0) += 1;
            card.process_cards(&mut card_map);
        }
        card_map.values().sum::<u32>()
    }
}

fn main() {
    solve_main::<Day4>();
}
