use std::collections::VecDeque;
use problem::{solve_main, Problem};


/*
This is a much more efficient way of solving this problem, I did not write this
fn first_disjoint_substring(input: &[u8], window: usize) -> Option<usize> {
    let mut counts = [0; 26];
    let mut unique = 0;
    for n in 0..input.len() {
        let i = (input[n] - b'a') as usize;
        if counts[i] == 0 {
            unique += 1;
        }
        counts[i] += 1;
        if n >= window {
            let o = (input[n - window] - b'a') as usize;
            if counts[o] == 1 {
                unique -= 1;
            }
            counts[o] -= 1;
        }
        if unique == window {
            return Some(n + 1);
        }
    }
    None
}
 */

fn deque_is_unique(deque: &VecDeque<char>) -> bool {
    for i in 0..(deque.len() - 1) {
        for j in (i + 1)..deque.len() {
            if deque[i] == deque[j] {
                return false;
            }
        }
    }
    true
}

struct Device {
    data_stream: String
}

impl Device {
    fn from_vec(i: &Vec<String>) -> Self {
        Self {data_stream: i[0].clone() }
    }

    fn first_marker_index(&self, distinct_length: usize) -> u32 {
        let mut deq: VecDeque<char> = VecDeque::new();
        let chars: Vec<char> = self.data_stream.chars().collect();
        for i in 0..distinct_length {
            deq.push_back(chars[i]);
        }
        if deque_is_unique(&deq) {
            return distinct_length as u32;
        }
        else {
            let mut j = distinct_length;
            for c in &chars[j..] {
                deq.pop_front();
                deq.push_back(c.clone());
                j += 1;
                if deque_is_unique(&deq) {
                    return j as u32;
                }
            }
        }
        0
    }
}

struct Day6;

impl Problem for Day6 {
    type Input = Vec<String>;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let device = Device::from_vec(input);
        device.first_marker_index(4)
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let device = Device::from_vec(input);
        device.first_marker_index(14)
    }
}

fn main() {
    solve_main::<Day6>();
}
