use problem::{solve_main, Coord, Grid, Problem};

// This implementation is pretty slow, ~4.7s per part on debug mode on my laptop. There must be a faster way to run this
// See Conways Game of Life

fn set_broken_corners(grid: &mut Grid<char>) {
    grid.map.insert(Coord{ x: 0, y: 0 }, '#');
    grid.map.insert(Coord{ x: grid.width - 1, y: 0 }, '#');
    grid.map.insert(Coord{ x: 0, y: grid.height - 1 }, '#');
    grid.map.insert(Coord{ x: grid.width - 1, y: grid.height - 1 }, '#');
}

fn get_neighbors_with_values<'a >(grid: &'a Grid<char>, coord: &Coord) -> Vec<&'a char> {
    grid
    .neighbors(coord, false)
    .iter()
    .filter(|x| x.is_some())
    .map(|x| {
        grid.map.get(&x.unwrap().coord).unwrap()
    })
    .collect()
}

fn simulate_step(grid: Grid<char>, is_part_two: bool) -> Grid<char> {
    let mut out_grid = grid.clone();

    for x in 0..grid.width {
        for y in 0..grid.height {
            let coord = Coord{ x, y };

            let neighbors = get_neighbors_with_values(&grid, &coord);

            let neighbors_who_are_on = neighbors.iter().filter(|x| ***x == '#').count();

            match grid.map.get(&coord).unwrap() {
                '.' => {
                    if neighbors_who_are_on == 3 {
                        out_grid.map.insert(coord, '#');
                    }
                },
                '#' => {
                    if neighbors_who_are_on != 2 && neighbors_who_are_on != 3 {
                        out_grid.map.insert(coord, '.');
                    }
                },
                _ => panic!("Got an invalid light state when checking grid")
            }
        }
    }
    if is_part_two {
        set_broken_corners(&mut out_grid);
    }
    out_grid
}

struct Day18;

impl Problem for Day18 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut grid: Grid<char> = Grid::from_text_grid(input, |x| x);

        let steps = 100;
        for _ in 0..steps {
            grid = simulate_step(grid, false);
        }
        grid.map.into_values().filter(|c| *c == '#').count()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut grid: Grid<char> = Grid::from_text_grid(input, |x| x);
        set_broken_corners(&mut grid);

        let steps = 100;
        for _ in 0..steps {
            grid = simulate_step(grid, true);
        }
        grid.map.into_values().filter(|c| *c == '#').count()
    }
}

fn main() {
    solve_main::<Day18>();
}
