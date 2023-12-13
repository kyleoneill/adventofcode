use std::collections::HashMap;
use problem::{solve_main, Problem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Terrain {
    Ash,
    Rock
}

impl Terrain {
    fn from_char(c: &char) -> Self {
        match c {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("Got invalid char when reading terrain")
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Coord { x, y }
    }
}

#[derive(Debug)]
enum LineOfReflection {
    Vertical(usize),
    Horizontal(usize)
}

impl LineOfReflection {
    fn to_usize(self) -> usize {
        match self {
            Self::Vertical(vert_dist) => vert_dist,
            Self::Horizontal(horiz_dist) => 100 * horiz_dist
        }
    }
}

#[derive(Clone, Debug)]
struct Pattern {
    pattern: HashMap<Coord, Terrain>,
    width: usize,
    height: usize
}

impl Pattern {
    fn read_input(input: &Vec<String>) -> Vec<Self> {
        let mut res: Vec<Self> = Vec::new();

        let mut buffer = Self { pattern: HashMap::new(), width: 0, height: 0 };
        let mut y = 0;
        // This is dumb but I'm trying to solve the problem fast
        let mut width: Option<usize> = None;
        for line in input.iter() {
            // We are starting a new pattern, empty our buffer
            if line.chars().all(|c| c.is_whitespace()) {
                buffer.width = width.unwrap();
                buffer.height = y;
                res.push(buffer);
                buffer = Self { pattern: HashMap::new(), width: 0, height: 0 };
                y = 0;
                width = None;
            }
            else {
                for (x, c) in line.chars().into_iter().enumerate() {
                    buffer.pattern.insert(Coord{x, y}, Terrain::from_char(&c));
                }
                if width.is_none() {
                    width = Some(line.chars().count());
                }
                y += 1;
            }
        }
        // Loop is done, empty our buffer. This isn't DRY or clean but this is my second AoC today and it's late
        buffer.width = width.unwrap();
        buffer.height = y;
        res.push(buffer);

        res
    }

    fn column_difference(&self, first: usize, second: usize, rotate: bool) -> usize {
        let mut difference = 0;
        match rotate {
            false => {
                for y in 0..self.height {
                    // Vec access would be a lot better here but I have no idea what part 2 would bring, so inefficient and future-proof it is
                    let t_one = self.pattern.get(&Coord::new(first, y)).unwrap();
                    let t_two = self.pattern.get(&Coord::new(second, y)).unwrap();
                    if t_one != t_two {
                        difference += 1;
                    }
                    if difference >= 2 {
                        return difference
                    }
                }
                difference
            },
            true => {
                for x in 0..self.width {
                    let t_one = self.pattern.get(&Coord::new(x, first)).unwrap();
                    let t_two = self.pattern.get(&Coord::new(x, second)).unwrap();
                    if t_one != t_two {
                        difference += 1;
                    }
                    if difference >= 2 {
                        return difference
                    }
                }
                difference
            }
        }
    }

    fn check_for_reflection(&self, rotate: bool, version: usize) -> Option<usize> {
        let length = match rotate {
            true => self.height,
            false => self.width
        };
        for col_two in 1..length {
            let col_one = col_two - 1;
            let mut column_difference = self.column_difference(col_one, col_two, rotate);
            let mut distance: usize = 1;
            // Check if the current column and the previous column are equal (or only have a difference of 1 for part 2)
            while (version == 1 && column_difference == 0) || (version >= 2 && column_difference <= 1) {
                // Check if all available columns moving out from the matching columns are also equal
                // Get our potential left and right columns
                let left = col_one.checked_sub(distance);
                let right = col_two + distance;

                // If either column (or both) don't exist then we have a full reflection and break the loop
                if left.is_none() || right >= length {
                    break;
                }

                // Update our total difference in tiles for the two columns being compared
                column_difference += self.column_difference(left.unwrap(), right, rotate);

                // Increment the distance counter and continue
                distance += 1;
            }
            if (version == 1 && column_difference == 0) || (version > 1 && column_difference == 1) {
                return Some(col_one + 1)
            }
        }
        None
    }

    fn get_reflection(&self, version: usize) -> LineOfReflection {
        // Try to find a vertical reflection
        match self.check_for_reflection(false, version) {
            Some(vertical_distance) => LineOfReflection::Vertical(vertical_distance),
            None => {
                match self.check_for_reflection(true, version) {
                    Some(horizontal_distance) => LineOfReflection::Horizontal(horizontal_distance),
                    None => panic!("Did not find a vertical or horizontal reflection")
                }
            }
        }
    }
}

struct Day13;

impl Problem for Day13 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        // What happens when I call map().map().sum()?
        // The first map will generate an iterator which internally holds two things, the input iterator and a function signature.
        // Every time next() is called on this iterator, it will internally call next() on the iter it holds and pass the Some(val) to its function signature.
        // The second map call takes the first map and internally holds onto it as its own iterator and has its own mapped function signature. When
        // next() is called on the second map, it will call next() on its iter which actually causes the first iter to call next() on itself, pass that Some(val)
        // to its function signature, return that value to the second map iter, which itself will pass that value to its function signature.
        // BUT maps are _lazy_. This means that after `Pattern::read_input().iter().map().map()` is called, nothing has been iterated on.
        // When sum() is called, the second map iterator will run and internally run the first map iterator
        // The first map here takes a function that returns a value, not a reference, so the second map's function signature _must_ take a value and not a reference
        Pattern::read_input(input).iter().map(|pattern| pattern.get_reflection(1)).map(LineOfReflection::to_usize).sum()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        Pattern::read_input(input).iter().map(|pattern| pattern.get_reflection(2)).map(LineOfReflection::to_usize).sum()
    }
}

fn main() {
    solve_main::<Day13>();
}
