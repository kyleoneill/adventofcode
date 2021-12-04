use std::io::BufRead;
use problem::{Problem, ProblemInput, solve};
use std::str::FromStr;

#[cfg(windows)]
const DOUBLE_LINE_ENDING: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const DOUBLE_LINE_ENDING: &'static str = "\n\n";

type BingoBoard = [[i32; 5]; 5];

struct BingoGame {
    draw_numbers: Vec<i32>,
    boards: Vec<BingoBoard>
}

impl ProblemInput for BingoGame {
    type Error = Error;
    fn parse<R: BufRead>(mut reader: R) -> Result<Self, Self::Error> {
        let mut some_str = String::new();
        reader.read_to_string(&mut some_str).unwrap();
        Ok(BingoGame::from_str(&some_str).unwrap())
    }
}

impl FromStr for BingoGame {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut draw_numbers = Vec::<i32>::new();
        let mut boards = Vec::<BingoBoard>::new();
        let mut split = s.split(DOUBLE_LINE_ENDING);
        let numbers = split.next().unwrap().split(',');
        for n in numbers {
            draw_numbers.push(n.parse().unwrap());
        }
        for board in split {
            let mut new_board: BingoBoard = [[0; 5]; 5];
            let rows = board.lines();
            for (index, row) in rows.enumerate() {
                let spaces = row.split(' ').filter(|s| !s.is_empty());
                for (n, space) in spaces.enumerate() {
                    new_board[index][n] = space.parse().unwrap();
                }
            }
            boards.push(new_board);
        }
        Ok(BingoGame { draw_numbers, boards })
    }
}

/// Take a board and determine the earliest turn it wins.
/// This function first finds the earliest turn the board wins using only rows, and then
/// the earliest turn it wins using only columns. The min of those two values is then returned.
/// A row/column win is determined by getting the maximum draw_numbers index for the numbers in
/// a row/column sequence, as all numbers will have been seen in the draw by the max index for
/// the sequence.
///
/// The index of a number in draw_numbers is equal to the turn that number is pulled. If
/// a draw_numbers looks like [0, 5, 7, 9] then 7 is at index 2 and drawn on turn 2 (if turns
/// were zero indexed). We can use the index of a number within draw_numbers and the turn
/// interchangeably.
fn find_turn_board_wins(board: &BingoBoard, draw_numbers: &Vec<i32>) -> i32 {
    let mut row_minimum = i32::MAX;
    let mut column_minimum = i32::MAX;

    for row in 0..5 {
        let mut greatest_num_in_row = 0;
        for column in 0..5 {
            let index = draw_numbers.iter().position(|&num| num == board[row][column] ).unwrap() as i32;
            if index > greatest_num_in_row { greatest_num_in_row = index };
        }
        if greatest_num_in_row < row_minimum { row_minimum = greatest_num_in_row }
    }

    for column in 0..5 {
        let mut greatest_num_in_column = 0;
        for row in 0..5 {
            let index = draw_numbers.iter().position(|&num| num == board[row][column] ).unwrap() as i32;
            if index > greatest_num_in_column { greatest_num_in_column = index };
        }
        if greatest_num_in_column < column_minimum { column_minimum = greatest_num_in_column }
    }
    std::cmp::min(row_minimum, column_minimum)
}

/// Score a board by iterating through each number it contains and adding that number to a sum
/// if the number has not been drawn. A number has not yet been drawn if its draw turn is
/// greater than the winning turn.
fn score_board(board: &BingoBoard, draw_numbers: &Vec<i32>, winning_turn: i32) -> i32 {
    // See find_turn_board_wins function for an explanation of draw_numbers
    let mut sum = 0;
    for row in 0..5 {
        for column in 0..5 {
            let turn_called = draw_numbers.iter().position(|&num| num == board[row][column] ).unwrap() as i32;
            if turn_called > winning_turn {
                sum += board[row][column];
            }
        }
    }
    sum * draw_numbers[winning_turn as usize]
}

fn solve_1(input: &BingoGame) -> Option<i32> {
    let winning_board = input.boards.iter().min_by_key(|&board| find_turn_board_wins(board, &input.draw_numbers)).unwrap();
    let turn_it_wins = find_turn_board_wins(winning_board, &input.draw_numbers);
    Some(score_board(winning_board, &input.draw_numbers, turn_it_wins))
}

fn solve_2(input: &BingoGame) -> Option<i32> {
    let worst_board = input.boards.iter().max_by_key(|&board| find_turn_board_wins(board, &input.draw_numbers)).unwrap();
    let turn_it_wins = find_turn_board_wins(worst_board, &input.draw_numbers);
    Some(score_board(worst_board, &input.draw_numbers, turn_it_wins))
}

#[derive(Debug)]
enum Error {
    NoSolution
}

struct Day4;
impl Problem for Day4 {
    type Input = BingoGame;
    type Part1Output = i32;
    type Part2Output = i32;
    type Error = Error;

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        let result = solve_1(input).ok_or(Error::NoSolution)?;
        Ok(result)
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        let result = solve_2(input).ok_or(Error::NoSolution)?;
        Ok(result)
    }
}

fn main() {
    solve::<Day4>("input").unwrap();
}
