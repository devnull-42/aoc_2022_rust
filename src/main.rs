mod days;

use std::env;
use std::fs;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    let mut day: u8 = 0;
    // Check if an argument was provided
    if args.len() > 1 {
        day = args[1].parse::<u8>().unwrap_or(0);
    }

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
        4 => {
            let file = fs::read_to_string("./input/day04.txt").unwrap();
            println!("Part 1: {}", days::day04::part1(&file));
            println!("Part 2: {}", days::day04::part2(&file));
        }
        5 => {
            let file = fs::read_to_string("./input/day05.txt").unwrap();
            println!("Part 1: {}", days::day05::part1(&file));
            println!("Part 2: {}", days::day05::part2(&file));
        }
        6 => {
            let file = fs::read_to_string("./input/day06.txt").unwrap();
            println!("Part 1: {}", days::day06::part1(&file));
            println!("Part 2: {}", days::day06::part2(&file));
        }
        7 => {
            let file = fs::read_to_string("./input/day07.txt").unwrap();
            println!("Part 1: {}", days::day07::part1(&file));
            println!("Part 2: {}", days::day07::part2(&file));
        }
        8 => {
            let file = fs::read_to_string("./input/day08.txt").unwrap();
            println!("Part 1: {}", days::day08::part1(&file));
            println!("Part 2: {}", days::day08::part2(&file));
        }
        9 => {
            let file = fs::read_to_string("./input/day09.txt").unwrap();
            println!("Part 1: {}", days::day09::part1(&file));
            println!("Part 2: {}", days::day09::part2(&file));
        }
        10 => {
            let file = fs::read_to_string("./input/day10.txt").unwrap();
            println!("Part 1: {}", days::day10::part1(&file));
            println!("Part 2: {}", days::day10::part2(&file));
        }
        0 => {
            println!("=======    Day 01    =======");
            let file = fs::read_to_string("./input/day01.txt").unwrap();
            println!("Part 1: {}", days::day01::part1(&file));
            println!("Part 2: {}", days::day01::part2(&file));
            println!();

            println!("=======    Day 02    =======");
            let file = fs::read_to_string("./input/day02.txt").unwrap();
            println!("Part 1: {}", days::day02::part1(&file));
            println!("Part 2: {}", days::day02::part2(&file));
            println!();

            println!("=======    Day 03    =======");
            let file = fs::read_to_string("./input/day03.txt").unwrap();
            println!("Part 1: {}", days::day03::part1(&file));
            println!("Part 2: {}", days::day03::part2(&file));
            println!();

            println!("=======    Day 04    =======");
            let file = fs::read_to_string("./input/day04.txt").unwrap();
            println!("Part 1: {}", days::day04::part1(&file));
            println!("Part 2: {}", days::day04::part2(&file));
            println!();

            println!("=======    Day 05    =======");
            let file = fs::read_to_string("./input/day05.txt").unwrap();
            println!("Part 1: {}", days::day05::part1(&file));
            println!("Part 2: {}", days::day05::part2(&file));
            println!();

            println!("=======    Day 06    =======");
            let file = fs::read_to_string("./input/day06.txt").unwrap();
            println!("Part 1: {}", days::day06::part1(&file));
            println!("Part 2: {}", days::day06::part2(&file));
            println!();

            println!("=======    Day 07    =======");
            let file = fs::read_to_string("./input/day07.txt").unwrap();
            println!("Part 1: {}", days::day07::part1(&file));
            println!("Part 2: {}", days::day07::part2(&file));
            println!();

            println!("=======    Day 08    =======");
            let file = fs::read_to_string("./input/day08.txt").unwrap();
            println!("Part 1: {}", days::day08::part1(&file));
            println!("Part 2: {}", days::day08::part2(&file));
            println!();

            println!("=======    Day 09    =======");
            let file = fs::read_to_string("./input/day09.txt").unwrap();
            println!("Part 1: {}", days::day09::part1(&file));
            println!("Part 2: {}", days::day09::part2(&file));
            println!();

            println!("=======    Day 010    =======");
            let file = fs::read_to_string("./input/day10.txt").unwrap();
            println!("Part 1: {}", days::day10::part1(&file));
            println!("Part 2: {}", days::day10::part2(&file));
            println!();
        }
        _ => println!("invalid day"),
    }
}
