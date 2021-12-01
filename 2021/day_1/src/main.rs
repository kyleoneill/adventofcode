use problem::{Problem, solve};

fn solve_1(values: &[i32]) -> Option<i32> {
    let mut last_depth = None;
    let mut times_increased = 0;
    for depth in values.iter() {
        if last_depth.is_none() {
            last_depth = Some(depth);
        }
        else {
            if depth > last_depth.unwrap() {
                times_increased += 1;
            }
            last_depth = Some(depth)
        }
    }
    Some(times_increased)
}

fn solve_2(values: &[i32]) -> Option<i32> {
    let list_length = values.len() - 1;
    let mut times_increased = 0;
    let mut last_sliding_sum = None;
    for n in 0..=list_length {
        if n + 2 > list_length {
            break;
        }
        let current_sliding_window = values[n] + values[n + 1] + values[n + 2];
        if last_sliding_sum.is_none() {
            last_sliding_sum = Some(current_sliding_window);
        }
        else {
            if current_sliding_window > last_sliding_sum.unwrap() {
                times_increased += 1;
            }
            last_sliding_sum = Some(current_sliding_window);
        }
    }
    Some(times_increased)
}

#[derive(Debug)]
enum Error {
    NoSolution,
}

struct Day1;
impl Problem for Day1 {
    type Input = Vec<i32>;
    type Part1Output = i32;
    type Part2Output = i32;
    type Error = Error;

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        let result = solve_1(input.as_slice()).ok_or(Error::NoSolution)?;
        Ok(result)
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        let result = solve_2(input.as_slice()).ok_or(Error::NoSolution)?;
        Ok(result)
    }
}

fn main() {
    solve::<Day1>("input").unwrap();
}
