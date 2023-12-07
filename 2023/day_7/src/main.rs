use problem::{solve_main, Problem};
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
    hand_type: HandType
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
        let hand_type = Self::get_hand_type(&hand);
        Ok(Self { hand, bid, hand_type })
    }

    fn get_hand_type(hand: &[u32; 5]) -> HandType {
        // We need our input array to be sorted
        let mut copy = hand.clone();
        copy.sort();

        // After sorting, check for pairings. Ignore all wildcards, which have a value of 1
        let mut current_char = copy[0];
        let mut running_tally = 1;
        let mut kind_tallies: Vec<u32> = Vec::new();
        for i in 1..5 {
            if copy[i] == current_char && current_char != 1 {
                running_tally += 1;
                if i == 4 {
                    kind_tallies.push(running_tally);
                }
            }
            else {
                if current_char != 1 {
                    kind_tallies.push(running_tally);
                }
                current_char = copy[i];
                running_tally = 1;
            }
        }

        // Get our number of wildcards. If we have 5 or 4, then we must have a five of kind
        let joker_count = copy.iter().filter(|&n| *n == 1).count();
        if joker_count == 5 || joker_count == 4 {
            return HandType::FiveOfKind
        }

        let hand_type = if kind_tallies.contains(&5) {
            HandType::FiveOfKind
        }
        else if kind_tallies.contains(&4) {
            if joker_count == 1 {
                HandType::FiveOfKind
            }
            else {
                HandType::FourOfKind
            }
        }
        else if kind_tallies.contains(&3) && kind_tallies.contains(&2) {
            // A full house means we have no jokers, as all 5 cards are counted
            // and we did not count jokers, so there must be 0 jokers
            return HandType::FullHouse
        }
        else if kind_tallies.contains(&3) {
            if joker_count == 2 {
                HandType::FiveOfKind
            }
            else if joker_count == 1 {
                HandType::FourOfKind
            }
            else {
                HandType::ThreeOfKind
            }
        }
        else if kind_tallies.contains(&2) && kind_tallies.iter().filter(|&n| *n == 2).count() == 2 {
            // A two pair means 4 cards were counted, we can only have 1 joker as we did not count jokers with cards
            if joker_count == 1 {
                // We have two pairs, the joker will make this a 3 and 2 match, which is a full house
                HandType::FullHouse
            }
            else {
                HandType::TwoPair
            }
        }
        else if kind_tallies.contains(&2) {
            if joker_count == 3 {
                HandType::FiveOfKind
            }
            else if joker_count == 2 {
                HandType::FourOfKind
            }
            else if joker_count == 1 {
                // This cannot make a full house since the TwoPair arm if block did not match, the 3 non-accounted for cards here are all
                // unequal, so the wildcard must match with the OnePair to make a 3 of a kind
                HandType::ThreeOfKind
            }
            else {
                HandType::OnePair
            }
        }
        else {
            // We have no pairings, so we need to check if our jokers can make anything other than just a high card
            // joker counts of 4 or 5 are handled earlier
            if joker_count == 3 {
                // 3 jokers will wildcard with one other card to make a four pair
                HandType::FourOfKind
            }
            else if joker_count == 2 {
                // 2 jokers will wildcard with one other to make a ThreeOfKind
                // It cannot be a FullHouse or it would have fallen into the OnePair arm
                HandType::ThreeOfKind
            }
            else if joker_count == 1 {
                // Joker will match with another card to make a single pair
                HandType::OnePair
            }
            else {
                // No jokers, no pairings
                HandType::HighCard
            }
        };
        hand_type
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
        // Get a vec of hands
        let mut hands = Hand::new(input, 2);
        // Re-arrange our vec to follow their rank
        hands.sort();
        // add up the result of each hands winnings
        let mut sum = 0;
        for (i, hand) in hands.iter().enumerate() {
            sum += hand.bid * (i as u32 + 1u32);
        }
        sum
    }
}

fn main() {
    solve_main::<Day7>();
}
