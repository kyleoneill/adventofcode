use problem::{Problem, solve};
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point

}

impl Line {
    fn new(start_x: i32, start_y: i32, finish_x: i32, finish_y: i32) -> Self {
        Line { start: Point { x: start_x, y: start_y }, end: Point { x: finish_x, y: finish_y }}
    }
    fn is_straight(&self) -> bool {
        if self.start.x == self.end.x || self.start.y == self.end.y {
            return true
        }
        false
    }
    fn get_points_line_covers(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        let move_x = (self.end.x - self.start.x).signum();
        let move_y = (self.end.y - self.start.y).signum();
        let mut current_point = self.start.clone();
        points.push(current_point.clone());
        while current_point != self.end {
            current_point.x += move_x;
            current_point.y += move_y;
            points.push(current_point.clone());
        }
        points
    }
}

impl FromStr for Line {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let start = split.next().unwrap();
        split.next().unwrap();
        let end = split.next().unwrap();
        let mut start_split = start.split(',');
        let mut end_split = end.split(',');
        let start_x = start_split.next().unwrap().parse::<i32>().unwrap();
        let start_y = start_split.next().unwrap().parse::<i32>().unwrap();
        let end_x = end_split.next().unwrap().parse::<i32>().unwrap();
        let end_y = end_split.next().unwrap().parse::<i32>().unwrap();
        Ok(Line::new(start_x, start_y, end_x, end_y))
    }
}

fn solve_1(input: &Vec<Line>) -> Option<i32> {
    let mut vent_map: HashMap<Point, i32> = HashMap::new();
    for line in input {
        if line.is_straight() {
            let points = line.get_points_line_covers();
            for point in points {
                let entry = vent_map.entry(point).or_insert(0);
                *entry += 1;
            }
        }
    }
    let count_of_dangerous_points = vent_map.values().filter(|&x| *x >= 2).count();
    Some(count_of_dangerous_points as i32)
}

fn solve_2(input: &Vec<Line>) -> Option<i32> {
    let mut vent_map: HashMap<Point, i32> = HashMap::new();
    for line in input {
        let points = line.get_points_line_covers();
        for point in points {
            let entry = vent_map.entry(point).or_insert(0);
            *entry += 1;
        }
    }
    let count_of_dangerous_points = vent_map.values().filter(|&x| *x >= 2).count();
    Some(count_of_dangerous_points as i32)
}

#[derive(Debug)]
enum Error {
    NoSolution
}

struct Day5;
impl Problem for Day5 {
    type Input = Vec<Line>;
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
    solve::<Day5>("input").unwrap();
}
