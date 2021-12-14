use problem::{Problem, solve};

#[derive(Debug)]
enum Error {
    NoSolution
}

fn is_open(c: char) -> bool {
    match c {
        '(' => true,
        '[' => true,
        '{' => true,
        '<' => true,
        _ => false
    }
}

fn open_close_match(open: char, close: char) -> bool {
    if open == '(' && close == ')' ||
        open == '[' && close == ']' ||
        open == '{' && close == '}' ||
        open == '<' && close == '>' {
            return true;
        }
    false
}

fn solve_1(input: &Vec<String>) -> Option<i32> {
    let mut score = 0;
    for line in input.iter() {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            if is_open(c) {
                stack.push(c);
            }
            else {
                if open_close_match(stack[stack.len() - 1], c) {
                    stack.pop().unwrap();
                }
                else {
                    // Line is corrupted, add score
                    score += match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => 0
                    };
                    break;
                }
            }
        }
    }
    Some(score)
}

fn solve_2(input: &Vec<String>) -> Option<u64> {
    let mut scores: Vec<u64> = Vec::new();
    'outer: for line in input.iter() {
        let mut line_score = 0;
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            if is_open(c) {
                stack.push(c);
            }
            else {
                if open_close_match(stack[stack.len() - 1], c) {
                    stack.pop().unwrap();
                }
                else {
                    // Line is corrupted, skip
                    continue 'outer;
                }
            }
        }
        for c in stack.iter().rev() {
            line_score *= 5;
            line_score += match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0
            };
        }
        scores.push(line_score);
    }
    scores.sort();
    Some(scores[scores.len() / 2])
}

struct Day10;
impl Problem for Day10 {
    type Input = Vec<String>;
    type Part1Output = i32;
    type Part2Output = u64;
    type Error = Error;

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        let result = solve_1(input).ok_or(Error::NoSolution)?;
        Ok(result)
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        let result = solve_2(input).ok_or(Error::NoSolution)?;
        Ok(result)
    }
}

fn main() {
    solve::<Day10>("input").unwrap();
}
