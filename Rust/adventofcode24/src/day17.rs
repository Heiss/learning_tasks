use itertools::Itertools;
use std::str::FromStr;

const REG_A: usize = 0;
const REG_B: usize = 1;
const REG_C: usize = 2;

struct Program {
    register: [isize; 3],
    program: Vec<usize>,
    output: Vec<isize>,
    pointer: usize,
}

impl Program {
    fn exec(&mut self) -> String {
        while self.is_running() {
            self.exec_instruction();
        }
        self.output.iter().map(|&x| x.to_string()).join(",")
    }

    fn is_running(&self) -> bool {
        self.pointer < self.program.len()
    }

    fn get_instruction_and_operand(&self) -> (Instruction, Operand) {
        let instr = Instruction::from(self.program[self.pointer]);
        let op = Operand::new(self.program[self.pointer + 1]);
        (instr, op)
    }

    fn exec_instruction(&mut self) {
        if !self.is_running() {
            return;
        }

        let reg_a = self.register[REG_A];
        let reg_b = self.register[REG_B];
        let reg_c = self.register[REG_C];

        println!(
            "Pointer: {}, Registers: {:?}, Output: {:?}",
            self.pointer, self.register, self.output
        );
        let (instr, op) = self.get_instruction_and_operand();
        match instr {
            Instruction::Adv => {
                self.register[REG_A] = reg_a / 2isize.pow(op.get_combo(&self.register) as u32)
            }
            Instruction::Bxl => self.register[REG_B] = reg_b ^ op.get_literal(),
            Instruction::Bst => self.register[REG_B] = op.get_combo(&self.register) % 8,
            Instruction::Jnz => {
                if reg_a != 0 {
                    self.pointer = (op.get_literal()) as usize;
                    return;
                }
            }
            Instruction::Bxc => self.register[REG_B] = reg_b ^ reg_c,
            Instruction::Out => {
                let val = op.get_combo(&self.register) % 8;
                self.output.push(val);
            }
            Instruction::Bdv => {
                self.register[REG_B] = reg_a / 2isize.pow(op.get_combo(&self.register) as u32)
            }
            Instruction::Cdv => {
                self.register[REG_C] = reg_a / 2isize.pow(op.get_combo(&self.register) as u32)
            }
        }
        self.pointer += 2;
    }
}

fn get_numbers(s: &str) -> Vec<isize> {
    let mut numbers = Vec::new();
    let mut current_nmbr = String::new();
    for c in s.chars() {
        if c.is_digit(10) {
            current_nmbr.push(c);
        } else if !current_nmbr.is_empty() {
            numbers.push(current_nmbr.parse().unwrap());
            current_nmbr.clear();
        }
    }

    if !current_nmbr.is_empty() {
        numbers.push(current_nmbr.parse().unwrap());
    }

    numbers
}

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.lines();
        let mut register = [0; 3];
        let frst_line = it.next().unwrap();
        register[0] = get_numbers(frst_line)[0];
        let scnd_line = it.next().unwrap();
        register[1] = get_numbers(scnd_line)[0];
        let thrd_line = it.next().unwrap();
        register[2] = get_numbers(thrd_line)[0];
        it.next(); // skip empty line

        let mut numbers = Vec::new();
        for line in it {
            numbers.extend(get_numbers(line).iter().map(|&x| x as usize));
        }

        Ok(Program {
            register,
            program: numbers,
            output: Vec::new(),
            pointer: 0,
        })
    }
}

struct Operand {
    value: usize,
}

impl Operand {
    fn new(value: usize) -> Self {
        Operand { value }
    }

    fn get_literal(&self) -> isize {
        self.value as isize
    }

    fn get_combo(&self, reg: &[isize; 3]) -> isize {
        match self.value {
            0..=3 => self.value as isize,
            4 => reg[REG_A],
            5 => reg[REG_B],
            6 => reg[REG_C],
            _ => panic!("Invalid operand"),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<usize> for Instruction {
    fn from(value: usize) -> Self {
        match value {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!("Invalid instruction"),
        }
    }
}

fn part1(input: &str) -> String {
    Program::from_str(input).unwrap().exec()
}

fn part2(input: &str) -> usize {
    0
}

pub fn day() -> String {
    let input = include_str!("../../../input/Rust/adventofcode24/day17.txt");
    format!(
        "Day 17\tPart 1: {}\t Part 2: {}",
        part1(input),
        part2(input)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;
    #[test]
    fn day1() {
        assert_eq!(part1(INPUT), "4,6,3,5,6,3,5,2,1,0");
    }
    #[test]
    fn day2() {
        assert_eq!(part2(INPUT), 117440);
    }
}
