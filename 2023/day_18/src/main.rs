use problem::{solve_main, Problem, Direction, Coord};

#[derive(Debug)]
struct LineSegment {
    length: isize,
    direction: Direction
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    count: isize
}

impl Instruction {
    fn custom_from_str(s: &str, version: usize) -> Self {
        let mut split = s.split(" ");
        if version == 1 {
            let direction = match split.next().unwrap().chars().next().unwrap() {
                'R' => Direction::Right,
                'L' => Direction::Left,
                'U' => Direction::Up,
                'D' => Direction::Down,
                _ => panic!("Got invalid instruction direction part one")
            };
            let count: isize = split.next().unwrap().parse().unwrap();
            Self { direction, count }
        }
        else {
            let paint_str_with_padding = split.skip(2).next().unwrap();
            let count = isize::from_str_radix(&paint_str_with_padding[2..paint_str_with_padding.len() - 2], 16).unwrap();
            let direction = match paint_str_with_padding.chars().nth(paint_str_with_padding.len() - 2).unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => panic!("Got invalid instruction direction part two")
            };
            Self { direction, count }

        }
    }
    fn from_input(input: &Vec<String>, version: usize) -> Vec<Self> {
        let mut ret: Vec<Self> = Vec::new();
        for line in input {
            ret.push(Instruction::custom_from_str(line, version));
        }
        ret
    }
    fn get_edge_coordinates(instructions: &Vec<Self>) -> Vec<Coord<isize>> {
        // We need to draw our shape with line segments and then get the coordinates to feed to the shoelace algorithm
        // The line segment shape is not going to be exactly equal to the shape drawn by the input, we will sometimes need
        // to add or subtract 1 to a line segment depending on what its corners look like. This accounts for weird edge space
        let mut line_segments: Vec<LineSegment> = Vec::new();
        for (i, instruction) in instructions.iter().enumerate() {
            let previous_direction = match i > 0 {
                // Check if we're on the first instruction. If we are, the previous instruction is going to be the final one. This
                // is because our instruction set builds a closed loop shape
                true => {
                    instructions[i - 1].direction
                },
                false => {
                    instructions[instructions.len() - 1].direction
                }
            };
            let next_direction = match i < instructions.len() - 1 {
                // If we're on the last instruction, the next one is going to be the first one
                true => {
                    instructions[i + 1].direction
                },
                false => {
                    instructions[0].direction
                }
            };
            // Get the three segment curve of previous to current and then current to next. This tells us if there are 0, 1, or 2
            // clockwise edges. The length depends on the number of clockwise edges to account for changes in area due to U and S shaped
            // curves/bends in our polygon
            let is_previous_clockwise = previous_direction.is_clockwise_turn(instruction.direction);
            let is_next_clockwise = instruction.direction.is_clockwise_turn(next_direction);
            let segment_length = instruction.count - 1 + (is_previous_clockwise as isize) + (is_next_clockwise as isize);
            line_segments.push(LineSegment { length: segment_length, direction: instruction.direction })
        }

        let mut current_coord = Coord::<isize>::new(0, 0);
        let mut edge_coords = Vec::from([current_coord]);
        for line in line_segments {
            current_coord = match line.direction {
                Direction::Up => {
                    Coord::new(current_coord.x, current_coord.y - line.length)
                },
                Direction::Down => {
                    Coord::new(current_coord.x, current_coord.y + line.length)
                },
                Direction::Left => {
                    Coord::new(current_coord.x - line.length, current_coord.y)
                },
                Direction::Right => {
                    Coord::new(current_coord.x + line.length, current_coord.y)
                }
            };
            edge_coords.push(current_coord);
        }
        edge_coords
    }
}

fn shoelace(corners: Vec<Coord<isize>>) -> usize {
    let mut area: isize = 0;
    for i in 0..corners.len() {
        let current_coord = corners[i];
        let next_coord = if i == corners.len() - 1 { corners[0] } else { corners[i + 1] };
        area += (current_coord.y + next_coord.y) * (current_coord.x - next_coord.x);
    }
    (area / 2) as usize
}

struct Day18;

impl Problem for Day18 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    /*
    Edge = Direction and a length
    For each edge, look at the direction of the previous edge and the direction of the next edge.
    num_of_clockwise_corners is 0, 1, or 2 depending on whether the n-1 to n corner is clockwise and n to n+1 corner is clockwise
    EdgeLength = InstructionLength - 1 + num_of_clockwise_corners

    Above generates the lines, we then want a Vec<Coord> to pass to shoelace algorithm
    start at 0,0 and then add the edge length along our direction

    Then run shoelace algorithm one set of coordinates at a time to get the area contribution for each edge
    area for an edge will sometimes be negative. avoid unsigned integers
    */

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let instructions = Instruction::from_input(input, 1);
        let corner_points = Instruction::get_edge_coordinates(&instructions);
        shoelace(corner_points)
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let instructions = Instruction::from_input(input, 2);
        let corner_points = Instruction::get_edge_coordinates(&instructions);
        shoelace(corner_points)
    }
}

fn main() {
    solve_main::<Day18>();
}
