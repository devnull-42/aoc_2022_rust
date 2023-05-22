use std::collections::HashSet;

enum MoveCmd {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
    Diag((i32, i32)),
    Stay,
}

struct Rope {
    num_moves: u32,
    sections: Vec<(i32, i32)>,
    tail_positions: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(sections: usize) -> Rope {
        let rope = Rope {
            num_moves: 0,
            sections: vec![(0, 0); sections],
            tail_positions: HashSet::new(),
        };
        return rope;
    }

    fn instruction(&mut self, m: MoveCmd) {
        self.num_moves += 1;
        match m {
            MoveCmd::Up(v) => {
                for _i in 0..v {
                    self.move_section(0, MoveCmd::Up(1));
                    for i in 1..self.sections.len() {
                        let move_command = self.compare(i);
                        self.move_section(i, move_command);
                    }
                }
            }
            MoveCmd::Down(v) => {
                for _i in 0..v {
                    self.move_section(0, MoveCmd::Down(1));
                    for i in 1..self.sections.len() {
                        let move_command = self.compare(i);
                        self.move_section(i, move_command);
                    }
                }
            }
            MoveCmd::Left(v) => {
                for _i in 0..v {
                    self.move_section(0, MoveCmd::Left(1));
                    for i in 1..self.sections.len() {
                        let move_command = self.compare(i);
                        self.move_section(i, move_command);
                    }
                }
            }
            MoveCmd::Right(v) => {
                for _i in 0..v {
                    self.move_section(0, MoveCmd::Right(1));
                    for i in 1..self.sections.len() {
                        let move_command = self.compare(i);
                        self.move_section(i, move_command);
                    }
                }
            }
            _ => {}
        }
    }

    fn move_section(&mut self, section: usize, m: MoveCmd) {
        match m {
            MoveCmd::Up(v) => {
                self.sections[section] = (self.sections[section].0, self.sections[section].1 + v)
            }
            MoveCmd::Down(v) => {
                self.sections[section] = (self.sections[section].0, self.sections[section].1 - v)
            }
            MoveCmd::Left(v) => {
                self.sections[section] = (self.sections[section].0 - v, self.sections[section].1)
            }
            MoveCmd::Right(v) => {
                self.sections[section] = (self.sections[section].0 + v, self.sections[section].1)
            }
            MoveCmd::Diag(v) => {
                self.sections[section] = (
                    self.sections[section].0 + v.0,
                    self.sections[section].1 + v.1,
                )
            }
            MoveCmd::Stay => {}
        }
        if section == self.sections.len() - 1 {
            self.tail_positions
                .insert(self.sections.last().unwrap().clone());
        }
    }

    fn compare(&self, section: usize) -> MoveCmd {
        let x_dist = self.sections[section - 1].0 - self.sections[section].0;
        let y_dist = self.sections[section - 1].1 - self.sections[section].1;

        if x_dist.abs() > 1 && y_dist == 0 {
            if x_dist.is_positive() {
                return MoveCmd::Right(1);
            } else {
                return MoveCmd::Left(1);
            }
        } else if x_dist.abs() == 0 && y_dist.abs() > 1 {
            if y_dist.is_positive() {
                return MoveCmd::Up(1);
            } else {
                return MoveCmd::Down(1);
            }
        } else if x_dist.abs() > 1 && y_dist.abs() >0 || x_dist.abs() >0 && y_dist.abs() > 1 {
            let x = x_dist / x_dist.abs();
            let y = y_dist / y_dist.abs();
            return MoveCmd::Diag((x, y));
        }

        if x_dist.abs() > 2 || y_dist.abs() > 2 {
            dbg!(
                self.num_moves,
                self.sections[section - 1],
                self.sections[section]
            );
        }
        return MoveCmd::Stay;
    }
}

fn parse_command(input: &str) -> MoveCmd {
    let move_cmd = input.split_whitespace().collect::<Vec<&str>>();
    let dist = move_cmd[1].parse::<i32>().unwrap();

    match move_cmd[0] {
        "U" => MoveCmd::Up(dist),
        "D" => MoveCmd::Down(dist),
        "L" => MoveCmd::Left(dist),
        "R" => MoveCmd::Right(dist),
        _ => MoveCmd::Stay,
    }
}

pub fn part1(input: &str) -> String {
    let mut rope = Rope::new(2);

    let lines = input.lines();
    for line in lines {
        let mc = parse_command(line);
        rope.instruction(mc);
    }

    rope.tail_positions.len().to_string()
}

pub fn part2(input: &str) -> String {
    let mut rope = Rope::new(10);

    let lines = input.lines();
    for line in lines {
        let mc = parse_command(line);
        rope.instruction(mc);
    }

    rope.tail_positions.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let result = part1(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn test_part2a() {
        const INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let result = part2(INPUT);
        assert_eq!(result, "36");
    }

    #[test]
    fn test_part2b() {
        const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let result = part2(INPUT);
        assert_eq!(result, "1");
    }
}
