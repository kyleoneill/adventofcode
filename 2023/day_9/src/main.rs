use problem::{solve_main, Problem};

fn read_input(input: &Vec<String>, version: i32) -> Vec<Vec<i64>> {
    let mut res: Vec<Vec<i64>> = Vec::new();
    for line in input {
        let mut history: Vec<i64> = Vec::new();
        for i in line.split(" ") {
            let val = i.parse::<i64>().expect("Failed to convert value to i64");
            history.push(val);
        }
        if version != 1 {
            history.reverse();
        }
        res.push(history);
    }
    res 
}

fn sequence_history(history: &Vec<i64>) -> i64 {
    if history.iter().all(|n| *n == 0 ) {
        return 0
    }
    else {
        let mut next_history: Vec<i64> = Vec::new();
        for i in 1..history.len() {
            next_history.push(history[i] - history[i - 1]);
        }
        return history.last().unwrap() + sequence_history(&next_history);
    }
}

struct Day9;

impl Problem for Day9 {
    type Input = Vec<String>;
    type PartOne = i64;
    type PartTwo = i64;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        read_input(input, 1).iter().map(sequence_history).sum()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        read_input(input, 2).iter().map(sequence_history).sum()
    }
}

fn main() {
    solve_main::<Day9>();
}
