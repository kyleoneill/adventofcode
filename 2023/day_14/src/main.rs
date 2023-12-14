use std::{fmt, collections::HashMap};
use problem::{solve_main, Problem};

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self {x, y}
    }
}

enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Platform {
    plane: Vec<Vec<Tile>>,
    width: usize,
    height: usize
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.plane {
            for tile in row {
                write!(f, "{}", tile).unwrap()
            }
            write!(f, "\n").unwrap()
        }
        fmt::Result::Ok(())
    }
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

    /// Ray cast down the row
    /// Save the coordinate of the first empty tile we find
    /// Save the coordinate of every round rock we find, until we hit a cube rock
    /// When we hit a cube rock, replace every observed circle rock with ground and then add circle rocks to the first empty tile we saved
    /// Repeat this after finding the first empty tile after that cube rock, until we reach the last column
    fn slide_rocks(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                for x_index in 0..self.width {
                    let mut empty_tile: Option<Coord> = None;
                    let mut observed_round_rock_locations: Vec<Coord> = Vec::new();
                    for y_index in 0..self.height {
                        self.check_tile(&mut empty_tile, &mut observed_round_rock_locations, x_index, y_index, &direction);
                    }
                    if empty_tile.is_some() {
                        self.move_rocks(&observed_round_rock_locations, x_index, self.height, &empty_tile.unwrap(), &direction);
                    }
                }
            },
            Direction::East => {
                for y_index in 0..self.height {
                    let mut empty_tile: Option<Coord> = None;
                    let mut observed_round_rock_locations: Vec<Coord> = Vec::new();
                    for x_index in (0..self.width).rev() {
                        self.check_tile(&mut empty_tile, &mut observed_round_rock_locations, x_index, y_index, &direction);
                    }
                    if empty_tile.is_some() {
                        self.move_rocks(&observed_round_rock_locations, self.width, y_index, &empty_tile.unwrap(), &direction);
                    }
                }
            },
            Direction::South => {
                for x_index in 0..self.width {
                    let mut empty_tile: Option<Coord> = None;
                    let mut observed_round_rock_locations: Vec<Coord> = Vec::new();
                    for y_index in (0..self.height).rev() {
                        self.check_tile(&mut empty_tile, &mut observed_round_rock_locations, x_index, y_index, &direction);
                    }
                    if empty_tile.is_some() {
                        self.move_rocks(&observed_round_rock_locations, x_index, self.height, &empty_tile.unwrap(), &direction);
                    }
                }
            },
            Direction::West => {
                for y_index in 0..self.height {
                    let mut empty_tile: Option<Coord> = None;
                    let mut observed_round_rock_locations: Vec<Coord> = Vec::new();
                    for x_index in 0..self.width {
                        self.check_tile(&mut empty_tile, &mut observed_round_rock_locations, x_index, y_index, &direction);
                    }
                    if empty_tile.is_some() {
                        self.move_rocks(&observed_round_rock_locations, self.width, y_index, &empty_tile.unwrap(), &direction);
                    }
                }
            }
        }
    }

    fn check_tile(&mut self, empty_tile: &mut Option<Coord>, observed_round_rock_locations: &mut Vec<Coord>, x_index: usize, y_index: usize, direction: &Direction) {
        match self.plane[y_index][x_index] {
            Tile::RoundRock => {
                if empty_tile.is_some() {
                    observed_round_rock_locations.push(Coord::new(x_index, y_index));
                }
            },
            Tile::Ground => {
                if empty_tile.is_none() {
                    *empty_tile = Some(Coord::new(x_index, y_index));
                }
            },
            Tile::CubeRock => {
                if empty_tile.is_some() {
                    self.move_rocks(observed_round_rock_locations, x_index, y_index, empty_tile.as_ref().unwrap(), direction);
                    *observed_round_rock_locations = Vec::new();
                    *empty_tile = None;
                }
            }
        }
    }

    fn move_rocks(&mut self, observed_round_rock_locations: &Vec<Coord>, x_index: usize, y_index: usize, empty_tile_coord: &Coord, direction: &Direction) {
        let round_rocks_to_move = observed_round_rock_locations.len();
        for coord in observed_round_rock_locations {
            self.plane[coord.y][coord.x] = Tile::Ground;
        }
        match direction {
            Direction::North => {
                for i in 0..round_rocks_to_move {
                    self.plane[empty_tile_coord.y + i][x_index] = Tile::RoundRock;
                }
            },
            Direction::East => {
                for i in 0..round_rocks_to_move {
                    self.plane[y_index][empty_tile_coord.x - i] = Tile::RoundRock;
                }
            },
            Direction::South => {
                for i in 0..round_rocks_to_move {
                    self.plane[empty_tile_coord.y - i][x_index] = Tile::RoundRock;
                }
            },
            Direction::West => {
                for i in 0..round_rocks_to_move {
                    self.plane[y_index][empty_tile_coord.x + i] = Tile::RoundRock;
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

    fn cycle(&mut self) {
        let mut map: HashMap<Self, i64> = HashMap::new();
        let mut i = 0;
        while i < 1_000_000_000 {
            match map.get(self) {
                Some(iteration_seen) => {
                    let distance_to_end = 1_000_000_000 - i;
                    // the length until we loop is the current index i minus when we first saw it
                    // Ex, the insert at i=1 got a result at i=3 so we do 3-1 to get a loop length of 2
                    let loop_len = i - iteration_seen;
                    let distance_to_jump = (distance_to_end / loop_len) * loop_len;
                    i += distance_to_jump;
                },
                None => { map.insert(self.clone(), i); }
            };
            self.slide_rocks(Direction::North);
            self.slide_rocks(Direction::West);
            self.slide_rocks(Direction::South);
            self.slide_rocks(Direction::East);
            i += 1;
        }
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
        let mut platform = Platform::from_input(input);
        platform.cycle();
        platform.count_load()
    }
}

fn main() {
    solve_main::<Day14>();
}
