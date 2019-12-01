use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut required_fuel: i64 = 0;
    for line in reader.lines() {
        let mass: i32 = line?.parse().unwrap();
        required_fuel += get_module_fuel(mass);
    }
    println!("The total required fuel is: {}", required_fuel);
    Ok(())
}

fn get_module_fuel(starting_mass: i32) -> i64 {
    let mut total_module_fuel: i64 = (starting_mass / 3 - 2).into();
    let mut step_fuel = total_module_fuel;
    loop {
        step_fuel = step_fuel / 3 - 2;
        if step_fuel > 0 {
            total_module_fuel += step_fuel;
        }
        else {
            break;
        }
    }
    return total_module_fuel
}
