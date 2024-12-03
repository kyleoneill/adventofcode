use problem::{solve_main, Problem};

struct Day1;

impl Problem for Day1 {
    type Input = Vec<String>;
    type PartOne = i32;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut floor: Self::PartOne = 0;
        for op in input[0].chars() {
            match op {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => panic!("Got an undefined move op")
            }
        }
        floor
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut floor: Self::PartOne = 0;
        for (position, op) in input[0].chars().enumerate() {
            match op {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => panic!("Got an undefined move op")
            }
            if floor < 0 {
                return position + 1
            }
        }
        panic!("Did not find a solution for part 2")
    }
}

fn main() {
    solve_main::<Day1>();
}
