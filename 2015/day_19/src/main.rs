use std::collections::{HashMap, HashSet};

use problem::{solve_main, Problem};

#[derive(Debug)]
struct Compound<'a> {
    compound: String,
    map: &'a mut HashMap<String, Vec<String>>,
}

fn parse_input(input: &Vec<String>) -> (String, HashMap<String, Vec<String>>) {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut itr = input.into_iter();
    let mut compound = "";
    while let Some(s) = itr.next() {
        if s.is_empty() {
            // Next line is the compound and then we exit
            compound = itr.next().unwrap().trim();
            break;
        }
        else {
            let split: Vec<&str> = s.split("=>").map(|s| s.trim()).collect();
            let key = split[0].trim();
            if map.contains_key(split[0]) {
                map.get_mut(key).unwrap().push(split[1].to_owned());
            }
            else {
                map.insert(key.to_owned(), vec![split[1].to_owned()]);
            }
        }
    }
    (compound.to_owned(), map)
}

impl<'a> Compound<'a> {    
    fn new(s: String, map: &'a mut HashMap<String, Vec<String>>) -> Self {
        Self { compound: s, map }
    }

    fn step(&mut self, slice: &str) -> HashSet<String> {
        let mut index = 0;
        let mut molecules: HashSet<String> = HashSet::new();
        while slice.len() != index {
            for key in self.map.keys() {
                let key_len = key.len();
                if key_len + index <= slice.len() && &slice[index..(index + key_len)] == key {
                    for val in self.map.get(key).unwrap() {
                        let mut new_str = String::new();
                        new_str.push_str(&slice[0..index]);
                        new_str.push_str(val);
                        new_str.push_str(&slice[(index + key_len)..]);
                        molecules.insert(new_str);
                    }
                }
            }
            index += 1
        }
        molecules
    }

    fn count_elements(&self) -> HashMap<String, usize> {
        let mut index = 0;
        let mut map: HashMap<String, usize> = HashMap::new();
        let slice = self.compound.as_str();
        while index < self.compound.len() {
            let mut current_element = slice[index..=index].to_owned();
            if index + 1 < self.compound.len() {
                let next = &slice[(index + 1)..=(index + 1)];
                let chars: Vec<char> = next.chars().collect();
                let next_char = chars[0];
                if next_char.is_lowercase() {
                    current_element.push(next_char);
                    index += 1;
                }
            }
            index += 1;

            match map.contains_key(current_element.as_str()) {
                true => {
                    let val = map.get_mut(current_element.as_str()).unwrap();
                    *val += 1;
                },
                false => { map.insert(current_element, 1); }
            }
        }
        map
    }
}

struct Day19;

impl Problem for Day19 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let (comp, mut map) = parse_input(input);
        let mut compound = Compound::new(comp, &mut map);
        let medicine = compound.compound.clone();
        let molecules = compound.step(medicine.as_str());
        molecules.len()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let (comp, mut map) = parse_input(input);
        let compound = Compound::new(comp, &mut map);

        // This problem involves a trick using the structure of the input, it is not meant to be solved in a 
        // general manner
        //
        // The solution is a formula:
        // Solution = T - x - 2y - 1
        //  where
        //  T = total elements
        //  x = Count of Rn and Ar
        //  y = Count of Y

        // See 2015 notes.md for an explanation

        let element_counts = compound.count_elements();
        let total_elements: usize = element_counts.values().into_iter().sum();
        let x_elements: usize = {
            let rn_count = element_counts.get("Rn").unwrap();
            let ar_count = element_counts.get("Ar").unwrap();
            rn_count + ar_count
        };
        let y_elements = element_counts.get("Y").unwrap();
        total_elements - x_elements - (y_elements * 2) - 1
    }
}

fn main() {
    solve_main::<Day19>();
}
