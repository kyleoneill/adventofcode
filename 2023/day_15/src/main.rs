use problem::{solve_main, Problem};

fn hash(s: &str) -> usize {
    let mut ret: u32 = 0;
    for c in s.chars() {
        ret += c as u32;
        ret *= 17;
        ret %= 256;
    }
    ret as usize
}

#[derive(Debug)]
enum Operation {
    Remove,
    Add(usize)
}

#[derive(Debug)]
struct Instruction {
    label: String,
    operation: Operation
}

impl Instruction {
    fn fromstr(s: &str) -> Self {
        if s.contains("=") {
            let mut split = s.split("=");
            let label = split.next().unwrap().to_owned();
            let focal_length = split.next().unwrap().parse::<usize>().unwrap();
            let operation = Operation::Add(focal_length);
            Self { label, operation }
        }
        else if s.contains("-") {
            let label: String = s[..s.len() - 1].to_owned();
            let operation = Operation::Remove;
            Self { label, operation }
        }
        else {
            panic!("Got an invalid instruction")
        }
    }
}

struct Lens {
    label: String,
    focal_length: usize
}

struct Map {
    boxes: Vec<Vec<Lens>>
}

impl Map {
    fn new(size: usize) -> Self {
        let mut boxes = Vec::new();
        boxes.resize_with(size, || Vec::new());
        Self { boxes }
    }

    fn get_index(&self, label: &str) -> (usize, Option<usize>) {
        let box_to_check = hash(label);
        for (i, lens) in self.boxes[box_to_check].iter().enumerate() {
            if lens.label.as_str() == label {
                return (box_to_check, Some(i))
            }
        }
        (box_to_check, None)
    }

    fn remove(&mut self, label: &str) {
        match self.get_index(label) {
            (box_index, Some(place_in_box)) => { self.boxes[box_index].remove(place_in_box); },
            (_, None) => ()
        }
    }

    fn add(&mut self, new_lens: Lens) {
        match self.get_index(&new_lens.label) {
            (box_index, Some(place_in_box)) => self.boxes[box_index][place_in_box] = new_lens,
            (box_index, None) => self.boxes[box_index].push(new_lens)
        }
    }

    fn get_focusing_power(&self) -> usize {
        let mut power = 0;
        for (box_number, current_box) in self.boxes.iter().enumerate() {
            for (slot, lens) in current_box.iter().enumerate() {
                let first = 1 + box_number;
                let second = slot + 1;
                power += first * second * lens.focal_length;
            }
        }
        power
    }
}

struct Day15;

impl Problem for Day15 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        input[0].split(",").map(hash).sum()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let instructions: Vec<Instruction> = input[0].split(",").map(Instruction::fromstr).collect();
        let mut map = Map::new(256);
        for instruction in instructions {
            match &instruction.operation {
                Operation::Remove => map.remove(&instruction.label),
                Operation::Add(focal_length) => {
                    let new_lens = Lens { label: instruction.label.clone(), focal_length: *focal_length };
                    map.add(new_lens);
                }
            }
        }
        map.get_focusing_power()
    }
}

fn main() {
    solve_main::<Day15>();
}
