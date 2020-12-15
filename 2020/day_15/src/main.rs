use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let start = Instant::now();
    let input = vec![2, 0, 1, 9, 5, 19];
    let solution = solve(input);
    println!("Found solution in {} microseconds", start.elapsed().as_micros());
    println!("The solution is {}", solution);
}

fn solve(input: Vec<u32>) -> u32 {
    let mut tracker: HashMap<u32, u32> = HashMap::new(); //Tracker<number (key), last turn seen (value)>
    let final_number = 30000000;
    for n in 0..input.len() - 1 {
        tracker.insert(input[n], n as u32);
    }
    let mut prev_number = input[input.len() - 1];
    for n in input.len()..final_number {
        match tracker.get(&prev_number).cloned() {
            Some(val) => {
                tracker.insert(prev_number, n as u32 - 1);
                prev_number = (n as u32 - 1) - val;
            },
            None => {
                tracker.insert(prev_number, n as u32 - 1);
                prev_number = 0;
            }
        }
    }
    prev_number
}
