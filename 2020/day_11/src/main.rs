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
                    let adjacent_seats = count_adjacent_seats_part_two(&round_start, j as isize, i as isize);
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

fn count_direction(ferry_seats: &Vec<Vec<char>>, x: isize, y: isize, dx: isize, dy: isize) -> usize {
    let length = ferry_seats[0].len() as isize;
    let height = ferry_seats.len() as isize;
    let mut distance = 1;
    loop {
        let sx = x + distance * dx;
        let sy = y + distance * dy;
        if (sx >= 0 && sy >= 0) && (sx < length && sy < height) {
            match ferry_seats[sy as usize][sx as usize] {
                '#' => {
                    return 1
                },
                'L' => return 0,
                '.' => (),
                _ => panic!("Undefined location")
            }
            distance += 1;
        }
        else {
            return 0;
        }
    }
}

fn count_adjacent_seats_part_two(ferry_seats: &Vec<Vec<char>>, x: isize, y: isize) -> i32 {
    let mut counter = 0;
    const DIRECTIONS: [(isize, isize); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];
    for &(dx, dy) in DIRECTIONS.iter() {
        counter += count_direction(ferry_seats, x, y, dx, dy)
    }
    counter as i32
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