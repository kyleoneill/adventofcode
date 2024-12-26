use problem::{solve_main, Problem};

fn parse_input(input: &Vec<String>) -> Vec<usize> {
    let mut ret: Vec<usize> = input.iter().map(|n| n.parse::<usize>().unwrap()).collect();
    ret.sort();
    ret.reverse();
    ret
}

// we can think of `containers` as a sub_array trying to get to desired_size, and break this down into
// a smaller subarray with a smaller size
// [20, 15, 10, 5, 5] and container size of 25, after taking out the 20, can be broken down into
// [15, 10, 5, 5] and container size 5
fn get_container_options(containers: &[usize], desired_size: usize) -> Vec<Vec<usize>> {
    let mut combinations: Vec<Vec<usize>> = Vec::new();
    for i in 0..containers.len() {
        let n = containers[i];
        if n > desired_size {
            continue;
        }
        else if n == desired_size {
            combinations.push(vec![n]);
        }
        else {
            if (i + 1) >= containers.len() {
                continue;
            }
            let sub_solution = get_container_options(&containers[i + 1..], desired_size - n);
            let mut sub_solution_new = Vec::new();
            for mut thing in sub_solution {
                thing.push(n);
                sub_solution_new.push(thing);
            }
            combinations.append(&mut sub_solution_new);
        }
    }
    combinations
}

struct Day17;

impl Problem for Day17 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let containers = parse_input(input);
        let liters: usize = 150;
        let combinations = get_container_options(&containers, liters);
        combinations.len()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let containers = parse_input(input);
        let liters: usize = 150;
        let combinations = get_container_options(&containers, liters);
        let shortest_combo = combinations.iter().map(|x| x.len()).min().unwrap();
        let shortest_combinations: Vec<Vec<usize>> = combinations.into_iter().filter(|x| x.len() == shortest_combo).collect();
        shortest_combinations.len()
    }
}

fn main() {
    solve_main::<Day17>();
}
