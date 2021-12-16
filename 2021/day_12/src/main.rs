use std::str::FromStr;
use std::collections::{VecDeque, HashMap};

use problem::{Problem, solve_main};
use anyhow::{Result, Error};

#[derive(Debug)]
struct CaveConnection {
    name: String,
    connects_to: String
}

impl FromStr for CaveConnection {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("-");
        Ok(CaveConnection{ 
            name: split.next().unwrap().to_string(),
            connects_to: split.next().unwrap().to_string(),
        })
    }
}

fn is_small(s: &str) -> bool {
    s.chars().all(|x| x.is_ascii_lowercase())
}

struct Day11;
impl Problem for Day11 {
    type Input = Vec<CaveConnection>;
    type PartOne = usize;
    type PartTwo = usize;

    // breadth first search

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut paths: VecDeque<Vec<&str>> = VecDeque::new();
        let mut path_count = 0;
        let mut cave_map: HashMap<&str, Vec<&str>> = HashMap::new();
        for connection in input {
            let connects_to = cave_map.entry(&connection.name).or_insert(Vec::new());
            connects_to.push(&connection.connects_to);
            let connects_from = cave_map.entry(&connection.connects_to).or_insert(Vec::new());
            connects_from.push(&connection.name);
        }
        paths.push_back(vec!["start"]);
        while let Some(path) = paths.pop_front() {
            let current_room = path[path.len() - 1];
            if current_room == "end" {
                path_count += 1;
                continue;
            }
            let connections = &cave_map[current_room];
            for next_room in connections {
                if is_small(*next_room) && path.contains(next_room) {
                    continue;
                }
                let mut cloned_path = path.clone();
                cloned_path.push(next_room);
                paths.push_back(cloned_path);
            }
        }
        path_count
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut paths: VecDeque<(Vec<&str>, bool)> = VecDeque::new();
        let mut path_count = 0;
        let mut cave_map: HashMap<&str, Vec<&str>> = HashMap::new();
        for connection in input {
            let connects_to = cave_map.entry(&connection.name).or_insert(Vec::new());
            connects_to.push(&connection.connects_to);
            let connects_from = cave_map.entry(&connection.connects_to).or_insert(Vec::new());
            connects_from.push(&connection.name);
        }
        paths.push_back((vec!["start"], false));
        while let Some((path, revisited)) = paths.pop_front() {
            let current_room = path[path.len() - 1];
            if current_room == "end" {
                path_count += 1;
                continue;
            }
            let connections = &cave_map[current_room];
            for next_room in connections {
                let is_small_room = is_small(*next_room);
                if (is_small_room && path.contains(next_room) && revisited) || next_room == &"start" {
                    continue;
                }
                let mut cloned_path = path.clone();
                cloned_path.push(next_room);
                paths.push_back((cloned_path, (is_small_room && path.contains(next_room)) || revisited));
            }
        }
        path_count
    }
}

fn main() {
    solve_main::<Day11>();
}
