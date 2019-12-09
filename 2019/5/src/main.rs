use std::fmt;
use std::io::Write;
use std::io::{stdin, stdout};

const INPUT_FILE: &str = "input.txt";

fn main() {
    println!("running part 1");
    part1();
}

fn part1() {
    Cpu::new(None).run();
}

#[derive(Debug, Copy, Clone)]
enum Parameter {
    Position(i32),
    Immediate(i32),
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Parameter::Position(v) => write!(f, "P({})", v),
            Parameter::Immediate(v) => write!(f, "I({})", v),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    ADD(Vec<Parameter>),
    MUL(Vec<Parameter>),
    INPUT(Vec<Parameter>),
    OUTPUT(Vec<Parameter>),
    JUMP(bool, Vec<Parameter>),
    LESSTHAN(Vec<Parameter>),
    EQUALS(Vec<Parameter>),
    HALT,
}

struct Cpu {
    ip: usize,
    pub memory: Vec<i32>,
}

impl Cpu {
    fn new(mem: Option<Vec<i32>>) -> Cpu {
        if let Some(memory) = mem {
            Cpu { ip: 0, memory }
        } else {
            Cpu {
                ip: 0,
                memory: process_input(),
            }
        }
    }

    // Build a vector of |cnt| parameters for the instruction based on
    // the flags in the opcode representing the parameter modes.
    fn pack_parameters(&mut self, cnt: usize) -> Vec<Parameter> {
        let mut vec = Vec::new();
        let mut flags = self.memory[self.ip - 1] / 100;
        for i in 0..cnt {
            let val = self.memory[self.ip + i] as i32;
            let param = if flags % 10 == 1 {
                Parameter::Immediate(val)
            } else {
                Parameter::Position(val)
            };
            flags /= 10;
            vec.push(param);
        }
        self.ip += cnt;
        vec
    }

    fn unpack_parameter(&self, p: Parameter) -> i32 {
        let v = match p {
            Parameter::Immediate(x) => x,
            Parameter::Position(x) => self.memory[x as usize],
        };
        v
    }

    fn fetch_and_decode(&mut self) -> Instruction {
        self.ip += 1;
        let opcode = self.memory[self.ip - 1] % 100;
        match opcode {
            1 => Instruction::ADD(self.pack_parameters(3)),
            2 => Instruction::MUL(self.pack_parameters(3)),
            3 => Instruction::INPUT(self.pack_parameters(1)),
            4 => Instruction::OUTPUT(self.pack_parameters(1)),
            5 => Instruction::JUMP(true, self.pack_parameters(2)),
            6 => Instruction::JUMP(false, self.pack_parameters(2)),
            7 => Instruction::LESSTHAN(self.pack_parameters(3)),
            8 => Instruction::EQUALS(self.pack_parameters(3)),
            99 => Instruction::HALT,
            _ => panic!("Invalid opcode: {} at position {}", opcode, self.ip - 1),
        }
    }

    fn run(mut self) -> Cpu {
        while self.ip < self.memory.len() {
            let instruction = self.fetch_and_decode();
            match instruction {
                Instruction::ADD(args) => self.op_add(args),
                Instruction::MUL(args) => self.op_mul(args),
                Instruction::INPUT(args) => self.op_input(args),
                Instruction::OUTPUT(args) => self.op_output(args),
                Instruction::JUMP(test, args) => self.op_jump(test, args),
                Instruction::LESSTHAN(args) => self.op_lessthan(args),
                Instruction::EQUALS(args) => self.op_equals(args),
                Instruction::HALT => break,
            }
        }
        self
    }

    // Instruction implementations
    fn op_add(&mut self, args: Vec<Parameter>) {
        assert_eq!(args.len(), 3);
        if let Parameter::Position(dest) = args[2] {
            self.memory[dest as usize] =
                self.unpack_parameter(args[0]) + self.unpack_parameter(args[1]);
        } else {
            panic!("Dest argument should never be immediate");
        }
    }

    fn op_mul(&mut self, args: Vec<Parameter>) {
        assert_eq!(args.len(), 3);
        if let Parameter::Position(dest) = args[2] {
            self.memory[dest as usize] =
                self.unpack_parameter(args[0]) * self.unpack_parameter(args[1]);
        } else {
            panic!("Dest argument should never be immediate");
        }
    }

    fn op_input(&mut self, args: Vec<Parameter>) {
        assert_eq!(args.len(), 1);

        if let Parameter::Position(dest) = args[0] {
            print!("$ ");
            std::io::stdout().flush().unwrap();
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            self.memory[dest as usize] = buffer.trim().parse().unwrap();
        } else {
            panic!("Dest argument should never be immediate");
        }
    }

    fn op_output(&self, args: Vec<Parameter>) {
        assert_eq!(args.len(), 1);
        println!("> {}", self.unpack_parameter(args[0]));
    }

    fn op_jump(&mut self, test: bool, args: Vec<Parameter>) {
        assert_eq!(args.len(), 2);
        if (self.unpack_parameter(args[0]) != 0) == test {
            self.ip = self.unpack_parameter(args[1]) as usize;
        }
    }

    fn op_lessthan(&mut self, args: Vec<Parameter>) {
        assert_eq!(args.len(), 3);
        if let Parameter::Position(dest) = args[2] {
            self.memory[dest as usize] =
                (self.unpack_parameter(args[0]) < self.unpack_parameter(args[1])) as i32;
        } else {
            panic!("Dest argument should never be immediate");
        }
    }

    fn op_equals(&mut self, args: Vec<Parameter>) {
        assert_eq!(args.len(), 3);
        if let Parameter::Position(dest) = args[2] {
            self.memory[dest as usize] =
                (self.unpack_parameter(args[0]) == self.unpack_parameter(args[1])) as i32;
        } else {
            panic!("Dest argument should never be immediate");
        }
    }
}

fn process_input() -> Vec<i32> {
    std::fs::read_to_string(INPUT_FILE)
        .unwrap()
        .trim()
        .split(',')
        .map(|mass| mass.parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parse() {
        let v = process_input();
        assert_eq!(v[0], 3);
        assert_eq!(v[1], 225);
        assert_eq!(v[2], 1);
        assert_eq!(v[3], 225);
        assert_eq!(v[v.len() - 2], 99);
        assert_eq!(v[v.len() - 1], 226);
    }

    #[test]
    fn example1() {
        {
            let cpu = Cpu::new(Some(vec![1101, 100, -1, 4, 0])).run();
            assert_eq!(cpu.memory[4], 99);
        }
        {
            let cpu = Cpu::new(Some(vec![1002, 4, 3, 4, 33])).run();
            assert_eq!(cpu.memory[4], 99);
        }
    }
}
