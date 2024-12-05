use problem::{solve_main, Problem};

const ILLEGAL_SUBSTRINGS: [&'static str; 4] = ["ab", "cd", "pq", "xy"];
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn is_string_nice(text: &str) -> bool {
    let mut vowel_count: usize = 0;
    let mut two_in_a_row: bool = false;

    // This is a hack and only works because '!' is not part of a valid string.
    // Using this instead of making this var an Option<char> since it will only be None for the first iteration
    // and we don't need to waste compute on every subsequent iteration checking if it's None or Some
    let mut last_char = '!';

    for current_char in text.chars() {
        if VOWELS.contains(&current_char) {
            vowel_count += 1;
        }
        let mut current_substring = String::new();
        current_substring.push(last_char);
        current_substring.push(current_char);

        if ILLEGAL_SUBSTRINGS.contains(&current_substring.as_str()) {
            return false
        }

        if last_char == current_char {
            two_in_a_row = true;
        }

        last_char = current_char;
    }
    vowel_count >= 3 && two_in_a_row
}

fn is_string_nice_two(text: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();
    let mut has_two_pairs: bool = false;
    let mut has_separated_repeat: bool = false;

    if chars.len() < 2 {
        return false;
    }

    let mut i: usize = 1;
    'outer: loop {
        if chars.len() < i + 3 {
            break;
        }
        for j in (i + 1)..chars.len() - 1 {
            if chars[i - 1] == chars[j] && chars[i] == chars[j + 1] {
                has_two_pairs = true;
                break 'outer;
            }
        }
        i += 1;
    }

    // Doing this separately instead of in the previous loop is fine since o(2n) == o(n)
    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 2] {
            has_separated_repeat = true;
            break;
        }
    }

    has_two_pairs && has_separated_repeat
}

struct Day5;

impl Problem for Day5 {
    type Input = Vec<String>;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut nice_count: Self::PartOne = 0;
        for text in input {
            if is_string_nice(text.as_str()) {
                nice_count += 1;
            }
        }
        nice_count
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut nice_count: Self::PartOne = 0;
        for text in input {
            if is_string_nice_two(text.as_str()) {
                nice_count += 1;
            }
        }
        nice_count
    }
}

fn main() {
    solve_main::<Day5>();
}
