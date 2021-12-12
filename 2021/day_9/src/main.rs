use problem::{Problem, solve};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Coordinate { x, y }
    }
}

#[derive(Clone)]
struct VentRow {
    values: Vec<i32>
}

struct VentMap {
    vents: Vec<VentRow>
}

impl VentMap {
    fn new(input: &Vec<VentRow>) -> Self {
        VentMap { vents: input.to_vec() }
    }
    fn val_at_cord(&self, cord: &Coordinate) -> Option<i32> {
        if cord.y < self.vents.len() && cord.x < self.vents[0].values.len() {
            return Some(self.vents[cord.y].values[cord.x])
        }
        None
    }
}

impl FromStr for VentRow {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const RADIX: u32 = 10;
        let mut values: Vec<i32> = Vec::new();
        for c in s.chars() {
            values.push(c.to_digit(RADIX).unwrap() as i32);
        }
        Ok(VentRow{values})
    }
}

#[derive(Debug)]
enum Error {
    NoSolution
}

fn is_low_point(input: &Vec<VentRow>, i: usize, j: usize, val: &i32) -> bool {
    let mut is_low_point = true;
    // check up
    if i != 0 {
        if val >= &input[i - 1].values[j] {
            is_low_point = false;
        }
    }
    // check down
    if i != input.len() - 1 {
        if val >= &input[i + 1].values[j] {
            is_low_point = false;
        }
    }
    // check left
    if j != 0 {
        if val >= &input[i].values[j - 1] {
            is_low_point = false;
        }
    }
    // check right
    if j != input[0].values.len() - 1 {
        if val >= &input[i].values[j + 1] {
            is_low_point = false;
        }
    }
    is_low_point
}

fn solve_1(input: &Vec<VentRow>) -> Option<i32> {
    let mut risk_total = 0;
    for (i, row) in input.iter().enumerate() {
        for (j, val) in row.values.iter().enumerate() {
            if is_low_point(input, i, j, val) {
                risk_total += val + 1
            }
        }
    }
    Some(risk_total)
}

fn build_slope(vent_map: &VentMap, cord_looking_at: &Coordinate, basins: &mut Vec<Coordinate>) {
    let val = vent_map.val_at_cord(cord_looking_at).unwrap();
    if val == 9 || basins.contains(cord_looking_at) {
        return;
    }
    basins.push(*cord_looking_at);
    // check up
    if cord_looking_at.y != 0 {
        if val < vent_map.vents[cord_looking_at.y - 1].values[cord_looking_at.x] {
            let new_cord = Coordinate::new(cord_looking_at.x, cord_looking_at.y - 1);
            build_slope(vent_map, &new_cord, basins);
        }
    }
    // check down
    if cord_looking_at.y != vent_map.vents.len() - 1 {
        if val < vent_map.vents[cord_looking_at.y + 1].values[cord_looking_at.x] {
            let new_cord = Coordinate::new(cord_looking_at.x, cord_looking_at.y + 1);
            build_slope(vent_map, &new_cord, basins);
        }
    }
    // check left
    if cord_looking_at.x != 0 {
        if val < vent_map.vents[cord_looking_at.y].values[cord_looking_at.x - 1] {
            let new_cord = Coordinate::new(cord_looking_at.x - 1, cord_looking_at.y);
            build_slope(vent_map, &new_cord, basins);
        }
    }
    // check right
    if cord_looking_at.x != vent_map.vents[0].values.len() - 1 {
        if val < vent_map.vents[cord_looking_at.y].values[cord_looking_at.x + 1] {
            let new_cord = Coordinate::new(cord_looking_at.x + 1, cord_looking_at.y);
            build_slope(vent_map, &new_cord, basins);
        }
    }
}

fn solve_2(input: &Vec<VentRow>) -> Option<i32> {
    let vent_map = VentMap::new(input);
    let mut basin_areas: Vec<i32> = Vec::new();
    for(i, row) in input.iter().enumerate() {
        for (j, val) in row.values.iter().enumerate() {
            if is_low_point(input, i, j, val) {
                let cord = Coordinate::new(j, i);
                let mut basin_cords: Vec<Coordinate> = Vec::new();
                build_slope(&vent_map, &cord, &mut basin_cords);
                basin_areas.push(basin_cords.len() as i32);
            }
        }
    }
    basin_areas.sort();
    Some(basin_areas.iter().rev().take(3).product())
}

struct Day9;
impl Problem for Day9 {
    type Input = Vec<VentRow>;
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
    solve::<Day9>("input").unwrap();
}
