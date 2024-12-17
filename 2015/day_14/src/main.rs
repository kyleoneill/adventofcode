use std::{collections::HashMap, str::FromStr};

use problem::{solve_main, Problem};

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: usize,
    flying_time: usize,
    resting_time: usize,
}

#[derive(Debug)]
struct ParseReindeerErr;
impl FromStr for Reindeer {
    type Err = ParseReindeerErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split_whitespace().collect();
        let name = split[0].to_owned();
        let speed = split[3].parse::<usize>().expect("Failed to parse token to speed usize");
        let flying_time = split[6].parse::<usize>().expect("Failed to parse token to flying_time usize");
        let resting_time = split[13].parse::<usize>().expect("Failed to parse token to resting_time usize");
        Ok(Self { name, speed, flying_time, resting_time })
    }
}

impl Reindeer {
    fn from_input(input: &Vec<String>) -> Vec<Self> {
        let mut reindeers: Vec<Self> = Vec::new();
        for line in input {
            reindeers.push(line.parse::<Reindeer>().expect("Failed to convert line to reindeer"));
        }
        reindeers
    }

    fn get_winning_distance(reindeer: &Vec<Self>, seconds: usize) -> usize {
        let mut leading_reindeer: (String, usize) = ("".to_owned(), 0);
        for contestant in reindeer {
            // A period is the total time of run_time + rest_time
            // Get the number of periods run, plus the extra from an un-finished period
            let period = contestant.flying_time + contestant.resting_time;
            let periods_run = (seconds as f32 / period as f32).floor() as usize;
            let extra_time = seconds % period;

            // Get the total number of seconds spend flying from each period and
            // from the remainder period
            let flying_seconds = periods_run * contestant.flying_time;
            let extra_time_flying_seconds = extra_time.min(contestant.flying_time);

            // Get the total distance for a reindeer, speed * total_time_flying
            let total_distance = (flying_seconds + extra_time_flying_seconds) * contestant.speed;
            if total_distance > leading_reindeer.1 {
                leading_reindeer = (contestant.name.clone(), total_distance);
            }
        }
        leading_reindeer.1
    }

    fn get_points_new_scoring_system(reindeer: &Vec<Self>, seconds: usize) -> usize {
        // This is inefficient and it would be nice to make this better
        let mut distances: HashMap<String, usize> = HashMap::new();
        let mut scores: HashMap<String, usize> = HashMap::new();
        for contestant in reindeer {
            distances.insert(contestant.name.clone(), 0);
            scores.insert(contestant.name.clone(), 0);
        }
        for n in 0..seconds {
            let mut max_distance: usize = 0;

            // Simulate distances traveled for this second
            for contestant in reindeer {
                let is_flying_second = {
                    let seconds_into_period = n % (contestant.flying_time + contestant.resting_time);
                    seconds_into_period < contestant.flying_time
                };
                if is_flying_second {
                    let val = distances.get_mut(contestant.name.as_str()).unwrap();
                    *val += contestant.speed;
                }
                let distance = *distances.get(contestant.name.as_str()).unwrap();
                if distance > max_distance {
                    max_distance = distance;
                }
            }

            // Find out who is in the lead for this second, there can be multiple if there is a tie
            let mut leaders: Vec<String> = Vec::new();
            for contestant in distances.keys() {
                if *distances.get(contestant.as_str()).unwrap() == max_distance {
                    leaders.push(contestant.to_owned());
                }
            }

            // Award 1 point to all the reindeer in first place
            for leader in leaders {
                let score = scores.get_mut(leader.as_str()).unwrap();
                *score += 1;
            }
        }
        *scores.values().max().unwrap()
    }
}

struct Day14;

impl Problem for Day14 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let reindeer = Reindeer::from_input(input);
        Reindeer::get_winning_distance(&reindeer, 2503)
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let reindeer = Reindeer::from_input(input);
        Reindeer::get_points_new_scoring_system(&reindeer, 2503)
    }
}

fn main() {
    solve_main::<Day14>();
}
