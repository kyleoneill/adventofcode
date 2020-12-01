use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut grid = HashMap::new();
    for (index, line) in reader.lines().enumerate() {
        let mut current_coord = (0, 0);
        for s in line?.split(",") {
            let movement = remove_first(s).unwrap().parse::<u32>().unwrap();
            let direction = s.chars().next().unwrap();
            match direction {
                'R' => for _ in 0 .. movement {
                    current_coord = (current_coord.0 + 1, current_coord.1);
                    grid = update_grid(current_coord, grid, index);
                },
                'L' => for _ in 0 .. movement {
                    current_coord = (current_coord.0 - 1, current_coord.1);
                    grid = update_grid(current_coord, grid, index);
                },
                'U' => for _ in 0 .. movement {
                    current_coord = (current_coord.0, current_coord.1 + 1);
                    grid = update_grid(current_coord, grid, index);
                },
                'D' => for _ in 0 .. movement {
                    current_coord = (current_coord.0, current_coord.1 - 1);
                    grid = update_grid(current_coord, grid, index);
                },
                _ => panic!("Unexpected movement: {}", direction)
            }
        }
    }
    let solution = grid.iter().filter(|(key, value)| **value == 3).map(|(key, value)| key.0.abs() + key.1.abs()).min().unwrap();
    println!("Solution is: {}", solution);
    Ok(())
}

fn update_grid(coordinate: (i32, i32), mut grid: HashMap<(i32, i32), u32>, current_wire: usize) -> HashMap<(i32, i32), u32> {
    //0 none, 1 wire 1, 2 wire 2, 3 both
    if grid.get(&coordinate).is_none() {grid.insert(coordinate, 0);}
    match grid[&coordinate] {
        0 => *grid.get_mut(&coordinate).unwrap() = current_wire as u32 + 1,
        1 | 2 => *grid.get_mut(&coordinate).unwrap() |= current_wire as u32 + 1,
        3 => (),
        _ => panic!("Unexpected grid value: {:?}", &coordinate)
    }
    grid
}

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}
