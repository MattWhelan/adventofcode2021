use std::fmt::{Display, Formatter};
use std::mem;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum SnailNum {
    Reg(u32),
    Pair(Box<SnailNum>, Box<SnailNum>),
}

impl Display for SnailNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailNum::Reg(n) => {
                write!(f, "{}", n)
            }
            SnailNum::Pair(l, r) => {
                write!(f, "[{},{}]", l, r)
            }
        }
    }
}

impl SnailNum {
    fn pair(chs: &[char]) -> (SnailNum, usize) {
        assert_eq!(&'[', &chs[0]);
        let (left, off) = SnailNum::num(&chs[1..]);
        assert_eq!(&',', &chs[off + 1]);
        let (right, off2) = SnailNum::num(&chs[off + 2..]);
        assert_eq!(&']', &chs[off + 2 + off2]);
        (
            SnailNum::Pair(Box::new(left), Box::new(right)),
            off + 3 + off2,
        )
    }

    fn num(chs: &[char]) -> (SnailNum, usize) {
        match &chs[0] {
            '[' => SnailNum::pair(chs),
            '0'..='9' => (SnailNum::Reg(*&chs[0].to_digit(10).unwrap()), 1),
            ch => panic!("Unexpected {}", ch),
        }
    }

    pub fn add(&self, rhs: &Self) -> Self {
        let result = SnailNum::Pair(Box::new(self.clone()), Box::new(rhs.clone()));

        result.reduce()
    }

    fn locate_num<'a>(&'a mut self, opt: &mut Option<&'a mut u32>) {
        match self {
            SnailNum::Reg(n) => {
                opt.replace(n);
            }
            SnailNum::Pair(l, r) => {
                l.locate_num(opt);
                if opt.is_none() {
                    r.locate_num(opt);
                }
            }
        }
    }

    fn find_explode<'a>(
        &'a mut self,
        depth: usize,
        left_num: &mut Option<&'a mut u32>,
        right_num: &mut Option<&'a mut u32>,
    ) -> Option<Box<Self>> {
        let mut ret = match self {
            SnailNum::Reg(n) => {
                left_num.replace(n);
                None
            }
            SnailNum::Pair(_, _) => {
                if depth == 4 {
                    let mut ret = Box::new(SnailNum::Reg(0));
                    mem::swap(self, &mut *ret);
                    Some(ret)
                } else {
                    if let SnailNum::Pair(left, right) = self {
                        let left_explode = left.find_explode(depth + 1, left_num, right_num);

                        if left_explode.is_some() {
                            if right_num.is_none() {
                                right.locate_num(right_num);
                            }
                            left_explode
                        } else {
                            right.find_explode(depth + 1, left_num, right_num)
                        }
                    } else {
                        panic!("WTF")
                    }
                }
            }
        };
        if depth == 0 {
            if let Some(exploding) = &mut ret {
                let expl_ref: &mut SnailNum = &mut *exploding;
                if let SnailNum::Pair(left_box, right_box) = expl_ref {
                    let left_ref: &mut SnailNum = &mut *left_box;
                    let right_ref: &mut SnailNum = &mut *right_box;
                    if let (SnailNum::Reg(left), SnailNum::Reg(right)) = (left_ref, right_ref) {
                        if let Some(left_target) = left_num {
                            **left_target += *left;
                            left_num.take();
                        }
                        if let Some(right_target) = right_num {
                            **right_target += *right;
                            right_num.take();
                        }
                    }
                }
            }
        }
        ret
    }

    fn explode(&mut self) -> bool {
        self.find_explode(0, &mut None, &mut None).is_some()
    }

    fn split(&mut self) -> bool {
        match self {
            SnailNum::Reg(n) if *n >= 10 => {
                let left_half = *n / 2;
                let right_half = *n - left_half;

                *self = SnailNum::Pair(
                    Box::new(SnailNum::Reg(left_half)),
                    Box::new(SnailNum::Reg(right_half)),
                );
                true
            }
            SnailNum::Pair(left, right) => left.split() || right.split(),
            _ => false,
        }
    }

    fn reduce(mut self) -> Self {
        while self.explode() || self.split() {}
        self
    }

    pub fn magnitude(&self) -> u32 {
        match self {
            SnailNum::Reg(n) => *n,
            SnailNum::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl FromStr for SnailNum {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        let chs: Vec<char> = s.chars().collect();
        let (sn, off) = SnailNum::pair(&chs);
        assert_eq!(chs.len(), off);
        Ok(sn)
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    fn parse(s: &str) -> Vec<SnailNum> {
        s.lines().map(|l| l.parse().unwrap()).collect()
    }

    #[test]
    fn test_explode() {
        let mut num: SnailNum = "[[[[[9,8],1],2],3],4]".parse().unwrap();

        assert!(num.explode());
        assert_eq!("[[[[0,9],2],3],4]", &num.to_string());
    }

    #[test]
    fn test_add() {
        let input = r"[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]
";
        let num = parse(input);
        let added = num.into_iter().reduce(|a, b| a.add(&b)).unwrap();

        assert_eq!("[[[[5,0],[7,4]],[5,5]],[6,6]]", &added.to_string())
    }
}
