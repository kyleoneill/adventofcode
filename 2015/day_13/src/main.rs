use std::collections::{HashMap, HashSet};

use problem::{solve_main, Problem};

use itertools::Itertools;

#[derive(Debug)]
struct DinnerPatrons {
    patrons: HashSet<String>,
    relationships: HashMap<(String, String), isize>
}

impl DinnerPatrons {
    fn from_input(input: &Vec<String>) -> Self {
        let mut patrons: HashSet<String> = HashSet::new();
        let mut relationships: HashMap<(String, String), isize> = HashMap::new();
        for line in input {
            let split: Vec<&str> = line.split_whitespace().collect();
            let name = split[0].to_owned();
            let change = match split[2] {
                "gain" => split[3].parse::<isize>().expect("Failed to parse 'gain' to isize"),
                "lose" => split[3].parse::<isize>().expect("Failed to pase 'lose' to isize") * -1,
                _ => panic!("Got an invalid token in position 2 of a string"),
            };
            let target_slice = split[10];
            let target = target_slice[0..target_slice.len() - 1].to_owned();
            patrons.insert(name.clone());

            relationships.insert((name, target), change);
        }
        Self { patrons, relationships }
    }

    fn insert_self(&mut self) {
        for patron in &self.patrons {
            self.relationships.insert(("me".to_owned(), patron.clone()), 0);
            self.relationships.insert((patron.clone(), "me".to_owned()), 0);
        }
        self.patrons.insert("me".to_owned());
    }

    fn get_optimal_happiness(&self) -> isize {
        let mut highest_happiness: isize = 0;

        for permutation in self.patrons.iter().permutations(self.patrons.len()).unique() {
            let n = permutation.len();
            let mut current_happiness: isize = 0;
            for i in 0..n {
                let person = permutation[i].to_owned();
                let left_neighbor = permutation[(i + n - 1) % n].to_owned();
                let right_neighbor = permutation[(i + 1) % n].to_owned();
        
                if let Some(&happiness) = self.relationships.get(&(person.clone(), left_neighbor)) {
                    current_happiness += happiness;
                }
                if let Some(&happiness) = self.relationships.get(&(person, right_neighbor)) {
                    current_happiness += happiness;
                }
            }
            if current_happiness > highest_happiness {
                highest_happiness = current_happiness;
            }
        }
        highest_happiness
    }
}

struct Day13;

impl Problem for Day13 {
    type Input = Vec<String>;
    type PartOne = isize;
    type PartTwo = isize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let dinner_patrons = DinnerPatrons::from_input(input);
        dinner_patrons.get_optimal_happiness()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut dinner_patrons = DinnerPatrons::from_input(input);
        dinner_patrons.insert_self();
        dinner_patrons.get_optimal_happiness()
    }
}

fn main() {
    solve_main::<Day13>();
}
