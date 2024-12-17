use std::{collections::HashMap, fmt::Display, str::FromStr};

use derive_more::derive::From;
use itertools::Itertools;
use tracing::Level;
use utils::prelude::*;

#[macro_use]
extern crate tramp;
use tramp::{tramp, BorrowRec};

fn main() -> Result<()> {
    init_tracing();

    let input = read_input()?;

    let (reg_a, reg_b, reg_c, _, instructions) = input
        .lines()
        .collect_tuple()
        .ok_or_else(|| parse_error("not enough lines", &input))?;

    let reg_a = reg_a
        .strip_prefix("Register A: ")
        .ok_or_else(|| parse_error("could not parse reg A", reg_a))?
        .parse()?;

    let reg_b = reg_b
        .strip_prefix("Register B: ")
        .ok_or_else(|| parse_error("could not parse reg B", reg_b))?
        .parse()?;

    let reg_c = reg_c
        .strip_prefix("Register C: ")
        .ok_or_else(|| parse_error("could not parse reg C", reg_c))?
        .parse()?;

    let registers = [('A', reg_a), ('B', reg_b), ('C', reg_c)]
        .into_iter()
        .collect();

    let instructions: Vec<_> = instructions
        .strip_prefix("Program: ")
        .ok_or_else(|| parse_error("could not parse program", instructions))?
        .split(',')
        .map(|s| s.parse())
        .try_collect()?;

    let computer = Computer {
        registers,
        instructions,
        pointer: 0,
    };

    print_part_1(&part_one(computer.clone()).map(|out| out.into_iter().join(",")));
    print_part_2(&part_two(computer));

    Ok(())
}

#[tracing::instrument(level=Level::DEBUG,skip(computer))]
fn part_one(mut computer: Computer) -> Result<Vec<u64>> {
    computer.run()
}

#[tracing::instrument(level=Level::DEBUG,skip(computer))]
fn part_two(mut computer: Computer) -> Result<u64> {
    fn solve_position(computer: &mut Computer, n: usize, offset: u64) -> Option<u64> {
        (0..8).find_map(|i| {
            let j = offset + i * 8_u64.pow(n as u32);
            computer.reset(j, 0, 0);
            match computer.run() {
                Ok(out) if out.get(n) == computer.instructions.get(n) => {
                    tracing::debug!(
                        position = n,
                        offset = offset,
                        "found solution: {} ({} {:03b})",
                        j,
                        i,
                        i
                    );
                    if n == 0 {
                        Some(j)
                    } else {
                        solve_position(computer, n - 1, j)
                    }
                }
                _ => None,
            }
        })
    }

    let n = computer.instructions.len() - 1;
    solve_position(&mut computer, n, 0)
        .ok_or_else(|| Error::SolutionNotFound("no solution found".to_owned()))
}

#[derive(Debug, From)]
struct Operand(u64);

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
enum Instruction {
    DIV(char), // Division, char <- A / 2^O
    BXL,       // Bitwise XOR, B <- B XOR O
    BST,       // B <- O % 8
    JNZ,       // if A == 0 { NOP } else { JUMP O }
    BXC,       // B <- B XOR C
    OUT,       // PRINT O % 8
}

impl TryFrom<u64> for Instruction {
    type Error = Error;

    fn try_from(value: u64) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DIV('A')),
            1 => Ok(Self::BXL),
            2 => Ok(Self::BST),
            3 => Ok(Self::JNZ),
            4 => Ok(Self::BXC),
            5 => Ok(Self::OUT),
            6 => Ok(Self::DIV('B')),
            7 => Ok(Self::DIV('C')),
            _ => Err(parse_error(
                "could not parse instruction",
                &format!("{:?}", value),
            )),
        }
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.chars().next() {
            Some(c) if c.is_ascii_digit() => {
                let n: u64 = c.to_digit(10).unwrap() as u64;
                n.try_into()
            }
            Some(c) => Err(parse_error("could not parse string", &format!("{c:?}"))),
            None => Err(parse_error("could not parse empty string", "")),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::DIV(c) => write!(f, "{}dv", c.to_ascii_lowercase()),
            Instruction::BXL => write!(f, "bxl"),
            Instruction::BST => write!(f, "bst"),
            Instruction::JNZ => write!(f, "jnz"),
            Instruction::BXC => write!(f, "bxc"),
            Instruction::OUT => write!(f, "out"),
        }
    }
}

