use std::collections::{HashSet, HashMap};

use problem::{solve_main, Problem};

#[derive(Debug)]
struct Edge {
    destination: String,
    cost: usize,
}

#[derive(Debug)]
struct Graph {
    vertices: HashSet<String>,
    edges: HashMap<String, Vec<Edge>>,
}

impl Graph {
    fn new() -> Self {
        Self { vertices: HashSet::new(), edges: HashMap::new() }
    }

    fn from_input(input: &Vec<String>) -> Self {
        let mut graph: Self = Self::new();
        for line in input {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let cost = tokens[4].parse::<usize>().expect("Failed to parse token into cost");
            graph.insert_edges(tokens[0], tokens[2], cost);
        }
        graph
    }

    fn get_vertices(&self) -> Vec<&str> {
        self.vertices.iter().map(|x| x.as_str()).collect()
    }

    fn insert_edges(&mut self, a: &str, b: &str, cost: usize) {
        let a: String = a.to_owned();
        let b: String = b.to_owned();
        self.vertices.insert(a.clone());
        self.vertices.insert(b.clone());
        
        let vertex_pair = Edge { destination: b.clone(), cost };
        if self.edges.contains_key(a.as_str()) {
            self.edges.get_mut(a.as_str()).unwrap().push(vertex_pair);
        }
        else {
            self.edges.insert(a.clone(), vec![vertex_pair]);
        }

        let vertex_pair = Edge { destination: a, cost };
        if self.edges.contains_key(b.as_str()) {
            self.edges.get_mut(b.as_str()).unwrap().push(vertex_pair);
        }
        else {
            self.edges.insert(b, vec![vertex_pair]);
        }
    }

    fn get_path(&self, start: &str, mut unvisited_nodes: HashSet<String>, traveled: usize, get_shortest: bool) -> Option<usize> {
        unvisited_nodes.remove(start);

        if unvisited_nodes.len() == 0 {
            return Some(traveled);
        }

        let edges = self.edges.get(start).expect("Failed to locate node in edges");
        let mut superlative_path = match get_shortest {
            true => usize::MAX,
            false => 0
        };
        for edge in edges {
            if !unvisited_nodes.contains(edge.destination.as_str()) {
                continue;
            }
            match self.get_path(edge.destination.as_str(), unvisited_nodes.clone(), traveled + edge.cost, get_shortest) {
                Some(val) => match get_shortest {
                    true => superlative_path = superlative_path.min(val),
                    false => superlative_path = superlative_path.max(val)
                },
                None => ()
            }
        }

        if get_shortest && superlative_path == usize::MAX || superlative_path == 0 {
            return None;
        }
        else {
            return Some(superlative_path);
        }
    }
}

struct Day9;

impl Problem for Day9 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    // I think the runtime of this is very bad and it only works because the input size is only 28 lines
    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let graph = Graph::from_input(input);
        let mut shortest_observed_path: Self::PartOne = usize::MAX;
        for node in graph.get_vertices() {
            let possible_nodes = graph.vertices.clone();
            let shortest_path = graph.get_path(node, possible_nodes, 0, true).expect("Failed to find a shortest path");
            shortest_observed_path = usize::min(shortest_observed_path, shortest_path);
        }
        shortest_observed_path
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let graph = Graph::from_input(input);
        let mut longest_observed_path: Self::PartOne = 0;
        for node in graph.get_vertices() {
            let possible_nodes = graph.vertices.clone();
            let longest = graph.get_path(node, possible_nodes, 0, false).expect("Failed to find a longest path");
            longest_observed_path = usize::max(longest_observed_path, longest);
        }
        longest_observed_path
    }
}

fn main() {
    solve_main::<Day9>();
}
