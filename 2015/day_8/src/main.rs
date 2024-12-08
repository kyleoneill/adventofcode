use problem::{solve_main, Problem};

fn count_string_size(input: &str) -> (usize, usize) {
    let mut characters_of_code: usize = 0;
    let mut string_chars: usize = 0;

    let mut inside_string: bool = false;

    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        characters_of_code += 1;
        // Opening quotation
        if c == '"' && !inside_string {
            inside_string = true;
        }
        // Closing quotation
        else if c == '"' && inside_string {
            break;
        }
        // Escape
        else if c == '\\' {
            // All of the escaped sequences here represent one string character, so increment that counter
            string_chars += 1;
            // Since we're doing a chars.next(), have to also increment chars of code
            characters_of_code += 1;
            match chars.next().expect("Failed to read a char after reaching an escape backslash") {
                '\\' => (),
                '"' => (),
                'x' => {
                    // Increment chars of code once per chars.next()
                    characters_of_code += 2;
                    let _first_hex_digit = chars.next().expect("Failed to get first digit of hex character");
                    let _second_hex_digit = chars.next().expect("Failed to get second digit of hex character");
                },
                _ => panic!("Got an unhandled char after an escape backslash")
            }
        }
        else {
            // Normal character
            string_chars += 1;
        }
    }
    (characters_of_code, string_chars)
}

fn count_encoded_characters(input: &str) -> usize {
    // Encoded chars begins at 2 to account for the invisible beginning and ending quotation marks
    let mut encoded_characters: usize = 2;
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        encoded_characters += 1;
        if c == '"' || c == '\\' {
            // Increment to account for the escape needed
            encoded_characters += 1;
        }
    }
    encoded_characters
}

struct Day8;

impl Problem for Day8 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut total_char_of_code: usize = 0;
        let mut total_string_chars: usize = 0;
        for line in input {
            let (code_for_line, chars_for_line) = count_string_size(line.as_str());
            total_char_of_code += code_for_line;
            total_string_chars += chars_for_line;
        }
        total_char_of_code.checked_sub(total_string_chars).expect("String chars was greater than the total chars in code")
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut total_char_of_code: usize = 0;
        let mut total_encoded_characters: usize = 0;
        for line in input {
            // Just re-use the function for part 1 to get the first half of this problem
            // O(2n) == O(n) so it is fine :^)
            let (code_for_line, _) = count_string_size(line.as_str());
            let encoded_characters = count_encoded_characters(line.as_str());
            total_char_of_code += code_for_line;
            total_encoded_characters += encoded_characters;
        }
        total_encoded_characters.checked_sub(total_char_of_code).expect("Original sum of code characters was greater than the sum of encoded characters")
    }
}

fn main() {
    solve_main::<Day8>();
}
