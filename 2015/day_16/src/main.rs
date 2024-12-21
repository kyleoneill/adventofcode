use std::str::FromStr;

use problem::{solve_main, Problem};

#[derive(Debug)]
struct AuntSue {
    id: usize,
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

#[derive(Debug)]
struct ParseAuntErr;
impl FromStr for AuntSue {
    type Err = ParseAuntErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(&[',', ':'][..], "");
        let mut split = s.split_whitespace();
        split.next();
        let id = split.next().expect("Failed to get ID").parse::<usize>().expect("Failed to parse ID");
        let mut aunt_sue = AuntSue::new(id);
        // Every aunt in the input has three defined fields
        for _ in 0..3 {
            match split.next().unwrap() {
                "children" => {
                    aunt_sue.children = Some(split.next().unwrap().parse::<usize>().unwrap())
                },
                "cats" => {
                    aunt_sue.cats = Some(split.next().unwrap().parse::<usize>().unwrap())
                },
                "samoyeds" => {
                    aunt_sue.samoyeds = Some(split.next().unwrap().parse::<usize>().unwrap())
                },
                "pomeranians" => {
                    aunt_sue.pomeranians = Some(split.next().unwrap().parse::<usize>().unwrap())
                },
                "akitas" => {
                    aunt_sue.akitas = Some(split.next().unwrap().parse::<usize>().unwrap())
                },
                "vizslas" => {
                    aunt_sue.vizslas = Some(split.next().unwrap().parse::<usize>().unwrap())
                },
                "goldfish" => {
                    aunt_sue.goldfish = Some(split.next().unwrap().parse::<usize>().unwrap())
                },
                "trees" => {
                    aunt_sue.trees = Some(split.next().unwrap().parse::<usize>().unwrap())
                },
                "cars" => {
                    aunt_sue.cars = Some(split.next().unwrap().parse::<usize>().unwrap())
                },
                "perfumes" => {
                    aunt_sue.perfumes = Some(split.next().unwrap().parse::<usize>().unwrap())
                },
                _ => panic!("Got an invalid string when parsing an Aunt Sue")
            }
        }
        Ok(aunt_sue)
    }
}

impl AuntSue {
    fn new(id: usize) -> Self {
        AuntSue { id, children: None, cats: None, samoyeds: None, pomeranians:None, akitas: None, vizslas: None, goldfish: None, trees: None, cars: None, perfumes: None }
    }

    fn from_input(input: &Vec<String>) -> Vec<Self> {
        input.iter().map(|s| AuntSue::from_str(s).unwrap()).collect()
    }

    fn get_matching_aunts<'a>(aunts: &'a Vec<Self>, desired_aunt: &Self, part_one: bool) -> &'a Self {
        for aunt in aunts {
            if aunt.matches(desired_aunt, part_one) {
                return aunt;
            }
        }
        panic!("Failed to find a matching Aunt Sue")
    }

    fn matches(&self, other: &Self, part_one: bool) -> bool {
        if self.children.is_some() {
            if self.children.unwrap() != other.children.unwrap() {
                return false;
            }
        }
        if self.cats.is_some() {
            if (part_one && self.cats.unwrap() != other.cats.unwrap()) || (!part_one && self.cats.unwrap() <= other.cats.unwrap()) {
                return false;
            }
        }
        if self.samoyeds.is_some() {
            if self.samoyeds.unwrap() != other.samoyeds.unwrap() {
                return false;
            }
        }
        if self.pomeranians.is_some() {
            if (part_one && self.pomeranians.unwrap() != other.pomeranians.unwrap()) || (!part_one && self.pomeranians.unwrap() >= other.pomeranians.unwrap()) {
                return false;
            }
        }
        if self.akitas.is_some() {
            if self.akitas.unwrap() != other.akitas.unwrap() {
                return false;
            }
        }
        if self.vizslas.is_some() {
            if self.vizslas.unwrap() != other.vizslas.unwrap() {
                return false;
            }
        }
        if self.goldfish.is_some() {
            if (part_one && self.goldfish.unwrap() != other.goldfish.unwrap()) || (!part_one && self.goldfish.unwrap() >= other.goldfish.unwrap()) {
                return false;
            }
        }
        if self.trees.is_some() {
            if (part_one && self.trees.unwrap() != other.trees.unwrap()) || (!part_one && self.trees.unwrap() <= other.trees.unwrap()) {
                return false;
            }
        }
        if self.cars.is_some() {
            if self.cars.unwrap() != other.cars.unwrap() {
                return false;
            }
        }
        if self.perfumes.is_some() {
            if self.perfumes.unwrap() != other.perfumes.unwrap() {
                return false;
            }
        }
        true
    }
}

struct Day16;

impl Problem for Day16 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let aunts = AuntSue::from_input(input);
        let desired_aunt = AuntSue {
            id: 999,
            children: Some(3),
            cats: Some(7),
            samoyeds: Some(2),
            pomeranians: Some(3),
            akitas: Some(0),
            vizslas: Some(0),
            goldfish: Some(5),
            trees: Some(3),
            cars: Some(2),
            perfumes: Some(1),
        };
        AuntSue::get_matching_aunts(&aunts, &desired_aunt, true).id
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let aunts = AuntSue::from_input(input);
        let desired_aunt = AuntSue {
            id: 999,
            children: Some(3),
            cats: Some(7),
            samoyeds: Some(2),
            pomeranians: Some(3),
            akitas: Some(0),
            vizslas: Some(0),
            goldfish: Some(5),
            trees: Some(3),
            cars: Some(2),
            perfumes: Some(1),
        };
        AuntSue::get_matching_aunts(&aunts, &desired_aunt, false).id
    }
}

fn main() {
    solve_main::<Day16>();
}
