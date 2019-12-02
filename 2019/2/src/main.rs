const INPUT_FILE: &str = "input.txt";

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}

fn part1() -> usize {
    let mut input = process_input();
    // Modifications per the question
    input[1] = 12;
    input[2] = 2;
    let cpu = Cpu::new(Some(input)).run();
    cpu.insts[0]
}

const PART2_GOAL: usize = 19_690_720;
fn part2() -> usize {
    let input = process_input();
    // Modifications per the question
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut memory = input.clone();
            memory[1] = noun;
            memory[2] = verb;

            let cpu = Cpu::new(Some(memory)).run();
            if cpu.insts[0] == PART2_GOAL {
                return 100 * noun + verb;
            }
        }
    }
    panic!("Never found the target {}", PART2_GOAL);
}

enum Instruction {
    ADD,
    MUL,
    HALT,
}

struct Cpu {
    ip: usize,
    pub insts: Vec<usize>,
}

impl Cpu {
    const INSTRUCTION_WIDTH: usize = 4;
    fn op_to_instruction(op: usize) -> Instruction {
        match op {
            1 => Instruction::ADD,
            2 => Instruction::MUL,
            99 => Instruction::HALT,
            _ => panic!(),
        }
    }

    fn new(mut insts: Option<Vec<usize>>) -> Cpu {
        if let Some(insts) = insts.take() {
            Cpu { ip: 0, insts }
        } else {
            Cpu {
                ip: 0,
                insts: process_input(),
            }
        }
    }

    fn add(&mut self, arg1: usize, arg2: usize, dest: usize) {
        self.insts[dest] = self.insts[arg1] + self.insts[arg2];
    }

    fn mul(&mut self, arg1: usize, arg2: usize, dest: usize) {
        self.insts[dest] = self.insts[arg1] * self.insts[arg2];
    }

    fn run(mut self) -> Cpu {
        while self.ip < self.insts.len() {
            let instruction = Cpu::op_to_instruction(self.insts[self.ip]);
            match instruction {
                Instruction::ADD | Instruction::MUL => {
                    let arg1 = self.insts[self.ip + 1];
                    let arg2 = self.insts[self.ip + 2];
                    let dest = self.insts[self.ip + 3];
                    match instruction {
                        Instruction::ADD => self.add(arg1, arg2, dest),
                        Instruction::MUL => self.mul(arg1, arg2, dest),
                        _ => panic!(),
                    }
                }
                Instruction::HALT => break,
            };
            self.ip += Cpu::INSTRUCTION_WIDTH;
        }
        self
    }
}

fn process_input() -> Vec<usize> {
    std::fs::read_to_string(INPUT_FILE)
        .unwrap()
        .trim()
        .split(',')
        .map(|mass| mass.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const PART1_ANSWER: usize = 4_330_636;
    const PART2_ANSWER: usize = 6086;

    #[test]
    fn input_parse() {
        let v = process_input();
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 0);
        assert_eq!(v[2], 0);
        assert_eq!(v[3], 3);
        assert_eq!(v[v.len() - 3], 14);
        assert_eq!(v[v.len() - 1], 0);
    }

    #[test]
    fn example1() {
        {
            let cpu = Cpu::new(Some(vec![1, 0, 0, 0, 99])).run();
            assert_eq!(cpu.insts[0], 2);
        }
        {
            let cpu = Cpu::new(Some(vec![2, 3, 0, 3, 99])).run();
            assert_eq!(cpu.insts[3], 6);
        }
        {
            let cpu = Cpu::new(Some(vec![2, 4, 4, 5, 99, 0])).run();
            assert_eq!(cpu.insts[5], 9801);
        }
        {
            let cpu = Cpu::new(Some(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])).run();
            assert_eq!(cpu.insts[0], 30);
            assert_eq!(cpu.insts[4], 2);
        }
    }

    #[test]
    fn part1_regression() {
        assert_eq!(part1(), PART1_ANSWER);
    }

    #[test]
    fn part2_regression() {
        assert_eq!(part2(), PART2_ANSWER);
    }

}
