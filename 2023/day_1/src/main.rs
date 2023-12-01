use problem::{solve_main, Problem};

struct Day1;

fn get_digit_num_from_line(line: &str) -> Option<u32> {
    let mut first_digit: Option<char> = None;
    let mut second_digit: Option<char> = None;
    for c in line.chars() {
        if c.is_digit(10) && first_digit.is_none() {
            first_digit = Some(c);
        }
        if c.is_digit(10) {
            second_digit = Some(c);
        }
    }
    if first_digit.is_none() || second_digit.is_none() {
        return None
    }
    Some(format!("{}{}", first_digit.unwrap(), second_digit.unwrap()).parse().unwrap())
}

fn str_to_char_digit(input: &str) -> Option<char> {
    match input {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None
    }
}

fn get_num_from_line(line: &str) -> u32 {
    let mut first_digit: Option<char> = None;
    let mut second_digit: Option<char> = None;
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            if first_digit.is_none() {
                first_digit = Some(c);
            }
            second_digit = Some(c);
        }
        else {
            // Start at current char and begin a new iteration trying to build a number
            let char_vec: Vec<char> = line.chars().collect();
            let mut building_string: String = "".to_owned();
            let mut nums_read = 0;
            for j in i..char_vec.len() {
                nums_read += 1;
                let current_char = char_vec[j];
                // We hit a digit, so we failed to build a stringified number
                if current_char.is_digit(10) {
                    break;
                }

                building_string.push(current_char);
                match str_to_char_digit(building_string.as_str()) {
                    Some(val) => {
                        // We built a number. Set first/last digit and then break this inner loop
                        if first_digit.is_none() {
                            first_digit = Some(val);
                        }
                        second_digit = Some(val);
                        break;
                    },
                    None => ()
                }

                // There are no numbers 1-10 longer than 5 digits, so we failed to build one
                if nums_read >= 5 {
                    break;
                }
            }
        }
    }
    format!("{}{}", first_digit.unwrap(), second_digit.unwrap()).parse().unwrap()
}

impl Problem for Day1 {
    type Input = Vec<String>;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut sum = 0;
        for line in input {
            match get_digit_num_from_line(line) {
                Some(val) => sum += val,
                None => ()
            }
        }
        sum
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut sum = 0;
        for line in input {
            sum += get_num_from_line(line);
        }
        sum
    }
}

fn main() {
    solve_main::<Day1>();
}
