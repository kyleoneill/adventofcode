use problem::{Problem, solve_main};
use anyhow::Result;

#[derive(Clone)]
struct Grid {
    values: [[u8; 10]; 10]
}

impl problem::Input for Grid {
    fn parse<R: std::io::BufRead>(reader: R) -> Result<Self> {
        let mut result = Grid { values: [[0; 10]; 10] };
        for (y, line) in reader.lines().enumerate() {
            for (x, c) in line.unwrap().chars().enumerate() {
                // Subtract b'0' from a number to convert it from a character code to an int literal
                result.values[x][y] = c as u8 - b'0';
            }
        }
        Ok(result)
    }
}

impl Grid {
    fn step(&mut self) -> usize {
        for x in 0..10 {
            for y in 0..10 {
                self.values[x][y] += 1;
            }
        }
        let mut flashes = 0;
        let mut done = false;
        while !done {
            done = true;
            for x in 0..10 {
                for y in 0..10 {
                    if self.values[x][y] > 9 {
                        self.values[x][y] = 0;
                        flashes += 1;
                        done = false;
                        for dx in -1isize..=1 {
                            for dy in -1isize..=1 {
                                let nx = x as isize + dx;
                                let ny = y as isize + dy;
                                if nx >= 0 && ny >= 0 && nx < 10 && ny < 10 {
                                    if self.values[nx as usize][ny as usize] != 0 {
                                        self.values[nx as usize][ny as usize] += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        flashes
    }
}

struct Day11;
impl Problem for Day11 {
    type Input = Grid;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut grid = input.clone();
        let mut total_flashes = 0;
        for _n in 0..100 {
            total_flashes += grid.step();
        }
        total_flashes
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut grid = input.clone();
        let mut counter = 0;
        let mut done = false;
        while !done {
            counter += 1;
            let flashes = grid.step();
            if flashes == 100 {
                done = true;
            }
        }
        counter
    }
}

fn main() {
    solve_main::<Day11>();
}
