use std::time::Instant;
use std::fs;
use std::io::BufRead;

fn main() {
    let start = Instant::now();
    let input = get_input("input.txt");
    let solution = solve(input);
    println!("Found solution in {} microseconds", start.elapsed().as_micros());
    println!("The solution is {}", solution);
}

fn solve(mut input: Vec<Vec<char>>) -> i32 {
    let occupied_seats;
    loop {
        let res = shuffle_seats(&mut input);
        if !res.0 {
            occupied_seats = res.1;
            break;
        }
    }
    occupied_seats
}

fn shuffle_seats(ferry_seats: &mut Vec<Vec<char>>) -> (bool, i32) {
    let mut seat_has_changed = false;
    let mut occupied_seats = 0;
    let round_start = ferry_seats.clone();
    for i in 0..ferry_seats.len() {
        for j in 0..ferry_seats[i].len() {
            let seat = round_start[i][j];
            //nested match halves the program run time, don't check adjacent seats for floor space
            match seat {
                '#' | 'L' => {
                    let adjacent_seats = count_adjacent_seats_part_two(&round_start, j, i);
                    match seat {
                        '#' => {
                            occupied_seats += 1;
                            if adjacent_seats >= 5 {
                                ferry_seats[i][j] = 'L';
                                seat_has_changed = true;
                            }
                        },
                        'L' => {
                            if adjacent_seats == 0 {
                                ferry_seats[i][j] = '#';
                                seat_has_changed = true;
                            }
                        },
                        _ => panic!("This can't be reached")
                    }
                },
                '.' => {
                    continue
                },
                _ => panic!("Undefined ferry seat")
            }
        }
    }
    (seat_has_changed, occupied_seats)
}

fn count_adjacent_seats_part_two(ferry_seats: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut counter = 0;
    let length = ferry_seats[0].len();
    let height = ferry_seats.len();
    for x_pos in (0..x).rev() {
        match ferry_seats[y][x_pos] {
            '.' => {
                continue;
            },
            'L' => {
                break;
            },
            '#' => {
                counter += 1;
                break;
            },
            _ => {
                panic!("Undefined");
            }
        }
    }
    for x_pos in x..length {
        match ferry_seats[y][x_pos] {
            '.' => {
                continue;
            },
            'L' => {
                break;
            },
            '#' => {
                counter += 1;
                break;
            },
            _ => {
                panic!("Undefined");
            }
        }
    }
    counter
}

#[allow(dead_code)]
fn count_adjacent_seats_part_one(ferry_seats: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut counter = 0;
    for top in -1..=1 {
        for side in -1..=1 {
            if x as i32 + side >= 0 && 
               x as i32 + side < ferry_seats[0].len() as i32 &&
               y as i32 + top >= 0 &&
               y as i32 + top < ferry_seats.len() as i32 {
                if top != 0 || side != 0 {
                    let neighbor = &ferry_seats[(y as i32 + top) as usize][(x as i32 + side) as usize];
                    if neighbor == &'#' {
                        counter +=1;
                    }
                }
            }
        }
    }
    counter
}

fn get_input(filename: &str) -> Vec<Vec<char>> {
    let mut contents: Vec<Vec<char>> = Vec::new();
    let file = fs::File::open(filename).expect("Failed to open input file");
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(line) = line {
            contents.push(line.chars().collect());
        }
    }
    contents
}