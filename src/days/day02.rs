use std::str::FromStr;
#[derive(Debug)]
enum Turn {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl Turn {
    fn resolve(&self, other: &Turn) -> u32 {
        match self {
            Turn::Rock => match other {
                Turn::Rock => Turn::Rock as u32 + 3,
                Turn::Paper => Turn::Rock as u32,
                Turn::Scissors => Turn::Rock as u32 + 6,
            },
            Turn::Paper => match other {
                Turn::Rock => Turn::Paper as u32 + 6,
                Turn::Paper => Turn::Paper as u32 + 3,
                Turn::Scissors => Turn::Paper as u32,
            },
            Turn::Scissors => match other {
                Turn::Rock => Turn::Scissors as u32,
                Turn::Paper => Turn::Scissors as u32 + 6,
                Turn::Scissors => Turn::Scissors as u32 + 3,
            },
        }
    }

    fn get_turn(&self, other: &Outcome) -> u32 {
        match self {
            Turn::Rock => match other {
                Outcome::Lose => Turn::Scissors as u32,
                Outcome::Draw => Turn::Rock as u32 + 3,
                Outcome::Win => Turn::Paper as u32 + 6,
            },
            Turn::Paper => match other {
                Outcome::Lose => Turn::Rock as u32,
                Outcome::Draw => Turn::Paper as u32 + 3,
                Outcome::Win => Turn::Scissors as u32 + 6,
            },
            Turn::Scissors => match other {
                Outcome::Lose => Turn::Paper as u32,
                Outcome::Draw => Turn::Scissors as u32 + 3,
                Outcome::Win => Turn::Rock as u32 + 6,
            },
        }
    }
}

impl FromStr for Turn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Turn::Rock),
            "B" | "Y" => Ok(Turn::Paper),
            "C" | "Z" => Ok(Turn::Scissors),
            _ => Err("invalid move".to_string()),
        }
    }
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err("invalid move".to_string()),
        }
    }
}

pub fn part1(input: &str) -> String {
    let mut total: u32 = 0;
    input.lines().for_each(|line| {
        let turns: Vec<Turn> = line
            .split(" ")
            .map(|s| s.parse::<Turn>().unwrap())
            .collect();

        total += turns.get(1).unwrap().resolve(&turns.get(0).unwrap());
    });
    total.to_string()
}

pub fn part2(input: &str) -> String {
    let mut total: u32 = 0;
    input.lines().for_each(|line| {
        let mut parts = line.split(" ");
        let opp = parts.next().unwrap().parse::<Turn>().unwrap();
        let res = parts.next().unwrap().parse::<Outcome>().unwrap();
        total += opp.get_turn(&res);
    });
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, "12");
    }
}
