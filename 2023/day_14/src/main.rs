use std::fmt;
use problem::{solve_main, Problem};

enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Debug, PartialEq)]
enum Tile {
    RoundRock,
    CubeRock,
    Ground
}

impl Tile {
    fn from_char(c: &char) -> Self {
        match c {
            'O' => Self::RoundRock,
            '#' => Self::CubeRock,
            '.' => Self::Ground,
            _ => panic!("Got an invalid tile when converting a char to Tile")
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CubeRock => write!(f, "#"),
            Self::RoundRock => write!(f, "O"),
            Self::Ground => write!(f, ".")
        }
    }
}

#[derive(Debug)]
struct Platform {
    plane: Vec<Vec<Tile>>,
    width: usize,
    height: usize
}

impl Platform {
    fn from_input(input: &Vec<String>) -> Self {
        let mut res: Vec<Vec<Tile>> = Vec::new();
        for line in input {
            let mut row: Vec<Tile> = Vec::new();
            for c in line.chars() {
                row.push(Tile::from_char(&c));
            }
            res.push(row);
        }
        let width = res[0].len();
        let height = res.len();
        Self { plane: res, width, height }
    }

    fn slide_rocks(&mut self, direction: Direction) {
        for column_index in 0..self.width {
            // Ray cast down the row
            // Save the coordinate of the first empty tile we find
            // Save the coordinate of every round rock we find, until we hit a cube rock
            // When we hit a cube rock, replace every observed circle rock with ground and then add circle rocks to the first empty tile we saved
            // Repeat this after finding the first empty tile after that cube rock, until we reach the last column
            let mut empty_tile: Option<usize> = None;
            let mut observed_round_rock_locations: Vec<usize> = Vec::new();
            for row_index in 0..self.height {
                match self.plane[row_index][column_index] {
                    Tile::RoundRock => {
                        if empty_tile.is_some() {
                            observed_round_rock_locations.push(row_index);
                        }
                    },
                    Tile::Ground => {
                        if empty_tile.is_none() {
                            empty_tile = Some(row_index);
                        }
                    },
                    Tile::CubeRock => {
                        if empty_tile.is_some() {
                            let round_rocks_to_move = observed_round_rock_locations.len();
                            for val in observed_round_rock_locations {
                                self.plane[val][column_index] = Tile::Ground;
                            }
                            let slide_location = empty_tile.unwrap();
                            for i in 0..round_rocks_to_move {
                                self.plane[slide_location + i][column_index] = Tile::RoundRock;
                            }
                            observed_round_rock_locations = Vec::new();
                            empty_tile = None;
                        }
                    }
                }
            }
            // If we reached the end of the column without hitting a square rock, but we have circle rocks queued to move, move them now
            if empty_tile.is_some() {
                let round_rocks_to_move = observed_round_rock_locations.len();
                for val in observed_round_rock_locations {
                    self.plane[val][column_index] = Tile::Ground;
                }
                let slide_location = empty_tile.unwrap();
                for i in 0..round_rocks_to_move {
                    self.plane[slide_location + i][column_index] = Tile::RoundRock;
                }
            }
        }
    }

    fn count_load(&self) -> usize {
        let mut sum = 0;
        let mut inverted_height = self.height;
        for row in &self.plane {
            sum += row.iter().filter(|&n| n == &Tile::RoundRock).count() * inverted_height;
            inverted_height -= 1;
        }
        sum
    }
}

struct Day14;

impl Problem for Day14 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut platform = Platform::from_input(input);
        platform.slide_rocks(Direction::North);
        platform.count_load()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        0
    }
}

fn main() {
    solve_main::<Day14>();
}
