mod days;

use std::env;
use std::fs;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if an argument was provided
    if args.len() < 2 {
        println!("Please provide an argument.");
        return;
    }

    // Get the first argument
    let day = &args[1].parse::<i32>().unwrap_or(0);

    match day {
        1 => {
            let file = fs::read_to_string("./input/day01.txt").unwrap();
            println!("Part 1: {}", days::day01::part1(&file));
            println!("Part 2: {}", days::day01::part2(&file));
        }
        2 => {
            let file = fs::read_to_string("./input/day02.txt").unwrap();
            println!("Part 1: {}", days::day02::part1(&file));
            println!("Part 2: {}", days::day02::part2(&file));
        }
        3 => {
            let file = fs::read_to_string("./input/day03.txt").unwrap();
            println!("Part 1: {}", days::day03::part1(&file));
            println!("Part 2: {}", days::day03::part2(&file));
        }
        _ => println!("invalid day"),
    }
}
