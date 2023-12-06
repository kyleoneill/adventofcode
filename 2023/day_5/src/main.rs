use problem::{solve_main, Problem};

#[derive(Debug)]
struct Map {
    destination_range: u64,
    source_range: u64,
    range_length: u64
}

#[derive(Debug, Clone, Copy)]
struct CRange {
    pub start: u64,
    pub length: u64
}

impl CRange {
    pub fn end(&self) -> u64 {
        self.start + self.length
    }
}

impl Map {
    fn new(destination_range: u64, source_range: u64, range_length: u64) -> Self {
        Self { destination_range, source_range, range_length }
    }
    pub fn from_input(input: &Vec<String>) -> Vec<Vec<Map>> {
        let mut ret: Vec<Vec<Self>> = Vec::new();
        let mut buffer: Vec<Self> = Vec::new();
        let mut iter = input.iter().skip(3);

        while let Some(iter_val) = iter.next() {
            let as_str = iter_val.as_str();
            if as_str == "" {
                // The current section is over, empty our buffer into the map
                ret.push(buffer);
                buffer = Vec::new();

                // Skip the next value as it will be text
                iter.next();
                continue;
            }
            let split: Vec<&str> = as_str.split(" ").collect();
            let destination_range = split[0].parse::<u64>().expect("Failed to convert destination start");
            let source_range = split[1].parse::<u64>().expect("Failed to convert source start");
            let range_length = split[2].parse::<u64>().expect("Failed to convert range length");
            let map = Self::new(destination_range, source_range, range_length);
            buffer.push(map);
        }
        // After the final iteration we need to empty the buffer into our map one last time
        ret.push(buffer);
        ret
    }
    pub fn map_value(map: &Vec<Map>, mut value_vec: Vec<CRange>) -> Vec<CRange> {
        let mut results: Vec<CRange> = Vec::new();
        for map_value in map {
            let mut new_value_vec: Vec<CRange> = Vec::new();
            while let Some(value) = value_vec.pop() {
                let map_range_end = map_value.source_range + map_value.range_length;
                let value_end = value.end();

                // The map range completely contains the value range
                if value.start >= map_value.source_range && value_end <= map_range_end {
                    let offset = value.start - map_value.source_range;
                    results.push(CRange { start: map_value.destination_range + offset, length: value.length });
                }

                // The input start begins before the map range and ends inside the map range
                else if value.start < map_value.source_range && value_end > map_value.source_range && value_end < map_range_end {
                    // outside left
                    let offset = map_value.source_range - value.start;
                    new_value_vec.push(CRange { start: value.start, length: offset });

                    // inside
                    let offset = value_end - map_value.source_range;
                    results.push(CRange { start: map_value.destination_range, length: offset });
                }

                // The input start begins inside the map range and ends outside the map range
                else if value.start >= map_value.source_range && value.start < map_range_end && value_end >= map_range_end {
                    // inside
                    let offset = map_range_end - value.start;
                    let distance_into_map = value.start - map_value.source_range;
                    results.push(CRange { start: map_value.destination_range + distance_into_map, length: offset });

                    // outside right
                    let offset = value_end - map_range_end;
                    new_value_vec.push(CRange { start: map_range_end, length: offset });
                }
                // The input completely contains the map range and must be broken into a before, during, and after
                else if map_value.source_range >= value.start && map_range_end <= value_end {
                    // outside left
                    let offset = map_value.source_range - value.start;
                    new_value_vec.push(CRange { start: value.start, length: offset });

                    // inside
                    let offset = map_range_end - map_value.source_range;
                    results.push(CRange { start: map_value.destination_range, length: offset });

                    // outside right
                    let offset = value_end - map_range_end;
                    new_value_vec.push(CRange { start: map_range_end, length: offset });
                }
                else {
                    new_value_vec.push(value);
                }
            }
            value_vec = new_value_vec;
        }
        results.append(&mut value_vec);
        results
    }
}

#[derive(Debug)]
struct Seed;

impl Seed {
    pub fn find_smallest_location(seed_numbers: Vec<CRange>, map_map: Vec<Vec<Map>>) -> u64 {
        let soil = Map::map_value(&map_map[0], seed_numbers);
        let fertilizer = Map::map_value(&map_map[1], soil);
        let water = Map::map_value(&map_map[2], fertilizer);
        let light = Map::map_value(&map_map[3], water);
        let temperature = Map::map_value(&map_map[4], light);
        let humidity = Map::map_value(&map_map[5], temperature);
        let location = Map::map_value(&map_map[6], humidity);
        location.iter().map(|x| x.start).min().unwrap()
    }
    pub fn get_seed_numbers(input: &Vec<String>) -> Vec<CRange> {
        let mut output: Vec<CRange> = Vec::new();
        let vals = input[0].split(" ").skip(1);
        for val in vals {
            let as_num = val.parse::<u64>().expect("Failed to convert seed num to u64");
            let c_range = CRange { start: as_num, length: 1u64 };
            output.push(c_range);
        }
        output
    }
    pub fn get_seed_ranges(input: &Vec<String>) -> Vec<CRange> {
        let mut output: Vec<CRange> = Vec::new();
        let mut buffer: [u64; 2] = [0, 0];
        let vals = input[0].split(" ").skip(1);
        for (i, val) in vals.enumerate() {
            let as_num = val.parse::<u64>().expect("Failed to convert seed num to u64");
            let modu = i % 2;
            buffer[modu] = as_num;
            if modu == 1 {
                output.push(CRange { start: buffer[0], length: buffer[1] })
            }
        }
        output
    }
}

struct Day5;

impl Problem for Day5 {
    type Input = Vec<String>;
    type PartOne = u64;
    type PartTwo = u64;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let map_map: Vec<Vec<Map>> = Map::from_input(input);
        let seed_numbers = Seed::get_seed_numbers(input);
        Seed::find_smallest_location(seed_numbers, map_map)
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let map_map: Vec<Vec<Map>> = Map::from_input(input);
        let seed_numbers = Seed::get_seed_ranges(input);
        Seed::find_smallest_location(seed_numbers, map_map)
    }
}

fn main() {
    solve_main::<Day5>();
}
