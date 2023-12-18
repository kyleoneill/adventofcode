use std::collections::{HashMap, BinaryHeap};

use problem::{solve_main, Problem, Grid, Coord, Neighbor, Direction};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct MultiDimensionalKey {
    position: Coord,
    direction: Direction,
    dir_count: usize
}

#[derive(PartialEq, Eq, Debug)]
struct State {
    dumb: MultiDimensionalKey,
    cost: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // other.cost.cmp().then_with() will resolve ties. Do we need to resolve a tie? How do we compare the coords?
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_candidates(grid: &Grid<u32>, node: &State, min_dir_count: usize, max_dir_count: usize) -> Vec<Neighbor> {
    let mut neighbors = Vec::new();
    // There is definitely a better way to do this
    // actual_neighbors = [ up, down, left, right ]
    let actual_neighbors = grid.neighbors(&node.dumb.position, true);
    match node.dumb.direction {
        Direction::Up => {
            // left is left
            if actual_neighbors[2].is_some() && node.dumb.dir_count >= min_dir_count {
                neighbors.push(actual_neighbors[2].unwrap());
            }
            // right is right
            if actual_neighbors[3].is_some() && node.dumb.dir_count >= min_dir_count {
                neighbors.push(actual_neighbors[3].unwrap());
            }
            // forward is up
            if node.dumb.dir_count < max_dir_count && actual_neighbors[0].is_some() {
                neighbors.push(actual_neighbors[0].unwrap())
            }
        },
        Direction::Down => {
            // left is right
            if actual_neighbors[3].is_some() && node.dumb.dir_count >= min_dir_count {
                neighbors.push(actual_neighbors[3].unwrap());
            }
            // right is left
            if actual_neighbors[2].is_some() && node.dumb.dir_count >= min_dir_count {
                neighbors.push(actual_neighbors[2].unwrap());
            }
            // forward is down
            if node.dumb.dir_count < max_dir_count && actual_neighbors[1].is_some() {
                neighbors.push(actual_neighbors[1].unwrap())
            }
        }, Direction::Left => {
            // left is down
            if actual_neighbors[1].is_some() && node.dumb.dir_count >= min_dir_count {
                neighbors.push(actual_neighbors[1].unwrap());
            }
            // right is up
            if actual_neighbors[0].is_some() && node.dumb.dir_count >= min_dir_count {
                neighbors.push(actual_neighbors[0].unwrap());
            }
            // forward is left
            if node.dumb.dir_count < max_dir_count && actual_neighbors[2].is_some() {
                neighbors.push(actual_neighbors[2].unwrap())
            }
        },
        Direction::Right => {
            // left is up
            if actual_neighbors[0].is_some() && node.dumb.dir_count >= min_dir_count {
                neighbors.push(actual_neighbors[0].unwrap());
            }
            // right is down
            if actual_neighbors[1].is_some() && node.dumb.dir_count >= min_dir_count {
                neighbors.push(actual_neighbors[1].unwrap());
            }
            // forward is right
            if node.dumb.dir_count < max_dir_count && actual_neighbors[3].is_some() {
                neighbors.push(actual_neighbors[3].unwrap())
            }
        }
    }
    neighbors
}

fn heuristic(a: &Coord, b: &Coord) -> usize {
    // Manhattan distance
    // This is wrong, we care about the heat diff and not the distance diff
    return ((a.x as i64 - b.x as i64).abs() as usize) + ((a.y as i64 - b.y as i64).abs() as usize)
}

fn unwind(map: &HashMap<MultiDimensionalKey, MultiDimensionalKey>, mut tail: MultiDimensionalKey) -> Vec<Coord> {
    let mut path = Vec::new();
    loop {
        path.push(tail.position);
        tail = match map.get(&tail) {
            Some(val) => *val,
            None => break
        }
    }
    path
}

fn pathfind(grid: &Grid<u32>, start: Coord, finish: Coord, min_dir_count: usize, max_dir_count: usize) -> Vec<Coord> {
    let mut frontier: BinaryHeap<State> = BinaryHeap::new();
    let fn_coord = Coord::new(start.x + 1, start.y);
    let fn_value = grid.map.get(&fn_coord).unwrap();
    let fn_key = MultiDimensionalKey { position: fn_coord, direction: Direction::Right, dir_count: 1 };
    let sn_coord = Coord::new(start.x, start.y + 1);
    let sn_value = grid.map.get(&sn_coord).unwrap();
    let sn_key = MultiDimensionalKey { position: sn_coord, direction: Direction::Down, dir_count: 1 };
    frontier.push(State { dumb: fn_key, cost: *fn_value });
    frontier.push(State { dumb: sn_key, cost: *sn_value });

    let mut came_from: HashMap<MultiDimensionalKey, MultiDimensionalKey> = HashMap::new();
    
    let mut cost_so_far: HashMap<MultiDimensionalKey, u32> = HashMap::new();
    cost_so_far.insert(MultiDimensionalKey { position: fn_coord, direction: Direction::Right, dir_count: 1}, *fn_value);
    cost_so_far.insert(MultiDimensionalKey { position: sn_coord, direction: Direction::Down, dir_count: 1}, *sn_value);
    
    while let Some(node) = frontier.pop() {
        if node.dumb.position == finish && node.dumb.dir_count >= min_dir_count {
            return unwind(&came_from, node.dumb);
        }
        for neighbor in get_candidates(grid, &node, min_dir_count, max_dir_count) {
            let new_cost = cost_so_far.get(&node.dumb).unwrap() + grid.map.get(&neighbor.coord).unwrap();
            let neighbor_dir_count = match node.dumb.direction == neighbor.direction {
                true => node.dumb.dir_count + 1,
                false => 1
            };
            let neighbor_key = MultiDimensionalKey { position: neighbor.coord, direction: neighbor.direction, dir_count: neighbor_dir_count };
            let potentially_known_cost = cost_so_far.get(&neighbor_key);
            if potentially_known_cost.is_none() || new_cost < *potentially_known_cost.unwrap() {
                cost_so_far.entry(neighbor_key).and_modify(|val| *val = new_cost).or_insert(new_cost);
                let priority = new_cost + heuristic(&finish, &neighbor.coord) as u32;
                frontier.push(State { dumb: neighbor_key, cost: priority });
                came_from.insert(neighbor_key, node.dumb);
            }
        }
    }
    vec![]
}

struct Day17;

impl Problem for Day17 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        // There have been so many text grids this year. Finally just made a generic function to handle constructing a map
        let city_map: Grid<u32> = Grid::from_text_grid(input, |c| c as u32 - 48);
        let path = pathfind(&city_map, Coord::new(0, 0), Coord::new(city_map.width - 1, city_map.height - 1), 1, 3);
        path.iter().map(|c| city_map.map.get(c).unwrap().to_owned() as usize).sum()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let city_map: Grid<u32> = Grid::from_text_grid(input, |c| c as u32 - 48);
        let path = pathfind(&city_map, Coord::new(0, 0), Coord::new(city_map.width - 1, city_map.height - 1), 4, 10);
        path.iter().map(|c| city_map.map.get(c).unwrap().to_owned() as usize).sum()
    }
}

fn main() {
    solve_main::<Day17>();
}
