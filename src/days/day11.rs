#![allow(dead_code)]
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    multi::separated_list1,
    sequence::{delimited, preceded},
    *,
};
use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    inspections: u64,
    items: VecDeque<u64>,
    operation: Operation,
    test: MonkeyTest,
}

impl Monkey {
    fn catch_item(&mut self, item: u64) {
        self.items.push_back(item);
    }

    fn inspect_item(&mut self, item: u64) -> u64 {
        self.inspections += 1;
        match &self.operation {
            Operation::Add(val) => match val.1 {
                Value::Num(v) => item + v,
                Value::Old => item + item,
            },
            Operation::Multiply(val) => match val.1 {
                Value::Num(v) => item * v,
                Value::Old => item * item,
            },
        }
    }

    fn test_item(&self, item: u64) -> usize {
        if item % self.test.divisible_by == 0 {
            self.test.true_monkey as usize
        } else {
            self.test.false_monkey as usize
        }
    }
}

#[derive(Debug)]
enum Operation {
    Multiply((Value, Value)),
    Add((Value, Value)),
}

#[derive(Debug)]
enum Value {
    Old,
    Num(u64),
}

#[derive(Debug)]
struct MonkeyTest {
    divisible_by: u64,
    true_monkey: u64,
    false_monkey: u64,
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _id) = delimited(tag("Monkey "), nom::character::complete::u64, tag(":"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) = parse_starting_items(input)?;
    let (input, _) = multispace1(input)?;
    let (input, operation) = parse_operation(input)?;
    let (input, _) = multispace1(input)?;
    let (input, test) = parse_test(input)?;

    Ok((
        input,
        Monkey {
            inspections: 0,
            items,
            operation,
            test,
        },
    ))
}

fn parse_starting_items(input: &str) -> IResult<&str, VecDeque<u64>> {
    let (input, items) = preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), nom::character::complete::u64),
    )(input)?;

    let vd_items: VecDeque<u64> = items.into();
    Ok((input, vd_items))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("Operation: new = old")(input)?;
    let (input, operator) = delimited(multispace1, alt((tag("*"), tag("+"))), multispace1)(input)?;
    let (input, v1) = alt((
        tag("old").map(|_| Value::Old),
        nom::character::complete::u64.map(Value::Num),
    ))(input)?;

    match operator {
        "*" => Ok((input, Operation::Multiply((Value::Old, v1)))),
        "+" => Ok((input, Operation::Add((Value::Old, v1)))),
        _ => Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Char,
        })),
    }
}

fn parse_test(input: &str) -> IResult<&str, MonkeyTest> {
    let (input, _) = tag("Test: divisible by ")(input)?;
    let (input, divisible_by) = nom::character::complete::u64(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("If true: throw to monkey ")(input)?;
    let (input, true_monkey) = nom::character::complete::u64(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("If false: throw to monkey ")(input)?;
    let (input, false_monkey) = nom::character::complete::u64(input)?;

    Ok((
        input,
        MonkeyTest {
            divisible_by,
            true_monkey,
            false_monkey,
        },
    ))
}

pub fn part1(input: &str) -> String {
    const ROUNDS: u64 = 20;

    // get monkeys
    let mut monkeys: Vec<Monkey> = Vec::new();
    let lines = input.split("\n\n").collect::<Vec<&str>>();
    for line in lines {
        let (_line, monkey) = parse_monkey(line).unwrap();
        monkeys.push(monkey);
    }

    // process monkeys
    for _i in 0..ROUNDS {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let item = monkeys[i].inspect_item(item);
                let item = item / 3;
                let monkey_num = monkeys[i].test_item(item);
                monkeys[monkey_num].catch_item(item)
            }
        }
    }

    // calculate result
    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<u64>>();

    inspections.sort();
    inspections.reverse();

    let result = inspections[0] * inspections[1];
    result.to_string()
}

pub fn part2(input: &str) -> String {
    const ROUNDS: u64 = 10_000;

    // get monkeys
    let mut monkeys: Vec<Monkey> = Vec::new();
    let lines = input.split("\n\n").collect::<Vec<&str>>();
    for line in lines {
        let (_line, monkey) = parse_monkey(line).unwrap();
        monkeys.push(monkey);
    }

    // get divisor
    let divisor = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product::<u64>();

    // process monkeys
    for _i in 0..ROUNDS {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let item = monkeys[i].inspect_item(item);
                let item = item % divisor;
                let monkey_num = monkeys[i].test_item(item);
                monkeys[monkey_num].catch_item(item)
            }
        }
    }

    // calculate result
    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<u64>>();

    inspections.sort();
    inspections.reverse();

    let result = inspections[0] * inspections[1];
    result.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "10605");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, "2713310158");
    }
}
