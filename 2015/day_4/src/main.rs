use md5::Digest;
use problem::{solve_main, Problem};

fn compute_digest(prefix: &str, num: u32) -> Digest {
    let mut str_to_test = prefix.to_owned();
    str_to_test.push_str(&num.to_string());
    md5::compute(str_to_test)
}

struct Day4;

impl Problem for Day4 {
    type Input = Vec<String>;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut num = 0;
        loop {
            let digest = compute_digest(input[0].as_str(), num);

            // // This is easier to read but slightly slower than using bitmasking
            // let text_representation = format!("{:x}", digest);
            // if text_representation.starts_with("00000") {
            //     return num;
            // }

            if digest.0[0] & 0xFF == 0x00 && digest.0[1] & 0xFF == 0x00 && digest.0[2] & 0xF0 == 0x00 {
                return num;
            }

            num += 1;
        }
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let prefix = input[0].as_str();
        let mut num = 0;

        // Get the digest of our starting point
        let first_digest = compute_digest(prefix, num);

        // Find when the first byte of the digest changes so we know we are at the beginning of a range
        loop {
            num += 1;
            let digest = compute_digest(prefix, num);
            if digest.0[0] != first_digest.0[0] {
                break
            }
        }

        // Find when the first byte of the digest changes again so we know how large a range is
        let beginning_of_range = num;
        let second_digest = compute_digest(prefix, num);
        loop {
            num += 1;
            let digest = compute_digest(prefix, num);
            if digest.0[0] != second_digest.0[0] {
                break
            }
        }

        let range = num - beginning_of_range;

        // We are now at the beginning of a range
        loop {
            let digest = compute_digest(prefix, num);

            // If the digest does not begin with 0x00 then skip this range
            if digest.0[0] & 0xFF != 0x00 {
                num += range;
                continue;
            }

            // Change final one to 0xFF for part 2
            if digest.0[0] & 0xFF == 0x00 && digest.0[1] & 0xFF == 0x00 && digest.0[2] & 0xFF == 0x00 {
                return num;
            }

            num += 1;
        }
    }
}

fn main() {
    solve_main::<Day4>();
}
