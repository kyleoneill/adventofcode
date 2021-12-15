use problem::{Problem, solve_main};
use std::{collections::HashMap, panic};

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

fn encode_pattern(input: &str) -> u8 {
    let mut result = 0;
    for c in input.chars() {
        result |= match c {
            'a' => 0b00000001,
            'b' => 0b00000010,
            'c' => 0b00000100,
            'd' => 0b00001000,
            'e' => 0b00010000,
            'f' => 0b00100000,
            'g' => 0b01000000,
            _ => panic!("Invalid character")
        }
    }
    result
}

const DISPLAY_PATTERNS: [u8; 10] = [
    // gfedcba
    0b01110111,
    0b00100100,
    0b01011101,
    0b01101101,
    0b00101110,
    0b01101011,
    0b01111011,
    0b00100101,
    0b01111111,
    0b01101111
];

struct Day8;
impl Problem for Day8 {
    type Input = Vec<String>;
    type PartOne = i32;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        solve_1(input).unwrap()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut counter = 0;
        for line in input {
            let mut line_counter = 0;

            // Build a list of candidates for each encoded pattern
            let mut split = line.split(" | ");
            let unique_signal_patterns = split.next().unwrap();
            let mut encoded_patterns: [u8; 10] = [0; 10];
            let mut candidates: [Vec<usize>; 10] = [vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![]];
            // Figure out the map between patterns and numbers on the display
            for (index, pattern) in unique_signal_patterns.split(" ").enumerate() {
                let encoded_pattern = encode_pattern(pattern);
                encoded_patterns[index] = encoded_pattern;
                for n in 0..10 {
                    if DISPLAY_PATTERNS[n].count_ones() == encoded_pattern.count_ones() {
                        candidates[index].push(n);
                    }
                }
            }

            // Loop while we match encoded patterns to a display number
            let mut done = false;
            while !done {
                done = true;
                for i in 0..10 {
                    // There are more than 1 candidate, we still have to reduce
                    if candidates[i].len() > 1 {
                        done = false;
                        for j in 0..10 {
                            // candidates[i] is not solved but candidates[j] is
                            if candidates[j].len() == 1 {
                                let shared_bits = (encoded_patterns[i] & encoded_patterns[j]).count_ones();
                                let mut next_candidates = candidates[i].clone();
                                next_candidates.retain(|c| (DISPLAY_PATTERNS[*c] & DISPLAY_PATTERNS[candidates[j][0]]).count_ones() == shared_bits);
                                candidates[i] = next_candidates;
                            }
                        }
                    }
                }
            }

            // Add up numbers seen on the display
            let output_value = split.next().unwrap();
            for pattern in output_value.split(" ") {
                let encoded_pattern = encode_pattern(pattern);
                let index = encoded_patterns.iter().position(|p| *p == encoded_pattern).unwrap();
                let digit = candidates[index][0];
                line_counter *= 10;
                line_counter += digit;
            }
            counter += line_counter;
        }
        counter
    }
}

fn main() {
    solve_main::<Day8>();
}
