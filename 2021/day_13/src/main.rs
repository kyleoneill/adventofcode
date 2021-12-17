use std::fmt::{Display, Formatter};
use std::io::BufRead;

use problem::{Input, Problem, solve_main};
use anyhow::Result;

// Input is parsed as two tuple vecs. The dots are a series of tuples where
// 0 is x and 1 is y. The folds are a series of tuples where the bool signifies if the fold
// is on the x or y axis. A true value means fold x, a false means fold y. The i32 is where the
// fold will be along.
#[derive(Debug, Clone)]
struct TransparentPaper {
    dots: Vec<(i32, i32)>,
    folds: Vec<(bool, i32)>
}

impl Input for TransparentPaper {
    fn parse<R: BufRead>(reader: R) -> Result<Self> {
        let mut input_iter = reader.lines();
        let mut dots: Vec<(i32, i32)> = Vec::new();
        let mut folds: Vec<(bool, i32)> = Vec::new();
        let mut current_line = input_iter.next().unwrap()?;
        while !current_line.is_empty() {
            let mut split = current_line.split(",");
            dots.push((split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap()));
            current_line = input_iter.next().unwrap()?;
        }
        for line in input_iter {
            let line = line?;
            let first_split: Vec<&str> = line.split_whitespace().collect();
            let second_split: Vec<&str> = first_split[2].split("=").collect();
            folds.push((second_split[0] == "x", second_split[1].parse().unwrap()));
        }
        Ok(TransparentPaper { dots, folds })
    }
}

impl Display for TransparentPaper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut max_x = 0;
        let mut max_y = 0;
        for dot in &self.dots {
            if dot.0 > max_x {
                max_x = dot.0;
            }
            if dot.1 > max_y {
                max_y = dot.1;
            }
        }
        for y in 0..= max_y {
            for x in 0..=max_x {
                if self.dots.contains(&(x, y)) {
                    write!(f, "#").unwrap();
                }
                else {
                    write!(f, ".").unwrap();
                }
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}

impl TransparentPaper {
    fn process_folds(&mut self) {
        for instruction in &self.folds {
            Self::fold_paper(&instruction, &mut self.dots);
        }
    }
    fn fold_paper(instruction: &(bool, i32), dots: &mut Vec<(i32, i32)>) {
        for dot in dots.iter_mut() {
            // Fold x
            if instruction.0 {
                if dot.0 > instruction.1 {
                    dot.0 = instruction.1 + (instruction.1 - dot.0);
                }
            }
            // Fold y
            else {
                if dot.1 > instruction.1 {
                    dot.1 = instruction.1 + (instruction.1 - dot.1);
                }
            }
        }
        // If dots overlap, remove the dot
        dots.sort_unstable();
        dots.dedup();
    }
    fn count_visible_dots(&self) -> usize {
        self.dots.len()
    }
}

struct Day13;
impl Problem for Day13 {
    type Input = TransparentPaper;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut paper = input.clone();
        let first_instruction = paper.folds.iter().next().unwrap();
        TransparentPaper::fold_paper(first_instruction, &mut paper.dots);
        paper.count_visible_dots()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut paper = input.clone();
        paper.process_folds();
        println!("{}", paper);
        paper.count_visible_dots()
    }
}

fn main() {
    solve_main::<Day13>();
}
