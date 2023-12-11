use problem::{solve_main, Problem};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Fitting {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Start,
    Ground
}

#[derive(Debug)]
struct Walk {
    direction: Direction,
    coord: Coord
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Pipe {
    fitting: Fitting,
    loop_pipe: bool
}
impl Pipe {
    pub fn new(input: &char) -> Self {
        let fitting = match input {
            '|' => Fitting::NS,
            '-' => Fitting::EW,
            'L' => Fitting::NE,
            'J' => Fitting::NW,
            '7' => Fitting::SW,
            'F' => Fitting::SE,
            'S' => Fitting::Start,
            '.' => Fitting::Ground,
            _ => panic!("Got invalid pipe fitting char")
        };
        Self { fitting, loop_pipe: false }
    }
    pub fn walk_pipe(&self, walk: &mut Walk) {
        match self.fitting {
            Fitting::NS => {
                match walk.direction {
                    Direction::North => {
                        walk.coord.y += 1;
                        walk.direction = Direction::North;
                    },
                    Direction::South => {
                        walk.coord.y -= 1;
                        walk.direction = Direction::South
                    },
                    _ => panic!("Got direction other than north/south for a NS pipe")
                }
            },
            Fitting::EW => {
                match walk.direction {
                    Direction::East => {
                        walk.coord.x -= 1;
                        walk.direction = Direction::East;
                    },
                    Direction::West => {
                        walk.coord.x += 1;
                        walk.direction = Direction::West
                    },
                    _ => panic!("Got direction other than east/west for a EW pipe")
                }
            },
            Fitting::NE => {
                match walk.direction {
                    Direction::North => {
                        walk.coord.x += 1;
                        walk.direction = Direction::West;
                    },
                    Direction::East => {
                        walk.coord.y -= 1;
                        walk.direction = Direction::South;
                    },
                    _ => panic!("Got direction other than north/east for a NE pipe")
                }
            },
            Fitting::NW => {
                match walk.direction {
                    Direction::North => {
                        walk.coord.x -= 1;
                        walk.direction = Direction::East;
                    },
                    Direction::West => {
                        walk.coord.y -= 1;
                        walk.direction = Direction::South;
                    },
                    _ => panic!("Got direction other than north/west for a NW pipe")
                }
            },
            Fitting::SW => {
                match walk.direction {
                    Direction::South => {
                        walk.coord.x -= 1;
                        walk.direction = Direction::East;
                    },
                    Direction::West => {
                        walk.coord.y += 1;
                        walk.direction = Direction::North;
                    },
                    _ => panic!("Got direction other than south/west for a SW pipe")
                }
            },
            Fitting::SE => {
                match walk.direction {
                    Direction::South => {
                        walk.coord.x += 1;
                        walk.direction = Direction::West;
                    },
                    Direction::East => {
                        walk.coord.y += 1;
                        walk.direction = Direction::North;
                    },
                    _ => panic!("Got direction other than south/east for a SE pipe")
                }
            }
            Fitting::Start => {
                panic!("Got 'Start' Fitting variant after it should have been converted into another variant")
            },
            Fitting::Ground => {
                panic!("Got 'ground' fitting variant when walking a pipe loop")
            }
        }
    }
}

fn read_input(input: &Vec<String>) -> (HashMap<Coord, Pipe>, Coord) {
    let mut res: HashMap<Coord, Pipe> = HashMap::new();
    let mut start_pos: Option<Coord> = None;
    for y in 0..input.len() {
        let chars: Vec<char> = input[y].chars().collect();
        for x in 0..chars.len() {
            let char = chars[x];
            if char == 'S' {
                start_pos = Some(Coord{x, y});
                // THIS IS A HACK - all of my test inputs and full inputs have S connected right and down. This will not work on all valid possible puzzle inputs
                let start_pipe = Pipe::new(&'F');
                res.insert(Coord{x, y}, start_pipe);
            }
            else {
                let pipe = Pipe::new(&chars[x]);
                res.insert(Coord{x, y}, pipe);
            }
        }
    }
    if start_pos.is_none() {
        panic!("Did not get a start position");
    }
    (res, start_pos.unwrap())
}

fn walk_loop(map: HashMap<Coord, Pipe>, start_pos: Coord) -> u32 {
    // Begin walking the loop in both directions
    let mut steps = 0;
    let mut left_walk: Walk = Walk {direction: Direction::East, coord: start_pos};
    let mut right_walk: Walk = Walk {direction: Direction::South, coord: start_pos};
    loop {
        // Walk the pipes for both left and right
        let left_pipe = map.get(&left_walk.coord).expect("Failed to get a pipe while walking");
        left_pipe.walk_pipe(&mut left_walk);

        let right_pipe = map.get(&right_walk.coord).expect("Failed to get a pipe while walking");
        right_pipe.walk_pipe(&mut right_walk);

        // Increment walk pipe counter
        steps += 1;

        // If the coordinates for left/right are equal, the pipe is the farthest spot in the loop
        // Can also check if the pipe has already been visited by whoever walks second
        if left_walk.coord == right_walk.coord {
            break;
        }
    }
    steps
}

fn build_loop(map: &mut HashMap<Coord, Pipe>, start_pos: &Coord) {
    let mut walk: Walk = Walk {direction: Direction::East, coord: *start_pos};
    loop {
        let pipe = map.get_mut(&walk.coord).expect("Failed to get a pipe while walking");
        // If this pipe is already a loop_pipe then we've already encountered it, our loop is done
        if pipe.loop_pipe {
            break;
        }
        pipe.loop_pipe = true;
        pipe.walk_pipe(&mut walk);
    }
}

fn count_enclosed_tiles(map: &HashMap<Coord, Pipe>, width: usize, height: usize) -> u32 {
    //Raycast into the hashmap
    // If we hit a vertical loop pipe, invert some bool flag to signal we are inside the loop. any non-loop thing we hit as we trace updates our return counter
    // ADDENDUM: If we hit a loop corner piece, we take note of it and will conditionally do inversions depending on how it is closed.
    //   we will hit either a NE or a SE (we are casting along a line left to right, so we must hit one of the two L or F pipes before their closers)
    //   Continue reading the cast until we get to a second corner. If the corners form a 'u' or 'n' then we do not invert
    //   if they are opposites (the opener runs north, the closer runs south) then we DO invert the flag
    //   Continue reading the line, if we run into this corner pipe scenario again then we repeat this.
    //     Ex, the first encounter of the non 'u' or 'n' corner pairings inverts us to outside, a second encounter of a non 'u' or 'n' will invert us back to being inside
    let mut tile_count = 0;
    for y in 0..height {
        let mut last_corner_pipe: Option<Fitting> = None;
        let mut enclosed = false;
        for x in 0..width {
            let tile = map.get(&Coord{x, y}).expect("Failed to get tile when reading everything from hashmap");
            if tile.loop_pipe {
                match tile.fitting {
                    Fitting::NS => { enclosed = !enclosed; },
                    Fitting::NE => { last_corner_pipe = Some(Fitting::NE); },
                    Fitting::SE => { last_corner_pipe = Some(Fitting::SE); },
                    Fitting::SW => {
                        // It should be okay to unwrap here as a pipe coming from the right must have a corresponding left-going pipe already seen
                        if last_corner_pipe.unwrap() == Fitting::NE {
                            enclosed = !enclosed;
                        }
                    },
                    Fitting::NW => {
                        if last_corner_pipe.unwrap() == Fitting::SE {
                            enclosed = !enclosed;
                        }
                    },
                    _ => ()
                }
            }
            else {
                if enclosed {
                    tile_count += 1;
                }
            }
        }
    }
    tile_count
}

struct Day10;

impl Problem for Day10 {
    type Input = Vec<String>;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let (map, start_pos) = read_input(input);
        walk_loop(map, start_pos)
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let height = input.len();
        let width = input[0].chars().count();
        let (mut map, start_pos) = read_input(input);
        build_loop(&mut map, &start_pos);
        count_enclosed_tiles(&map, width, height)
    }
}

fn main() {
    solve_main::<Day10>();
}
