use std::collections::HashMap;

use problem::{solve_main, Problem, Coord};

fn input_to_chars(input: &Vec<String>) -> Vec<char> {
    // Input currently throws an error when T is Vec<char>, this is a hack rather than fixing it
    if input.len() != 1 {
        panic!("Input must be a single line")
    }
    input[0].chars().collect()
}

struct Day3;

impl Problem for Day3 {
    type Input = Vec<String>;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let actual_input = input_to_chars(input);

        let mut position: Coord<i32> = Coord{ x: 0, y: 0 };
        let mut grid: HashMap<Coord<i32>, bool> = HashMap::new();
        grid.insert(position, true);

        for next_move in actual_input {
            match next_move {
                '^' => position.y += 1,
                '>' => position.x += 1,
                'v' => position.y -= 1,
                '<' => position.x -= 1,
                _ => panic!("Got an undefined direction")
            };
            grid.insert(position, true);
        }
        grid.len() as u32
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let actual_input = input_to_chars(input);

        let mut position_santa: Coord<i32> = Coord{ x: 0, y: 0 };
        let mut position_robot: Coord<i32> = Coord{ x: 0, y: 0 };
        let mut grid: HashMap<Coord<i32>, bool> = HashMap::new();
        grid.insert(position_santa, true);

        // This is not really DRY / abstracted well, but it's a quick solution
        let mut santas_turn = true;
        for next_move in actual_input {
            if santas_turn {
                match next_move {
                    '^' => position_santa.y += 1,
                    '>' => position_santa.x += 1,
                    'v' => position_santa.y -= 1,
                    '<' => position_santa.x -= 1,
                    _ => panic!("Got an undefined direction")
                };
                grid.insert(position_santa, true);
            }
            else {
                match next_move {
                    '^' => position_robot.y += 1,
                    '>' => position_robot.x += 1,
                    'v' => position_robot.y -= 1,
                    '<' => position_robot.x -= 1,
                    _ => panic!("Got an undefined direction")
                };
                grid.insert(position_robot, true);
            }
            santas_turn = !santas_turn;
        }
        grid.len() as u32
    }
}

fn main() {
    solve_main::<Day3>();
}
