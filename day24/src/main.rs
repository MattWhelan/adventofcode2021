use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use anyhow::Result;
use itertools::{Either, Itertools};
use rayon::prelude::*;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Reg {
    W,
    X,
    Y,
    Z,
}

impl From<char> for Reg {
    fn from(ch: char) -> Self {
        match ch {
            'w' => Reg::W,
            'x' => Reg::X,
            'y' => Reg::Y,
            'z' => Reg::Z,
            _ => panic!("bad register {}", ch),
        }
    }
}

impl FromStr for Reg {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Reg::from(s.chars().next().unwrap()))
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Instruction {
    INP(Reg),
    ADD(Reg, Either<i64, Reg>),
    MUL(Reg, Either<i64, Reg>),
    DIV(Reg, Either<i64, Reg>),
    MOD(Reg, Either<i64, Reg>),
    EQL(Reg, Either<i64, Reg>),
}

impl Instruction {
    fn parse_right(s: &str) -> Either<i64, Reg> {
        if s.starts_with(|ch: char| ch.is_lowercase()) {
            Either::Right(s.parse().unwrap())
        } else {
            Either::Left(s.parse().unwrap())
        }
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let left: Reg = parts[1].parse().unwrap();
        let ins = match parts[0] {
            "inp" => Instruction::INP(left),
            "add" => Instruction::ADD(left, Instruction::parse_right(parts[2])),
            "mul" => Instruction::MUL(left, Instruction::parse_right(parts[2])),
            "div" => Instruction::DIV(left, Instruction::parse_right(parts[2])),
            "mod" => Instruction::MOD(left, Instruction::parse_right(parts[2])),
            "eql" => Instruction::EQL(left, Instruction::parse_right(parts[2])),

            _ => panic!("unknown instruction {}", parts[0]),
        };
        Ok(ins)
    }
}

struct Alu {
    reg: HashMap<Reg, i64>,
}

impl Alu {
    pub fn new() -> Alu {
        Alu {
            reg: HashMap::from([(Reg::W, 0), (Reg::X, 0), (Reg::Y, 0), (Reg::Z, 0)]),
        }
    }

    pub fn set(&mut self, reg: Reg, n: i64) {
        self.reg.insert(reg, n);
    }

    fn right_val(&self, operand: &Either<i64, Reg>) -> i64 {
        match operand {
            Either::Left(n) => *n,
            Either::Right(r) => self.reg[r],
        }
    }

    fn exec<It: Iterator<Item = i64>>(&mut self, ins: &Instruction, input_it: &mut It) {
        match ins {
            Instruction::INP(left) => {
                let val = input_it.next().unwrap();
                self.reg.insert(*left, val);
            }
            Instruction::ADD(left, right) => {
                let val_l = self.reg[left];
                let val_r = self.right_val(right);
                self.reg.insert(*left, val_l + val_r);
            }
            Instruction::MUL(left, right) => {
                let val_l = self.reg[left];
                let val_r = self.right_val(right);
                self.reg.insert(*left, val_l * val_r);
            }
            Instruction::DIV(left, right) => {
                let val_l = self.reg[left];
                let val_r = self.right_val(right);
                self.reg.insert(*left, val_l / val_r);
            }
            Instruction::MOD(left, right) => {
                let val_l = self.reg[left];
                let val_r = self.right_val(right);
                self.reg.insert(*left, val_l % val_r);
            }
            Instruction::EQL(left, right) => {
                let val_l = self.reg[left];
                let val_r = self.right_val(right);
                self.reg.insert(*left, if val_l == val_r { 1 } else { 0 });
            }
        }
    }

    pub fn run_program(&mut self, prog: &[Instruction], input: &[i64]) {
        let mut inp_it = input.iter().copied();
        for ins in prog {
            self.exec(ins, &mut inp_it);
        }
    }

    pub fn validate(&mut self, prog: &[Instruction], input: &[i64]) -> i64 {
        self.run_program(prog, input);

        self.reg[&Reg::Z]
    }
}

fn main() -> Result<()> {
    let program: Vec<Instruction> = INPUT.lines().map(|l| l.parse().unwrap()).collect();

    let sections: Vec<_> = program
        .split(|ins| *ins == Instruction::INP(Reg::W))
        .filter(|s| !s.is_empty())
        .collect();
    {
        let success_log = pick_lock(&sections, true);
        let input_log = find_inputs(&sections, &success_log);

        //test it
        {
            let mut alu = Alu::new();
            let z = alu.validate(&program, &input_log);
            assert_eq!(0, z);
        }
        println!(
            "Part1: {}",
            input_log.iter().map(|i| i.to_string()).join("")
        );
    }
    {
        let success_log = pick_lock(&sections, false);
        let input_log = find_inputs(&sections, &success_log);
        println!(
            "Part2: {}",
            input_log.iter().map(|i| i.to_string()).join("")
        );
    }

    Ok(())
}

fn pick_lock(sections: &Vec<&[Instruction]>, get_max: bool) -> Vec<HashMap<i64, i64>> {
    let mut targets = HashSet::new();
    targets.insert(0);
    let mut success_log = Vec::with_capacity(sections.len());
    for section in sections.iter().rev() {
        let successes = solve_segment(section, &targets, get_max);
        assert!(!successes.is_empty());
        targets = successes.keys().copied().collect();
        success_log.push(successes);
        println!(".");
    }
    println!("\nPicked the lock.");
    success_log.reverse();
    success_log
}

fn find_inputs(sections: &Vec<&[Instruction]>, success_log: &Vec<HashMap<i64, i64>>) -> Vec<i64> {
    let mut z = 0;
    let mut input_log = Vec::new();
    for (i, success_map) in success_log.iter().enumerate() {
        let inp = success_map[&z];
        input_log.push(inp);
        let mut alu = Alu::new();
        alu.set(Reg::Z, z);
        alu.set(Reg::W, inp);
        z = alu.validate(&sections[i], &[inp]);
    }
    input_log
}

fn solve_segment(
    program: &[Instruction],
    targets: &HashSet<i64>,
    get_max: bool,
) -> HashMap<i64, i64> {
    let inputs: Vec<_> = if get_max {
        (1..=9).rev().collect()
    } else {
        (1..=9).collect()
    };

    let z_limit = targets.iter().max().unwrap() * 50 + 50;
    (0..z_limit)
        .into_par_iter()
        .filter_map(|z| {
            inputs
                .iter()
                .filter_map(|inp| {
                    let mut alu = Alu::new();
                    alu.set(Reg::Z, z);
                    alu.set(Reg::W, *inp);
                    let result = alu.validate(program, &[*inp]);
                    if targets.contains(&result) {
                        Some((z, *inp))
                    } else {
                        None
                    }
                })
                .next()
        })
        .collect()
}

//Notes: w is only written for input
//x and y are cleared before use in each section
//z is the only register that survives across inputs.
const INPUT: &str = r#"inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 15
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 16
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 4
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 14
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 1
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 5
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 1
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -3
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 3
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 3
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -4
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 7
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 5
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -5
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 3
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y
"#;
