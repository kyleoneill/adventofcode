use std::collections::HashMap;

use problem::{solve_main, Problem};

#[derive(Debug, Clone)]
enum Spring {
    Unknown,
    Operational,
    Broken
}

impl Spring {
    pub fn from_char(c: char) -> Self {
        match c {
            '?' => Self::Unknown,
            '.' => Self::Operational,
            '#' => Self::Broken,
            _ => panic!("Got unsupported spring char")
        }
    }
}

#[derive(Debug)]
struct Record {
    springs: Vec<Spring>,
    checksum: Vec<usize>
}

impl Record {
    fn from_input(input: &Vec<String>, version: usize) -> Vec<Self> {
        let mut res: Vec<Self> = Vec::new();
        for line in input {
            let split: Vec<&str> = line.split(" ").collect();
            let springs: Vec<Spring> = split[0].chars().map(Spring::from_char).collect();
            let checksum: Vec<usize> = split[1].split(",").map(|i| i.parse::<usize>().expect("Failed to convert str to int when reading input")).collect();
            if version != 1 {
                let mut unfolded_springs: Vec<Spring> = springs.clone();
                for _ in 0..4 {
                    unfolded_springs.push(Spring::Unknown);
                    unfolded_springs.extend(springs.iter().cloned());
                }
                res.push(Self { springs: unfolded_springs, checksum: checksum.repeat(5) })
            }
            else {
                res.push(Self { springs, checksum });
            }
        }
        res
    }

    fn all_solutions(&self) -> usize {
        let mut cache: HashMap<(usize, usize, usize), usize> = HashMap::new();
        self.solutions(0, 0, 0, &mut cache)
    }

    fn solutions(&self, spring_index: usize, checksum_index: usize, observed_broken_springs: usize, cache: &mut HashMap<(usize, usize, usize), usize>) -> usize {
        // observed_broken_springs is how many we have seen in a row
        if let Some(result) = cache.get(&(spring_index, checksum_index, observed_broken_springs)) {
            return *result;
        }
        let result = {
            if spring_index == self.springs.len() {
                // We are checking the final spring
                if checksum_index == self.checksum.len() || (checksum_index + 1 == self.checksum.len() && self.checksum[checksum_index] == observed_broken_springs) {
                    // This is a valid solution, return 1
                    1
                }
                else {
                    // This is an invalid solution, do not count it
                    0
                }
            }
            else {
                match self.springs[spring_index] {
                    // there are more springs to check, recursively solve depending on what kind of spring we're looking at
                    Spring::Operational => self.solutions_operational(spring_index, checksum_index, observed_broken_springs, cache),
                    Spring::Broken => self.solutions_broken(spring_index, checksum_index, observed_broken_springs, cache),
                    Spring::Unknown => self.solutions_operational(spring_index, checksum_index, observed_broken_springs, cache) + self.solutions_broken(spring_index, checksum_index, observed_broken_springs, cache)
                }
            }
        };
        cache.insert((spring_index, checksum_index, observed_broken_springs), result);
        result
    }

    fn solutions_operational(&self, spring_index: usize, checksum_index: usize, observed_broken_springs: usize, cache: &mut HashMap<(usize, usize, usize), usize>) -> usize {
        // We have not seen a broken spring since observing the last working spring, just move on
        if observed_broken_springs == 0 {
            self.solutions(spring_index + 1, checksum_index, 0, cache)
        }
        // The number of broken springs in a row that we have observed is invalid according to our checksum, this solution is invalid
        else if observed_broken_springs != self.checksum[checksum_index] {
            0
        }
        // The number of broken springs in a row that we have observed is valid according to our checksum, move to the next spring,
        // the next checksum value, and reset the broken springs in a row counter
        else {
            self.solutions(spring_index + 1, checksum_index + 1, 0, cache)
        }
    }
    
    fn solutions_broken(&self, spring_index: usize, checksum_index: usize, observed_broken_springs: usize, cache: &mut HashMap<(usize, usize, usize), usize>) -> usize {
        // Verify we are in the bounds of the checksum. Check that we have not observed more broken springs in a row than the checksum allows
        if checksum_index < self.checksum.len() && observed_broken_springs < self.checksum[checksum_index] {
            // If so, increment the spring counter, pass forward the current checksum, increment the number of broken springs in a row observed
            self.solutions(spring_index + 1, checksum_index, observed_broken_springs + 1, cache)
        }
        else {
            // If not, this is not a valid solution
            0
        }
    }
}

struct Day12;

impl Problem for Day12 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        Record::from_input(input, 1).iter().map(Record::all_solutions).sum()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        Record::from_input(input, 2).iter().map(Record::all_solutions).sum()
    }
}

fn main() {
    solve_main::<Day12>();
}
