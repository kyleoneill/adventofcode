use problem::{solve_main, Problem};

fn look_and_say(input: &str) -> String {
    let mut chars = input.chars();
    let mut running_count: usize = 1;
    let mut current_char = chars.next().expect("Got an empty string for look and say");
    let mut build_string = String::new();
    while let Some(next_char) = chars.next() {
        if next_char == current_char {
            running_count += 1;
        }
        else {
            build_string.push_str(running_count.to_string().as_str());
            // The number will only ever be 1, 2, or 3 so we can always assume a char is valid here
            build_string.push(current_char);
            running_count = 1;
            current_char = next_char;
        }
    }
    build_string.push_str(running_count.to_string().as_str());
    build_string.push(current_char);

    build_string
}

struct Day10;

impl Problem for Day10 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let steps: usize = 40;
        let mut result: String = input[0].clone();
        for _ in 0..steps {
            result = look_and_say(result.as_str());
        }
        result.len()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let steps: usize = 50;
        let mut result: String = input[0].clone();
        for _ in 0..steps {
            result = look_and_say(result.as_str());
        }
        result.len()
    }
}

fn main() {
    solve_main::<Day10>();
}
