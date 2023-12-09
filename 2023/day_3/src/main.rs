use problem::{solve_main, Problem};
use std::collections::HashMap;

struct Day3;

#[derive(Debug, Clone)]
struct PartNumber {
    num: String,
    coords: Vec<(usize, usize)>,
    start: (usize, usize)
}

impl PartNumber {
    pub fn new(num: String, coords: Vec<(usize, usize)>, start: (usize, usize)) -> Self {
        Self { num, coords, start }
    }
}

#[derive(Debug)]
enum PartOrSymbol {
    Part(PartNumber),
    Symbol(char)
}

#[derive(Debug)]
struct PartMap {
    pub map: HashMap<(usize, usize), PartOrSymbol>
}

impl PartMap {
    pub fn new(input: &Vec<String>) -> Self {
        let mut map: HashMap<(usize, usize), PartOrSymbol> = HashMap::new();
        for (y, line) in input.iter().enumerate() {
            let mut build_number: String = "".to_owned();
            let mut build_number_coords: Vec<(usize, usize)> = Vec::new();
            let mut start_pos = (0, 0);
            for (x, char) in line.chars().enumerate() {
                // We have a digit, append to the part number
                if char.is_digit(10) {
                    // If the part number is empty then we need to record the coords where it starts
                    if build_number.as_str() == "" {
                        start_pos = (x, y);
                    }
                    build_number_coords.push((x, y));
                    build_number.push(char);
                }
                else {
                    // Digit is not a number. If we have a built number then we need to construct it
                    if build_number.as_str() != "" {
                        // reset buffer, reset start position, store num
                        let end_pos = (x - 1, y);
                        let part_num = PartOrSymbol::Part(PartNumber::new(build_number.clone(), build_number_coords.clone(), start_pos));
                        map.insert(end_pos, part_num);
                        build_number = "".to_owned();
                        start_pos = (0, 0);
                        build_number_coords = Vec::new();
                    }
                    // Match on the non digit char. Ignore periods, store symbols
                    match char {
                        '.' => continue,
                        _ => map.insert((x, y), PartOrSymbol::Symbol(char))
                    };
                }
            }
            // Line is over, check if anything is in the built str
            if build_number.as_str() != "" {
                let end_pos = (line.len() - 1, y);
                let part_num = PartOrSymbol::Part(PartNumber::new(build_number.clone(), build_number_coords, start_pos));
                map.insert(end_pos, part_num);
            }
        }
        Self { map }
    }

    fn check_coords(&self, coords: Vec<(Option<usize>, Option<usize>)>, toggle: &mut bool) {
        for option_coord in coords {
            // This sucks
            if option_coord.0.is_some() && option_coord.1.is_some() {
                let coord = (option_coord.0.unwrap(), option_coord.1.unwrap());
                if self.map.contains_key(&coord) {
                    match self.map.get(&coord).unwrap() {
                        PartOrSymbol::Part(_) => (),
                        PartOrSymbol::Symbol(_) => {
                            *toggle = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn sum_unused_parts(&self) -> u32 {
        let mut sum = 0;
        for (coord, part_or_symbol) in &self.map {
            match part_or_symbol {
                PartOrSymbol::Part(part_number) => {
                    let length = coord.0 - part_number.start.0;
                    let mut is_used = false;
                    for (i, _digit) in part_number.num.chars().enumerate() {
                        let current_coord = (part_number.start.0 + i, part_number.start.1);
                        // This is so ugly wow
                        // first digit has to check up-left, left, down-left
                        if i == 0 && current_coord.0 != 0 {
                            let up_left = (current_coord.0.checked_sub(1), current_coord.1.checked_sub(1));
                            let left = (current_coord.0.checked_sub(1), Some(current_coord.1));
                            let down_left = (current_coord.0.checked_sub(1), Some(current_coord.1 + 1));
                            let coords = vec![up_left, left, down_left];
                            self.check_coords(coords, &mut is_used);
                        }
                        // last digit has to check up-right, right, down-right
                        if i == length {
                            let up_right = (Some(current_coord.0 + 1), current_coord.1.checked_sub(1));
                            let right = (Some(current_coord.0 + 1), Some(current_coord.1));
                            let down_right = (Some(current_coord.0 + 1), Some(current_coord.1 + 1));
                            let coords = vec![up_right, right, down_right];
                            self.check_coords(coords, &mut is_used);
                        }
                        // We need to check up and down for all digits
                        let up = (Some(current_coord.0), current_coord.1.checked_sub(1));
                        let down = (Some(current_coord.0), Some(current_coord.1 + 1));
                        let coords = vec![up, down];
                        self.check_coords(coords, &mut is_used);

                        if is_used {
                            break;
                        }
                    }
                    if is_used {
                        let as_num = part_number.num.parse::<u32>().expect("Failed to convert str to u32");
                        sum += as_num;
                    }
                }
                PartOrSymbol::Symbol(_symbol) => ()
            }
        }
        sum
    }

    fn get_number_from_coord(&self, check_coord: (usize, usize)) -> Option<PartNumber> {
        for (_coord, part_or_symbol) in &self.map {
            match part_or_symbol {
                PartOrSymbol::Symbol(_) => (),
                PartOrSymbol::Part(part) => {
                    if part.coords.contains(&check_coord) {
                        return Some(part.clone());
                    }
                }
            }
        }
        None
    }

    pub fn sum_gear_ratio(&self) -> u32 {
        let mut sum = 0;
        for (coord, part_or_symbol) in &self.map {
            match part_or_symbol {
                PartOrSymbol::Symbol(symbol) => {
                    if *symbol == '*' {
                        let mut read_coords: Vec<(usize, usize)> = Vec::new();
                        let mut adjacent_numbers: Vec<u32> = Vec::new();
                        for x in -1i32..=1i32 {
                            for y in -1i32..=1i32 {
                                if adjacent_numbers.len() > 2 {
                                    break;
                                }
                                // We have to make sure we are not checking out of bounds
                                let check_x = coord.0 as i32 + x;
                                let check_y = coord.1 as i32 + y;
                                if check_x < 0 || check_y < 0 {
                                    continue
                                }
                                let check_coord = (check_x as usize, check_y as usize);
                                // We don't want to check ourselves or a spot that has already been read from
                                if (x == 0 && y == 0) || read_coords.contains(&check_coord) {
                                    continue
                                }
                                let potential_number = self.get_number_from_coord(check_coord);
                                if potential_number.is_some() {
                                    let num = potential_number.unwrap();
                                    read_coords.append(&mut num.coords.clone());
                                    adjacent_numbers.push(num.num.parse::<u32>().unwrap())
                                }
                            }
                        }
                        // Check if there are exactly 2 adjacent numbers
                        if adjacent_numbers.len() == 2 {
                            // Multiply the two numbers together to get the gear ratio
                            sum += adjacent_numbers[0] * adjacent_numbers[1];
                        }
                    }
                }
                PartOrSymbol::Part(_) => ()
            }
        }
        sum
    }
}

impl Problem for Day3 {
    type Input = Vec<String>;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let map = PartMap::new(input);
        map.sum_unused_parts()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let map = PartMap::new(input);
        map.sum_gear_ratio()
    }
}

fn main() {
    solve_main::<Day3>();
}
