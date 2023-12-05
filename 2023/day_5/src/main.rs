use problem::{solve_main, Problem};

#[derive(Debug)]
struct Map {
    destination_range: u64,
    source_range: u64,
    range_length: u64
}

struct C_Range {
    start: u64,
    length: u64
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
    pub fn map_value(map: &Vec<Map>, value: u64) -> u64 {
        for val in map {
            let end_range = val.source_range + val.range_length;
            let range = val.source_range..end_range;
            if range.contains(&value) {
                let i = value - val.source_range;
                return val.destination_range + i;
            }
        }
        value
    }
}

#[derive(Debug)]
struct Seed;

impl Seed {
    pub fn find_smallest_location(seed_numbers: Vec<u64>, map_map: Vec<Vec<Map>>) -> u64 {
        let mut smallest_location = u64::MAX;
        for (i, seed_number) in seed_numbers.iter().enumerate() {
            let soil = Map::map_value(&map_map[0], *seed_number);
            let fertilizer = Map::map_value(&map_map[1], soil);
            let water = Map::map_value(&map_map[2], fertilizer);
            let light = Map::map_value(&map_map[3], water);
            let temperature = Map::map_value(&map_map[4], light);
            let humidity = Map::map_value(&map_map[5], temperature);
            let location = Map::map_value(&map_map[6], humidity);
            if location < smallest_location {
                smallest_location = location;
            }
        }
        smallest_location
    }
    pub fn get_seed_numbers(input: &Vec<String>) -> Vec<C_Range> {
        let mut output: Vec<C_Range> = Vec::new();
        let vals = input[0].split(" ").skip(1);
        for val in vals {
            let as_num = val.parse::<u64>().expect("Failed to convert seed num to u64");
            let c_range = C_Range { start: as_num, length: 1u64 };
            output.push(c_range);
        }
        output
    }
    pub fn get_seed_ranges(input: &Vec<String>) -> Vec<u64> {
        let mut output: Vec<u64> = Vec::new();
        let mut buffer: [u64; 2] = [0, 0];
        let vals = input[0].split(" ").skip(1);
        for (i, val) in vals.enumerate() {
            let as_num = val.parse::<u64>().expect("Failed to convert seed num to u64");
            let modu = i % 2;
            buffer[modu] = as_num;
            if modu == 1 {
                let end_range = buffer[0] + buffer[1];
                for j in buffer[0]..end_range {
                    output.push(j);
                }
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
        // Part 2 has a two minute runtime which is unacceptably long
        // Better solution: Rather than converting the seed range into seeds ((79 14) into (79, 80, 81, 82...)) it is
        //  possible to process the seed ranges themselves, drastically cutting down on iterations
        let map_map: Vec<Vec<Map>> = Map::from_input(input);
        let seed_numbers = Seed::get_seed_ranges(input);
        Seed::find_smallest_location(seed_numbers, map_map)
    }
}

fn main() {
    solve_main::<Day5>();
}
