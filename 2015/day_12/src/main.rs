use serde_json::Value;

use problem::{solve_main, Problem};

fn walk_object(obj: &Value) -> i64 {
    match obj {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(number) => number.as_i64().expect("Failed to convert a number to an i64"),
        Value::String(_) => 0,
        Value::Array(vec) => {
            let mut sum: i64 = 0;
            for thing in vec {
                sum += walk_object(thing);
            }
            sum
        },
        Value::Object(map) => {
            if map.contains_key("red") {
                return 0;
            }
            let mut sum: i64 = 0;
            for thing in map.values() {
                if let Value::String(thing_string) = thing {
                    if thing_string == "red" {
                        return 0;
                    }
                }
                sum += walk_object(thing);
            }
            sum
        },
    }
}

struct Day12;

impl Problem for Day12 {
    type Input = Vec<String>;
    type PartOne = isize;
    type PartTwo = isize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        // Since we just want to sum all numbers in the doc, iterate every single char in the string, build numbers, and sum them.
        // This is a lot faster than serializing the string into a JSON object and then walking through the object. On my machine,
        // the part 1 solution runs an order of magnitude faster than part 2 where the JSON is serialized and walked
        let document = input[0].clone();

        let mut sum: isize = 0;
        let mut built_number = String::new();
        let mut prev_char: char = '0';
        for c in document.chars() {
            if c == '-' {
                // We have a '-'. Flush the built_number buffer and mark that we just saw a negative sign
                if built_number.len() != 0 {
                    let num = built_number.parse::<isize>().expect(&format!("Failed to convert '{}' to an isize", built_number));
                    sum += num;
                    built_number = String::new();
                }
                prev_char = '-';
                continue;
            }
            let char_code = c as u8;
            if char_code >= 48 && char_code <= 57 {
                // The char is within the ASCII range of '0' to '9', add it to the buffer
                if prev_char == '-' {
                    // The char right before this was a negative sign, so append that before the digit
                    built_number.push('-');
                    prev_char = c;
                }
                built_number.push(c);
                continue;
            }
            if built_number.len() != 0 {
                // The char is not a number or negative sign, so convert the buffer to a number and then reset it
                let num = built_number.parse::<isize>().expect(&format!("Failed to convert '{}' to an isize", built_number));
                sum += num;
                built_number = String::new();
            }
            prev_char = c;
        }
        sum
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        // Could probably re-use the part 1 solution here so this can run faster, but I am being lazy and just serializing to a JSON obj. Would
        // have to keep track of what layer of an obj we are in by counting '{' and '}' chars and flipping some "ignore" flag if an object had "red" as a key
        let document = input[0].clone();
        let value: Value = serde_json::from_str(&document).expect("Failed to serialize input str into JSON object");
        walk_object(&value) as isize
    }
}

fn main() {
    solve_main::<Day12>();
}
