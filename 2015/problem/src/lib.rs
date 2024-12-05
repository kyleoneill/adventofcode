use anyhow::{anyhow, Context, Error, Result};
use std::{
    env,
    fmt::{self, Display},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::{from_utf8, FromStr},
    time::{Duration, Instant},
    collections::HashMap
};

// TODO: This should probably be moved out of the year folder (2015) since it is shared across all AoC stuff

// TODO: I added a lot of structs and fluff impls for the Grid type, should maybe move most of this to a new file as it is cluttering this one
//       and making it difficult to read

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Coord<T = usize> {
    pub x: T,
    pub y: T
}

impl<T: FromStr> FromStr for Coord<T>
where
    T::Err: std::error::Error + Send + Sync + 'static,
{
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Coord<T>, Self::Err> {
        let mut split = s.split(",");
        let x: T = split.next().context("Did not get an X coord")?.parse::<T>().context("Failed to parse X coord")?;
        let y: T = split.next().context("Did not get a Y coord")?.parse::<T>().context("Failed to parse Y coord")?;
        Ok( Coord{ x, y } )
    }
}

impl<T> Coord<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right
}

impl Direction {
    pub fn is_clockwise_turn(self, other: Self) -> bool {
        (self == Direction::Up && other == Direction::Right) ||
        (self == Direction::Right && other == Direction::Down) ||
        (self == Direction::Down && other == Direction::Left) ||
        (self == Direction::Left && other == Direction::Up)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Neighbor {
    pub coord: Coord,
    pub direction: Direction
}

impl Neighbor {
    pub fn new(coord: Coord, direction: Direction) -> Self {
        Self { coord, direction }
    }
}

pub struct Grid<T> {
    pub map: HashMap<Coord, T>,
    pub height: usize,
    pub width: usize
}

impl<T: Display> Grid<T> {
    pub fn from_text_grid(input: &Vec<String>, f: impl Fn(char) -> T) -> Self {
        let mut map: HashMap<Coord, T> = HashMap::new();
        let height = input.len();
        assert!(height > 0, "Passed 2d text grid had a height of 0");
        let width = input[0].chars().into_iter().count();
        for (y, row) in input.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                map.insert(Coord { x, y }, f(c));
            }
        }
        Self { map, height, width }
    }
    pub fn neighbors(&self, current_coord: &Coord, only_immediate: bool) -> Vec<Option<Neighbor>> {
        let mut neighbors: Vec<Option<Neighbor>> = Vec::new();
        // If only_immediate is true, we only want the neighbors above, below, left, and right of current_coord
        match only_immediate {
            true => {
                // Check up
                if current_coord.y > 0 {
                    let up_coord = Coord::new(current_coord.x, current_coord.y - 1);
                    match self.map.get(&up_coord) {
                        Some(_) => neighbors.push(Some(Neighbor::new(up_coord, Direction::Up))),
                        None => neighbors.push(None)
                    }
                }
                else {
                    neighbors.push(None)
                }
                // Check down
                let down_coord = Coord::new(current_coord.x, current_coord.y + 1);
                match self.map.get(&down_coord) {
                    Some(_) => neighbors.push(Some(Neighbor::new(down_coord, Direction::Down))),
                    None => neighbors.push(None)
                }
                // Check left
                if current_coord.x > 0 {
                    let left_coord = Coord::new(current_coord.x - 1, current_coord.y);
                    match self.map.get(&left_coord) {
                        Some(_) => neighbors.push(Some(Neighbor::new(left_coord, Direction::Left))),
                        None => neighbors.push(None)
                    }
                }
                else {
                    neighbors.push(None)
                }
                // Check right
                let right_coord = Coord::new(current_coord.x + 1, current_coord.y);
                match self.map.get(&right_coord) {
                    Some(_) => neighbors.push(Some(Neighbor::new(right_coord, Direction::Right))),
                    None => neighbors.push(None)
                }
            },
            // If only_immediate is false, we want all 8 neighbors around our coord in the grid
            false => {
                unimplemented!("Have not implemented this yet");
            }
        }
        neighbors
    }
    pub fn debug_print_coord_path(&self, coords: &Vec<Coord>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coord::new(x, y);
                if coords.contains(&coord) {
                    print!("X");
                }
                else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.map.get(&Coord { x, y }).expect("Failed to get coord in grid map"))?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

pub trait Input: Sized {
    fn parse<R: BufRead>(reader: R) -> Result<Self>;
}

impl<T: FromStr> Input for Vec<T>
where
    T::Err: Display,
{
    fn parse<R: BufRead>(reader: R) -> Result<Self> {
        reader
            .lines()
            .enumerate()
            .map(|(line_number, line)| {
                T::from_str(&line.context("Failed to read line")?)
                    .map_err(|e| anyhow!("Failed to parse line {}: {}", line_number + 1, e))
            })
            .collect()
    }
}

pub struct Unimplemented;

impl Display for Unimplemented {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "unimplemented")
    }
}

pub trait Problem {
    type Input: Input;
    type PartOne: Display;
    type PartTwo: Display;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne;
    fn solve_part_two(input: &Self::Input) -> Self::PartTwo;
}

pub struct CSV<T>(Vec<T>);

impl<T> CSV<T> {
    pub fn values(&self) -> &[T] {
        &self.0
    }
}

impl<T: FromStr> FromStr for CSV<T>
where
    T::Err: std::error::Error + Send + Sync + 'static,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::parse(s.as_bytes())
    }
}

impl<T: FromStr> Input for CSV<T>
where
    T::Err: std::error::Error + Send + Sync + 'static,
{
    fn parse<R: BufRead>(reader: R) -> Result<Self> {
        let values = reader
            .split(b',')
            .map(|x| Ok(from_utf8(&x?)?.parse()?))
            .collect::<Result<_, Error>>()?;
        Ok(Self(values))
    }
}

pub struct Solution<T> {
    result: T,
    duration: Duration,
}

impl<T: Display> Display for Solution<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Solution: {}", self.result)?;
        writeln!(f, "  Elapsed:  {} seconds", self.duration.as_secs_f64())?;
        Ok(())
    }
}

fn time_solve<F: FnOnce() -> T, T>(f: F) -> Solution<T> {
    let start = Instant::now();
    let result = f();
    let duration = Instant::now().duration_since(start);
    Solution { result, duration }
}

pub type SolveResult<P> = Result<(
    Solution<<P as Problem>::PartOne>,
    Solution<<P as Problem>::PartTwo>,
)>;

pub fn solve<P: Problem>(path: &Path) -> SolveResult<P> {
    let input_file = BufReader::new(File::open(path).context("Failed to open input file")?);
    let input = Input::parse(input_file).context("Failed to parse input")?;

    Ok((
        time_solve(|| P::solve_part_one(&input)),
        time_solve(|| P::solve_part_two(&input)),
    ))
}

pub fn solve_main<P: Problem>() {
    // let path = env::args().nth(0).unwrap();
    // let split: Vec<&str> = path.split('/').collect();
    // let mut day_path = split[2].to_owned();
    // day_path.push_str("/full.input");
    // let path = env::args().nth(1).unwrap_or(day_path);
    let path = env::args().nth(1).unwrap();
    let (part_one, part_two) = solve::<P>(path.as_ref()).expect("failed to solve problem");

    println!("Part one:");
    println!("{}", part_one);
    println!("Part two:");
    println!("{}", part_two);
}
