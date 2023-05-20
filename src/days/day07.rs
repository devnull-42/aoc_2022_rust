#![allow(dead_code)]
use std::collections::HashMap;

enum Command {
    Dir(String),
    DirUp,
    File(u32),
    Other(String),
}

fn parse_command(line: &str) -> Command {
    if line.starts_with("$ cd") {
        let d = line.trim_start_matches("$ cd ");
        match d {
            ".." => return Command::DirUp,
            _ => return Command::Dir(d.to_string()),
        }
    } else if line.starts_with("$ ls") || line.starts_with("dir ") {
        return Command::Other(line.to_string());
    } else {
        let file_size = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .get(0)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        return Command::File(file_size);
    }
}

fn parse_commands(input: &str) -> HashMap<String, u32> {
    // create hashmap to use to store directory names and sizes
    let mut dir_structure = HashMap::<String, u32>::new();

    // create a vec to store the directory path
    let mut dir_path = Vec::<String>::new();

    let lines = input.lines();
    for line in lines {
        let c = parse_command(line);
        match c {
            Command::Dir(val) => {
                let current_dir = dir_path.last().clone().unwrap_or(&val).to_string() + &val;
                dir_path.push(current_dir.clone());
                dir_structure.entry(current_dir.clone()).or_insert(0);
            }
            Command::DirUp => {
                dir_path.pop();
            }
            Command::File(file_size) => {
                for d in &dir_path {
                    dir_structure
                        .entry(d.clone())
                        .and_modify(|e| *e += file_size);
                }
            }
            Command::Other(_l) => {}
        }
    }

    return dir_structure;
}

pub fn part1(input: &str) -> String {
    let dir_structure = parse_commands(input);

    let result = &dir_structure
        .into_iter()
        .map(|(_, v)| v)
        .filter(|v| v.le(&100_000))
        .sum::<u32>();
    result.to_string()
}

pub fn part2(input: &str) -> String {
    const TOTAL_SPACE: u32 = 70_000_000;
    const NEEDED_SPACE: u32 = 30_000_000;

    let dir_structure = parse_commands(input);

    let mut sizes = dir_structure
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<u32>>();

    sizes.sort();

    let used_space = sizes.last().clone().unwrap();
    let space_target = NEEDED_SPACE - (TOTAL_SPACE - used_space);

    for size in &sizes {
        if size.gt(&space_target) {
            return size.to_string();
        }
    }
    used_space.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "95437");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, "24933642");
    }
}
