use problem::{solve_main, Problem};
use std::str::FromStr;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

// Card in puzzle is a char but it's more convenient for comparison to store them as a u32
// 2=2, 3=3, etc until T=10, J=11, Q=12, K=13, A=14
#[derive(Debug)]
struct Hand {
    hand: [u32; 5],
    bid: u32,
    hand_type: HandType,
    version: u32
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.hand == other.hand
    }
}
impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let initial = self.hand_type.cmp(&other.hand_type);
        if initial != Ordering::Equal {
            return initial;
        }
        for (i, _) in self.hand.iter().enumerate() {
            if self.hand[i] == other.hand[i] {
                continue;
            }
            else {
                if self.hand[i] > other.hand[i] {
                    return Ordering::Greater;
                }
                return Ordering::Less;
            }
        }
        panic!("Reached undefined area when comparing two hands")
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn c_from_str(s: &str, version: u32) -> Result<Self, ParseHandError> {
        let split: Vec<&str> = s.split(" ").collect();
        let mut hand: [u32; 5] = [0,0,0,0,0];
        for (i, char) in split[0].chars().enumerate() {
            hand[i] = match char {
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'T' => 10,
                'J' => {
                    match version {
                        1 => 11,
                        _ => 1
                    }
                },
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => return Err(ParseHandError)
            };
        }
        let bid = split[1].parse::<u32>().expect("Failed to read bid from input line");
        let hand_type = Self::get_hand_type(&hand, version);
        Ok(Self { hand, bid, hand_type, version })
    }

    fn get_hand_type(hand: &[u32; 5], version: u32) -> HandType {
        // We need our input array to be sorted
        let mut copy = hand.clone();
        copy.sort();

        // After sorting, store the number of like-cards in a vec
        let mut current_char = copy[0];
        let mut running_tally = 1;
        let mut kind_tallies: Vec<u32> = Vec::new();
        for i in 1..5 {
            if copy[i] == current_char {
                running_tally += 1;
                if i == 4 {
                    kind_tallies.push(running_tally);
                }
            }
            else {
                current_char = copy[i];
                kind_tallies.push(running_tally);
                running_tally = 1;
            }
        }

        // Check the like-cards in our vec to determine what kind of hand we have
        if kind_tallies.contains(&5) {
            return HandType::FiveOfKind
        }
        else if kind_tallies.contains(&4) {
            return HandType::FourOfKind
        }
        else if kind_tallies.contains(&3) && kind_tallies.contains(&2) {
            return HandType::FullHouse
        }
        else if kind_tallies.contains(&3) {
            return HandType::ThreeOfKind
        }
        else if kind_tallies.contains(&2) && kind_tallies.iter().filter(|&n| *n == 2).count() == 2 {
            return HandType::TwoPair
        }
        else if kind_tallies.contains(&2) {
            return HandType::OnePair
        }
        else {
            return HandType::HighCard
        }
    }
    pub fn new(input: &Vec<String>, version: u32) -> Vec<Self> {
        let mut res = Vec::new();
        for line in input {
            let hand = Hand::c_from_str(line, version).expect("Failed to read a line from input");
            res.push(hand);
        }
        res
    }
}

struct Day7;

impl Problem for Day7 {
    type Input = Vec<String>;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        // Get a vec of hands
        let mut hands = Hand::new(input, 1);
        // Re-arrange our vec to follow their rank
        hands.sort();
        // add up the result of each hands winnings
        let mut sum = 0;
        for (i, hand) in hands.iter().enumerate() {
            sum += hand.bid * (i as u32 + 1u32);
        }
        sum
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        // // Get a vec of hands
        // let mut hands = Hand::new(input, 2);
        // // Re-arrange our vec to follow their rank
        // hands.sort();
        // // add up the result of each hands winnings
        // let mut sum = 0;
        // for (i, hand) in hands.iter().enumerate() {
        //     sum += hand.bid * (i as u32 + 1u32);
        // }
        // sum
        0
    }
}

fn main() {
    solve_main::<Day7>();
}
