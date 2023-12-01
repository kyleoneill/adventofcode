use std::collections::HashMap;
use problem::{solve_main, Problem};

const RADIX: u32 = 10;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: u32,
    y: u32
}

impl Coordinate {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

fn read_input(input: &Vec<String>) -> HashMap<Coordinate, u8> {
    let mut map = HashMap::new();
    let mut i = 0;
    for line in input.iter() {
        let mut j = 0;
        for c in line.chars() {
            map.insert(Coordinate::new(j, i), c.to_digit(RADIX).unwrap() as u8);
            j += 1;
        }
        i += 1;
    }
    map
}

struct Day8;

impl Problem for Day8 {
    type Input = Vec<String>;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut forest = read_input(input);
        let mut visible_trees: Vec<&Coordinate> = Vec::new();
        for position in forest.keys() {
            let tree = forest.get(position).unwrap();
            let mut is_visible = false;
            println!("Tree {:?}: {:?}", position, tree);
            // Figure out if the tree is visible
            // If it is, add coord to visible vec
            if is_visible {
                visible_trees.push(position);
            }
        }
        visible_trees.len() as u32
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        0
    }
}

fn main() {
    solve_main::<Day8>();
}
