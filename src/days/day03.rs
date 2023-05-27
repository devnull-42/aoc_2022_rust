use std::collections::HashMap;

pub fn part1(input: &str) -> String {
    let mut total: u32 = 0;
    let mut priorities: HashMap<char, u8> = HashMap::new();
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .for_each(|(idx, c)| {
            priorities.insert(c, idx as u8 + 1);
        });

    input.lines().for_each(|line| {
        let midpoint = line.len() / 2;
        let (first, second) = line.split_at(midpoint);
        let mut found = "".to_string();

        first.chars().for_each(|c| {
            if second.contains(c) && !found.contains(c) {
                found.push(c);
                let a = priorities.get(&c).unwrap();
                total += *a as u32;
            }
        })
    });
    total.to_string()
}

pub fn part2(input: &str) -> String {
    let mut total: u32 = 0;
    let mut priorities: HashMap<char, u8> = HashMap::new();
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .for_each(|(idx, c)| {
            priorities.insert(c, idx as u8 + 1);
        });

    let lines: Vec<&str> = input.lines().collect();
    let chunks = lines.chunks(3);
    chunks.for_each(|chunk| {
        let mut found = "".to_string();
        chunk[0].chars().for_each(|c| {
            if chunk[1].contains(c) && chunk[2].contains(c) && !found.contains(c) {
                found.push(c);
                let a = priorities.get(&c).unwrap();
                total += *a as u32;
            }
        })
    });
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, "70");
    }
}
