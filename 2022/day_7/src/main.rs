use std::borrow::Borrow;
use std::collections::HashMap;
use problem::{solve_main, Problem};

fn go_up_dir(input: &str) -> String {
    if input == "/" {
        return "/".to_owned();
    }
    let rev: Vec<char> = input.chars().rev().collect();
    for i in 1..rev.len() {
        if rev[i] == '/' {
            return rev[i..].iter().rev().collect()
        }
    }
    "/".to_owned()
}

#[derive(Debug)]
struct Directory {
    files: Vec<u32>,
    directories: Vec<String>,
    size: u32
}

impl Directory {
    fn new() -> Self {
        Self {files: Vec::new(), directories: Vec::new(), size: 0 }
    }
}

#[derive(Debug)]
struct FileSystem {
    filesystem: HashMap<String, Directory>
}

impl FileSystem {
    fn new(input: &Vec<String>) -> Self {
        let mut filesystem = HashMap::new();
        filesystem.insert("/".to_owned(), Directory::new());
        let mut current_dir = "/".to_owned();
        for i in 2..input.len() {
            let line: &str = input[i].borrow();
            let split: Vec<&str> = line.split_whitespace().collect();
            match split[0] {
                "$" => {
                    // This line is a command
                    if split[1] == "cd" {
                        // We are changing directories
                        let chars: Vec<char> = split[2].chars().collect();
                        let dir_changing_to: String;
                        if chars[0] == '/' {
                            // Our path is absolute
                            if split[2] == "/" {
                                dir_changing_to = "/".to_owned();
                            }
                            else {
                                dir_changing_to = split[2].to_owned() + "/";
                            }
                        }
                        else if split[2] == ".." {
                            // We are moving up a dir
                            if current_dir == "/" {
                                continue
                            }
                            dir_changing_to = go_up_dir(&current_dir);

                            // When we go up, we need to set our size
                            let current_size = filesystem.get(&current_dir).unwrap().size;
                            let mut one_up_dir = filesystem.get_mut(&dir_changing_to).unwrap();
                            one_up_dir.size += current_size;
                        }
                        else {
                            // We are going down a dir
                            dir_changing_to = current_dir.to_owned() + split[2] + "/";
                        }
                        if filesystem.contains_key(&dir_changing_to) {
                            current_dir = dir_changing_to;
                        }
                    }
                    // We do not care about the ls command
                },
                "dir" => {
                    // The current folder contains a sub-folder
                    let sub_folder = current_dir.to_owned() + split[1] + "/";
                    filesystem.insert(sub_folder.clone(), Directory::new());
                    let mutable_ref = filesystem.get_mut(&current_dir).unwrap();
                    mutable_ref.directories.push(sub_folder);
                },
                _ => {
                    // The current line is a file
                    let get_dir = filesystem.get_mut(&current_dir).unwrap();
                    get_dir.files.push(split[0].parse().unwrap());
                    get_dir.size += split[0].parse::<u32>().unwrap();
                }
            }
        }
        while current_dir != "/" {
            // We need to walk back to the top to fill in size info
            let current_size = filesystem.get(&current_dir).unwrap().size;
            let one_up = go_up_dir(&current_dir);
            let mut one_up_dir = filesystem.get_mut(&one_up).unwrap();
            one_up_dir.size += current_size;
            current_dir = one_up;
        }
        Self { filesystem }
    }
}

struct Day7;

impl Problem for Day7 {
    type Input = Vec<String>;
    type PartOne = u32;
    type PartTwo = u32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let filesystem = FileSystem::new(input);
        let all_dirs: Vec<&Directory> = filesystem.filesystem.values().collect();
        let mut sum = 0;
        for dir in all_dirs {
            if dir.size <= 100000 {
                sum += dir.size;
            }
        }
        sum
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        // This one is also poorly made but I did three advents today okay
        let filesystem = FileSystem::new(input);
        let total_space_used = filesystem.filesystem.get("/").unwrap().size;
        let available_space = 70_000_000 as u32 - total_space_used;
        let need_to_free = 30_000_000 - available_space;
        let all_dirs: Vec<&Directory> = filesystem.filesystem.values().collect();
        let mut valid_dirs: Vec<u32> = Vec::new();
        for dir in all_dirs {
            if dir.size >= need_to_free {
                valid_dirs.push(dir.size);
            }
        }
        valid_dirs.into_iter().min().unwrap()
    }
}

fn main() {
    solve_main::<Day7>();
}
