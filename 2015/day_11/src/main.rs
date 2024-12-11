use std::collections::HashSet;

use problem::{solve_main, Problem};

const BANNED_LIST: [u8; 3] = [105, 108, 111];

struct Password {
    data: [u8; 8],
}

impl ToString for Password {
    fn to_string(&self) -> String {
        self.data.iter().map(|n| *n as char).collect()
    }
}

impl Password {
    fn from_input(input: &str) -> Self {
        assert!(input.len() == 8, "Only passwords with 8 characters are valid");
        let mut map = input.chars().map(|c| c as u8);
        let mut data: [u8; 8] = [0; 8];
        for i in 0..8 {
            data[i] = map.next().expect("Failed to read char of password");
        }
        Password { data }
    }

    fn is_valid(&self) -> bool {
        // Passwords must contain 3 ascending letters in a row (a, b, c; x, y, z)
        let mut passed_first_rule = false;
        for i in 0..self.data.len() - 2 {
            if self.data[i] + 1 == self.data[i + 1] && self.data[i + 1] + 1 == self.data[i + 2] {
                passed_first_rule = true;
                break;
            }
        }
        if !passed_first_rule {
            return false
        }

        // Passwords cannot contain 'i', 'o', or 'l'
        for i in 0..self.data.len() {
            if BANNED_LIST.contains(&self.data[i]) {
                return false
            }
        }

        // Passwords must contain two different pairs of letters, like 'aa' and 'zz'
        let mut two_in_a_rows: HashSet<u8> = HashSet::new();
        for i in 0..self.data.len() - 1 {
            if self.data[i] == self.data[i + 1] {
                two_in_a_rows.insert(self.data[i]);
            }
        }
        two_in_a_rows.len() > 1
    }

    fn increment(&mut self, index: usize) {
        let new_value = self.data[index] + 1;
        if new_value == 123 {
            // 122 is the ASCII code for 'z', so we need to wrap back around to 'a'
            self.data[index] = 97;

            // We need to move the current index one digit to the left
            // Unless we are already at index 0, where we wrap back around
            let index = match index.checked_sub(1) {
                Some(i) => i,
                None => {
                    // I think this is an invalid overflow and should panic,
                    // but I also don't think it occurs in the problem space
                    self.data.len() - 1
                }
            };

            // Need to repeat this increment for the next digit, since we have overflowed in incrementing
            // the current digit
            self.increment(index);
        }
        else {
            self.data[index] = new_value;
        }
    }

    fn generate_new(&mut self) {
        let increment_index = self.data.len() - 1;
        loop {
            self.increment(increment_index);
            if self.is_valid() {
                return
            }
        }
    }
}

struct Day11;

impl Problem for Day11 {
    type Input = Vec<String>;
    type PartOne = String;
    type PartTwo = String;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut password = Password::from_input(input[0].as_str());
        password.generate_new();
        password.to_string()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut password = Password::from_input(input[0].as_str());
        password.generate_new();
        password.generate_new();
        password.to_string()
    }
}

fn main() {
    solve_main::<Day11>();
}
