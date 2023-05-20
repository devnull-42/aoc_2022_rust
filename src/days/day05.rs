use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::IResult;
use std::str::FromStr;

fn parse_instruction(input: &str) -> IResult<&str, (u8, u8, u8)> {
    let (input, _) = tag("move ")(input)?;
    let (input, num) = digit1(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = digit1(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = digit1(input)?;

    let num = u8::from_str(num).unwrap();
    let from = u8::from_str(from).unwrap();
    let to = u8::from_str(to).unwrap();

    Ok((input, (num, from, to)))
}

fn parse_instructions(input: &str) -> Vec<(u8, u8, u8)> {
    let mut instructions: Vec<(u8, u8, u8)> = vec![];
    input.lines().for_each(|line| {
        let (_, instruction) = parse_instruction(line).unwrap();
        instructions.push(instruction);
    });
    instructions
}

fn parse_crates(input: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![];
    let lines = input.lines();

    for line in lines {
        let mut stacknum = 0;
        line.chars().collect::<Vec<char>>().chunks(4).for_each(|c| {
            if stacknum >= stacks.len() {
                stacks.push(vec![]);
            }
            let check_char = c.get(1).unwrap();
            if check_char.is_alphabetic() {
                stacks[stacknum].push(check_char.clone());
            }
            stacknum += 1;
        });
    }
    for stack in &mut stacks {
        stack.reverse();
    }
    stacks
}

pub fn part1(input: &str) -> String {
    // split input into stacks and instructions
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut stacks = parse_crates(parts[0]);
    let instructions = parse_instructions(parts[1]);

    for instruction in instructions {
        for _ in 0..instruction.0 {
            let c = stacks[(instruction.1 - 1) as usize].pop().unwrap();
            stacks[(instruction.2 - 1) as usize].push(c);
        }
    }

    let mut result = "".to_string();
    for stack in &mut stacks {
        result.push(stack.pop().unwrap());
    }
    result.to_string()
}

pub fn part2(input: &str) -> String {
    // split input into stacks and instructions
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut stacks = parse_crates(parts[0]);
    let instructions = parse_instructions(parts[1]);

    for instruction in instructions {
        // remove crates
        let mut mover = Vec::new();
        for _ in 0..instruction.0 {
            let c = stacks[(instruction.1 - 1) as usize].pop().unwrap();
            mover.push(c);
        }

        // place crates
        mover.reverse();
        mover.iter().for_each(|c| {
            stacks[(instruction.2 - 1) as usize].push(*c);
        })
    }

    let mut result = "".to_string();
    for stack in &mut stacks {
        result.push(stack.pop().unwrap());
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, "MCD");
    }
}
