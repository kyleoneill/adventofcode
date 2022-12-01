use problem::{solve_main, Problem};

struct Elf {
    total_calories: u32
}

impl Elf {
    pub fn from_str_list(input: &Vec<String>) -> Vec<Self> {
        let mut elves: Vec<Self> = Vec::new();
        let mut food: Vec<u32> = Vec::new();
        for line in input {
            if line.is_empty() {
                let elf: Self = Self {
                    total_calories: food.iter().sum()
                };
                elves.push(elf);
                food = Vec::new();
            }
            else {
                food.push(line.parse().expect("Failed to parse string into u32"));
            }
        }
        if food.len() > 0 {
            let elf: Self = Self {
                total_calories: food.iter().sum()
            };
            elves.push(elf);
        }

        elves
    }

    pub fn sort_elves_by_total_calories(mut elves: Vec<Self>) -> Vec<Self> {
        elves.sort_by(|a, b| b.total_calories.cmp(&a.total_calories));
        elves
    }
}

struct Day1;

impl Problem for Day1 {
    type Input = Vec<String>;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let elves = Elf::from_str_list(input);
        let elves = Elf::sort_elves_by_total_calories(elves);
        elves[0].total_calories
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let elves = Elf::from_str_list(input);
        let elves = Elf::sort_elves_by_total_calories(elves);
        let mut sum: Self::PartTwo = 0;
        for i in 0..3 {
            sum += elves[i].total_calories;
        }
        sum
    }
}

fn main() {
    solve_main::<Day1>();
}
