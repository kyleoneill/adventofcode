use std::collections::HashMap;
use problem::{solve_main, Problem};

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
struct Beam {
    direction: Direction,
    coord: Coord
}

impl Beam {
    fn new(coord: Coord, direction: Direction) -> Self {
        Self { direction, coord}
    }
    fn next_coord(&self) -> Option<Coord> {
        match self.direction {
            Direction::Up => {
                let new_y = match self.coord.y.checked_sub(1) {
                    Some(y) => y,
                    None => return None
                };
                Some(Coord::new(self.coord.x, new_y))
            },
            Direction::Down => {
                Some(Coord::new(self.coord.x, self.coord.y + 1))
            },
            Direction::Left => {
                let new_x = match self.coord.x.checked_sub(1) {
                    Some(x) => x,
                    None => return None
                };
                Some(Coord::new(new_x, self.coord.y))
            },
            Direction::Right => {
                Some(Coord::new(self.coord.x + 1, self.coord.y))
            }
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Coord { x, y }
    }
}

#[derive(Debug, Clone)]
struct Tile {
    value: TileValue,
    energized_left: bool,
    energized_right: bool,
    energized_up: bool,
    energized_down: bool
}

impl Tile {
    fn from_char(c: &char) -> Self {
        let value = TileValue::from_char(c);
        Self { value, energized_left: false, energized_right: false, energized_down: false, energized_up: false }
    }
    fn try_to_energize(&mut self, direction: &Direction) -> bool {
        match direction {
            Direction::Up => { if self.energized_up { false } else { self.energized_up = true; true } },
            Direction::Down => { if self.energized_down { false } else { self.energized_down = true; true }},
            Direction::Left => { if self.energized_left { false } else { self.energized_left = true; true }},
            Direction::Right => { if self.energized_right { false } else { self.energized_right = true; true }}
        }
    }
    fn is_energized(&self) -> bool {
        self.energized_up || self.energized_down || self.energized_left || self.energized_right
    }
}

#[derive(Debug, Clone)]
enum TileValue {
    Empty,
    VerticalSplitter,
    HorizontalSplitter,
    // /
    SlashMirror,
    // \
    BackslashMirror
}

impl TileValue {
    fn from_char(c: &char) -> Self {
        match c {
            '.' => Self::Empty,
            '|' => Self::VerticalSplitter,
            '-' => Self::HorizontalSplitter,
            '/' => Self::SlashMirror,
            '\\' => Self::BackslashMirror,
            _ => panic!("Got an invalid char while parsing tile values")
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    map: HashMap<Coord, Tile>,
    height: usize,
    width: usize
}

impl Grid {
    fn from_input(input: &Vec<String>) -> Self {
        let mut map: HashMap<Coord, Tile> = HashMap::new();
        let height = input.len();
        let width = input[0].as_str().chars().count();
        for (y, row) in input.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let tile = Tile::from_char(&c);
                map.insert(Coord::new(x, y), tile);
            }
        }
        Self { map, height, width }
    }
    fn step_beam(&mut self, mut beam: Beam) -> (Option<Beam>, Option<Beam>) {
        // Energize the tile the beam is currently on
        match self.map.get_mut(&beam.coord) {
            Some(tile) => {
                // We want to encode the direction being energized from here to catch if our beam is looping to kill it
                // We cannot just kill the beam if the tile has been energized from any direction, as this will kill non-looping beams
                match tile.try_to_energize(&beam.direction) {
                    true => (),
                    false => return (None, None)
                }
            },
            None => return (None, None)
        };

        // Check if we need to change direction or split
        // If we split, return a new beam
        let new_beam = match self.map.get(&beam.coord) {
            Some(tile) => {
                match tile.value {
                    TileValue::Empty => None,
                    TileValue::VerticalSplitter => {
                        if beam.direction == Direction::Left || beam.direction == Direction::Right {
                            beam.direction = Direction::Up;
                            Some(Beam::new(beam.coord.clone(), Direction::Down))
                        }
                        else {
                            None
                        }
                    },
                    TileValue::HorizontalSplitter => {
                        if beam.direction == Direction::Up || beam.direction == Direction::Down {
                            beam.direction = Direction::Left;
                            Some(Beam::new(beam.coord.clone(), Direction::Right))
                        }
                        else {
                            None
                        }
                    },
                    // /
                    TileValue::SlashMirror => {
                        beam.direction = match beam.direction {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Down,
                            Direction::Right => Direction::Up,
                        };
                        None
                    },
                    // \
                    TileValue::BackslashMirror => {
                        beam.direction = match beam.direction {
                            Direction::Up => Direction::Left,
                            Direction::Down => Direction::Right,
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                        };
                        None
                    },
                }
            },
            None => panic!("Failed to get a coord that was verified to exist")
        };

        // Get the next coord of our main beam, which may cease to exist if it's invalid
        let main_beam = match beam.next_coord() {
            Some(new_coord) => {
                beam.coord = new_coord;
                Some(beam)
            },
            None => None
        };

        // Check if the new beam exists and set its new coord if it does
        let moved_new_beam = match new_beam {
            Some(mut n) => {
                match n.next_coord() {
                    Some(n_new_coord) => {
                        n.coord = n_new_coord;
                        Some(n)
                    },
                    None => None
                }
            },
            None => None
        };
        (main_beam, moved_new_beam)
    }

    fn get_grid_energy(mut self, start_beam: Beam) -> usize {
        let mut beams: Vec<Beam> = vec![start_beam];
        while let Some(beam) = beams.pop() {
            let step_res = self.step_beam(beam);
            if step_res.0.is_some() { beams.push(step_res.0.unwrap()) }
            if step_res.1.is_some() { beams.push(step_res.1.unwrap()) }
        }
        self.map.values().filter(|&n| n.is_energized()).count()
    }
}

struct Day16;

impl Problem for Day16 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let beam = Beam::new(Coord::new(0, 0), Direction::Right);
        Grid::from_input(input).get_grid_energy(beam)
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut most_energized = 0;
        let grid = Grid::from_input(input);
        // Quick and dirty, if verbose and repetitive
        // Check all values going down from the top row
        for x in 0..grid.width {
            let start_beam = Beam::new(Coord::new(x, 0), Direction::Down);
            let energy = grid.clone().get_grid_energy(start_beam);
            if energy > most_energized { most_energized = energy }
        }
        // Check all values going right from the left side
        for y in 0..grid.height {
            let start_beam = Beam::new(Coord::new(0, y), Direction::Right);
            let energy = grid.clone().get_grid_energy(start_beam);
            if energy > most_energized { most_energized = energy }
        }
        // Check all values going left from the right side
        for y in 0..grid.height {
            let start_beam = Beam::new(Coord::new(grid.width - 1, y), Direction::Left);
            let energy = grid.clone().get_grid_energy(start_beam);
            if energy > most_energized { most_energized = energy }
        }
        // Check all values going up from the bottom
        for x in 0..grid.width {
            let start_beam = Beam::new(Coord::new(x, grid.height - 1), Direction::Up);
            let energy = grid.clone().get_grid_energy(start_beam);
            if energy > most_energized { most_energized = energy }
        }
        most_energized
    }
}

fn main() {
    solve_main::<Day16>();
}
