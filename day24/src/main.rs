use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Result;
use itertools::Either;

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

#[derive(Debug)]
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
    fn new() -> Alu {
        Alu {
            reg: HashMap::from([
                (Reg::W, 0),
                (Reg::X, 0),
                (Reg::Y, 0),
                (Reg::Z, 0),
            ]),
        }
    }

    fn right_val(&self, operand: &Either<i64, Reg>) -> i64 {
        match operand {
            Either::Left(n) => *n,
            Either::Right(r) => self.reg[r],
        }
    }

    fn exec<It: Iterator<Item=i64>>(&mut self, ins: &Instruction, input_it: &mut It) {
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

    dbg!(&program);
    let mut alu1 = Alu::new();
    let result = alu1.validate(&program, &[0; 14]);
    dbg!(result);

    Ok(())
}

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
