use problem::{solve_main, Problem};

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64
}

impl Race {
    pub fn read_input(input: &Vec<String>) -> Vec<Self> {
        let mut times: Vec<u64> = Vec::new();
        let mut distances: Vec<u64> = Vec::new();
        for time in input[0].split_whitespace().skip(1) {
            times.push(time.parse::<u64>().expect("Failed to convert time to u64"));
        }
        for distance in input[1].split_whitespace().skip(1) {
            distances.push(distance.parse::<u64>().expect("Failed to convert distance to u64"));
        }
        let mut res: Vec<Self> = Vec::new();
        for i in 0..times.len() {
            let race = Self { time: times[i], distance: distances[i] };
            res.push(race);
        }
        res
    }
    pub fn read_input_two(input: &Vec<String>) -> Self {
        let mut time: String = "".to_owned();
        let mut distance: String = "".to_owned();
        for time_piece in input[0].split_whitespace().skip(1) {
            time = format!("{}{}", time, time_piece);
        }
        for distance_piece in input[1].split_whitespace().skip(1) {
            distance = format!("{}{}", distance, distance_piece);
        }
        let num_time = time.parse::<u64>().expect("Failed to convert time to u64");
        let num_distance = distance.parse::<u64>().expect("Failed to convert distance to u64");
        Self { time: num_time, distance: num_distance }
    }
    pub fn ways_to_win(&self) -> u64 {
        // The brute force solution has a 0.4s runtime for part two which isn't great
        let mut ways_to_win = 0;
        for i in 0..=self.time {
            let time_at_speed = self.time - i;
            let distance = time_at_speed * i;
            if distance > self.distance {
                ways_to_win += 1;
            }
        }
        ways_to_win
    }
    pub fn ways_to_win_efficient(&self) -> u64 {
        // relationship: time_held == speed.    time_held(time - time_held) = distance
        // d here is actually (d + 1) because we cannot tie, we want to beat the current leader
        // h(t - h) = d
        // th - h^2 = d
        // -h^2 + th - d = 0
        // quadratic formula the above where a=-1, b=t, c=-d
        // The +/- solutions give us the max and min amount of time to hold. The distance between the two is the answer
        // The quadratic re-arranged gives us   (1/2)(t +/- sqrt(t^2 - 4d))
        // The re-arranged formula gives us fractional answers, we want to ceil the lower bound and floor the upper bound because we want
        //   integer times
        // Runtime change of 0.4s to 0.000006751s for full input of part 2
        let time = self.time as f64;
        let distance = self.distance as f64 + 1f64;
        let upper_bound: f64 = (1f64 / 2f64) * (time + (time.powi(2) - (distance * 4f64)).sqrt());
        let lower_bound: f64 = (1f64 / 2f64) * (time - (time.powi(2) - (distance * 4f64)).sqrt());
        (upper_bound.floor() - lower_bound.ceil()) as u64 + 1u64
    }
}

struct Day6;

impl Problem for Day6 {
    type Input = Vec<String>;
    type PartOne = u64;
    type PartTwo = u64;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let races = Race::read_input(&input);
        let mut sum = 0;
        for race in races {
            if sum == 0 {
                sum = race.ways_to_win();
            }
            else {
                sum *= race.ways_to_win();
            }
        }
        sum.try_into().unwrap()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let race = Race::read_input_two(input);
        race.ways_to_win_efficient()
    }
}

fn main() {
    solve_main::<Day6>();
}
