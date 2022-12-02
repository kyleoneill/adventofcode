use std::str::FromStr;
use std::string::ParseError;
use problem::{solve_main, Problem};

enum RPS {
    Rock,
    Paper,
    Scissors
}

struct Round {
    my_play: RPS,
    opponent_play: RPS
}

impl FromStr for Round {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent, me) = s.split_once(' ').unwrap();
        let opponent_play = match opponent {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => panic!("Invalid first char passed from input line")
        };
        let my_play = match me {
            "X" => RPS::Rock,
            "Y" => RPS::Paper,
            "Z" => RPS::Scissors,
            _ => panic!("Invalid second char passed from input line")
        };
        Ok(Self {
            my_play,
            opponent_play
        })
    }
}

impl Round {
    pub fn get_round_score_part_one(&self) -> u32 {
        let mut score = 0;
        match self.my_play {
            RPS::Rock => {
                score += 1;
                match self.opponent_play {
                    RPS::Rock => score += 3,
                    RPS::Paper => (),
                    RPS::Scissors => score += 6
                };
            },
            RPS::Paper => {
                score += 2;
                match self.opponent_play {
                    RPS::Rock => score += 6,
                    RPS::Paper => score += 3,
                    RPS::Scissors => ()
                };
            },
            RPS::Scissors => {
                score += 3;
                match self.opponent_play {
                    RPS::Rock => (),
                    RPS::Paper => score += 6,
                    RPS::Scissors => score += 3
                };
            }
        }
        score
    }

    pub fn get_round_score_part_two(&self) -> u32 {
        let mut score = 0;
        match self.opponent_play {
            // The naming schema breaks down here, but such is life when the input
            // is changed halfway through
            // my_play.Rock = lose
            // my_play.Paper = draw
            // my_play.Scissor = Win
            RPS::Rock => {
                match self.my_play {
                    RPS::Rock => score += 3,
                    RPS::Paper => score += 4,
                    RPS::Scissors => score += 8
                };
            },
            RPS::Paper => {
                match self.my_play {
                    RPS::Rock => score += 1,
                    RPS::Paper => score += 5,
                    RPS::Scissors => score += 9
                };
            },
            RPS::Scissors => {
                match self.my_play {
                    RPS::Rock => score += 2,
                    RPS::Paper => score += 6,
                    RPS::Scissors => score += 7
                };
            }
        }
        score
    }
}

struct Day2;

impl Problem for Day2 {
    type Input = Vec<String>;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut score = 0;
        for line in input {
            let round = Round::from_str(line).unwrap();
            score += round.get_round_score_part_one();
        }
        score
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut score = 0;
        for line in input {
            let round = Round::from_str(line).unwrap();
            score += round.get_round_score_part_two();
        }
        score
    }
}

fn main() {
    solve_main::<Day2>();
}
