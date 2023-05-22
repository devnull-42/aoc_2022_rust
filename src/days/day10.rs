use std::collections::HashSet;

enum Instruction {
    Noop,
    Addx(i32),
}

struct ClockCircuit {
    cycle: i32,
    register: i32,
    next_val: i32,
    instruction_queue: Vec<Instruction>,
    processing: bool,
    gpu_out: Vec<char>,
}

impl ClockCircuit {
    fn new() -> ClockCircuit {
        return ClockCircuit {
            cycle: 0,
            register: 1,
            next_val: 0,
            instruction_queue: Vec::new(),
            processing: false,
            gpu_out: Vec::new(),
        };
    }

    fn process(&mut self) -> Option<(i32, i32)> {
        let instruction = self.instruction_queue.pop();
        match instruction {
            Some(val) => {
                self.cycle += 1;
                let current_register = self.register.clone();
                if self.processing {
                    self.register += self.next_val;
                    self.next_val = 0;
                    self.processing = false;
                }
                match val {
                    Instruction::Addx(v) => {
                        self.processing = true;
                        self.next_val = v;
                        return Some((self.cycle, self.register));
                    }
                    Instruction::Noop => return Some((self.cycle, current_register)),
                }
            }
            None => return None,
        }
    }

    fn process_gpu(&mut self) -> bool {
        let instruction = self.instruction_queue.pop();
        match instruction {
            Some(val) => {
                let sprite = (self.register - 1)..=(self.register + 1);
                if sprite.contains(&(self.cycle % 40)) {
                    self.gpu_out.push('#');
                } else {
                    self.gpu_out.push('.');
                }

                self.cycle += 1;

                if self.processing {
                    self.register += self.next_val;
                    self.next_val = 0;
                    self.processing = false;
                }

                match val {
                    Instruction::Addx(v) => {
                        self.processing = true;
                        self.next_val = v;
                        return true;
                    }
                    Instruction::Noop => return true,
                }
            }
            None => return false,
        }
    }
}

fn parse_commands(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::<Instruction>::new();
    for line in input.lines() {
        let c = line.split_whitespace().collect::<Vec<&str>>();
        if c[0] == "addx" {
            let val = c[1].parse::<i32>().unwrap();
            instructions.push(Instruction::Addx(val));
        }
        instructions.push(Instruction::Noop);
    }
    instructions.reverse();
    return instructions;
}

pub fn part1(input: &str) -> String {
    let mut clock = ClockCircuit::new();
    let mut checkpoints = HashSet::<i32>::new();
    checkpoints.insert(20);
    checkpoints.insert(60);
    checkpoints.insert(100);
    checkpoints.insert(140);
    checkpoints.insert(180);
    checkpoints.insert(220);

    clock.instruction_queue = parse_commands(input);

    let mut r = 0;

    while let Some(result) = clock.process() {
        if checkpoints.contains(&result.0) {
            r += result.1 * result.0;
        }
    }

    r.to_string()
}

pub fn part2(input: &str) -> String {
    let mut clock = ClockCircuit::new();
    clock.instruction_queue = parse_commands(input);

    while clock.process_gpu() {}
    let lines = clock
        .gpu_out
        .chunks(40)
        .map(|l| l.into_iter().collect::<String>())
        .collect::<Vec<String>>();

    let result = format!("\n{}", lines.join("\n"));

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "13140");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, "\n##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....");
    }
}