#[derive(Clone)]
struct Computer {
    pointer: usize,
    registers: HashMap<char, u64>,
    instructions: Vec<u64>,
}

impl Computer {
    fn reset(&mut self, reg_a: u64, reg_b: u64, reg_c: u64) {
        self.registers.insert('A', reg_a);
        self.registers.insert('B', reg_b);
        self.registers.insert('C', reg_c);
        self.pointer = 0;
    }

    fn run(&mut self) -> Result<Vec<u64>> {
        fn inner(computer: &mut Computer, mut acc: Vec<u64>) -> BorrowRec<Result<Vec<u64>>> {
            match computer.step() {
                Ok((cont, out)) => {
                    if let Some(out) = out {
                        acc.push(out);
                    }

                    if cont {
                        rec_call!(inner(computer, acc))
                    } else {
                        rec_ret!(Ok(acc))
                    }
                }
                Err(e) => rec_ret!(Err(e)),
            }
        }

        tramp(inner(self, vec![]))
    }

    fn step(&mut self) -> Result<(bool, Option<u64>)> {
        match self
            .instructions
            .get(self.pointer)
            .zip(self.instructions.get(self.pointer + 1))
        {
            Some((&instruction, &operand)) => {
                let (pointer, output, changes) =
                    self.handle(&instruction.try_into()?, &operand.into());
                tracing::trace!( "A: {} B: {} C: {} executing {:03}: {}( {} ) -> output: {:?}, changes: {:?}, pointer: {:03}", self.register(&'A'), self.register(&'B'), self.register(&'C'), self.pointer, instruction, operand, output, changes, pointer );
                changes.into_iter().for_each(|(reg, val)| {
                    self.registers.insert(reg, val);
                });
                self.pointer = pointer;
                Ok((true, output))
            }
            None => Ok((false, None)),
        }
    }

    fn handle(
        &self,
        instruction: &Instruction,
        operand: &Operand,
    ) -> (usize, Option<u64>, Vec<(char, u64)>) {
        match instruction {
            Instruction::DIV(reg) => {
                let result = self.register(&'A') / 2_u64.pow(self.combo(operand) as u32);
                (self.pointer + 2, None, vec![(*reg, result)])
            }
            Instruction::BXL => {
                let result = self.register(&'B') ^ operand.0;
                (self.pointer + 2, None, vec![('B', result)])
            }
            Instruction::BST => {
                let result = self.combo(operand) % 8;
                (self.pointer + 2, None, vec![('B', result)])
            }
            Instruction::JNZ => {
                if self.register(&'A') == 0 {
                    (self.pointer + 2, None, vec![])
                } else {
                    (operand.0.try_into().unwrap(), None, vec![])
                }
            }
            Instruction::BXC => {
                let result = self.register(&'B') ^ self.register(&'C');
                (self.pointer + 2, None, vec![('B', result)])
            }
            Instruction::OUT => (self.pointer + 2, Some(self.combo(operand) % 8), vec![]),
        }
    }

    fn register(&self, r: &char) -> u64 {
        *self.registers.get(r).unwrap()
    }

    fn combo(&self, operand: &Operand) -> u64 {
        match operand {
            &Operand(v) if v < 4 => v,
            Operand(4) => self.register(&'A'),
            Operand(5) => self.register(&'B'),
            Operand(6) => self.register(&'C'),
            Operand(_) => unreachable!(),
        }
    }
}
