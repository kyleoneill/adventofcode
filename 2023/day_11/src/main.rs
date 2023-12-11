use problem::{solve_main, Problem};

struct Coord {
    x: usize,
    y: usize
}

impl Coord {
    pub fn new(x: usize, y:usize) -> Self {
        Coord { x, y }
    }
    pub fn distance_to_other(&self, other: &Self) -> u64 {
        let length = (self.x as i64 - other.x as i64).abs();
        let height = (self.y as i64 - other.y as i64).abs();
        length as u64 + height as u64
    }
}

fn expand_space(input: &Vec<String>) -> Vec<Vec<char>> {
    let mut space: Vec<Vec<char>> = Vec::new();
    for line in input {
        space.push(line.chars().collect());
    }

    let mut column_length = space.len();
    let mut row_length = space[0].len();

    // Check for rows that must be added
    let mut current_row = 0;
    loop {
        if current_row == column_length {
            break;
        }
        if space[current_row].iter().all(|n| *n == '.') {
            space.insert(current_row, vec!['O'; row_length]);
            current_row += 1;
            column_length += 1;
        }
        current_row += 1;
    }

    // Check for columns that must be added
    let mut current_column = 0;
    loop {
        if current_column == row_length {
            break;
        }
        let mut column_is_empty = true;
        for current_row in 0..column_length {
            if space[current_row][current_column] == '#' {
                column_is_empty = false;
                break;
            }
        }
        if column_is_empty {
            for current_row in 0..column_length {
                space[current_row].insert(current_column, 'O');
            }
            current_column += 1;
            row_length += 1;
        }
        current_column += 1;
    }
    space
}

fn map_galaxies(space: Vec<Vec<char>>, version: i32) -> Vec<Coord> {
    let mut galaxies: Vec<Coord> = Vec::new();
    let mut actual_y = 0;
    for y in 0..space.len() {
        let mut actual_x = 0;
        // If every char of the current row is O, we add a million to actual_y
        if version != 1 && space[y].iter().all(|n| *n == 'O') {
            actual_y += 999_998;
        }
        else {
            for x in 0..space[y].len() {
                if version != 1 && space[y][x] == 'O' {
                    actual_x += 999_998;
                }
                else if space[y][x] == '#' {
                    galaxies.push(Coord::new(actual_x, actual_y));
                }
                actual_x += 1
            }
        }
        actual_y += 1;
    }
    galaxies
}

fn sum_shortest_paths(galaxy_map: Vec<Coord>) -> u64 {
    let mut sum: u64 = 0;
    for i in 0..(galaxy_map.len() - 1) {
        // We want to check the distance from i to every element after i
        // We do not want the last element, as it has nothing further to compare to
        for j in (i + 1)..galaxy_map.len() {
            // Find the shortest distance from i to j, add it to our sum
            let first_coord = &galaxy_map[i];
            let second_coord = &galaxy_map[j];
            sum += first_coord.distance_to_other(second_coord);
        }
    }
    sum
}

struct Day11;

impl Problem for Day11 {
    type Input = Vec<String>;
    type PartOne = u64;
    type PartTwo = u64;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let expanded_space = expand_space(input);
        let galaxy_map = map_galaxies(expanded_space, 1);
        sum_shortest_paths(galaxy_map)
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let expanded_space = expand_space(input);
        let galaxy_map = map_galaxies(expanded_space, 2);
        sum_shortest_paths(galaxy_map)
    }
}

fn main() {
    solve_main::<Day11>();
}
