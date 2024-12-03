use std::str::FromStr;
use problem::{solve_main, Problem};

#[derive(Debug)]
struct PresentBox {
    length: u32,
    width: u32,
    height: u32
}

#[derive(Debug)]
struct ParseBoxErr;
impl FromStr for PresentBox {
    type Err = ParseBoxErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('x');
        let length = split.next().expect("Failed to get length").parse::<u32>().expect("Failed to parse length to u32");
        let width = split.next().expect("Failed to get width").parse::<u32>().expect("Failed to parse width to u32");
        let height = split.next().expect("Failed to get height").parse::<u32>().expect("Failed to parse height to u32");
        Ok(Self{ length, width, height })
    }
}

impl PresentBox {
    fn smallest_side_area(&self) -> u32 {
        let first = self.length * self.width;
        let second = self.width * self.height;
        let third = self.height * self.length;
        let mut sides = vec![first, second, third];
        sides.sort();
        sides[0]
    }

    fn surface_area(&self) -> u32 {
        (2 * self.length * self.width) + (2 * self.width * self.height) + (2 * self.height * self.length)
    }

    fn required_wrapping_paper(&self) -> u32 {
        self.surface_area() + self.smallest_side_area()
    }

    fn smallest_perimeter(&self) -> u32 {
        let mut dimensions = vec![self.length, self.width, self.height];
        dimensions.sort();
        (2 * dimensions[0]) + (2 * dimensions[1])
    }

    fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }

    fn required_ribbon(&self) -> u32 {
        self.smallest_perimeter() + self.volume()
    }
}

struct Day2;

impl Problem for Day2 {
    type Input = Vec<String>;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut total_wrapping_paper: Self::PartOne = 0;
        for dimensions in input {
            let present = PresentBox::from_str(&dimensions).expect("Failed to convert string to present box");
            total_wrapping_paper += present.required_wrapping_paper();
        }
        total_wrapping_paper
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut total_ribbon: Self::PartTwo = 0;
        for dimensions in input {
            let present = PresentBox::from_str(&dimensions).expect("Failed to convert string to present box");
            total_ribbon += present.required_ribbon();
        }
        total_ribbon
    }
}

fn main() {
    solve_main::<Day2>();
}
